#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, OHLCV};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, ADI};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChaikinOscillator {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub method: RegularMethods,
	pub window: PeriodType, // from 0 to ...
}

impl IndicatorConfig for ChaikinOscillator {
	fn validate(&self) -> bool {
		self.period1 < self.period2
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

impl<T: OHLCV> IndicatorInitializer<T> for ChaikinOscillator {
	type Instance = ChaikinOscillatorInstance<T>;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let adi = ADI::new(cfg.window, candle);

		Self::Instance {
			ma1: method(cfg.method, cfg.period1, adi.get_value()),
			ma2: method(cfg.method, cfg.period2, adi.get_value()),
			adi,
			cross_over: Cross::default(),
			cfg,
		}
	}
}

impl Default for ChaikinOscillator {
	fn default() -> Self {
		Self {
			period1: 3,
			period2: 10,
			method: RegularMethods::EMA,
			window: 0,
		}
	}
}

#[derive(Debug)]
pub struct ChaikinOscillatorInstance<T: OHLCV> {
	cfg: ChaikinOscillator,

	adi: ADI<T>,
	ma1: RegularMethod,
	ma2: RegularMethod,
	cross_over: Cross,
}

impl<T: OHLCV> IndicatorInstance<T> for ChaikinOscillatorInstance<T> {
	type Config = ChaikinOscillator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let adi = self.adi.next(candle);

		let data1 = self.ma1.next(adi);
		let data2 = self.ma2.next(adi);

		let value = data1 - data2;

		let signal = self.cross_over.next((value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
