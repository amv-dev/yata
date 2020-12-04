#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, Method, PeriodType, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{LowerReversalSignal, UpperReversalSignal};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PivotReversalStrategy {
	pub left: PeriodType,
	pub right: PeriodType,
}

impl IndicatorConfig for PivotReversalStrategy {
	const NAME: &'static str = "PivotReversalStrategy";

	fn validate(&self) -> bool {
		self.left >= 1 && self.right >= 1 && self.left.saturating_add(self.right) < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"left" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.left = value,
			},
			"right" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.right = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for PivotReversalStrategy {
	type Instance = PivotReversalStrategyInstance<T>;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			ph: UpperReversalSignal::new(cfg.left, cfg.right, candle.high())?,
			pl: LowerReversalSignal::new(cfg.left, cfg.right, candle.low())?,
			window: Window::new(cfg.right, candle),
			hprice: 0.,
			lprice: 0.,
			cfg,
		})
	}
}

impl Default for PivotReversalStrategy {
	fn default() -> Self {
		Self { left: 4, right: 2 }
	}
}

#[derive(Debug)]
pub struct PivotReversalStrategyInstance<T: OHLC> {
	cfg: PivotReversalStrategy,

	ph: UpperReversalSignal,
	pl: LowerReversalSignal,
	window: Window<T>,
	hprice: ValueType,
	lprice: ValueType,
}

impl<T: OHLC> IndicatorInstance<T> for PivotReversalStrategyInstance<T> {
	type Config = PivotReversalStrategy;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	#[allow(clippy::similar_names)]
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
