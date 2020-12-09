#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Change, Cross, EMA};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SMIErgodicIndicator {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub method: RegularMethods,
	pub source: Source,
}

impl IndicatorConfig for SMIErgodicIndicator {
	type Instance = SMIErgodicIndicatorInstance;

	const NAME: &'static str = "SMIErgodicIndicator";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
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
			ma: method(cfg.method, cfg.period3, 0.)?,
			cross: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period1 > 1 && self.period2 > 1 && self.period3 > 1
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
			"method" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
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

impl Default for SMIErgodicIndicator {
	fn default() -> Self {
		Self {
			period1: 5,
			period2: 20,
			period3: 5,
			method: RegularMethods::EMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct SMIErgodicIndicatorInstance {
	cfg: SMIErgodicIndicator,

	change: Change,
	ema11: EMA,
	ema12: EMA,
	ema21: EMA,
	ema22: EMA,
	ma: RegularMethod,
	cross: Cross,
}

impl IndicatorInstance for SMIErgodicIndicatorInstance {
	type Config = SMIErgodicIndicator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);
		let change = self.change.next(src);

		let temp_change = self.ema12.next(self.ema11.next(change));

		let temp_abs_change = self.ema22.next(self.ema21.next(change.abs()));

		let smi = if temp_abs_change > 0. {
			temp_change / temp_abs_change
		} else {
			0.
		};
		let sig: ValueType = self.ma.next(smi);

		let signal = self.cross.next((smi, sig));

		IndicatorResult::new(&[smi, sig], &[signal])
	}
}
