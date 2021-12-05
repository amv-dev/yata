#![allow(unused_imports)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Candle, Error, Method, MovingAverageConstructor, PeriodType, Source, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
use crate::methods::Cross;

/// Moving average convergence/divergence (MACD)
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/MACD>
///
/// # 2 values
///
/// * `MACD` value
///
/// Range in \(`-inf`; `+inf`\).
///
/// * `Signal line` value
///
/// Range in \(`-inf`; `+inf`\).
///
/// # 2 signal
///
/// * When `MACD` crosses `Signal line` upwards, returns full buy signal.
/// When `MACD` crosses `Signal line` downwards, returns full sell signal.
/// Otherwise returns no signal.
///
/// * When `MACD` crosses zero line upwards, returns full buy signal.
/// When `MACD` crosses zero line downwards, returns full sell signal.
/// Otherwise returns no signal.
///
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MACD<M: MovingAverageConstructor = MA> {
	/// Fast MA type.
	///
	/// Default is [`EMA(12)`](crate::methods::EMA).
	///
	/// Period range in \[`2`; ma2's period\)
	pub ma1: M,

	/// Slow MA type.
	///
	/// Default is [`EMA(26)`](crate::methods::EMA).
	///
	/// Period range in \(ma1's period; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub ma2: M,

	/// Signal line MA type.
	///
	/// Default is [`EMA(9)`](crate::methods::EMA).
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub signal: M,

	/// Source value type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for MACD<M> {
	type Instance = MACDInstance<M>;

	const NAME: &'static str = "MACD";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if self.validate() {
			let cfg = self;
			let src = candle.source(cfg.source);
			Ok(Self::Instance {
				ma1: cfg.ma1.init(src)?,
				ma2: cfg.ma2.init(src)?,
				ma3: cfg.signal.init(src)?,
				cross1: Cross::default(),
				cross2: Cross::default(),
				cfg,
			})
		} else {
			Err(Error::WrongConfig)
		}
	}

	fn validate(&self) -> bool {
		self.ma1.ma_period() < self.ma2.ma_period()
			&& self.ma1.ma_period() > 1
			&& self.signal.ma_period() > 1
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
			"signal" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.signal = value,
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
		(2, 2)
	}
}

impl Default for MACD {
	fn default() -> Self {
		Self {
			ma1: MA::EMA(12),
			ma2: MA::EMA(26),
			signal: MA::EMA(9),
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MACDInstance<M: MovingAverageConstructor> {
	cfg: MACD<M>,

	ma1: M::Instance,
	ma2: M::Instance,
	ma3: M::Instance,
	cross1: Cross,
	cross2: Cross,
}

/// Just an alias for MACD
pub type MovingAverageConvergenceDivergence<M = MA> = MACD<M>;

impl<M: MovingAverageConstructor> IndicatorInstance for MACDInstance<M> {
	type Config = MACD<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	#[inline]
	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = &candle.source(self.cfg.source);

		let ema1 = self.ma1.next(src);
		let ema2 = self.ma2.next(src);

		let macd = ema1 - ema2;
		let sigline = self.ma3.next(&macd);

		let signal1 = self.cross1.next(&(macd, sigline));
		let signal2 = self.cross2.next(&(macd, 0.0));

		IndicatorResult::new(&[macd, sigline], &[signal1, signal2])
	}
}
