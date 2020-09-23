#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, PeriodType, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AverageDirectionalIndex {
	pub method1: RegularMethods,
	pub di_length: PeriodType,

	pub method2: RegularMethods,
	pub adx_smoothing: PeriodType,

	pub period1: PeriodType,
	pub zone: ValueType,
}

impl IndicatorConfig for AverageDirectionalIndex {
	fn validate(&self) -> bool {
		self.di_length >= 1
			&& self.adx_smoothing >= 1
			&& self.zone >= 0.
			&& self.zone <= 1.
			&& self.period1 >= 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"method1" => self.method1 = value.parse().unwrap(),
			"di_length" => self.di_length = value.parse().unwrap(),

			"method2" => self.method2 = value.parse().unwrap(),
			"adx_smoothing" => self.adx_smoothing = value.parse().unwrap(),

			"period1" => self.period1 = value.parse().unwrap(),
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
		(3, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for AverageDirectionalIndex {
	type Instance = AverageDirectionalIndexInstance<T>;
	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let tr = candle.tr(&candle);

		Self::Instance {
			prev_candle: candle,
			window: Window::new(cfg.period1, candle),
			tr_ma: method(cfg.method1, cfg.di_length, tr),
			plus_dm: method(cfg.method1, cfg.di_length, 0.0),
			minus_dm: method(cfg.method1, cfg.di_length, 0.0),
			ma2: method(cfg.method2, cfg.adx_smoothing, 0.0),
			cfg,
		}
	}
}

impl Default for AverageDirectionalIndex {
	fn default() -> Self {
		Self {
			method1: RegularMethods::RMA,
			di_length: 14,
			method2: RegularMethods::RMA,
			adx_smoothing: 14,
			period1: 1,
			zone: 0.2,
		}
	}
}

#[derive(Debug)]
pub struct AverageDirectionalIndexInstance<T: OHLC> {
	cfg: AverageDirectionalIndex,

	prev_candle: T,
	window: Window<T>,
	tr_ma: RegularMethod,
	plus_dm: RegularMethod,
	minus_dm: RegularMethod,
	ma2: RegularMethod,
}

impl<T: OHLC> AverageDirectionalIndexInstance<T> {
	fn dir_mov(&mut self, candle: T) -> (ValueType, ValueType) {
		let tr_ma = &mut self.tr_ma;
		let plus_dm = &mut self.plus_dm;
		let minus_dm = &mut self.minus_dm;

		let true_range = tr_ma.next(candle.tr(&self.prev_candle));
		let left_candle = self.window.push(candle);

		// prevIndex = zeroIndex(index - int(a.Period1))
		// prevCandle = a.candles[prevIndex]

		let (du, dd) = (
			candle.high() - left_candle.high(),
			left_candle.low() - candle.low(),
		);

		let plus_dm_value = if du > dd && du > 0. {
			plus_dm.next(du)
		} else {
			plus_dm.next(0.)
		};

		let minus_dm_value = if dd > du && dd > 0. {
			minus_dm.next(dd)
		} else {
			minus_dm.next(0.)
		};

		self.prev_candle = candle;

		(plus_dm_value / true_range, minus_dm_value / true_range)
	}

	fn adx(&mut self, plus: ValueType, minus: ValueType) -> ValueType {
		let s = plus + minus;

		let ma2 = &mut self.ma2;

		if s == 0. {
			return ma2.next(0.);
		}

		let t = (plus - minus).abs() / s;
		ma2.next(t)
	}
}

impl<T: OHLC> IndicatorInstance<T> for AverageDirectionalIndexInstance<T> {
	type Config = AverageDirectionalIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let (plus, minus) = self.dir_mov(candle);
		let adx = self.adx(plus, minus);

		// let signal: i8 = if adx > self.cfg.zone {
		// 	if plus > minus {
		// 		1
		// 	} else if plus < minus {
		// 		-1
		// 	} else {
		// 		0
		// 	}
		// } else {
		// 	0
		// };

		let signal = (adx > self.cfg.zone) as i8 * ((plus > minus) as i8 - (plus < minus) as i8);

		let values = [adx, plus, minus];
		let signals = [Action::from(signal)];

		IndicatorResult::new(&values, &signals)
	}
}
