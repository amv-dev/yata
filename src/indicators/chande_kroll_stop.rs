#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
// use std::str::FromStr;

use crate::core::{Action, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{Highest, Lowest, RMA};

//ChandeKrollStop p=10, x=1.0, q=9, version=1 {1,2,3}
//Индикатор не проверен и может иметь ошибки (Python-версия нерабочая)
// TODO: исправить

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChandeKrollStop {
	pub p: PeriodType,
	pub x: ValueType,
	pub q: PeriodType,
	pub source: Source,
	pub version: u8, // Version, // 1, 2 or 3
}

impl ChandeKrollStop {
	pub const VERSION1: u8 = 1;
	pub const VERSION2: u8 = 2;
	pub const VERSION3: u8 = 3;
}

impl IndicatorConfig for ChandeKrollStop {
	fn validate(&self) -> bool {
		self.x >= 0.0
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"p" => self.p = value.parse().unwrap(),
			"x" => self.x = value.parse().unwrap(),
			"q" => self.q = value.parse().unwrap(),
			"source" => self.source = value.parse().unwrap(),
			"version" => self.version = value.parse().unwrap(),

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

impl<T: OHLC> IndicatorInitializer<T> for ChandeKrollStop {
	type Instance = ChandeKrollStopInstance<T>;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		Self::Instance {
			rma: RMA::new(cfg.p, candle.high() - candle.low()),
			highest1: Highest::new(cfg.p, candle.high()),
			lowest1: Lowest::new(cfg.p, candle.low()),
			highest2: Highest::new(cfg.q, candle.high()),
			lowest2: Lowest::new(cfg.q, candle.low()),
			prev_candle: candle,
			cfg,
		}
	}
}

impl Default for ChandeKrollStop {
	fn default() -> Self {
		Self {
			p: 10,
			x: 1.0,
			q: 9,
			source: Source::Close,
			version: 1,
		}
	}
}

#[derive(Debug)]
pub struct ChandeKrollStopInstance<T: OHLC> {
	cfg: ChandeKrollStop,

	rma: RMA,
	highest1: Highest,
	lowest1: Lowest,
	highest2: Highest,
	lowest2: Lowest,
	prev_candle: T,
}

impl<T: OHLC> IndicatorInstance<T> for ChandeKrollStopInstance<T> {
	type Config = ChandeKrollStop;

	fn name(&self) -> &str {
		"ChandeKrollStop"
	}

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	#[allow(unreachable_code, unused_variables)]
	fn next(&mut self, candle: T) -> IndicatorResult {
		todo!("Проверить корректность реализации.");

		let tr = candle.tr(&self.prev_candle);
		self.prev_candle = candle;

		let atr = self.rma.next(tr);

		let highest = self.highest1;
		let lowest = self.lowest1;

		let first_high_stop;
		let first_low_stop;

		match self.cfg.version {
			Self::Config::VERSION1 => {
				first_high_stop = highest.next(candle.high()) - atr * self.cfg.x;
				first_low_stop = lowest.next(candle.low()) + atr * self.cfg.x;
			}
			Self::Config::VERSION2 => {
				first_high_stop = highest.next(candle.low()) - atr * self.cfg.x;
				first_low_stop = lowest.next(candle.high()) + atr * self.cfg.x;
			}
			Self::Config::VERSION3 => {
				first_low_stop = highest.next(candle.low()) - atr * self.cfg.x;
				first_high_stop = lowest.next(candle.high()) + atr * self.cfg.x;
			}
			_ => {
				first_high_stop = highest.next(candle.high()) - atr * self.cfg.x;
				first_low_stop = lowest.next(candle.low()) + atr * self.cfg.x;
			}
		};

		let stop_short = self.highest2.next(first_high_stop);
		let stop_long = self.lowest2.next(first_low_stop);

		let src = candle.source(self.cfg.source);
		let mut s = 0;

		if src > stop_long {
			s += 1;
		}

		if src < stop_short {
			s -= 1;
		}

		IndicatorResult::new(&[stop_long, stop_short], &[Action::from(s)])
	}
}
