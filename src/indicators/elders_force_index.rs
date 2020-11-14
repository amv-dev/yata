#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, Window, OHLC, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::Cross;

/// Elders Force Index
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Force_index>
/// * <https://www.investopedia.com/terms/f/force-index.asp>
///
/// # 1 value
///
/// * Main value \(range in \(-inf; +inf\)\)
///
/// # 1 signal
///
/// * Signal 1 appears when `main value` crosses zero line.
/// When `main value` crosses zero line upwards, returns full buy signal.
/// When `main value` crosses zero line downwards, returns full sell signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EldersForceIndex {
	/// MA period. Default is 13.
	/// 
	/// Range in \[2; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period1: PeriodType,
	/// Price change period. Default is 1.
	/// 
	/// Range in \[1; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period2: PeriodType,
	/// MA method. Default is [`EMA`](crate::methods::EMA)
	pub method: RegularMethods,
	/// Price source type of values. Default is [`Close`](crate::core::Source#variant.Close)
	pub source: Source,
}

impl IndicatorConfig for EldersForceIndex {
	const NAME: &'static str = "EldersForceIndex";

	fn validate(&self) -> bool {
		self.period1 > 1 && self.period2 >= 1
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
			"method" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
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

	fn is_volume_based(&self) -> bool {
		true
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl<T: OHLCV> IndicatorInitializer<T> for EldersForceIndex {
	type Instance = EldersForceIndexInstance<T>;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			ma: method(cfg.method, cfg.period1, 0.)?,
			window: Window::new(cfg.period2, candle),
			vol_sum: candle.volume() * cfg.period2 as ValueType,
			cross_over: Cross::default(),
			cfg,
		})
	}
}

impl Default for EldersForceIndex {
	fn default() -> Self {
		Self {
			period1: 13,
			period2: 1,
			method: RegularMethods::EMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct EldersForceIndexInstance<T: OHLCV> {
	cfg: EldersForceIndex,

	ma: RegularMethod,
	window: Window<T>,
	vol_sum: ValueType,
	cross_over: Cross,
}

impl<T: OHLCV> IndicatorInstance<T> for EldersForceIndexInstance<T> {
	type Config = EldersForceIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let left_candle = self.window.push(candle);

		self.vol_sum += candle.volume() - left_candle.volume();
		let r = (OHLC::source(&candle, self.cfg.source)
			- OHLC::source(&left_candle, self.cfg.source))
			* self.vol_sum;

		let value = self.ma.next(r);
		let signal = self.cross_over.next((value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
