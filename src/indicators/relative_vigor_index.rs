#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Method, PeriodType, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, SMA, SWMA};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelativeVigorIndex {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub method: RegularMethods,
	pub zone: ValueType,
}

impl IndicatorConfig for RelativeVigorIndex {
	fn validate(&self) -> bool {
		self.period1 >= 2 && self.zone >= 0. && self.zone <= 1. && self.period3 > 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"period3" => self.period3 = value.parse().unwrap(),
			"method" => self.method = value.parse().unwrap(),
			"zone" => self.zone = value.parse().unwrap(),

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
		(2, 2)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for RelativeVigorIndex {
	type Instance = RelativeVigorIndexInstance;
	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let d_close = candle.close() - candle.open();
		let d_hl = candle.high() - candle.low();
		let rvi = if d_hl != 0. { d_close / d_hl } else { 0. };
		Self::Instance {
			prev_close: candle.open(),
			swma1: SWMA::new(cfg.period2, d_close),
			sma1: SMA::new(cfg.period1, d_close),
			swma2: SWMA::new(cfg.period2, d_hl),
			sma2: SMA::new(cfg.period1, d_hl),
			ma: method(cfg.method, cfg.period3, rvi),
			cross: Cross::default(),
			cfg,
		}
	}
}

impl Default for RelativeVigorIndex {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 4,
			period3: 4,
			method: RegularMethods::SWMA,
			zone: 0.25,
		}
	}
}

#[derive(Debug)]
pub struct RelativeVigorIndexInstance {
	cfg: RelativeVigorIndex,

	prev_close: ValueType,
	swma1: SWMA,
	sma1: SMA,
	swma2: SWMA,
	sma2: SMA,
	ma: RegularMethod,
	cross: Cross,
}

impl<T: OHLC> IndicatorInstance<T> for RelativeVigorIndexInstance {
	type Config = RelativeVigorIndex;

	fn name(&self) -> &str {
		"RelativeVigorIndex"
	}

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let close_open = candle.close() - self.prev_close;
		let high_low = candle.high() - candle.low();

		self.prev_close = candle.close();

		let swma1 = self.swma1.next(close_open);
		let sma1 = self.sma1.next(swma1);
		let swma2 = self.swma2.next(high_low);
		let sma2 = self.sma2.next(swma2);

		let rvi = if sma2 != 0. { sma1 / sma2 } else { 0. };
		let sig: ValueType = self.ma.next(rvi);

		// let s2;

		let s1 = self.cross.next((rvi, sig));

		// if s1.sign().unwrap_or_default() < 0 && rvi > self.cfg.zone && sig > self.cfg.zone {
		// 	s2 = 1;
		// } else if s1.sign().unwrap_or_default() > 0 && rvi < -self.cfg.zone && sig < -self.cfg.zone
		// {
		// 	s2 = -1;
		// } else {
		// 	s2 = 0;
		// }

		let s2 = (s1.sign().unwrap_or_default() < 0 && rvi > self.cfg.zone && sig > self.cfg.zone)
			as i8 - (s1.sign().unwrap_or_default() > 0
			&& rvi < -self.cfg.zone
			&& sig < -self.cfg.zone) as i8;

		IndicatorResult::new(&[rvi, sig], &[s1, Action::from(s2)])
	}
}
