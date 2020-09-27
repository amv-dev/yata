#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{PeriodType, Source, ValueType, Window, OHLC};
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
	fn validate(&self) -> bool {
		self.period > 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period" => self.period = value.parse().unwrap(),
			"method" => self.method = value.parse().unwrap(),
			"source" => self.source = value.parse().unwrap(),

			_ => {
				dbg!(format!(
					"Unknown attribute `{:}` with value `{:}` for `{:}`",
					name,
					value,
					std::any::type_name::<Self>(),
				));
			}
		};
	}

	fn size(&self) -> (u8, u8) {
		(1, 0)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for DetrendedPriceOscillator {
	type Instance = DetrendedPriceOscillatorInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let src = candle.source(cfg.source);
		Self::Instance {
			sma: method(cfg.method, cfg.period, src),
			window: Window::new(cfg.period / 2 + 1, src),
			cfg,
		}
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

	fn name(&self) -> &'static str {
		"DetrendedPriceOscillator"
	}

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
