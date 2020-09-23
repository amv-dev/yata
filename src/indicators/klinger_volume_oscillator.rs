#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, ValueType, OHLCV};
use crate::helpers::{method, sign, RegularMethod, RegularMethods};
use crate::methods::Cross;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KlingerVolumeOscillator {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub method1: RegularMethods,
	pub method2: RegularMethods,
}

impl IndicatorConfig for KlingerVolumeOscillator {
	fn validate(&self) -> bool {
		self.period1 < self.period2
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"period3" => self.period3 = value.parse().unwrap(),
			"method1" => self.method1 = value.parse().unwrap(),
			"method2" => self.method2 = value.parse().unwrap(),

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
		(2, 2)
	}
}

impl<T: OHLCV> IndicatorInitializer<T> for KlingerVolumeOscillator {
	type Instance = KlingerVolumeOscillatorInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		Self::Instance {
			ma1: method(cfg.method1, cfg.period1, 0.),
			ma2: method(cfg.method1, cfg.period2, 0.),
			ma3: method(cfg.method2, cfg.period3, 0.),
			cross1: Cross::default(),
			cross2: Cross::default(),
			last_tp: candle.tp(),
			cfg,
		}
	}
}

impl Default for KlingerVolumeOscillator {
	fn default() -> Self {
		Self {
			period1: 34,
			period2: 55,
			period3: 13,
			method1: RegularMethods::EMA,
			method2: RegularMethods::EMA,
		}
	}
}

#[derive(Debug)]
pub struct KlingerVolumeOscillatorInstance {
	cfg: KlingerVolumeOscillator,

	ma1: RegularMethod,
	ma2: RegularMethod,
	ma3: RegularMethod,
	cross1: Cross,
	cross2: Cross,
	last_tp: ValueType,
}

impl<T: OHLCV> IndicatorInstance<T> for KlingerVolumeOscillatorInstance {
	type Config = KlingerVolumeOscillator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let tp = candle.tp();

		let d = tp - self.last_tp;
		self.last_tp = tp;

		// let vol = if d > 0. {
		// 	candle.volume()
		// } else if d < 0. {
		// 	-candle.volume()
		// } else {
		// 	0.
		// };

		let vol = sign(d) * candle.volume();

		let ma1: ValueType = self.ma1.next(vol);
		let ma2: ValueType = self.ma2.next(vol);
		let ko = ma1 - ma2;

		let ma3: ValueType = self.ma3.next(ko);

		let s1 = self.cross1.next((ko, 0.));
		let s2 = self.cross2.next((ko, ma3));

		IndicatorResult::new(&[ko, ma3], &[s1, s2])
	}
}
