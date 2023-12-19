#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::HLC;
use crate::core::{Error, Method, MovingAverageConstructor, PeriodType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
use crate::methods::Cross;

/// Ease Of Movement
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Ease_of_movement>
/// * <https://www.investopedia.com/terms/e/easeofmovement.asp>
///
/// # 1 value
///
/// * Main value
///
/// Range in \(`-inf`; `+inf`\)
///
/// # 1 signal
///
/// * Signal 1 appears when `main value` crosses zero line.
/// When `main value` crosses zero line upwards, returns full buy signal.
/// When `main value` crosses zero line downwards, returns full sell signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EaseOfMovement<M: MovingAverageConstructor = MA> {
	/// Main moving average type.
	///
	/// Default is [`SMA(13)`](crate::methods::SMA)
	///
	/// Period range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub ma: M,
	/// Differencial period size. Default is `1`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)\].
	pub period2: PeriodType,
}

impl<M: MovingAverageConstructor> IndicatorConfig for EaseOfMovement<M> {
	type Instance = EaseOfMovementInstance<M>;

	const NAME: &'static str = "EaseOfMovement";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			m1: cfg.ma.init(0.)?, //method(cfg.method, cfg.period1, 0.)?,
			w: Window::new(cfg.period2, HLC::from(candle)),
			cross: Cross::new((), &(0.0, 0.0))?,

			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.ma.ma_period() > 1 && self.ma.ma_period() < PeriodType::MAX && self.period2 >= 1
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"ma" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl Default for EaseOfMovement<MA> {
	fn default() -> Self {
		Self {
			ma: MA::SMA(13),
			period2: 1,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EaseOfMovementInstance<M: MovingAverageConstructor = MA> {
	cfg: EaseOfMovement<M>,

	m1: M::Instance,
	w: Window<HLC>,
	cross: Cross,
}

impl<M: MovingAverageConstructor> IndicatorInstance for EaseOfMovementInstance<M> {
	type Config = EaseOfMovement<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let prev_candle = self.w.push(HLC::from(candle));

		let d_high = candle.high() - prev_candle.high();
		let d_low = candle.low() - prev_candle.low();

		let d = (d_high + d_low) * 0.5;

		let v = if candle.volume() == 0.0 {
			0.0
		} else {
			d * (candle.high() - candle.low()) / candle.volume()
		};

		debug_assert!(v.is_finite() && !v.is_nan());

		let value = self.m1.next(&v);

		// let signal = if value > 0. {
		// 	1
		// } else if value < 0. {
		// 	-1
		// } else {
		// 	0
		// };
		let signal = self.cross.next(&(value, 0.0));

		IndicatorResult::new(&[value], &[signal])
	}
}
