#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, ValueType, Window, OHLCV};
use crate::methods::{Cross, ADI};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChaikinMoneyFlow {
	pub size: PeriodType,
	// phantom:  PhantomData<T>,
}

impl IndicatorConfig for ChaikinMoneyFlow {
	fn validate(&self) -> bool {
		self.size > 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"size" => self.size = value.parse().unwrap(),

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

	fn is_volume_based(&self) -> bool {
		true
	}

	fn size(&self) -> (u8, u8) {
		(1, 1)
	}
}

impl<T: OHLCV> IndicatorInitializer<T> for ChaikinMoneyFlow {
	type Instance = ChaikinMoneyFlowInstance<T>;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		Self::Instance {
			adi: ADI::new(cfg.size, candle),
			vol_sum: candle.volume() * cfg.size as ValueType,
			window: Window::new(cfg.size, candle.volume()),
			cross_over: Cross::default(),
			cfg,
		}
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

	#[inline]
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

	fn name(&self) -> &str {
		"ChaikinMoneyFlow"
	}
}
