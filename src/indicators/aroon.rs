#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::{Cross, HighestIndex, LowestIndex};

// https://www.fidelity.com/learning-center/trading-investing/technical-analysis/technical-indicator-guide/aroon-indicator
// Aroon-Up = [(Period Specified – Periods Since the Highest High within Period Specified) / Period Specified]
// Aroon-Down = [(Period Specified – Periods Since the Lowest Low for Period Specified) / Period Specified]
// If the Aroon-Up crosses above the Aroon-Down, then a new uptrend may start soon. Conversely, if Aroon-Down
// crosses above the Aroon-Up, then a new downtrend may start soon.
// When Aroon-Up reaches `1.0`, a new uptrend may have begun. If it remains persistently between `0.7` and `1.0`,
// and the Aroon-Down remains between 0 and 0.3, then a new uptrend is underway.
/// Aroon indicator
///
/// ## Links
///
/// * <https://www.fidelity.com/learning-center/trading-investing/technical-analysis/technical-indicator-guide/aroon-indicator>
///
/// # 2 values
///
/// * `AroonUp`
///
/// Range in \[`0.0`; `1.0`\]
///
/// * `AroonDown`
///
/// Range in \[`0.0`; `1.0`\]
///
/// # 3 signals
///
/// * When `AroonUp` crosses `AroonDown` upwards, gives full positive #0 signal.
///   When `AroonDown` crosses `AroonUp` upwards, gives full negative #0 signal.
///   Otherwise gives no #0 signal.
/// * When `AroonUp` rises up to 1.0, gives full positive #1 signal. When `AroonDown` rises up to 1.0, gives full negative #1 signal.
/// * Gives positive #2 signal when `AroonUp` stays above `(1.0-signal_zone)` and `AroonDown` stays under `signal_zone`.
///   Gives negative #2 signal when `AroonDown` stays above `(1.0-signal_zone)` and `AroonUp` stays under `signal_zone`.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Aroon {
	/// main period length. Default is `14`.
	///
	/// Range in *\[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)*
	pub period: PeriodType,
	/// zone value determines when signal #2 appears. Default is `0.3`.
	///
	/// Range in *\[`0.0`; `1.0`\]*
	pub signal_zone: ValueType,
	/// period until signal #2 appears in full strength. Default is `7`.
	///
	/// Range in *\[`1`; [`PeriodType::MAX`](crate::core::PeriodType)\)*
	pub over_zone_period: PeriodType,
}

impl IndicatorConfig for Aroon {
	type Instance = AroonInstance;

	const NAME: &'static str = "Aroon";

	fn init(self, candle: &dyn OHLC) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;

		Ok(Self::Instance {
			lowest_index: LowestIndex::new(cfg.period, &candle.low())?,
			highest_index: HighestIndex::new(cfg.period, &candle.high())?,
			cross: Cross::default(),
			uptrend: 0,
			downtrend: 0,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.signal_zone >= 0.0
			&& self.signal_zone <= 1.0
			&& self.period > 1
			&& self.period < PeriodType::MAX
			&& self.over_zone_period > 0
			&& self.over_zone_period < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"signal_zone" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.signal_zone = value,
			},
			"over_zone_period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.over_zone_period = value,
			},
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(2, 3)
	}
}

impl Default for Aroon {
	fn default() -> Self {
		Self {
			signal_zone: 0.3,
			period: 14,
			over_zone_period: 7,
		}
	}
}

/// Aroon state structure
#[derive(Debug)]
pub struct AroonInstance {
	cfg: Aroon,
	lowest_index: LowestIndex,
	highest_index: HighestIndex,
	cross: Cross,
	uptrend: isize,
	downtrend: isize,
}

impl IndicatorInstance for AroonInstance {
	type Config = Aroon;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: &dyn OHLC) -> IndicatorResult {
		let highest_index = self.highest_index.next(&candle.high());
		let lowest_index = self.lowest_index.next(&candle.low());

		let aroon_up =
			(self.cfg.period - highest_index) as ValueType / self.cfg.period as ValueType;

		let aroon_down =
			(self.cfg.period - lowest_index) as ValueType / self.cfg.period as ValueType;

		let trend_signal = self.cross.next(&(aroon_up, aroon_down));
		let edge_signal = (highest_index == 0) as i8 - (lowest_index == 0) as i8;

		let is_up_over = (aroon_up >= (1.0 - self.cfg.signal_zone)) as isize;
		let is_up_under = (aroon_up <= self.cfg.signal_zone) as isize;
		let is_down_over = (aroon_down >= (1.0 - self.cfg.signal_zone)) as isize;
		let is_down_under = (aroon_down <= self.cfg.signal_zone) as isize;

		self.uptrend = (self.uptrend + 1) * is_up_over * is_down_under;
		self.downtrend = (self.downtrend + 1) * is_down_over * is_up_under;

		let trend_value =
			(self.uptrend - self.downtrend) as ValueType / self.cfg.over_zone_period as ValueType;

		IndicatorResult::new(
			&[aroon_up, aroon_down],
			&[trend_signal, edge_signal.into(), trend_value.into()],
		)
	}
}
