#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{CrossAbove, CrossUnder, SMA};

/// Keltner Channel
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Keltner_channel>
///
/// # 3 values
///
/// * `upper bound`
///
/// Range of values is the same as the range of the `source` values.
///
/// * `source` value
/// * `lower bound`
///
/// Range of values is the same as the range of the `source` values.
///
/// # 1 digital signal
///
/// When `source` value goes above the `upper bound`, then returns full buy signal.
/// When `source` value goes under the `lower bound`, then returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeltnerChannel {
	/// Period for the middle moving average. Default is `20`.
	/// 
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period: PeriodType,
	
	/// Middle moving average type. Default is [`EMA`](crate::methods::EMA).
	pub method: RegularMethods,
	
	/// True range multiplier. Default is `1.0`.
	/// 
	/// Range in \(`0.0`; `+inf`\)
	pub sigma: ValueType,

	/// Middle moving average source value type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for KeltnerChannel {
	const NAME: &'static str = "KeltnerChannel";

	fn validate(&self) -> bool {
		self.period > 1 && self.sigma > 0.0
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"method" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
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

impl<T: OHLC> IndicatorInitializer<T> for KeltnerChannel {
	type Instance = KeltnerChannelInstance<T>;

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
			prev_candle: candle,
			ma: method(cfg.method, cfg.period, src)?,
			sma: SMA::new(cfg.period, candle.high() - candle.low())?,
			cross_above: CrossAbove::default(),
			cross_under: CrossUnder::default(),
			cfg,
		})
	}
}

impl Default for KeltnerChannel {
	fn default() -> Self {
		Self {
			period: 20,
			sigma: 1.0,
			source: Source::Close,
			method: RegularMethods::EMA,
		}
	}
}

#[derive(Debug)]
pub struct KeltnerChannelInstance<T: OHLC> {
	cfg: KeltnerChannel,

	prev_candle: T,
	ma: RegularMethod,
	sma: SMA,
	cross_above: CrossAbove,
	cross_under: CrossUnder,
}

impl<T: OHLC> IndicatorInstance<T> for KeltnerChannelInstance<T> {
	type Config = KeltnerChannel;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let source = candle.source(self.cfg.source);
		let tr = candle.tr(&self.prev_candle);
		let ma: ValueType = self.ma.next(source);
		let atr = self.sma.next(tr);

		let upper = atr.mul_add(self.cfg.sigma, ma);
		let lower = ma - atr * self.cfg.sigma;

		let signal =
			self.cross_under.next((source, lower)) - self.cross_above.next((source, upper));

		IndicatorResult::new(&[source, upper, lower], &[signal])
	}
}
