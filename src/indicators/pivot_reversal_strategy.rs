#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Method, PeriodType, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{ReverseHighSignal, ReverseLowSignal};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReverseReversalStrategy {
	pub left: PeriodType,
	pub right: PeriodType,
}

impl IndicatorConfig for ReverseReversalStrategy {
	fn validate(&self) -> bool {
		true
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"left" => self.left = value.parse().unwrap(),
			"right" => self.right = value.parse().unwrap(),

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
		(1, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for ReverseReversalStrategy {
	type Instance = ReverseReversalStrategyInstance<T>;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		Self::Instance {
			ph: ReverseHighSignal::new(cfg.left, cfg.right, candle.high()),
			pl: ReverseLowSignal::new(cfg.left, cfg.right, candle.low()),
			window: Window::new(cfg.right, candle),
			hprice: 0.,
			lprice: 0.,
			cfg,
		}
	}
}

impl Default for ReverseReversalStrategy {
	fn default() -> Self {
		Self { left: 4, right: 2 }
	}
}

#[derive(Debug)]
pub struct ReverseReversalStrategyInstance<T: OHLC> {
	cfg: ReverseReversalStrategy,

	ph: ReverseHighSignal,
	pl: ReverseLowSignal,
	window: Window<T>,
	hprice: ValueType,
	lprice: ValueType,
}

impl<T: OHLC> IndicatorInstance<T> for ReverseReversalStrategyInstance<T> {
	type Config = ReverseReversalStrategy;

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}
	fn next(&mut self, candle: T) -> IndicatorResult {
		let (high, low) = (candle.high(), candle.low());
		let past_candle = self.window.push(candle);

		let swh = self.ph.next(high);
		let swl = self.pl.next(low);

		let mut le = 0;
		let mut se = 0;

		if swh.analog() > 0 {
			self.hprice = past_candle.high();
		}

		if swh.analog() > 0 || candle.high() <= self.hprice {
			le = 1;
		}

		if swl.analog() > 0 {
			self.lprice = past_candle.low();
		}

		if swl.analog() > 0 || low >= self.lprice {
			se = 1;
		}

		let r = se - le;

		IndicatorResult::new(&[r as ValueType], &[Action::from(r)])
	}
}
