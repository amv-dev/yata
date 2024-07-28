#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::{Highest, Lowest};

/// Donchian Channel
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Donchian_channel>
///
/// # 3 values
///
/// * Lower bound
///
/// Range is the same as [`high`] values.
///
/// * Middle value
///
/// It is always middle value between `upper bound` and `lower bound`
///
/// Range is the same as [`high`] and [`low`] values.
///
/// * Upper bound
///
/// Range is the same as [`low`] values.
///
/// # 1 signal
///
/// * When [`high`] value hits `upper bound`, returns full buy signal.
///   When [`low`] value hits `lower bound`, returns full sell signal.
///   Otherwise returns no signal.
///   If both values hit both bounds, returns no signal.
///
/// [`high`]: crate::core::OHLCV::high
/// [`low`]: crate::core::OHLCV::low
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DonchianChannel {
	/// Main period. Default is `20`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period: PeriodType,
}

impl IndicatorConfig for DonchianChannel {
	type Instance = DonchianChannelInstance;

	const NAME: &'static str = "DonchianChannel";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;

		Ok(Self::Instance {
			highest: Highest::new(cfg.period, &candle.high())?,
			lowest: Lowest::new(cfg.period, &candle.low())?,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period > 1
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
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
		(3, 1)
	}
}

impl Default for DonchianChannel {
	fn default() -> Self {
		Self { period: 20 }
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DonchianChannelInstance {
	cfg: DonchianChannel,

	highest: Highest,
	lowest: Lowest,
}

impl IndicatorInstance for DonchianChannelInstance {
	type Config = DonchianChannel;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	#[inline]
	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let (high, low) = (candle.high(), candle.low());

		let highest = self.highest.next(&high);
		let lowest = self.lowest.next(&low);

		let middle = (highest + lowest) * 0.5;

		let signal1 = (high >= highest) as i8 - (low <= lowest) as i8;

		IndicatorResult::new(&[lowest, middle, highest], &[signal1.into()])
	}
}
