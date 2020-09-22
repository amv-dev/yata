#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Method, PeriodType, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{Highest, Lowest};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceChannelStrategy {
	pub period: PeriodType,
	pub sigma: ValueType,
}

impl IndicatorConfig for PriceChannelStrategy {
	fn validate(&self) -> bool {
		self.period > 1 && self.sigma > 0.
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period" => self.period = value.parse().unwrap(),
			"sigma" => self.sigma = value.parse().unwrap(),

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
		(2, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for PriceChannelStrategy {
	type Instance = PriceChannelStrategyInstance;
	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		Self::Instance {
			highest: Highest::new(cfg.period, candle.high()),
			lowest: Lowest::new(cfg.period, candle.low()),
			cfg,
		}
	}
}

impl Default for PriceChannelStrategy {
	fn default() -> Self {
		Self {
			period: 20,
			sigma: 1.0,
		}
	}
}

#[derive(Debug)]
pub struct PriceChannelStrategyInstance {
	cfg: PriceChannelStrategy,

	highest: Highest,
	lowest: Lowest,
}

impl<T: OHLC> IndicatorInstance<T> for PriceChannelStrategyInstance {
	type Config = PriceChannelStrategy;

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}
	fn next(&mut self, candle: T) -> IndicatorResult {
		let (high, low) = (candle.high(), candle.low());
		let highest = self.highest.next(high);
		let lowest = self.lowest.next(low);

		let middle = (highest + lowest) * 0.5;
		let delta = highest - middle;

		let upper = middle + delta * self.cfg.sigma;
		let lower = middle - delta * self.cfg.sigma;

		// let signal_up = if candle.high() >= upper { 1 } else { 0 };
		// let signal_down = if candle.low() <= lower { 1 } else { 0 };
		let signal_up = (candle.high() >= upper) as i8;
		let signal_down = (candle.low() <= lower) as i8;

		let signal = signal_up - signal_down;

		IndicatorResult::new(&[upper, lower], &[Action::from(signal)])
	}
}
