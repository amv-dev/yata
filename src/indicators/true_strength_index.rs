#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::methods::{Cross, CrossAbove, CrossUnder, EMA, TSI};


/// True Strength Index
/// 
/// ## Links
/// 
/// * <https://en.wikipedia.org/wiki/True_strength_index>
/// 
/// # 2 values
/// 
/// * `main` value
/// 
/// Range in \[`-1.0`; `1.0`\]
/// 
/// * `signal line` value
/// 
/// Range in \[`-1.0`; `1.0`\]
/// 
/// # 3 signals
/// 
/// * Signal #1.
/// 
/// When `main` value crosses upper `zone` upwards , returns full sell signal.
/// When `main` value crosses lower `-zone` downwards, returns full buy signal.
/// Otherwise returns no signal.
/// 
/// * Signal #2.
/// When `main` value crosses zero line upwards, returns full buy signal.
/// When `main` value crosses zero line downwards, returns full sell signal.
/// Otherwise returns no signal.
/// 
/// * Signal #3.
/// When `main` value crosses `signal line` upwards, returns full buy signal.
/// When `main` value crosses `signal line` downwards, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrueStrengthIndex {
	/// Long TSI period. Default is `25`.
	/// 
	/// Range in \[`period2`, [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period1: PeriodType,

	/// Short TSI period. Default is `13`.
	/// 
	/// Range in \(`2`, `period1`\].
	pub period2: PeriodType,

	/// Signal line MA period. Default is `13`.
	/// 
	/// Range in \[`2`, [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period3: PeriodType,

	/// Signal zone size. Default is `0.25`.
	/// 
	/// Range in \[`0.0`; `1.0`]
	pub zone: ValueType,

	/// Source type of values. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for TrueStrengthIndex {
	type Instance = TrueStrengthIndexInstance;

	const NAME: &'static str = "TrueStrengthIndex";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			tsi: TSI::new(cfg.period2, cfg.period1, src)?,
			ema: EMA::new(cfg.period3, 0.)?,
			cross_under: CrossUnder::default(),
			cross_above: CrossAbove::default(),
			cross_over1: Cross::default(),
			cross_over2: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period2 > 1 && 
		self.period2 <= self.period1 &&
		self.period1 < PeriodType::MAX &&
		self.period3 > 1 &&
		self.period3 < PeriodType::MAX && 
		self.zone >= 0. && self.zone <= 1.
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
			"zone" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
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
		(2, 3)
	}
}

impl Default for TrueStrengthIndex {
	fn default() -> Self {
		Self {
			period1: 25,
			period2: 13,
			period3: 13,
			zone: 0.25,
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct TrueStrengthIndexInstance {
	cfg: TrueStrengthIndex,

	tsi: TSI,
	ema: EMA,
	cross_under: CrossUnder,
	cross_above: CrossAbove,
	cross_over1: Cross,
	cross_over2: Cross,
}

impl IndicatorInstance for TrueStrengthIndexInstance {
	type Config = TrueStrengthIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);
		
		let tsi = self.tsi.next(src);

		let sig = self.ema.next(tsi);

		let s1 = self.cross_under.next((tsi, -self.cfg.zone))
			- self.cross_above.next((tsi, self.cfg.zone));
		let s2 = self.cross_over1.next((tsi, 0.));
		let s3 = self.cross_over2.next((tsi, sig));

		IndicatorResult::new(&[tsi, sig], &[s1, s2, s3])
	}
}
