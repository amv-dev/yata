#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, Method, PeriodType, Source, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::Change;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vidya {
	pub period: PeriodType,
	pub zone: ValueType,
	pub source: Source,
}

impl IndicatorConfig for Vidya {
	const NAME: &'static str = "Vidya";

	fn validate(&self) -> bool {
		self.period > 1 && self.zone >= 0. && self.zone <= 5.
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			"zone" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value.to_string()));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for Vidya {
	type Instance = VidyaInstance;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			f: 2. / (1 + cfg.period) as ValueType,
			up_sum: 0.,
			dn_sum: 0.,
			last_value: src,
			last_result: src,
			window: Window::new(cfg.period, 0.),
			change: Change::new(1, src)?,
			last_signal: 0,
			cfg,
		})
	}
}

impl Default for Vidya {
	fn default() -> Self {
		Self {
			period: 10,
			zone: 0.01,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct VidyaInstance {
	cfg: Vidya,

	f: ValueType,
	up_sum: ValueType,
	dn_sum: ValueType,
	last_value: ValueType,
	last_result: ValueType,
	window: Window<ValueType>,
	change: Change,
	last_signal: i8,
}

impl<T: OHLC> IndicatorInstance<T> for VidyaInstance {
	type Config = Vidya;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let change = self.change.next(src);

		let left_change = self.window.push(change);

		if left_change > 0. {
			self.up_sum -= left_change;
		} else if left_change < 0. {
			self.dn_sum -= left_change.abs();
		}

		if change > 0. {
			self.up_sum += change;
		} else if change < 0. {
			self.dn_sum += change.abs();
		}

		let value;

		if self.up_sum == 0. && self.dn_sum == 0. {
			value = self.last_result;
		} else {
			let cmo = ((self.up_sum - self.dn_sum) / (self.up_sum + self.dn_sum)).abs();
			let f_cmo = self.f * cmo;
			let result = src * f_cmo + (1.0 - f_cmo) * self.last_result;
			value = result;
			self.last_result = result;
		}

		self.last_value = src;

		let upper_zone = 1.0 + self.cfg.zone;
		let lower_zone = 1.0 - self.cfg.zone;

		let signal;

		if value * upper_zone > src {
			if self.last_signal != -1 {
				signal = -1;
			} else {
				signal = 0;
			}
		} else if value * lower_zone < src {
			if self.last_signal != 1 {
				signal = 1;
			} else {
				signal = 0;
			}
		} else {
			signal = 0;
		}

		if signal != 0 {
			self.last_signal = signal;
		}

		IndicatorResult::new(&[value], &[Action::from(signal)])
	}
}
