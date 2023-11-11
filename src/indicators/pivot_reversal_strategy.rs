#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::{LowerReversalSignal, UpperReversalSignal};

use super::HLC;

/// Pivot Reversal Strategy
///
/// Simply searches for pivot points and returns signal.
///
/// ## Links
///
/// * <https://www.incrediblecharts.com/technical/pivot_point_reversal.php>
///
/// # No values
///
/// # 1 signal
///
/// * `main` pivot signal
///
/// When low pivot happens, returns full buy signal.
/// When high pivot happens, returns full sell signal.
/// Otherwise returns no signal.
///
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PivotReversalStrategy {
	/// How many periods should left before pivot point.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)-`right`\).
	pub left: PeriodType,

	/// How many periods should appear after pivot point.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)-`left`\).
	pub right: PeriodType,
}

impl IndicatorConfig for PivotReversalStrategy {
	type Instance = PivotReversalStrategyInstance;

	const NAME: &'static str = "PivotReversalStrategy";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			ph: UpperReversalSignal::new(cfg.left, cfg.right, &candle.high())?,
			pl: LowerReversalSignal::new(cfg.left, cfg.right, &candle.low())?,
			window: Window::new(cfg.right, HLC::from(candle)),
			hprice: 0.,
			lprice: 0.,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.left >= 1 && self.right >= 1 && self.left.saturating_add(self.right) < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"left" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.left = value,
			},
			"right" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.right = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(0, 1)
	}
}

impl Default for PivotReversalStrategy {
	fn default() -> Self {
		Self { left: 4, right: 2 }
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PivotReversalStrategyInstance {
	cfg: PivotReversalStrategy,

	ph: UpperReversalSignal,
	pl: LowerReversalSignal,
	window: Window<HLC>,
	hprice: ValueType,
	lprice: ValueType,
}

impl IndicatorInstance for PivotReversalStrategyInstance {
	type Config = PivotReversalStrategy;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	#[allow(clippy::similar_names)]
	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let (high, low) = (candle.high(), candle.low());
		let past_candle = self.window.push(HLC::from(candle));

		let swh = self.ph.next(&high);
		let swl = self.pl.next(&low);

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

		IndicatorResult::new(&[], &[r.into()])
	}
}
