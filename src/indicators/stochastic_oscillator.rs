#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, ValueType, OHLC};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, CrossAbove, CrossUnder, Highest, Lowest};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StochasticOscillator {
	pub period: PeriodType,
	pub smooth_k: PeriodType,
	pub smooth_d: PeriodType,
	pub zone: ValueType,
	pub method: RegularMethods,
}

impl IndicatorConfig for StochasticOscillator {
	fn validate(&self) -> bool {
		self.period > 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period" => self.period = value.parse().unwrap(),
			"smooth_k" => self.smooth_k = value.parse().unwrap(),
			"smooth_d" => self.smooth_d = value.parse().unwrap(),
			"zone" => self.zone = value.parse().unwrap(),
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

	fn size(&self) -> (u8, u8) {
		(2, 3)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for StochasticOscillator {
	type Instance = StochasticOscillatorInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let k_rows = if candle.high() != candle.low() {
			(candle.close() - candle.low()) / (candle.high() - candle.low())
		} else {
			0.
		};

		Self::Instance {
			upper_zone: 1. - cfg.zone,
			highest: Highest::new(cfg.period, candle.high()),
			lowest: Lowest::new(cfg.period, candle.low()),
			ma1: method(cfg.method, cfg.smooth_k, k_rows),
			ma2: method(cfg.method, cfg.smooth_d, k_rows),
			cross_over: Cross::default(),
			cross_above1: CrossAbove::default(),
			cross_under1: CrossUnder::default(),
			cross_above2: CrossAbove::default(),
			cross_under2: CrossUnder::default(),
			cfg,
		}
	}
}

impl Default for StochasticOscillator {
	fn default() -> Self {
		Self {
			period: 14,
			smooth_k: 1,
			smooth_d: 3,
			method: RegularMethods::SMA,
			zone: 0.2,
		}
	}
}

#[derive(Debug)]
pub struct StochasticOscillatorInstance {
	cfg: StochasticOscillator,

	upper_zone: ValueType,
	highest: Highest,
	lowest: Lowest,
	ma1: RegularMethod,
	ma2: RegularMethod,
	cross_over: Cross,
	cross_above1: CrossAbove,
	cross_under1: CrossUnder,
	cross_above2: CrossAbove,
	cross_under2: CrossUnder,
}

impl<T: OHLC> IndicatorInstance<T> for StochasticOscillatorInstance {
	type Config = StochasticOscillator;

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let (close, high, low) = (candle.close(), candle.high(), candle.low());

		let highest = self.highest.next(high);
		let lowest = self.lowest.next(low);

		let k_rows = if highest != lowest {
			(close - lowest) / (highest - lowest)
		} else {
			0.
		};

		let f1 = self.ma1.next(k_rows);
		let f2 = self.ma2.next(f1);

		let s1 = self.cross_above1.next((f1, self.cfg.zone))
			- self.cross_under1.next((f1, self.upper_zone));

		let s2 = self.cross_above2.next((f2, self.cfg.zone))
			- self.cross_under2.next((f2, self.upper_zone));

		let s3 = self.cross_over.next((f1, f2));

		IndicatorResult::new(&[f1, f2], &[s1, s2, s3])
	}
}
