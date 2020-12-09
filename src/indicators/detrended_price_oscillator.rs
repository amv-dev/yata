#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, PeriodType, Source, ValueType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};

// The Formula for the Detrended Price Oscillator (DPO) is
// DPO=Price from X2+1 periods ago−X period SMA
// where:
// X = Number of periods used for the look-back period
// SMA = Simple Moving Average \begin{aligned}
// &DPO=Price from X/2+1 periods ago - X period SMA

/// Detrended Price Oscillator
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Detrended_price_oscillator>
///
/// # 1 value
///
/// * `DPO` value
///
/// Range in \(`-inf`; `+inf`\).
///
/// # Has no signals
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DetrendedPriceOscillator {
	/// MA period size. Default is `21`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period: PeriodType,

	/// MA method type. Default is [`SMA`](crate::methods::SMA)
	pub method: RegularMethods,

	/// Source type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for DetrendedPriceOscillator {
	type Instance = DetrendedPriceOscillatorInstance;

	const NAME: &'static str = "DetrendedPriceOscillator";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);
		Ok(Self::Instance {
			sma: method(cfg.method, cfg.period, src)?,
			window: Window::new(cfg.period / 2 + 1, src),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period > 1 && self.period < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"method" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
			},
			"source" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(1, 0)
	}
}

impl Default for DetrendedPriceOscillator {
	fn default() -> Self {
		Self {
			period: 21,
			method: RegularMethods::SMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct DetrendedPriceOscillatorInstance {
	cfg: DetrendedPriceOscillator,

	sma: RegularMethod,
	window: Window<ValueType>,
}

impl IndicatorInstance for DetrendedPriceOscillatorInstance {
	type Config = DetrendedPriceOscillator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let sma = self.sma.next(src);
		let left_src = self.window.push(src);

		let dpo = left_src - sma;

		IndicatorResult::new(&[dpo], &[])
	}
}
