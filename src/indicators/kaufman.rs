#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Method, PeriodType, Source, ValueType, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{Change, Cross, LinearVolatility, StDev};

// https://ru.wikipedia.org/wiki/%D0%90%D0%B4%D0%B0%D0%BF%D1%82%D0%B8%D0%B2%D0%BD%D0%B0%D1%8F_%D1%81%D0%BA%D0%BE%D0%BB%D1%8C%D0%B7%D1%8F%D1%89%D0%B0%D1%8F_%D1%81%D1%80%D0%B5%D0%B4%D0%BD%D1%8F%D1%8F_%D0%9A%D0%B0%D1%83%D1%84%D0%BC%D0%B0%D0%BD%D0%B0
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Kaufman {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub filter_period: PeriodType,
	pub square_smooth: bool,
	pub k: ValueType,
	pub source: Source,
}

impl IndicatorConfig for Kaufman {
	fn validate(&self) -> bool {
		self.period3 > self.period2
			&& self.period2 > 0
			&& self.period1 > 0
			&& (self.k > 0.0 || self.filter_period < 2)
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"period3" => self.period3 = value.parse().unwrap(),
			"filter_period" => self.filter_period = value.parse().unwrap(),
			"square_smooth" => self.square_smooth = value.parse().unwrap(),
			"k" => self.k = value.parse().unwrap(),
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
		(1, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for Kaufman {
	type Instance = KaufmanInstance;
	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let src = candle.source(cfg.source);
		Self::Instance {
			volatility: LinearVolatility::new(cfg.period1, src),
			change: Change::new(cfg.period1, src),
			fastest: 2. / (cfg.period2 + 1) as ValueType,
			slowest: 2. / (cfg.period3 + 1) as ValueType,
			st_dev: StDev::new(cfg.filter_period, src),
			cross: Cross::default(),
			last_signal: Action::None,
			last_signal_value: src,
			prev_value: src,
			cfg: cfg,
		}
	}
}

impl Default for Kaufman {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 2,
			period3: 30,
			k: 0.3,
			square_smooth: true,
			filter_period: 10,
			source: Source::Close,
		}
	}
}
#[derive(Debug)]
pub struct KaufmanInstance {
	cfg: Kaufman,

	volatility: LinearVolatility,
	change: Change,
	fastest: ValueType,
	slowest: ValueType,
	st_dev: StDev,
	cross: Cross,
	last_signal: Action,
	last_signal_value: ValueType,
	prev_value: ValueType,
}

impl<T: OHLC> IndicatorInstance<T> for KaufmanInstance {
	type Config = Kaufman;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let direction = self.change.next(src).abs();
		let volatility = self.volatility.next(src);

		let er = if volatility != 0. {
			direction / volatility
		} else {
			0.
		};
		let mut smooth = er.mul_add(self.fastest - self.slowest, self.slowest);

		if self.cfg.square_smooth {
			smooth = smooth * smooth;
		}

		let value = self.prev_value + smooth * (src - self.prev_value);
		self.prev_value = value;

		let cross = self.cross.next((src, value));

		let signal;
		if self.cfg.filter_period > 1 {
			let st_dev = self.st_dev.next(value);
			let filter = st_dev * self.cfg.k;

			if cross.is_some() {
				self.last_signal = cross;
				self.last_signal_value = value;
				signal = Action::None;
			} else if self.last_signal.is_some() && (value - self.last_signal_value).abs() > filter
			{
				signal = self.last_signal;
				self.last_signal = Action::None;
			} else {
				signal = Action::None;
			}
		} else {
			signal = cross;
		}

		IndicatorResult::new(&[value], &[signal])
	}
}
