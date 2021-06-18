#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, PeriodType, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};

/// Envelopes
///
/// ## Links
///
/// * <https://www.investopedia.com/terms/e/envelope.asp>
///
/// # 3 values
///
/// * `Upper bound`
///
/// Range of values is the same as the range of the `source` values.
///
/// * `Lower bound`
///
/// Range of values is the same as the range of the `source` values.Action
///
/// *  Raw `Source2` value
///
/// # 1 signal
///
/// * Signal 1 appears when `Source2` value crosses bounds.
/// When `Source2` value crosses `upper bound` upwards, returns full sell signal.
/// When `Source2` value crosses `lower bound` downwards, returns full buy signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Envelopes {
	/// MA period length. Default is `20`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period: PeriodType,
	/// Bound relative size. Default is `0.1`.
	///
	/// Range in (`0.0`; `+inf`).
	pub k: ValueType,
	/// MA method. Default is [`SMA`](crate::methods::SMA).
	pub method: RegularMethods,
	/// Source value type for bounds. Default is [`Close`](crate::core::Source::Close).
	pub source: Source,
	/// Source2 value type for actual price. Default is [`Close`](crate::core::Source::Close).
	pub source2: Source,
}

impl IndicatorConfig for Envelopes {
	type Instance = EnvelopesInstance;

	const NAME: &'static str = "Envelopes";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
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

	fn validate(&self) -> bool {
		self.k > 0.0 && self.period > 1
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"k" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.k = value,
			},
			"method" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
			},
			"source" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},
			"source2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source2 = value,
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

impl IndicatorInstance for EnvelopesInstance {
	type Config = Envelopes;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);
		let v = self.ma.next(&src);

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

		IndicatorResult::new(&[value1, value2, src2], &[Action::from(signal)])
	}
}
