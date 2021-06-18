#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
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
/// # 1 signal
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
	type Instance = KeltnerChannelInstance;

	const NAME: &'static str = "KeltnerChannel";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);
		Ok(Self::Instance {
			prev_close: candle.close(),
			ma: method(cfg.method, cfg.period, src)?,
			sma: SMA::new(cfg.period, &(candle.high() - candle.low()))?,
			cross_above: CrossAbove::default(),
			cross_under: CrossUnder::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period > 1 && self.sigma > 0.0
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"method" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
			},
			"sigma" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.sigma = value,
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
		(3, 1)
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
pub struct KeltnerChannelInstance {
	cfg: KeltnerChannel,

	prev_close: ValueType,
	ma: RegularMethod,
	sma: SMA,
	cross_above: CrossAbove,
	cross_under: CrossUnder,
}

impl IndicatorInstance for KeltnerChannelInstance {
	type Config = KeltnerChannel;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let source = candle.source(self.cfg.source);
		let tr = candle.tr_close(self.prev_close);
		self.prev_close = candle.close();

		let ma: ValueType = self.ma.next(&source);
		let atr = self.sma.next(&tr);

		let upper = atr.mul_add(self.cfg.sigma, ma);
		let lower = ma - atr * self.cfg.sigma;

		let signal =
			self.cross_under.next(&(source, lower)) - self.cross_above.next(&(source, upper));

		IndicatorResult::new(&[source, upper, lower], &[signal])
	}
}
