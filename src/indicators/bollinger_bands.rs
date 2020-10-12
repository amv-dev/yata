#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{StDev, SMA};

/// [Bollinget Bands](https://en.wikipedia.org/wiki/Bollinger_Bands)
///
/// # 3 values
///
/// * upper bound
/// * source value
/// * lower bound
///
/// # 1 digital signal
///
/// When `source` value goes above the upper bound, then returns full buy signal.
/// When `source` value goes under the lower bound, then returns full sell signal.
/// Otherwise returns relative position of the `source` value based on upper and lower values.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BollingerBands {
	/// Main period length. Default is 20
	pub avg_size: PeriodType,
	/// Standart deviation multiplier for bounds. Default is 2.0
	pub sigma: ValueType,
	/// Source type of values. Default is [Close](crate::core::Source#variant.Close)
	pub source: Source,
}

impl IndicatorConfig for BollingerBands {
	const NAME: &'static str = "BollingerBands";

	fn validate(&self) -> bool {
		self.sigma > 0.0 && self.avg_size > 2
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
				return Some(Error::ParameterParse(name.to_string(), value.to_string()));
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

		let values = [upper, middle, lower];

		let range = upper - lower;
		let relative = if range == 0.0 {
			(source - lower) / range
		} else {
			0.0
		};

		let signals = [Action::from(relative * 2.0 - 1.0)];
		IndicatorResult::new(&values, &signals)
	}
}
