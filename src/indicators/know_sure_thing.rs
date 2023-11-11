#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, MovingAverageConstructor, PeriodType, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
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
/// * `Signal line` value
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
pub struct KnowSureThing<M: MovingAverageConstructor = MA> {
	/// ROC1 period. Default is `10`.
	pub period1: PeriodType,

	/// ROC2 period. Default is `15`.
	pub period2: PeriodType,

	/// ROC3 period. Default is `20`.
	pub period3: PeriodType,

	/// ROC4 period. Default is `30`.
	pub period4: PeriodType,

	/// ROC1 moving average type. Default is [`SMA(10)`](crate::methods::SMA).
	pub ma1: M,
	/// ROC2 moving average type. Default is [`SMA(10)`](crate::methods::SMA).
	pub ma2: M,
	/// ROC3 moving average type. Default is [`SMA(10)`](crate::methods::SMA).
	pub ma3: M,
	/// ROC4 moving average type. Default is [`SMA(15)`](crate::methods::SMA).
	pub ma4: M,

	/// Signal line moving average type. Default is [`SMA(9)`](crate::methods::SMA).
	pub signal: M,
}

impl<M: MovingAverageConstructor> IndicatorConfig for KnowSureThing<M> {
	type Instance = KnowSureThingInstance<M>;

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
			ma1: cfg.ma1.init(0.)?,
			ma2: cfg.ma2.init(0.)?,
			ma3: cfg.ma3.init(0.)?,
			ma4: cfg.ma4.init(0.)?,
			ma5: cfg.signal.init(0.)?,
			cross: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.ma1.is_similar_to(&self.ma2)
			&& self.ma1.is_similar_to(&self.ma3)
			&& self.ma1.is_similar_to(&self.ma4)
			&& self.period1 < self.period2
			&& self.period2 < self.period3
			&& self.period3 < self.period4
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
			"ma1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma1 = value,
			},
			"ma2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma2 = value,
			},
			"ma3" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma3 = value,
			},
			"ma4" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma4 = value,
			},
			"signal" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.signal = value,
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
			ma1: MA::SMA(10),
			ma2: MA::SMA(10),
			ma3: MA::SMA(10),
			ma4: MA::SMA(15),
			signal: MA::SMA(9),
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KnowSureThingInstance<M: MovingAverageConstructor = MA> {
	cfg: KnowSureThing<M>,

	roc1v: RateOfChange,
	roc2v: RateOfChange,
	roc3v: RateOfChange,
	roc4v: RateOfChange,
	ma1: M::Instance,
	ma2: M::Instance,
	ma3: M::Instance,
	ma4: M::Instance,
	ma5: M::Instance,
	cross: Cross,
}

impl<M: MovingAverageConstructor> IndicatorInstance for KnowSureThingInstance<M> {
	type Config = KnowSureThing<M>;

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
