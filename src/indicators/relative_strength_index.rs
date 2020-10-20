#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Change, CrossAbove, CrossUnder};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelativeStrengthIndex {
	pub period: PeriodType,
	pub zone: ValueType,
	pub source: Source,
	pub method: RegularMethods,
}

impl IndicatorConfig for RelativeStrengthIndex {
	const NAME: &'static str = "RelativeStrengthIndex";

	fn validate(&self) -> bool {
		self.period > 2 && self.zone > 0. && self.zone <= 0.5
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"zone" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},
			"method" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for RelativeStrengthIndex {
	type Instance = RelativeStrengthIndexInstance;

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
			change: Change::new(1, src)?,
			posma: method(cfg.method, cfg.period, 0.)?,
			negma: method(cfg.method, cfg.period, 0.)?,
			cross_above: CrossAbove::default(),
			cross_under: CrossUnder::default(),
			cfg,
		})
	}
}

impl Default for RelativeStrengthIndex {
	fn default() -> Self {
		Self {
			period: 14,
			zone: 0.3,
			method: RegularMethods::RMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct RelativeStrengthIndexInstance {
	cfg: RelativeStrengthIndex,

	change: Change,
	posma: RegularMethod,
	negma: RegularMethod,
	cross_above: CrossAbove,
	cross_under: CrossUnder,
}

/// Just an alias for RelativeStrengthIndex
pub type RSI = RelativeStrengthIndex;

impl<T: OHLC> IndicatorInstance<T> for RelativeStrengthIndexInstance {
	type Config = RelativeStrengthIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let change = self.change.next(src);
		let pos: ValueType = self.posma.next(change.max(0.));
		let neg: ValueType = self.negma.next(change.min(0.)) * -1.;

		let value;
		if pos != 0. || neg != 0. {
			debug_assert!(pos + neg != 0.);
			value = pos / (pos + neg)
		} else {
			value = 0.;
		}

		let oversold = self.cross_under.next((value, self.cfg.zone));
		let overbought = self.cross_above.next((value, 1. - self.cfg.zone));
		let signal = oversold - overbought;

		IndicatorResult::new(&[value], &[signal])
	}
}
