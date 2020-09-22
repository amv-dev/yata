#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, ValueType, OHLC};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, RateOfChange};

// https://en.wikipedia.org/wiki/KST_oscillator
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KnowSureThing {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub period4: PeriodType,
	pub sma1: PeriodType,
	pub sma2: PeriodType,
	pub sma3: PeriodType,
	pub sma4: PeriodType,
	pub method1: RegularMethods,

	pub sma5: PeriodType,
	pub method2: RegularMethods,
}

impl IndicatorConfig for KnowSureThing {
	fn validate(&self) -> bool {
		self.period1 < self.period2 && self.period2 < self.period3 && self.period3 < self.period4
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"period3" => self.period3 = value.parse().unwrap(),
			"period4" => self.period4 = value.parse().unwrap(),
			"sma1" => self.sma1 = value.parse().unwrap(),
			"sma2" => self.sma2 = value.parse().unwrap(),
			"sma3" => self.sma3 = value.parse().unwrap(),
			"sma4" => self.sma4 = value.parse().unwrap(),
			"sma5" => self.sma5 = value.parse().unwrap(),
			"method1" => self.method1 = value.parse().unwrap(),
			"method2" => self.method2 = value.parse().unwrap(),

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

impl<T: OHLC> IndicatorInitializer<T> for KnowSureThing {
	type Instance = KnowSureThingInstance;
	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let close = candle.close();
		Self::Instance {
			roc1v: RateOfChange::new(cfg.period1, close),
			roc2v: RateOfChange::new(cfg.period2, close),
			roc3v: RateOfChange::new(cfg.period3, close),
			roc4v: RateOfChange::new(cfg.period4, close),
			ma1: method(cfg.method1, cfg.sma1, 0.),
			ma2: method(cfg.method1, cfg.sma2, 0.),
			ma3: method(cfg.method1, cfg.sma3, 0.),
			ma4: method(cfg.method1, cfg.sma4, 0.),
			ma5: method(cfg.method2, cfg.sma5, 0.),
			cross: Cross::default(),
			cfg,
		}
	}
}

impl Default for KnowSureThing {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 15,
			period3: 20,
			period4: 30,
			sma1: 10,
			sma2: 10,
			sma3: 10,
			sma4: 15,
			sma5: 9,
			method1: RegularMethods::SMA,
			method2: RegularMethods::SMA,
		}
	}
}

#[derive(Debug)]
pub struct KnowSureThingInstance {
	cfg: KnowSureThing,

	roc1v: RateOfChange,
	roc2v: RateOfChange,
	roc3v: RateOfChange,
	roc4v: RateOfChange,
	ma1: RegularMethod,
	ma2: RegularMethod,
	ma3: RegularMethod,
	ma4: RegularMethod,
	ma5: RegularMethod,
	cross: Cross,
}

impl<T: OHLC> IndicatorInstance<T> for KnowSureThingInstance {
	type Config = KnowSureThing;

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let close = candle.close();

		let roc1: ValueType = self.roc1v.next(close);
		let roc2: ValueType = self.roc2v.next(close);
		let roc3: ValueType = self.roc3v.next(close);
		let roc4: ValueType = self.roc4v.next(close);

		let rcma1: ValueType = self.ma1.next(roc1);
		let rcma2: ValueType = self.ma2.next(roc2);
		let rcma3: ValueType = self.ma3.next(roc3);
		let rcma4: ValueType = self.ma4.next(roc4);

		let kst = rcma1 + rcma2 * 2. + rcma3 * 3. + rcma4 * 4.;
		let sl: ValueType = self.ma5.next(kst);

		let signal = self.cross.next((kst, sl));

		IndicatorResult::new(&[kst, sl], &[signal])
	}
}
