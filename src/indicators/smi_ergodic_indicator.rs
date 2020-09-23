#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, Source, ValueType, OHLC};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Change, Cross, EMA};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SMIErgodicIndicator {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub method: RegularMethods,
	pub source: Source,
}

impl IndicatorConfig for SMIErgodicIndicator {
	fn validate(&self) -> bool {
		self.period1 > 1 && self.period2 > 1 && self.period3 > 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"period3" => self.period3 = value.parse().unwrap(),
			"method" => self.method = value.parse().unwrap(),
			"source" => self.source = value.parse().unwrap(),

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

impl<T: OHLC> IndicatorInitializer<T> for SMIErgodicIndicator {
	type Instance = SMIErgodicIndicatorInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let src = candle.source(cfg.source);
		Self::Instance {
			change: Change::new(1, src),
			ema11: EMA::new(cfg.period1, 0.),
			ema12: EMA::new(cfg.period2, 0.),
			ema21: EMA::new(cfg.period1, 0.),
			ema22: EMA::new(cfg.period2, 0.),
			ma: method(cfg.method, cfg.period3, 0.),
			cross: Cross::default(),
			cfg,
		}
	}
}

impl Default for SMIErgodicIndicator {
	fn default() -> Self {
		Self {
			period1: 5,
			period2: 20,
			period3: 5,
			method: RegularMethods::EMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct SMIErgodicIndicatorInstance {
	cfg: SMIErgodicIndicator,

	change: Change,
	ema11: EMA,
	ema12: EMA,
	ema21: EMA,
	ema22: EMA,
	ma: RegularMethod,
	cross: Cross,
}

impl<T: OHLC> IndicatorInstance<T> for SMIErgodicIndicatorInstance {
	type Config = SMIErgodicIndicator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);
		let change = self.change.next(src);

		let temp_change = self.ema12.next(self.ema11.next(change));

		let temp_abs_change = self.ema22.next(self.ema21.next(change.abs()));

		let smi = if temp_abs_change > 0. {
			temp_change / temp_abs_change
		} else {
			0.
		};
		let sig: ValueType = self.ma.next(smi);

		let signal = self.cross.next((smi, sig));

		IndicatorResult::new(&[smi, sig], &[signal])
	}
}
