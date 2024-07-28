#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use std::cmp::Ordering;

use super::HLC;

/// Parabolic Stop And Reverse
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Parabolic_SAR>
///
/// # 2 values
///
/// * `SAR` value
///
/// Range of values is the same as the range of the timeseries values.
///
/// * `trend` value
///
/// Can be one of the next values: {`-1.0`; `0.0`; `1.0`}
///
/// # 1 signal
/// * When `trend` changes it's value to positive, then returns full buy signal.
///   When `trend` changes it's value to negative, then returns full sell signal.
///   Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParabolicSAR {
	pub af_step: ValueType,
	pub af_max: ValueType,
}

impl IndicatorConfig for ParabolicSAR {
	type Instance = ParabolicSARInstance;

	const NAME: &'static str = "ParabolicSAR";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			trend: 1,
			trend_inc: 1,
			low: candle.low(),
			high: candle.high(),
			sar: candle.low(),
			prev_candle: HLC::from(candle),
			prev_trend: 0,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.af_step < self.af_max
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"af_step" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.af_step = value,
			},
			"af_max" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.af_max = value,
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

impl Default for ParabolicSAR {
	fn default() -> Self {
		Self {
			af_max: 0.2,
			af_step: 0.02,
		}
	}
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ParabolicSARInstance {
	cfg: ParabolicSAR,

	trend: i8,
	trend_inc: u32,
	low: ValueType,
	high: ValueType,
	sar: ValueType,
	prev_candle: HLC,
	prev_trend: i8,
}

/// Just an alias for `ParabolicSAR`
pub type ParabolicStopAndReverse = ParabolicSAR;

impl IndicatorInstance for ParabolicSARInstance {
	type Config = ParabolicSAR;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		match self.trend.cmp(&0) {
			Ordering::Greater => {
				if self.high < candle.high() {
					self.high = candle.high();
					self.trend_inc += 1;
				}
				if candle.low() < self.sar {
					self.trend *= -1;
					self.low = candle.low();
					self.trend_inc = 1;
					self.sar = self.high;
				}
			}
			Ordering::Less => {
				if self.low > candle.low() {
					self.low = candle.low();
					self.trend_inc += 1;
				}
				if candle.high() > self.sar {
					self.trend *= -1;
					self.high = candle.high();
					self.trend_inc = 1;
					self.sar = self.low;
				}
			}
			Ordering::Equal => {}
		}

		let trend = self.trend;
		let sar = self.sar;

		// af := math.Min(a.AfMax, a.AfStep*float64(trendI))
		let af = self
			.cfg
			.af_max
			.min(self.cfg.af_step * (self.trend_inc as ValueType));

		match self.trend.cmp(&0) {
			Ordering::Greater => {
				self.sar = af.mul_add(self.high - self.sar, self.sar);
				self.sar = self.sar.min(candle.low()).min(self.prev_candle.low());
			}
			Ordering::Less => {
				self.sar = af.mul_add(self.low - self.sar, self.sar);
				self.sar = self.sar.max(candle.high()).max(self.prev_candle.high());
			}
			Ordering::Equal => {}
		}

		self.prev_candle = HLC::from(candle);

		let signal = (self.prev_trend != trend) as i8 * trend;

		self.prev_trend = trend;

		IndicatorResult::new(&[sar, trend as ValueType], &[Action::from(signal)])
	}
}
