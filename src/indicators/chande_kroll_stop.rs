#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
// use std::str::FromStr;

use crate::core::{Action, Error, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Highest, Lowest};

// ChandeKrollStop p=10, x=1.0, q=9
/// [Chande Kroll Stop](https://patternswizard.com/chande-kroll-stop-indicator/)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChandeKrollStop {
	/// ATR period length. Default is 10.
	pub p: PeriodType,
	/// ATR method. Default is [`SMA`](crate::methods::SMA).
	pub method: RegularMethods,
	/// ATR multiplier. Default is 1.0.
	pub x: ValueType,
	/// multiplied highest/lowest period length. Default is 9.
	pub q: PeriodType,
	/// price source. Default is [`Close`](crate::core::Source#variant.Close)
	pub source: Source,
}

impl IndicatorConfig for ChandeKrollStop {
	const NAME: &'static str = "ChandeKrollStop";

	fn validate(&self) -> bool {
		self.x >= 0.0
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"p" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.p = value,
			},
			"x" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.x = value,
			},
			"q" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.q = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(3, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for ChandeKrollStop {
	type Instance = ChandeKrollStopInstance<T>;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			ma: method(cfg.method, cfg.p, candle.tr(&candle))?,

			highest1: Highest::new(cfg.p, candle.high())?,
			lowest1: Lowest::new(cfg.p, candle.low())?,

			highest2: Highest::new(cfg.q, candle.high())?,
			lowest2: Lowest::new(cfg.q, candle.low())?,

			prev_candle: candle,
			cfg,
		})
	}
}

impl Default for ChandeKrollStop {
	fn default() -> Self {
		Self {
			p: 10,
			method: RegularMethods::SMA,
			x: 1.0,
			q: 9,
			source: Source::Close,
		}
	}
}

/// Chande Kroll Stop state structure
#[derive(Debug)]
pub struct ChandeKrollStopInstance<T: OHLC> {
	cfg: ChandeKrollStop,

	ma: RegularMethod,
	highest1: Highest,
	lowest1: Lowest,
	highest2: Highest,
	lowest2: Lowest,
	prev_candle: T,
}

impl<T: OHLC> IndicatorInstance<T> for ChandeKrollStopInstance<T> {
	type Config = ChandeKrollStop;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	#[allow(clippy::similar_names)]
	fn next(&mut self, candle: T) -> IndicatorResult {
		let tr = candle.tr(&self.prev_candle);
		self.prev_candle = candle;

		let atr = self.ma.next(tr);

		let highest = &mut self.highest1;
		let lowest = &mut self.lowest1;

		let phs = highest.next(candle.high()) - atr * self.cfg.x;
		let pls = atr.mul_add(self.cfg.x, lowest.next(candle.low()));

		let stop_short = self.highest2.next(phs);
		let stop_long = self.lowest2.next(pls);

		let src = candle.source(self.cfg.source);

		let mid = (stop_short + stop_long) * 0.5;
		let size = mid - stop_long;
		let value = (src - mid) / size;

		IndicatorResult::new(&[stop_long, src, stop_short], &[Action::from(value)])
	}
}
