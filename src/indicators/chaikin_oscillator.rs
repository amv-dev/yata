#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, ADI};

/// [Chaikin Oscillator](https://en.wikipedia.org/wiki/Chaikin_Analytics)
///
/// # 1 value
///
/// * oscillator value [-1.0; 1.0]
///
/// # 1 digital signal
///
/// When `oscillator` value goes above zero, then returns full buy signal.
/// When `oscillator` value goes below zero, then returns full sell signal.
/// Otherwise no signal
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChaikinOscillator {
	/// Short period for smoothing [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index). Default is 3 [1; period2)
	pub period1: PeriodType,
	/// Long period for smoothing [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index). Default is 10 (period1; ...)
	pub period2: PeriodType,
	/// Method for smoothing [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index). Default is EMA.
	pub method: RegularMethods,
	/// [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index) size. Default is 0 (windowless) [0; ...)
	pub window: PeriodType, // from 0 to ...
}

impl IndicatorConfig for ChaikinOscillator {
	const NAME: &'static str = "ChaikinOscillator";

	fn validate(&self) -> bool {
		self.period1 < self.period2
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

	fn is_volume_based(&self) -> bool {
		true
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl<T: OHLCV> IndicatorInitializer<T> for ChaikinOscillator {
	type Instance = ChaikinOscillatorInstance<T>;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let adi = ADI::new(cfg.window, candle)?;

		Ok(Self::Instance {
			ma1: method(cfg.method, cfg.period1, adi.get_value())?,
			ma2: method(cfg.method, cfg.period2, adi.get_value())?,
			adi,
			cross_over: Cross::default(),
			cfg,
		})
	}
}

impl Default for ChaikinOscillator {
	fn default() -> Self {
		Self {
			period1: 3,
			period2: 10,
			method: RegularMethods::EMA,
			window: 0,
		}
	}
}

#[derive(Debug)]
pub struct ChaikinOscillatorInstance<T: OHLCV> {
	cfg: ChaikinOscillator,

	adi: ADI<T>,
	ma1: RegularMethod,
	ma2: RegularMethod,
	cross_over: Cross,
}

impl<T: OHLCV> IndicatorInstance<T> for ChaikinOscillatorInstance<T> {
	type Config = ChaikinOscillator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let adi = self.adi.next(candle);

		let data1 = self.ma1.next(adi);
		let data2 = self.ma2.next(adi);

		let value = data1 - data2;

		let signal = self.cross_over.next((value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
