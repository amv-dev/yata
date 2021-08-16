#![allow(unused_imports)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Candle, Error, Method, MovingAverageConstructor, OHLCV, PeriodType, Source};
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
	pub ma1: M,
	pub ma2: M,
	pub signal: M,
	/*
	/// Fast MA period. Default is `12`.
	///
	/// Range in \[`2`; `period2`\)
	pub period1: PeriodType,

	/// Fast MA type. Default is [`EMA`](crate::methods::EMA).
	pub method1: RegularMethods,

	/// Slow MA period. Default is `26`.
	///
	/// Range in \(`period1`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period2: PeriodType,

	/// Slow MA type. Default is [`EMA`](crate::methods::EMA).
	pub method2: RegularMethods,

	/// Signal line MA period. Default is `9`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period3: PeriodType,

	/// Signal line MA type. Default is [`EMA`](crate::methods::EMA).
	pub method3: RegularMethods,
	*/
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
				ma1: cfg.ma1.init(src)?, // method(cfg.method1, cfg.period1, src)?,
				ma2: cfg.ma2.init(src)?, // method(cfg.method2, cfg.period2, src)?,
				ma3: cfg.signal.init(src)?, // method(cfg.method3, cfg.period3, src)?,
				cross1: Cross::default(),
				cross2: Cross::default(),
				cfg,
			})
		} else {
			Err(Error::WrongConfig)
		}
	}

	fn validate(&self) -> bool {
		self.ma1.ma_period() < self.ma2.ma_period() && self.ma1.ma_period() > 1 && self.signal.ma_period() > 1
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
			// period1: 12,
			// period2: 26,
			// period3: 9,
			// method1: RegularMethods::EMA,
			// method2: RegularMethods::EMA,
			// method3: RegularMethods::EMA,
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
