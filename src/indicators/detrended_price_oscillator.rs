#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, PeriodType, Source, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};

// The Formula for the Detrended Price Oscillator (DPO) is
// DPO=Price from X2+1 periods ago−X period SMA
// where:
// X = Number of periods used for the look-back period
// SMA = Simple Moving Average \begin{aligned}
// &DPO=Price from X/2+1 periods ago - X period SMA

/// [Detrended Price Oscillator](https://www.investopedia.com/terms/d/detrended-price-oscillator-dpo.asp)
///
/// # 1 value
///
/// * DPO \[-inf; +inf\]
///
/// # Has no signals
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DetrendedPriceOscillator {
	pub period: PeriodType,
	pub method: RegularMethods,
	pub source: Source,
}

impl IndicatorConfig for DetrendedPriceOscillator {
	const NAME: &'static str = "DetrendedPriceOscillator";

	fn validate(&self) -> bool {
		self.period > 1
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
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
		(1, 0)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for DetrendedPriceOscillator {
	type Instance = DetrendedPriceOscillatorInstance;

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
			sma: method(cfg.method, cfg.period, src)?,
			window: Window::new(cfg.period / 2 + 1, src),
			cfg,
		})
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

impl<T: OHLC> IndicatorInstance<T> for DetrendedPriceOscillatorInstance {
	type Config = DetrendedPriceOscillator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let sma = self.sma.next(src);
		let left_src = self.window.push(src);

		let dpo = left_src - sma;

		IndicatorResult::new(&[dpo], &[])
	}
}
