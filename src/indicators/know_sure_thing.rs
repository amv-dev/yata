#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, RateOfChange};

/// Know Sure Thing
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/KST_oscillator>
///
/// # 2 values
///
/// * `KST` value
///
/// Range in \(`-inf`; `+inf`\)
///
/// * `Sinal line` value
///
/// Range in \(`-inf`; `+inf`\)
///
/// # 1 signal
///
/// * When `KST` crosses `Signal line` upwards, returns full buy signal.
/// When `KST` crosses `Signal line` downwards, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KnowSureThing {
	/// ROC1 period. Default is `10`.
	pub period1: PeriodType,

	/// ROC2 period. Default is `15`.
	pub period2: PeriodType,

	/// ROC3 period. Default is `20`.
	pub period3: PeriodType,

	/// ROC4 period. Default is `30`.
	pub period4: PeriodType,

	/// ROC1 moving average period. Default is `10`.
	pub sma1: PeriodType,

	/// ROC2 moving average period. Default is `10`.
	pub sma2: PeriodType,

	/// ROC3 moving average period. Default is `10`.
	pub sma3: PeriodType,

	/// ROC4 moving average period. Default is `15`.
	pub sma4: PeriodType,

	/// ROCs lines moving average type. Defual is [`SMA`](crate::methods::SMA).
	pub method1: RegularMethods,

	/// Signal line moving average period. Default is `9`.
	pub sma5: PeriodType,

	/// Signal line moving average type. Defual is [`SMA`](crate::methods::SMA).
	pub method2: RegularMethods,
}

impl IndicatorConfig for KnowSureThing {
	type Instance = KnowSureThingInstance;

	const NAME: &'static str = "KnowSureThing";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let close = &candle.close();

		Ok(Self::Instance {
			roc1v: RateOfChange::new(cfg.period1, close)?,
			roc2v: RateOfChange::new(cfg.period2, close)?,
			roc3v: RateOfChange::new(cfg.period3, close)?,
			roc4v: RateOfChange::new(cfg.period4, close)?,
			ma1: method(cfg.method1, cfg.sma1, 0.)?,
			ma2: method(cfg.method1, cfg.sma2, 0.)?,
			ma3: method(cfg.method1, cfg.sma3, 0.)?,
			ma4: method(cfg.method1, cfg.sma4, 0.)?,
			ma5: method(cfg.method2, cfg.sma5, 0.)?,
			cross: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period1 < self.period2 && self.period2 < self.period3 && self.period3 < self.period4
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
			"period4" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period4 = value,
			},
			"sma1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.sma1 = value,
			},
			"sma2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.sma2 = value,
			},
			"sma3" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.sma3 = value,
			},
			"sma4" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.sma4 = value,
			},
			"sma5" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.sma5 = value,
			},
			"method1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method1 = value,
			},
			"method2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method2 = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(2, 1)
	}
}

impl Default for KnowSureThing {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 15,
			period3: 20,
			period4: 30,
			sma1: 10,
			sma2: 10,
			sma3: 10,
			sma4: 15,
			sma5: 9,
			method1: RegularMethods::SMA,
			method2: RegularMethods::SMA,
		}
	}
}

#[derive(Debug)]
pub struct KnowSureThingInstance {
	cfg: KnowSureThing,

	roc1v: RateOfChange,
	roc2v: RateOfChange,
	roc3v: RateOfChange,
	roc4v: RateOfChange,
	ma1: RegularMethod,
	ma2: RegularMethod,
	ma3: RegularMethod,
	ma4: RegularMethod,
	ma5: RegularMethod,
	cross: Cross,
}

impl IndicatorInstance for KnowSureThingInstance {
	type Config = KnowSureThing;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let close = &candle.close();

		let roc1: ValueType = self.roc1v.next(close);
		let roc2: ValueType = self.roc2v.next(close);
		let roc3: ValueType = self.roc3v.next(close);
		let roc4: ValueType = self.roc4v.next(close);

		let rcma1: ValueType = self.ma1.next(&roc1);
		let rcma2: ValueType = self.ma2.next(&roc2);
		let rcma3: ValueType = self.ma3.next(&roc3);
		let rcma4: ValueType = self.ma4.next(&roc4);

		let kst = rcma2.mul_add(2., rcma1) + rcma3.mul_add(3., rcma4 * 4.);
		let sl: ValueType = self.ma5.next(&kst);

		let signal = self.cross.next(&(kst, sl));

		IndicatorResult::new(&[kst, sl], &[signal])
	}
}
