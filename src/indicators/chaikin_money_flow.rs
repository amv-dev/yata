#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{Cross, ADI};

/// [Chaikin Money Flow](https://en.wikipedia.org/wiki/Chaikin_Analytics)
///
/// # 1 value
///
/// * oscillator value [-1.0; 1.0]
///
/// # 1 digital signal
///
/// When `oscillator` value goes above zero, then returns full buy signal.
/// When `oscillator` value goes below zero, then returns full sell signal.
/// Otherwise no signal
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChaikinMoneyFlow {
	/// main length size. Default is 20
	pub size: PeriodType,
	// phantom:  PhantomData<T>,
}

impl IndicatorConfig for ChaikinMoneyFlow {
	const NAME: &'static str = "ChaikinMoneyFlow";

	fn validate(&self) -> bool {
		self.size > 1
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"size" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.size = value,
			},
			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn is_volume_based(&self) -> bool {
		true
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl<T: OHLCV> IndicatorInitializer<T> for ChaikinMoneyFlow {
	type Instance = ChaikinMoneyFlowInstance<T>;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			adi: ADI::new(cfg.size, candle)?,
			vol_sum: candle.volume() * cfg.size as ValueType,
			window: Window::new(cfg.size, candle.volume()),
			cross_over: Cross::default(),
			cfg,
		})
	}
}

impl Default for ChaikinMoneyFlow {
	fn default() -> Self {
		Self {
			size: 20,
			// phantom: PhantomData::default(),
		}
	}
}

#[derive(Debug)]
pub struct ChaikinMoneyFlowInstance<T: OHLCV> {
	cfg: ChaikinMoneyFlow,

	adi: ADI<T>,
	vol_sum: ValueType,
	window: Window<ValueType>,
	cross_over: Cross,
}

impl<T: OHLCV> IndicatorInstance<T> for ChaikinMoneyFlowInstance<T> {
	type Config = ChaikinMoneyFlow;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let adi = self.adi.next(candle);
		self.vol_sum += candle.volume() - self.window.push(candle.volume());
		let value = adi / self.vol_sum;
		let signal = self.cross_over.next((value, 0.));

		IndicatorResult::new(&[value], &[signal])
	}
}
