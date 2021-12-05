#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, MovingAverageConstructor, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
use crate::methods::Cross;
use std::mem::replace;

/// Relative Strength Index
///
/// ## Links:
///
/// * <https://en.wikipedia.org/wiki/Relative_strength_index>
///
/// # 1 value
///
/// * `main` value
///
/// Range in \[`0.0`; `1.0`\]
///
/// # 2 signals
///
/// * Signal #1 on enters over-zone.
///
/// When main value crosses upper zone upwards, returns full sell signal.
/// When main value crosses lower zone downwards, returns full buy signal.
/// Otherwise returns no signal.
///
/// * Signal #2 on leaves over-zone.
///
/// When main value crosses upper zone downwards, returns full sell signal.
/// When main value crosses lower zone upwards, returns full buy signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelativeStrengthIndex<M: MovingAverageConstructor = MA> {
	/// Main MA type.
	///
	/// Default is [`EMA(14)`](crate::methods::EMA)
	///
	/// Period range in \[`3`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub ma: M,

	/// Overbought/oversell relative zone. Default is `0.3`.
	///
	/// Range in \(`0.0`; `0.5`\]
	pub zone: ValueType,

	/// Source type of values. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for RelativeStrengthIndex<M> {
	type Instance = RelativeStrengthIndexInstance<M>;

	const NAME: &'static str = "RelativeStrengthIndex";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			previous_input: src,
			posma: cfg.ma.init(0.)?,
			negma: cfg.ma.init(0.)?,
			cross_upper: Cross::new((), &(0.5, 1.0 - cfg.zone))?,
			cross_lower: Cross::new((), &(0.5, cfg.zone))?,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.ma.ma_period() > 2 && self.zone > 0. && self.zone <= 0.5
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"ma" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma = value,
			},
			"zone" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
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
		(1, 2)
	}
}

impl Default for RelativeStrengthIndex {
	fn default() -> Self {
		Self {
			ma: MA::EMA(14),
			zone: 0.3,
			source: Source::Close,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelativeStrengthIndexInstance<M: MovingAverageConstructor = MA> {
	cfg: RelativeStrengthIndex<M>,

	previous_input: ValueType,
	posma: M::Instance,
	negma: M::Instance,
	cross_upper: Cross,
	cross_lower: Cross,
}

/// Just an alias for `RelativeStrengthIndex`
pub type RSI<M = MA> = RelativeStrengthIndex<M>;

impl<M: MovingAverageConstructor> IndicatorInstance for RelativeStrengthIndexInstance<M> {
	type Config = RelativeStrengthIndex<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let change = src - replace(&mut self.previous_input, src);

		let pos: ValueType = self.posma.next(&change.max(0.));
		let neg: ValueType = self.negma.next(&change.min(0.)) * -1.;

		let value = if pos != 0. || neg != 0. {
			debug_assert!(pos + neg != 0.);
			pos / (pos + neg)
		} else {
			0.5
		};

		let oversold = self.cross_lower.next(&(value, self.cfg.zone)).analog();
		let overbought = self.cross_upper.next(&(value, 1. - self.cfg.zone)).analog();

		let signal1 = (oversold < 0) as i8 - (overbought > 0) as i8;
		let signal2 = (oversold > 0) as i8 - (overbought < 0) as i8;

		IndicatorResult::new(&[value], &[signal1.into(), signal2.into()])
	}
}
