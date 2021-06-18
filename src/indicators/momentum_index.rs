#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, Method, PeriodType, Source, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::Momentum;

/// Momentum Index
///
/// # 2 values
///
/// * `slow momentum` value
///
/// Range in \(`-inf`; `+inf`\)
///
/// * `fast momentum` value
///
/// Range in \(`-inf`; `+inf`\)
///
/// # 1 signal
///
/// * When both momentums are positive, returns full buy signal.
/// When both momentums are negative, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MomentumIndex {
	/// Slow momentum period. Default is `10`.
	///
	/// Range in \(`period2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period1: PeriodType,

	/// Fast momentum period. Default is `1`.
	///
	/// Range in \[`1`; `period1`\)
	pub period2: PeriodType,

	/// Source value type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for MomentumIndex {
	type Instance = MomentumIndexInstance;

	const NAME: &'static str = "MomentumIndex";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = &candle.source(cfg.source);

		Ok(Self::Instance {
			momentum1: Momentum::new(cfg.period1, src)?,
			momentum2: Momentum::new(cfg.period2, src)?,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period2 > 0 && self.period1 > self.period2
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"source" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
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

impl Default for MomentumIndex {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 1,
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone)]
pub struct MomentumIndexInstance {
	cfg: MomentumIndex,

	momentum1: Momentum,
	momentum2: Momentum,
}

impl IndicatorInstance for MomentumIndexInstance {
	type Config = MomentumIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = &candle.source(self.cfg.source);

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
