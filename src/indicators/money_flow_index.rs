#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::Cross;

/// Money Flow Index
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Money_flow_index>
///
/// # 3 values
///
/// * `upper bound` const value
///
/// Range in \[`0.5`; `1.0`\]
///
/// * `MFI` value
///
/// Range in \[`0.0`; `1.0`\]
///
/// * `lower bound` const value
///
/// Range in \[`0.0`; `0.5`\]
///
/// # 2 signals
///
/// * When `MFI` value crosses `lower bound` downwards, returns full buy signal.
/// When `MFI` value crosses `upper bound` upwards, returns full sell signal.
/// Otherwise returns no signal.
///
/// * When `MFI` value crosses `lower bound` upwards, returns full buy signal.
/// When `MFI` value crosses `upper bound` downwards, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MoneyFlowIndex {
	/// Main period size. Default is `14`.
	///
	/// Range is \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period: PeriodType,

	/// Signal zone size. Default is `0.2`.
	///
	/// Range is \[`0.0`; `0.5`\]. Value `0.5` means that the `lower bound` is the same as the `upper bound`.
	pub zone: ValueType,
}

impl IndicatorConfig for MoneyFlowIndex {
	const NAME: &'static str = "MoneyFlowIndex";

	fn validate(&self) -> bool {
		self.zone >= 0. && self.zone <= 0.5
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"zone" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn is_volume_based(&self) -> bool {
		true
	}

	fn size(&self) -> (u8, u8) {
		(3, 2)
	}
}

impl<T: OHLCV> IndicatorInitializer<T> for MoneyFlowIndex {
	type Instance = MoneyFlowIndexInstance<T>;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			window: Window::new(cfg.period, candle),
			prev_candle: candle,
			last_prev_candle: candle,
			pmf: 0.,
			nmf: 0.,
			cross_lower: Cross::default(),
			cross_upper: Cross::default(),
			cfg,
		})
	}
}

impl Default for MoneyFlowIndex {
	fn default() -> Self {
		Self {
			period: 14,
			zone: 0.2,
		}
	}
}

#[derive(Debug)]
pub struct MoneyFlowIndexInstance<T: OHLCV> {
	cfg: MoneyFlowIndex,

	window: Window<T>,
	prev_candle: T,
	last_prev_candle: T,
	pmf: ValueType,
	nmf: ValueType,
	cross_lower: Cross,
	cross_upper: Cross,
}

#[inline]
fn tfunc<T: OHLCV>(candle: &T, last_candle: &T) -> (ValueType, ValueType) {
	let tp1 = candle.tp();
	let tp2 = last_candle.tp();

	(
		(tp1 > tp2) as i8 as ValueType * candle.volume(),
		(tp1 < tp2) as i8 as ValueType * candle.volume(),
	)
}

impl<T: OHLCV> IndicatorInstance<T> for MoneyFlowIndexInstance<T> {
	type Config = MoneyFlowIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let (pos, neg) = tfunc(&candle, &self.prev_candle);

		let last_candle = self.window.push(candle);
		let (left_pos, left_neg) = tfunc(&last_candle, &self.last_prev_candle);

		self.last_prev_candle = last_candle;
		self.prev_candle = candle;

		self.pmf += pos - left_pos;
		self.nmf += neg - left_neg;

		let mfr = if self.nmf == 0.0 {
			1.
		} else {
			self.pmf / self.nmf
		};

		let value = 1. - (1. + mfr).recip();

		let upper = 1. - self.cfg.zone;
		let lower = self.cfg.zone;

		let cross_upper: i8 = self.cross_upper.next((value, upper)).into();
		let cross_lower: i8 = self.cross_lower.next((value, lower)).into();

		let enters_zone = (cross_lower < 0) as i8 - (cross_upper > 0) as i8;
		let leaves_zone = (cross_lower > 0) as i8 - (cross_upper < 0) as i8;

		IndicatorResult::new(
			&[upper, value, lower],
			&[enters_zone.into(), leaves_zone.into()],
		)
	}
}
