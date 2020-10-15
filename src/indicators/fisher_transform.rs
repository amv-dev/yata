#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, Highest, Lowest};

// https://www.investopedia.com/terms/f/fisher-transform.asp
// FT = 1/2 * ln((1+x)/(1-x)) = arctanh(x)
// x - transformation of price to a level between -1 and 1 for N periods

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FisherTransform {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub zone: ValueType,
	pub delta: PeriodType,
	pub method: RegularMethods,
	pub source: Source,
}

impl IndicatorConfig for FisherTransform {
	const NAME: &'static str = "FisherTransform";

	fn validate(&self) -> bool {
		self.period1 >= 3 && self.delta >= 1 && self.period2 >= 1
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"zone" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
			},
			"delta" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.delta = value,
			},
			"method" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(2, 2)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for FisherTransform {
	type Instance = FisherTransformInstance;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			ma1: method(cfg.method, cfg.period2, 0.)?,
			highest: Highest::new(cfg.period1, src)?,
			lowest: Lowest::new(cfg.period1, src)?,
			cross_over: Cross::default(),
			extreme: 0,
			prev_value: 0.,
			cfg,
		})
	}
}

impl Default for FisherTransform {
	fn default() -> Self {
		Self {
			period1: 9,
			period2: 1,
			zone: 1.5,
			delta: 1,
			method: RegularMethods::SMA,
			source: Source::TP,
		}
	}
}

#[derive(Debug)]
pub struct FisherTransformInstance {
	cfg: FisherTransform,

	ma1: RegularMethod,
	highest: Highest,
	lowest: Lowest,
	cross_over: Cross,
	extreme: i8,
	prev_value: ValueType,
}

const BOUND: ValueType = 0.999;

#[inline]
fn bound_value(value: ValueType) -> ValueType {
	value.min(BOUND).max(-BOUND)
}

impl<T: OHLC> IndicatorInstance<T> for FisherTransformInstance {
	type Config = FisherTransform;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		// converting original value to between -1.0 and 1.0 over period1
		let h = self.highest.next(src);
		let l = self.lowest.next(src);
		let is_different = ((h - l).abs() > ValueType::EPSILON) as i8 as ValueType;
		let v1 = is_different * (src * 2. - (h + l)) / (h - l + 1. - is_different);

		// bound value
		let bound_val = bound_value(v1);

		// calculating fisher transform value
		let fisher_transform: ValueType = bound_val.atanh(); //((1.0 + v2)/(1.0-v2)).ln();

		// if fisher_transform > self.cfg.zone {
		// 	self.extreme = -1;
		// } else if fisher_transform < self.cfg.zone {
		// 	self.extreme = 1;
		// }
		self.extreme =
			(fisher_transform < self.cfg.zone) as i8 - (fisher_transform > self.cfg.zone) as i8;

		let s1;
		{
			// We’ll take trade signals based on the following rules:
			// Long trades

			// 	Fisher Transform must be negative (i.e., the more negative the indicator is, the more “stretched” or excessively bearish price is)
			// 	Taken after a reversal of the Fisher Transform from negatively sloped to positively sloped (i.e., rate of change from negative to positive)

			// Short trades

			// 	Fisher Transform must be positive (i.e., price perceived to be excessively bullish)
			// 	Taken after a reversal in the direction of the Fisher Transform
			// s1 = if self.extreme == 1 && fisher_transform - self.prev_value < 0. {
			// 	-1
			// } else if self.extreme == -1 && fisher_transform - self.prev_value > 0. {
			// 	1
			// } else {
			// 	0
			// };
			s1 = (self.extreme == -1 && fisher_transform - self.prev_value > 0.) as i8
				- (self.extreme == 1 && fisher_transform - self.prev_value < 0.) as i8;
		}

		self.prev_value = fisher_transform;

		let s2;
		let fisher_transform_ma: ValueType;
		{
			// The Fisher Transform frequently has a signal line attached to it. This is a moving average of the Fisher Transform value,
			// so it moves slightly slower than the Fisher Transform line. When the Fisher Transform crosses the trigger line it is used
			// by some traders as a trade signal. For example, when the Fisher Transform drops below the signal line after hitting an
			// extreme high, that could be used as a signal to sell a current long position.
			fisher_transform_ma = self.ma1.next(fisher_transform);
			let cross = self
				.cross_over
				.next((fisher_transform, fisher_transform_ma))
				.analog();
			// s2 = if cross * self.extreme == 1 { cross } else { 0 };
			s2 = ((cross * self.extreme) == 1) as i8 * cross;
		}

		IndicatorResult::new(
			&[fisher_transform, fisher_transform_ma],
			&[Action::from(s1), Action::from(s2)],
		)
	}
}
