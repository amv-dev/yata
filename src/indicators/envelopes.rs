#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Envelopes {
	pub period: PeriodType,
	pub k: ValueType,
	pub method: RegularMethods,
	pub source: Source,
	pub source2: Source,
}

impl IndicatorConfig for Envelopes {
	const NAME: &'static str = "Envelopes";

	fn validate(&self) -> bool {
		true
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"k" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.k = value,
			},
			"method" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},
			"source2" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source2 = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(2, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for Envelopes {
	type Instance = EnvelopesInstance;
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
			ma: method(cfg.method, cfg.period, src)?,
			k_high: 1.0 + cfg.k,
			k_low: 1.0 - cfg.k,
			cfg,
		})
	}
}

impl Default for Envelopes {
	fn default() -> Self {
		Self {
			period: 20,
			k: 0.1,
			method: RegularMethods::SMA,
			source: Source::Close,
			source2: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct EnvelopesInstance {
	cfg: Envelopes,

	ma: RegularMethod,
	k_high: ValueType,
	k_low: ValueType,
}

impl<T: OHLC> IndicatorInstance<T> for EnvelopesInstance {
	type Config = Envelopes;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);
		let v = self.ma.next(src);

		let (value1, value2) = (v * self.k_high, v * self.k_low);

		let src2 = candle.source(self.cfg.source2);
		// let signal = if src2 < value2 {
		// 	1
		// } else if src2 > value1 {
		// 	-1
		// } else {
		// 	0
		// };

		let signal = (src2 < value2) as i8 - (src2 > value1) as i8;

		IndicatorResult::new(&[value1, value2], &[Action::from(signal)])
	}
}
