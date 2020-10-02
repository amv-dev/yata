#![allow(unused_imports)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Candle, Error, Method, PeriodType, Source, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::Cross;

// https://en.wikipedia.org/wiki/MACD
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MACD {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub method1: RegularMethods,
	pub method2: RegularMethods,
	pub method3: RegularMethods,
	pub source: Source,
}

impl IndicatorConfig for MACD {
	const NAME: &'static str = "MACD";

	fn validate(&self) -> bool {
		self.period1 < self.period2
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"period3" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period3 = value,
			},
			"method1" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method1 = value,
			},
			"method2" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method2 = value,
			},
			"method3" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method3 = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},
			_ => {
				return Some(Error::ParameterParse(name.to_string(), value.to_string()));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(2, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for MACD {
	type Instance = MACDInstance;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		match self.validate() {
			true => {
				let cfg = self;
				let src = candle.source(cfg.source);
				Ok(Self::Instance {
					ma1: method(cfg.method1, cfg.period1, src)?,
					ma2: method(cfg.method2, cfg.period2, src)?,
					ma3: method(cfg.method3, cfg.period3, src)?,
					cross: Cross::new((), (0.0, 0.0))?,
					cfg,
				})
			}
			false => Err(Error::WrongConfig),
		}
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
	cross: Cross,
}

/// Just an alias for MACD
pub type MovingAverageConvergenceDivergence = MACD;

impl<T: OHLC> IndicatorInstance<T> for MACDInstance {
	type Config = MACD;

	fn config(&self) -> &Self::Config
	where
		Self: Sized,
	{
		&self.cfg
	}

	#[inline]
	fn next(&mut self, candle: T) -> IndicatorResult
	where
		Self: Sized,
	{
		let src = candle.source(self.cfg.source);

		let ema1 = self.ma1.next(src);
		let ema2 = self.ma2.next(src);

		let macd = ema1 - ema2;
		let sigline = self.ma3.next(macd);

		let signal = self.cross.next((macd, sigline));

		IndicatorResult::new(&[macd, sigline], &[signal])
	}
}
