//! This is an example indicator
//!
//! It has a **Configuration** with parameters `price`, `period` and `source`.
//!
//! The idea is to find signals where price of timeseries crosses this config's `price` for the last `period` frames.

// Some core structures and traits
use crate::core::{Action, Error, IndicatorResult, PeriodType, Source, ValueType};
use crate::prelude::*;

// Cross method for searching crossover between price and our value
use crate::methods::Cross;

// If you are using `serde`, then it might be useful for you
// If you don't, you can just skip these lines
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// # Example config for the indicator **Configuration**
///
/// Must implement `Debug`, `Clone`, `Default`, [`IndicatorConfig`](crate::core::IndicatorConfig) and [`IndicatorInitializer`](crate::core::IndicatorInitializer) traits.
///
/// Also it may implements `serde::{Serialize, Deserialize}` - it's up to you.
///
/// See source code for the full example
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Example {
	price: ValueType,
	period: PeriodType,
	source: Source,
}

/// Implementing [`IndicatorConfig`](crate::core::IndicatorConfig) trait
impl IndicatorConfig for Example {
	const NAME: &'static str = "Example";

	/// Validates config values to be consistent
	fn validate(&self) -> bool {
		self.price > 0.0
	}

	/// Sets attributes of config by given name and value by `String`
	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"price" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.price = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	/// Our indicator will return single raw value and two signals
	fn size(&self) -> (u8, u8) {
		(1, 2)
	}
}

/// Implementing IndicatorInitializer to create **State** from the **Configration**
impl<T: OHLC> IndicatorInitializer<T> for Example {
	type Instance = ExampleInstance;

	fn init(self, _candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			cross: Cross::default(),
			last_signal: Action::None,
			last_signal_position: 0,
			cfg,
		})
	}
}

/// Implementing `Default` trait for default config
impl Default for Example {
	fn default() -> Self {
		Self {
			price: 2.0,
			period: 3,
			source: Source::Close,
		}
	}
}

/// # Example [`IndicatorInstance`](crate::core::IndicatorInstance) implementation
///
/// Must implement `Debug` and [`IndicatorInstance`](crate::core::IndicatorInstance) traits
///
/// See source code for the full example
#[derive(Debug, Clone, Copy)]
pub struct ExampleInstance {
	cfg: Example,

	cross: Cross,
	last_signal: Action,
	last_signal_position: PeriodType,
}

/// Implementing IndicatorInstance trait for Example
impl<T: OHLC> IndicatorInstance<T> for ExampleInstance {
	type Config = Example;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	/// Calculates next value by giving [`OHLC`](crate::core::OHLC)-object
	fn next(&mut self, candle: T) -> IndicatorResult {
		let new_signal = self.cross.next((candle.close(), self.cfg.price));

		let signal = match new_signal {
			Action::None => {
				self.last_signal = new_signal;
				self.last_signal_position = 0;
				new_signal
			}
			_ => {
				if let Action::None = self.last_signal {
					self.last_signal
				} else {
					self.last_signal_position += 1;
					if self.last_signal_position > self.cfg.period {
						self.last_signal = Action::None;
					}
					self.last_signal
				}
			}
		};

		let some_other_signal = Action::from(0.5);

		IndicatorResult::new(&[candle.close()], &[signal, some_other_signal])
	}
}
