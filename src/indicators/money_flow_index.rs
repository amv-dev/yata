#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{CrossAbove, CrossUnder};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MoneyFlowIndex {
	pub period: PeriodType,
	pub zone: ValueType,
}

impl IndicatorConfig for MoneyFlowIndex {
	const NAME: &'static str = "MoneyFlowIndex";

	fn validate(&self) -> bool {
		self.zone >= 0. && self.zone <= 0.5
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

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn is_volume_based(&self) -> bool {
		true
	}

	fn size(&self) -> (u8, u8) {
		(3, 1)
	}
}

impl<T: OHLCV> IndicatorInitializer<T> for MoneyFlowIndex {
	type Instance = MoneyFlowIndexInstance<T>;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			window: Window::new(cfg.period, candle),
			prev_candle: candle,
			last_prev_candle: candle,
			pmf: 0.,
			nmf: 0.,
			cross_under: CrossUnder::default(),
			cross_above: CrossAbove::default(),
			cfg,
		})
	}
}

impl Default for MoneyFlowIndex {
	fn default() -> Self {
		Self {
			period: 14,
			zone: 0.2,
		}
	}
}

#[derive(Debug)]
pub struct MoneyFlowIndexInstance<T: OHLCV> {
	cfg: MoneyFlowIndex,

	window: Window<T>,
	prev_candle: T,
	last_prev_candle: T,
	pmf: ValueType,
	nmf: ValueType,
	cross_under: CrossUnder,
	cross_above: CrossAbove,
}

#[inline]
fn tfunc<T: OHLCV>(candle: &T, last_candle: &T) -> (ValueType, ValueType) {
	let tp1 = candle.tp();
	let tp2 = last_candle.tp();

	// if tp1 < tp2 {
	// 	(0., tp1 * candle.volume())
	// } else if tp1 > tp2 {
	// 	(tp1 * candle.volume(), 0.)
	// } else {
	// 	(0., 0.)
	// }

	(
		(tp1 > tp2) as i8 as ValueType * candle.volume(),
		(tp1 < tp2) as i8 as ValueType * candle.volume(),
	)
}

impl<T: OHLCV> IndicatorInstance<T> for MoneyFlowIndexInstance<T> {
	type Config = MoneyFlowIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let (pos, neg) = tfunc(&candle, &self.prev_candle);

		let last_candle = self.window.push(candle);
		let (left_pos, left_neg) = tfunc(&last_candle, &self.last_prev_candle);

		self.last_prev_candle = last_candle;
		self.prev_candle = candle;

		self.pmf += pos - left_pos;
		self.nmf += neg - left_neg;

		let mfr;
		if self.nmf != 0.0 {
			mfr = self.pmf / self.nmf;
		} else {
			mfr = 1.;
		}

		let value = 1. - (1. + mfr).recip();

		let upper = self.cfg.zone;
		let lower = 1. - self.cfg.zone;

		let cross_under = self.cross_under.next((value, self.cfg.zone));
		let cross_above = self.cross_above.next((value, 1. - self.cfg.zone));

		let signal = cross_under - cross_above;

		IndicatorResult::new(&[upper, value, lower], &[signal])
	}
}
