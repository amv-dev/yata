#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{ReverseSignal, HMA};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HullMovingAverage {
	pub period: PeriodType,
	pub left: PeriodType,
	pub right: PeriodType,
	pub source: Source,
}

impl IndicatorConfig for HullMovingAverage {
	const NAME: &'static str = "HullMovingAverage";

	fn validate(&self) -> bool {
		self.period > 2 && self.left >= 1 && self.right >= 1
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"left" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.left = value,
			},
			"right" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.right = value,
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
		(1, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for HullMovingAverage {
	type Instance = HullMovingAverageInstance;

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
			hma: HMA::new(cfg.period, src)?,
			pivot: ReverseSignal::new(cfg.left, cfg.right, src)?,
			cfg,
		})
	}
}

impl Default for HullMovingAverage {
	fn default() -> Self {
		Self {
			period: 9,
			left: 3,
			right: 2,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct HullMovingAverageInstance {
	cfg: HullMovingAverage,

	hma: HMA,
	pivot: ReverseSignal,
}

impl<T: OHLC> IndicatorInstance<T> for HullMovingAverageInstance {
	type Config = HullMovingAverage;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let value = self.hma.next(candle.source(self.cfg.source));
		let signal = self.pivot.next(value);

		IndicatorResult::new(&[value], &[signal])
	}
}
