#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{Change, Cross, CrossAbove, CrossUnder, EMA};

// https://en.wikipedia.org/wiki/Trix_(technical_analysis)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrueStrengthIndex {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub zone: ValueType,
	pub source: Source,
}

impl IndicatorConfig for TrueStrengthIndex {
	const NAME: &'static str = "TrueStrengthIndex";

	fn validate(&self) -> bool {
		self.period1 > 2 && self.zone >= 0. && self.zone <= 1.
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"period3" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period3 = value,
			},
			"zone" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value.to_string()));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(2, 3)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for TrueStrengthIndex {
	type Instance = TrueStrengthIndexInstance;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			change: Change::new(1, src)?,
			ema11: EMA::new(cfg.period1, 0.)?,
			ema12: EMA::new(cfg.period2, 0.)?,
			ema21: EMA::new(cfg.period1, 0.)?,
			ema22: EMA::new(cfg.period2, 0.)?,
			ema: EMA::new(cfg.period3, 0.)?,
			cross_under: CrossUnder::default(),
			cross_above: CrossAbove::default(),
			cross_over1: Cross::default(),
			cross_over2: Cross::default(),
			cfg,
		})
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

#[derive(Debug)]
pub struct TrueStrengthIndexInstance {
	cfg: TrueStrengthIndex,

	change: Change,
	ema11: EMA,
	ema12: EMA,
	ema21: EMA,
	ema22: EMA,
	ema: EMA,
	cross_under: CrossUnder,
	cross_above: CrossAbove,
	cross_over1: Cross,
	cross_over2: Cross,
}

impl<T: OHLC> IndicatorInstance<T> for TrueStrengthIndexInstance {
	type Config = TrueStrengthIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);
		let m1 = self.change.next(src);
		let m2 = m1.abs();
		let ema11 = self.ema11.next(m1);
		let ema12 = self.ema12.next(ema11);
		let ema21 = self.ema21.next(m2);
		let ema22 = self.ema22.next(ema21);

		let value = if ema22 != 0. { ema12 / ema22 } else { 0. };

		let sig = self.ema.next(value);

		let s1 = self.cross_under.next((value, -self.cfg.zone))
			- self.cross_above.next((value, self.cfg.zone));
		let s2 = self.cross_over1.next((value, 0.));
		let s3 = self.cross_over2.next((value, sig));

		IndicatorResult::new(&[value, sig], &[s1, s2, s3])
	}
}
