#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, ADI};

/// Chaikin Oscillator
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Chaikin_Analytics>
///
/// # 1 value
///
/// * `oscillator` value
///
/// Range in \[`-1.0`; `1.0`\]
///
/// # 1 signal
///
/// When `oscillator` value goes above zero, then returns full buy signal.
/// When `oscillator` value goes below zero, then returns full sell signal.
/// Otherwise no signal
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChaikinOscillator {
	/// Short period for smoothing [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index). Default is 3.
	///
	/// Range in \[`1`; `period2`\)
	pub period1: PeriodType,
	/// Long period for smoothing [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index). Default is 10.
	///
	/// Range in \(`period1`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period2: PeriodType,
	/// Method for smoothing [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index). Default is [`EMA`](crate::methods::EMA).
	pub method: RegularMethods,
	/// [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index) size. Default is 0 (windowless)
	///
	/// Range in \[`0`; [`PeriodType::MAX`](crate::core::PeriodType)\]
	pub window: PeriodType, // from 0 to ...
}

impl IndicatorConfig for ChaikinOscillator {
	type Instance = ChaikinOscillatorInstance;

	const NAME: &'static str = "ChaikinOscillator";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let c: &dyn OHLCV = candle;
		let adi = ADI::new(cfg.window, c)?;

		Ok(Self::Instance {
			ma1: method(cfg.method, cfg.period1, adi.get_value())?,
			ma2: method(cfg.method, cfg.period2, adi.get_value())?,
			adi,
			cross_over: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period1 > 0 && self.period1 < self.period2 && self.period2 < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"method" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
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
pub struct ChaikinOscillatorInstance {
	cfg: ChaikinOscillator,

	adi: ADI,
	ma1: RegularMethod,
	ma2: RegularMethod,
	cross_over: Cross,
}

impl IndicatorInstance for ChaikinOscillatorInstance {
	type Config = ChaikinOscillator;

	// type Input = dyn OHLCV;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV + 'static>(&mut self, candle: &T) -> IndicatorResult {
		let adi = &self.adi.next(candle);

		let data1 = self.ma1.next(adi);
		let data2 = self.ma2.next(adi);

		let value = data1 - data2;

		let signal = self.cross_over.next(&(value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
