#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, PeriodType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, signi, RegularMethod, RegularMethods};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EaseOfMovement {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub method: RegularMethods,
}

impl IndicatorConfig for EaseOfMovement {
	fn validate(&self) -> bool {
		self.period1 > 1 && self.period2 >= 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"method" => self.method = value.parse().unwrap(),

			_ => {
				dbg!(format!(
					"Unknown attribute `{:}` with value `{:}` for `{:}`",
					name,
					value,
					std::any::type_name::<Self>(),
				));
			}
		};
	}

	fn is_volume_based(&self) -> bool {
		true
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl<T: OHLCV> IndicatorInitializer<T> for EaseOfMovement {
	type Instance = EaseOfMovementInstance<T>;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		Self::Instance {
			m1: method(cfg.method, cfg.period1, 0.),
			w: Window::new(cfg.period2, candle),

			cfg,
		}
	}
}

impl Default for EaseOfMovement {
	fn default() -> Self {
		Self {
			period1: 13,
			period2: 1,
			method: RegularMethods::SMA,
		}
	}
}

#[derive(Debug)]
pub struct EaseOfMovementInstance<T: OHLCV> {
	cfg: EaseOfMovement,

	m1: RegularMethod,
	w: Window<T>,
}

impl<T: OHLCV> IndicatorInstance<T> for EaseOfMovementInstance<T> {
	type Config = EaseOfMovement;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let prev_candle = self.w.push(candle);

		let d_high = candle.high() - prev_candle.high();
		let d_low = candle.low() - prev_candle.low();

		let d = (d_high + d_low) * 0.5;

		let v = d * (candle.high() - candle.low()) / candle.volume();
		debug_assert!(v.is_finite() && !v.is_nan());

		let value = self.m1.next(v);

		// let signal = if value > 0. {
		// 	1
		// } else if value < 0. {
		// 	-1
		// } else {
		// 	0
		// };
		let signal = signi(value);

		IndicatorResult::new(&[value], &[Action::from(signal)])
	}
}
