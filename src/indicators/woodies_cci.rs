#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::signi;
use crate::methods::{Cross, CCI};

const SCALE: ValueType = 1.0 / 1.5;

/// Woodies Commodity Channel Index
///
/// ## Links
///
/// * <https://tlc.thinkorswim.com/center/reference/Tech-Indicators/studies-library/V-Z/WoodiesCCI.html>
/// * <https://ftmo.com/en/woodies-cci-system/>
///
/// # 2 values
///
/// * `Turbo CCI`  value
///
/// Range in \(`-inf`; `+inf`\)
///
/// * `Trend CCI` value
///
/// Range in \(`-inf`; `+inf`\)
///
/// # 1 signals
///
/// * When `Trend CCI` stays above zero line for `s1_lag` bars, returns full buy signal.
/// When `Trend CCI` stays below zero line for `s1_lag` bars, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WoodiesCCI {
	/// `Turbo` CCI period
	pub period1: PeriodType,
	/// `Trend` CCI period
	pub period2: PeriodType,

	/// Signal #1 bars count to occur
	pub s1_lag: PeriodType,

	/// Source type of values. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for WoodiesCCI {
	type Instance = WoodiesCCIInstance;

	const NAME: &'static str = "WoodiesCCI";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			turbo: CCI::new(cfg.period1, src)?,
			trend: CCI::new(cfg.period2, src)?,
			s1_count: 0,
			s1_cross: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period1 < self.period2 && self.s1_lag > 0 && self.period2 < PeriodType::MAX && self.s1_lag < PeriodType
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
			"s1_lag" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.s1_lag = value,
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

impl Default for WoodiesCCI {
	fn default() -> Self {
		Self {
			period1: 6,
			period2: 14,
			s1_lag: 6,
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone)]
pub struct WoodiesCCIInstance {
	cfg: WoodiesCCI,

	turbo: CCI,
	trend: CCI,
	s1_count: isize,
	s1_cross: Cross,
}

impl IndicatorInstance for WoodiesCCIInstance {
	type Config = WoodiesCCI;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let turbo = self.turbo.next(src) * SCALE;
		let trend = self.trend.next(src) * SCALE;

		let s1_cross = self.s1_cross.next((trend, 0.0)).analog();

		if s1_cross == 0 {
			self.s1_count += signi(trend) as isize;
		} else {
			self.s1_count = s1_cross as isize;
		}

		#[allow(clippy::cast_possible_wrap)]
		let s1 = (self.s1_count.abs() == self.cfg.s1_lag as isize) as i8 * s1_cross;

		IndicatorResult::new(&[turbo, trend], &[s1.into()])
	}
}
