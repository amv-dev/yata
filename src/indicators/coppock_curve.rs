#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, RateOfChange, ReversalSignal};

/// Coppock curve
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Coppock_curve>
///
/// # 2 values
///
/// * `Main value`
///
/// Range of values is the same as the range of the `source` values.
///
/// * `Signal line` value
///
/// Range of values is the same as the range of the `source` values.
///
/// # 3 signals
///
/// * Signal 1 appears when `main value` crosses zero line. When `main value` crosses zero line upwards, returns full buy signal. When `main value` crosses zero line downwards, returns full sell signal.
/// * Signal 2 appears on reverse points of `main value`. When top reverse point appears,
/// * Signal 3 appears on `main value` crosses `signal line`. When `main value` crosses `signal line` upwards, returns full buy signal. When `main value` crosses `signal line` downwards, returns full sell signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CoppockCurve {
	/// MA period \(using `method1`\). Default is `10`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period1: PeriodType,

	/// Long rate of change period. Default is `14`.
	///
	/// Range in \(`period3`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period2: PeriodType,

	/// Short rate of change period. Default is `11`.
	///
	/// Range in \[`1`; `period2`\).
	pub period3: PeriodType,

	/// Signal 2 reverse points left limit. Default is `4`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)-`s2_right`\).
	pub s2_left: PeriodType,

	/// Signal 2 reverse points right limit. Default is `2`
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)-`s2_left`\).
	pub s2_right: PeriodType,

	/// Signal line period (using `method2`). Default is `5`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub s3_period: PeriodType,

	/// Source type. Default is [`Close`](crate::core::Source::Close).
	pub source: Source,

	/// Main MA type \(using `period1`\). Default is [`WMA`](crate::methods::WMA)
	pub method1: RegularMethods,

	/// Signal line MA type \(using `s3_period`\). Default is [`EMA`](crate::methods::EMA)
	pub method2: RegularMethods,
}

impl IndicatorConfig for CoppockCurve {
	type Instance = CoppockCurveInstance;

	const NAME: &'static str = "CoppockCurve";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = &candle.source(cfg.source);
		Ok(Self::Instance {
			roc1: RateOfChange::new(cfg.period2, src)?,
			roc2: RateOfChange::new(cfg.period3, src)?,
			ma1: method(cfg.method1, cfg.period1, 0.)?,
			ma2: method(cfg.method2, cfg.s3_period, 0.)?,
			cross_over1: Cross::default(),
			pivot: ReversalSignal::new(cfg.s2_left, cfg.s2_right, 0.)?,
			cross_over2: Cross::default(),

			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period1 > 1
			&& self.period2 > self.period3
			&& self.period2 < PeriodType::MAX
			&& self.period3 > 0
			&& self.s3_period > 1
			&& self.s2_left > 0
			&& self.s2_right > 0
			&& self.s2_left.saturating_add(self.s2_right) < PeriodType::MAX
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
			"s2_left" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.s2_left = value,
			},
			"s2_right" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.s2_right = value,
			},
			"s3_period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.s3_period = value,
			},
			"source" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},
			"method1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method1 = value,
			},
			"method2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method2 = value,
			},
			// "zone"		=> self.zone = value.parse().unwrap(),
			// "source"	=> self.source = value.parse().unwrap(),
			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(2, 3)
	}
}

impl Default for CoppockCurve {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 14,
			period3: 11,
			s2_left: 4,
			s2_right: 2,
			s3_period: 5,
			method1: RegularMethods::WMA,
			method2: RegularMethods::EMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct CoppockCurveInstance {
	cfg: CoppockCurve,

	roc1: RateOfChange,
	roc2: RateOfChange,
	ma1: RegularMethod,
	ma2: RegularMethod,
	cross_over1: Cross,
	pivot: ReversalSignal,
	cross_over2: Cross,
}

impl IndicatorInstance for CoppockCurveInstance {
	type Config = CoppockCurve;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = &candle.source(self.cfg.source);
		let roc1 = self.roc1.next(src);
		let roc2 = self.roc2.next(src);
		let value1 = self.ma1.next(&(roc1 + roc2));
		let value2 = self.ma2.next(&value1);

		let signal1 = self.cross_over1.next(&(value1, 0.));
		let signal2 = self.pivot.next(&value1);
		let signal3 = self.cross_over2.next(&(value1, value2));

		IndicatorResult::new(&[value1, value2], &[signal1, signal2, signal3])
	}
}
