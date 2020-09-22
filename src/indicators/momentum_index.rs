#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Method, PeriodType, Source, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::Momentum;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MomentumIndex {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub source: Source,
}

impl IndicatorConfig for MomentumIndex {
	fn validate(&self) -> bool {
		self.period1 > self.period2
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
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

	fn size(&self) -> (u8, u8) {
		(2, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for MomentumIndex {
	type Instance = MomentumIndexInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let src = candle.source(cfg.source);
		Self::Instance {
			momentum1: Momentum::new(cfg.period1, src),
			momentum2: Momentum::new(cfg.period2, src),
			cfg,
		}
	}
}

impl Default for MomentumIndex {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 1,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct MomentumIndexInstance {
	cfg: MomentumIndex,

	momentum1: Momentum,
	momentum2: Momentum,
}

impl<T: OHLC> IndicatorInstance<T> for MomentumIndexInstance {
	type Config = MomentumIndex;

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let v = self.momentum1.next(src);
		let s = self.momentum2.next(src);

		// let signal;
		// if v > 0. && s > 0. {
		// 	signal = 1;
		// } else if v < 0. && s < 0. {
		// 	signal = -1;
		// } else {
		// 	signal = 0;
		// }

		let signal = (v > 0. && s > 0.) as i8 - (v < 0. && s < 0.) as i8;

		IndicatorResult::new(&[v, s], &[Action::from(signal)])
	}
}
