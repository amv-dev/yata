#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{StDev, SMA};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BollingerBands {
	pub avg_size: PeriodType,
	pub sigma: ValueType,
	pub source: Source,
}

impl IndicatorConfig for BollingerBands {
	const NAME: &'static str = "BollingerBands";

	fn validate(&self) -> bool {
		self.sigma >= 0.0 && self.avg_size > 2
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"avg_size" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.avg_size = value,
			},
			"sigma" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.sigma = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(3, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for BollingerBands {
	type Instance = BollingerBandsInstance;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = T::source(&candle, cfg.source);
		Ok(Self::Instance {
			ma: SMA::new(cfg.avg_size, src)?,
			st_dev: StDev::new(cfg.avg_size, src)?,
			cfg,
		})
	}
}

impl Default for BollingerBands {
	fn default() -> Self {
		Self {
			avg_size: 20,
			sigma: 2.0,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct BollingerBandsInstance {
	cfg: BollingerBands,

	ma: SMA,
	st_dev: StDev,
}

impl<T: OHLC> IndicatorInstance<T> for BollingerBandsInstance {
	type Config = BollingerBands;

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let source = candle.source(self.cfg.source);
		let middle = self.ma.next(source);
		let sq_error = self.st_dev.next(source);

		let upper = middle + sq_error * self.cfg.sigma;
		let lower = middle - sq_error * self.cfg.sigma;

		// let signal = if source >= upper {
		// 	1
		// } else if source <= lower {
		// 	-1
		// } else {
		// 	0
		// };
		let signal = (source >= upper) as i8 - (source <= lower) as i8;

		let values = [upper, middle, lower];
		let signals = [Action::from(signal)];
		IndicatorResult::new(&values, &signals)
	}
}
