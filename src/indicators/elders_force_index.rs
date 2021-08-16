#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Candle, MovingAverageConstructor};
use crate::core::{Error, Method, PeriodType, Source, ValueType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
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
/// * Main value
///
/// Range in \(`-inf`; `+inf`\)
///
/// # 1 signal
///
/// * Signal 1 appears when `main value` crosses zero line.
/// When `main value` crosses zero line upwards, returns full buy signal.
/// When `main value` crosses zero line downwards, returns full sell signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EldersForceIndex<M: MovingAverageConstructor = MA> {
	pub ma: M,
	/*
	/// MA period. Default is `13`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period1: PeriodType,
	*/
	/// Price change period. Default is `1`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period2: PeriodType,
	/*
	/// MA method. Default is [`EMA`](crate::methods::EMA).
	pub method: RegularMethods,
	*/
	/// Price source type of values. Default is [`Close`](crate::core::Source::Close).
	pub source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for EldersForceIndex<M> {
	type Instance = EldersForceIndexInstance<M>;

	const NAME: &'static str = "EldersForceIndex";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			ma: cfg.ma.init(0.)?,// method(cfg.method, cfg.period1, 0.)?,
			window: Window::new(cfg.period2, Candle::from(candle)),
			vol_sum: candle.volume() * cfg.period2 as ValueType,
			cross_over: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.ma.ma_period() > 1 && self.period2 >= 1
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"ma" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
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

impl Default for EldersForceIndex<MA> {
	fn default() -> Self {
		Self {
			ma: MA::EMA(13),
			// period1: 13,
			period2: 1,
			// method: RegularMethods::EMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EldersForceIndexInstance<M: MovingAverageConstructor = MA> {
	cfg: EldersForceIndex<M>,

	ma: M::Instance,
	window: Window<Candle>,
	vol_sum: ValueType,
	cross_over: Cross,
}

impl<M: MovingAverageConstructor> IndicatorInstance for EldersForceIndexInstance<M> {
	type Config = EldersForceIndex<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let left_candle = self.window.push(Candle::from(candle));

		self.vol_sum += candle.volume() - left_candle.volume();
		let r = (OHLCV::source(candle, self.cfg.source)
			- OHLCV::source(&left_candle, self.cfg.source))
			* self.vol_sum;

		let value = self.ma.next(&r);
		let signal = self.cross_over.next(&(value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
