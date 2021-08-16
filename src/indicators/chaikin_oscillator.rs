#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, MovingAverageConstructor, OHLCV, PeriodType};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
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
pub struct ChaikinOscillator<M: MovingAverageConstructor = MA> {
	pub ma1: M,
	pub ma2: M,
	/*
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
	*/
	/// [AD index](https://en.wikipedia.org/wiki/Accumulation/distribution_index) size. Default is 0 (windowless)
	///
	/// Range in \[`0`; [`PeriodType::MAX`](crate::core::PeriodType)\]
	pub window: PeriodType, // from 0 to ...
}

impl<M: MovingAverageConstructor> IndicatorConfig for ChaikinOscillator<M> {
	type Instance = ChaikinOscillatorInstance<M>;

	const NAME: &'static str = "ChaikinOscillator";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let adi = ADI::new(cfg.window, candle)?;

		Ok(Self::Instance {
			ma1: cfg.ma1.init(adi.get_value())?, //method(cfg.method, cfg.period1, adi.get_value())?,
			ma2: cfg.ma2.init(adi.get_value())?, // method(cfg.method, cfg.period2, adi.get_value())?,
			adi,
			cross_over: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.ma1.is_similar_to(&self.ma2) && self.ma1.ma_period() > 0 && self.ma1.ma_period() < self.ma2.ma_period() && self.ma2.ma_period() < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"ma1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma1 = value,
			},
			"ma2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma2 = value,
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

impl Default for ChaikinOscillator<MA> {
	fn default() -> Self {
		Self {
			// period1: 3,
			// period2: 10,
			ma1: MA::EMA(3),
			ma2: MA::EMA(10),
			// method: RegularMethods::EMA,
			window: 0,
		}
	}
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChaikinOscillatorInstance<M: MovingAverageConstructor = MA> {
	cfg: ChaikinOscillator<M>,

	adi: ADI,
	ma1: M::Instance,
	ma2: M::Instance,
	cross_over: Cross,
}

impl<M: MovingAverageConstructor> IndicatorInstance for ChaikinOscillatorInstance<M> {
	type Config = ChaikinOscillator<M>;

	// type Input = dyn OHLCV;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let dyn_candle: &dyn OHLCV = candle;
		let adi = self.adi.next(dyn_candle);

		let data1 = self.ma1.next(&adi);
		let data2 = self.ma2.next(&adi);

		let value = data1 - data2;

		let signal = self.cross_over.next(&(value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
