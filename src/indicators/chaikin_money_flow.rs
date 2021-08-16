#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::{Cross, ADI};

/// Chaikin Money Flow
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Chaikin_Analytics>
///
/// # 1 value
///
/// * `main` value
///
/// Range in \[`-1.0`; `1.0`\]
///
/// # 1 signal
///
/// When `main` value goes above zero, then returns full buy signal.
/// When `main` value goes below zero, then returns full sell signal.
/// Otherwise no signal
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChaikinMoneyFlow {
	/// main length size. Default is `20`
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub size: PeriodType,
}

impl IndicatorConfig for ChaikinMoneyFlow {
	type Instance = ChaikinMoneyFlowInstance;

	const NAME: &'static str = "ChaikinMoneyFlow";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			adi: ADI::new(cfg.size, candle)?,
			vol_sum: candle.volume() * cfg.size as ValueType,
			window: Window::new(cfg.size, candle.volume()),
			cross_over: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.size > 1 && self.size < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"size" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.size = value,
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

impl Default for ChaikinMoneyFlow {
	fn default() -> Self {
		Self { size: 20 }
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChaikinMoneyFlowInstance {
	cfg: ChaikinMoneyFlow,

	adi: ADI,
	vol_sum: ValueType,
	window: Window<ValueType>,
	cross_over: Cross,
}

impl IndicatorInstance for ChaikinMoneyFlowInstance {
	type Config = ChaikinMoneyFlow;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let adi = self.adi.next(candle);
		self.vol_sum += candle.volume() - self.window.push(candle.volume());
		let value = adi / self.vol_sum;
		let signal = self.cross_over.next(&(value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
