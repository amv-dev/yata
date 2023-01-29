#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, MovingAverageConstructor, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
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
pub struct KeltnerChannel<M: MovingAverageConstructor = MA> {
	/// Middle moving average type.
	///
	/// Default is [`EMA(20)`](crate::methods::EMA).
	///
	/// Period range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub ma: M,

	/// True range multiplier. Default is `1.0`.
	///
	/// Range in \(`0.0`; `+inf`\)
	pub sigma: ValueType,

	/// Middle moving average source value type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for KeltnerChannel<M> {
	type Instance = KeltnerChannelInstance<M>;

	const NAME: &'static str = "KeltnerChannel";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);
		Ok(Self::Instance {
			prev_close: candle.close(),
			ma: cfg.ma.init(src)?, // method(cfg.method, cfg.period, src)?,
			sma: SMA::new(cfg.ma.ma_period(), &(candle.high() - candle.low()))?,
			cross_above: CrossAbove::default(),
			cross_under: CrossUnder::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.ma.ma_period() > 1 && self.sigma > 0.0
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"ma" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma = value,
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

impl Default for KeltnerChannel<MA> {
	fn default() -> Self {
		Self {
			ma: MA::EMA(20),
			sigma: 1.0,
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeltnerChannelInstance<M: MovingAverageConstructor = MA> {
	cfg: KeltnerChannel<M>,

	prev_close: ValueType,
	ma: M::Instance,
	sma: SMA,
	cross_above: CrossAbove,
	cross_under: CrossUnder,
}

impl<M: MovingAverageConstructor> IndicatorInstance for KeltnerChannelInstance<M> {
	type Config = KeltnerChannel<M>;

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
		let lower = atr.mul_add(-self.cfg.sigma, ma);

		let signal =
			self.cross_under.next(&(source, lower)) - self.cross_above.next(&(source, upper));

		IndicatorResult::new(&[source, upper, lower], &[signal])
	}
}
