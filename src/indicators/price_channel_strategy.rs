#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::{Highest, Lowest};

/// Price Channel Strategy
///
/// Calculates price channel by highes high and lowest low for last `period` candles.
///
/// ## Links
///
/// * <https://www.investopedia.com/terms/p/price-channel.asp>
///
/// # 2 values
///
/// * `Upper bound` value
///
/// Range of values is the same as the range of the source values.
///
/// * `Lower bound` value
///
/// Range of values is the same as the range of the source values.
///
/// # 1 signal
///
/// When current `high` price touches `upper bound`, returns full buy signal.
/// When current `low` price touches `lower bound`, returns full sell signal.
/// When both touches occure, or no toucher, then returns no signal.
///
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceChannelStrategy {
	/// Main period length. Default is `20`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period: PeriodType,

	/// Relative channel size. Default is `1.0`.
	///
	/// Range in \(`0.0`; `1.0`\]
	pub sigma: ValueType,
}

impl IndicatorConfig for PriceChannelStrategy {
	type Instance = PriceChannelStrategyInstance;

	const NAME: &'static str = "PriceChannelStrategy";

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
		self.period > 1 && self.sigma > 0. && self.sigma <= 1.0
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"sigma" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.sigma = value,
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

impl Default for PriceChannelStrategy {
	fn default() -> Self {
		Self {
			period: 20,
			sigma: 1.0,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceChannelStrategyInstance {
	cfg: PriceChannelStrategy,

	highest: Highest,
	lowest: Lowest,
}

impl IndicatorInstance for PriceChannelStrategyInstance {
	type Config = PriceChannelStrategy;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let (high, low) = (candle.high(), candle.low());
		let highest = self.highest.next(&high);
		let lowest = self.lowest.next(&low);

		let middle = (highest + lowest) * 0.5;
		let delta = highest - middle;

		let upper = delta.mul_add(self.cfg.sigma, middle);
		let lower = delta.mul_add(-self.cfg.sigma, middle);

		let signal_up = (candle.high() >= upper) as i8;
		let signal_down = (candle.low() <= lower) as i8;

		let signal = signal_up - signal_down;

		IndicatorResult::new(&[upper, lower], &[signal.into()])
	}
}
