#![allow(unused_imports)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Candle, Error, Method, PeriodType, Source, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
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
pub struct MACD {
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

	/// Source value type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for MACD {
	type Instance = MACDInstance;

	const NAME: &'static str = "MACD";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if self.validate() {
			let cfg = self;
			let src = candle.source(cfg.source);
			Ok(Self::Instance {
				ma1: method(cfg.method1, cfg.period1, src)?,
				ma2: method(cfg.method2, cfg.period2, src)?,
				ma3: method(cfg.method3, cfg.period3, src)?,
				cross1: Cross::default(),
				cross2: Cross::default(),
				cfg,
			})
		} else {
			Err(Error::WrongConfig)
		}
	}

	fn validate(&self) -> bool {
		self.period1 < self.period2 && self.period1 > 1 && self.period3 > 1
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
			"period3" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period3 = value,
			},
			"method1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method1 = value,
			},
			"method2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method2 = value,
			},
			"method3" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method3 = value,
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
			period1: 12,
			period2: 26,
			period3: 9,
			method1: RegularMethods::EMA,
			method2: RegularMethods::EMA,
			method3: RegularMethods::EMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct MACDInstance {
	cfg: MACD,

	ma1: RegularMethod,
	ma2: RegularMethod,
	ma3: RegularMethod,
	cross1: Cross,
	cross2: Cross,
}

/// Just an alias for MACD
pub type MovingAverageConvergenceDivergence = MACD;

impl IndicatorInstance for MACDInstance {
	type Config = MACD;

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
