#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{
	Error, Method, MovingAverageConstructor, PeriodType, Source, ValueType, Window, OHLCV,
};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;

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
pub struct DetrendedPriceOscillator<M: MovingAverageConstructor = MA> {
	/// Main moving average type.
	///
	/// Default is [`SMA(21)`](crate::methods::SMA)
	///
	/// Period range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub ma: M,
	/// Source type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for DetrendedPriceOscillator<M> {
	type Instance = DetrendedPriceOscillatorInstance<M>;

	const NAME: &'static str = "DetrendedPriceOscillator";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			sma: cfg.ma.init(src)?, // method(cfg.method, cfg.period, src)?,
			window: Window::new(cfg.ma.ma_period() / 2 + 1, src),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.ma.ma_period() > 1 && self.ma.ma_period() < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"ma" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma = value,
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

impl Default for DetrendedPriceOscillator<MA> {
	fn default() -> Self {
		Self {
			ma: MA::SMA(21),
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DetrendedPriceOscillatorInstance<M: MovingAverageConstructor = MA> {
	cfg: DetrendedPriceOscillator<M>,

	sma: M::Instance,
	window: Window<ValueType>,
}

impl<M: MovingAverageConstructor> IndicatorInstance for DetrendedPriceOscillatorInstance<M> {
	type Config = DetrendedPriceOscillator<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let sma = self.sma.next(&src);
		let left_src = self.window.push(src);

		let dpo = left_src - sma;

		IndicatorResult::new(&[dpo], &[])
	}
}
