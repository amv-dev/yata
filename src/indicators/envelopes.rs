#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, PeriodType, Source, ValueType, OHLC};
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
	fn validate(&self) -> bool {
		true
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period" => self.period = value.parse().unwrap(),
			"k" => self.k = value.parse().unwrap(),
			"method" => self.method = value.parse().unwrap(),
			"source" => self.source = value.parse().unwrap(),
			"source2" => self.source2 = value.parse().unwrap(),

			_ => {
				dbg!(format!(
					"Unknown attribute `{:}` with value `{:}` for `{:}`",
					name,
					value,
					std::any::type_name::<Self>(),
				));
			}
		};
	}

	fn size(&self) -> (u8, u8) {
		(2, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for Envelopes {
	type Instance = EnvelopesInstance;
	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let src = candle.source(cfg.source);

		Self::Instance {
			ma: method(cfg.method, cfg.period, src),
			k_high: 1.0 + cfg.k,
			k_low: 1.0 - cfg.k,
			cfg,
		}
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

	fn name(&self) -> &str {
		"Envelopes"
	}

	#[inline]
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
