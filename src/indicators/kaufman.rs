#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, Method, PeriodType, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::{Change, Cross, LinearVolatility, StDev};

/// Kaufman Adaptive Moving Average (KAMA)
/// # Links
///
/// * <https://corporatefinanceinstitute.com/resources/knowledge/trading-investing/kaufmans-adaptive-moving-average-kama/>
/// * <https://ru.wikipedia.org/wiki/%D0%90%D0%B4%D0%B0%D0%BF%D1%82%D0%B8%D0%B2%D0%BD%D0%B0%D1%8F_%D1%81%D0%BA%D0%BE%D0%BB%D1%8C%D0%B7%D1%8F%D1%89%D0%B0%D1%8F_%D1%81%D1%80%D0%B5%D0%B4%D0%BD%D1%8F%D1%8F_%D0%9A%D0%B0%D1%83%D1%84%D0%BC%D0%B0%D0%BD%D0%B0>
/// * <https://www.marketvolume.com/technicalanalysis/kama.asp>
///
/// # 1 value
///
/// * `KAMA` value
///
/// Range of `KAMA` values is the same as the range of the `source` values.
///
/// # 1 signal
///
/// * if `filter_period` is less or equal than `0`, then returns signal when `KAMA` crosses `source` value.
/// When `source` crosses `KAMA` upwards, returns full buy signal.
/// When `source` crosses `KAMA` downwards, returns full sell signal.
/// Otherwise returns no signal.
///
/// * if `filter_period` is greater than `1`, it uses same cross between `source` and `KAMA`, but with additional filtering using standard deviation.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Kaufman {
	/// Volatility calculation period. Default is `10`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period1: PeriodType,

	/// Fast period. Default is `2`.
	///
	/// Range in \[`1`; `period3`\).
	pub period2: PeriodType,

	/// Slow period. Default is `30`.
	///
	/// Range in \(`period2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period3: PeriodType,

	/// Filter period. Default is `10`.
	///
	/// Range in \[`0`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub filter_period: PeriodType,

	/// Apply double smoothing. Default is `true`.
	pub square_smooth: bool,

	/// Standard deviation multiplier. Default is `0.3`.
	///
	/// Range in \(`0.0`; `+inf`\)
	pub k: ValueType,

	/// Source type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

pub type KAMA = Kaufman;

impl IndicatorConfig for Kaufman {
	type Instance = KaufmanInstance;

	const NAME: &'static str = "Kaufman";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			volatility: LinearVolatility::new(cfg.period1, src)?,
			change: Change::new(cfg.period1, src)?,
			fastest: 2. / (cfg.period2 + 1) as ValueType,
			slowest: 2. / (cfg.period3 + 1) as ValueType,
			st_dev: StDev::new(cfg.filter_period, src)?,
			cross: Cross::default(),
			last_signal: Action::None,
			last_signal_value: src,
			prev_value: src,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period3 > self.period2
			&& self.period2 > 0
			&& self.period1 > 0
			&& (self.k > 0.0 || self.filter_period < 2)
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
			"period3" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period3 = value,
			},
			"filter_period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.filter_period = value,
			},
			"square_smooth" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.square_smooth = value,
			},
			"k" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.k = value,
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
		(1, 1)
	}
}

impl Default for Kaufman {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 2,
			period3: 30,
			k: 0.3,
			square_smooth: true,
			filter_period: 10,
			source: Source::Close,
		}
	}
}
#[derive(Debug, Clone)]
pub struct KaufmanInstance {
	cfg: Kaufman,

	volatility: LinearVolatility,
	change: Change,
	fastest: ValueType,
	slowest: ValueType,
	st_dev: StDev,
	cross: Cross,
	last_signal: Action,
	last_signal_value: ValueType,
	prev_value: ValueType,
}

impl IndicatorInstance for KaufmanInstance {
	type Config = Kaufman;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let direction = self.change.next(src).abs();
		let volatility = self.volatility.next(src);

		let er = if volatility == 0. {
			0.
		} else {
			direction / volatility
		};
		let mut smooth = er.mul_add(self.fastest - self.slowest, self.slowest);

		if self.cfg.square_smooth {
			smooth = smooth * smooth;
		}

		let value = smooth.mul_add(src - self.prev_value, self.prev_value);
		self.prev_value = value;

		let cross = self.cross.next((src, value));

		let signal;
		if self.cfg.filter_period > 1 {
			let st_dev = self.st_dev.next(value);
			let filter = st_dev * self.cfg.k;

			if cross.is_some() {
				self.last_signal = cross;
				self.last_signal_value = value;
				signal = Action::None;
			} else if self.last_signal.is_some() && (value - self.last_signal_value).abs() > filter
			{
				signal = self.last_signal;
				self.last_signal = Action::None;
			} else {
				signal = Action::None;
			}
		} else {
			signal = cross;
		}

		IndicatorResult::new(&[value], &[signal])
	}
}
