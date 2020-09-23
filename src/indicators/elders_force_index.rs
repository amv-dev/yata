#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, Source, ValueType, Window, OHLC, OHLCV};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::Cross;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EldersForceIndex {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub method: RegularMethods,
	pub source: Source,
}

impl IndicatorConfig for EldersForceIndex {
	fn validate(&self) -> bool {
		self.period1 > 1 && self.period2 >= 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"method" => self.method = value.parse().unwrap(),
			"source" => self.source = value.parse().unwrap(),

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

impl<T: OHLCV> IndicatorInitializer<T> for EldersForceIndex {
	type Instance = EldersForceIndexInstance<T>;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		Self::Instance {
			ma: method(cfg.method, cfg.period1, 0.),
			window: Window::new(cfg.period2, candle),
			vol_sum: candle.volume() * cfg.period2 as ValueType,
			cross_over: Cross::default(),
			cfg,
		}
	}
}

impl Default for EldersForceIndex {
	fn default() -> Self {
		Self {
			period1: 13,
			period2: 1,
			method: RegularMethods::SMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct EldersForceIndexInstance<T: OHLCV> {
	cfg: EldersForceIndex,

	ma: RegularMethod,
	window: Window<T>,
	vol_sum: ValueType,
	cross_over: Cross,
}

impl<T: OHLCV> IndicatorInstance<T> for EldersForceIndexInstance<T> {
	type Config = EldersForceIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let left_candle = self.window.push(candle);

		self.vol_sum += candle.volume() - left_candle.volume();
		let r = (OHLC::source(&candle, self.cfg.source)
			- OHLC::source(&left_candle, self.cfg.source))
			* self.vol_sum;

		let value = self.ma.next(r);
		let signal = self.cross_over.next((value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
