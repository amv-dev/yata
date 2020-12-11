#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, CrossAbove, CrossUnder, Highest, Lowest};

/// Stochastic Oscillator
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Stochastic_oscillator>
///
/// # 2 values
///
/// * `main` value
///
/// Range in \[`0.0`; `1.0`\].
///
/// * `signal line` value
///
/// Range in \[`0.0`; `1.0`\].
///
/// # 3 signals
///
/// * Signal #1
///
/// When `main` value crosses lower bound upwards, returns full buy signal.
/// When `main` value crosses upper bound downwards, returns full sell signal.
/// Otherwise returns no signal.
///
/// * Signal #2
///
/// When `signal line` value crosses lower bound upwards, returns full buy signal.
/// When `signal line` value crosses upper bound downwards, returns full sell signal.
/// Otherwise returns no signal.
///
/// * Signal #3
///
/// When `main` value crosses `signal line` upwards, returns full buy signal.
/// When `main` value crosses `signal line` downwards, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StochasticOscillator {
	/// Period for searching highest high and lowest low. Default is `14`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period: PeriodType,

	/// Period for smoothing `main` value. Default is `14`.
	///
	/// Usually it is equal to `period`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub smooth_k: PeriodType,

	/// MA method for smoothing `main` value. Default is [`SMA`](crate::methods::SMA).
	pub method_k: RegularMethods,

	/// Period for smoothing `signal line` value. Default is `3`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub smooth_d: PeriodType,

	/// MA method for smoothing `signal line` value. Default is [`SMA`](crate::methods::SMA).
	pub method_d: RegularMethods,

	/// Zone size for #1 and #2 signals.
	///
	/// Range in \[`0.0`; `0.5`\].
	pub zone: ValueType,
}

impl IndicatorConfig for StochasticOscillator {
	type Instance = StochasticOscillatorInstance;

	const NAME: &'static str = "StochasticOscillator";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		// we need to check division by zero, so we can really just check if `high` is equal to `low` without using any kind of round error checks
		#[allow(clippy::float_cmp)]
		let k_rows = if candle.high() == candle.low() {
			0.5
		} else {
			(candle.close() - candle.low()) / (candle.high() - candle.low())
		};

		Ok(Self::Instance {
			upper_zone: 1. - cfg.zone,
			highest: Highest::new(cfg.period, candle.high())?,
			lowest: Lowest::new(cfg.period, candle.low())?,
			ma1: method(cfg.method_k, cfg.smooth_k, k_rows)?,
			ma2: method(cfg.method_d, cfg.smooth_d, k_rows)?,
			cross_over: Cross::default(),
			cross_above1: CrossAbove::default(),
			cross_under1: CrossUnder::default(),
			cross_above2: CrossAbove::default(),
			cross_under2: CrossUnder::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period > 1 && self.zone >= 0.0 && self.zone <= 0.5
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"smooth_k" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.smooth_k = value,
			},
			"smooth_d" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.smooth_d = value,
			},
			"zone" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
			},
			"method_k" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method_k = value,
			},
			"method_d" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method_d = value,
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

impl Default for StochasticOscillator {
	fn default() -> Self {
		Self {
			period: 14,
			smooth_k: 14,
			smooth_d: 3,
			method_k: RegularMethods::SMA,
			method_d: RegularMethods::SMA,
			zone: 0.2,
		}
	}
}

#[derive(Debug)]
pub struct StochasticOscillatorInstance {
	cfg: StochasticOscillator,

	upper_zone: ValueType,
	highest: Highest,
	lowest: Lowest,
	ma1: RegularMethod,
	ma2: RegularMethod,
	cross_over: Cross,
	cross_above1: CrossAbove,
	cross_under1: CrossUnder,
	cross_above2: CrossAbove,
	cross_under2: CrossUnder,
}

impl IndicatorInstance for StochasticOscillatorInstance {
	type Config = StochasticOscillator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let (close, high, low) = (candle.close(), candle.high(), candle.low());

		let highest = self.highest.next(high);
		let lowest = self.lowest.next(low);

		// we need to check division by zero, so we can really just check if `highest` is equal to `lowest` without using any kind of round error checks
		#[allow(clippy::float_cmp)]
		let k_rows = if highest == lowest {
			0.5
		} else {
			(close - lowest) / (highest - lowest)
		};

		let f1 = self.ma1.next(k_rows);
		let f2 = self.ma2.next(f1);

		let s1 = self.cross_above1.next((f1, self.cfg.zone))
			- self.cross_under1.next((f1, self.upper_zone));

		let s2 = self.cross_above2.next((f2, self.cfg.zone))
			- self.cross_under2.next((f2, self.upper_zone));

		let s3 = self.cross_over.next((f1, f2));

		IndicatorResult::new(&[f1, f2], &[s1, s2, s3])
	}
}
