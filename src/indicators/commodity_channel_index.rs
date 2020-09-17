#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Method, PeriodType, Source, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::SMA;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommodityChannelIndex {
	pub period: PeriodType,
	pub zone: ValueType,
	pub source: Source,
	scale: ValueType, // doesnt change
}

impl IndicatorConfig for CommodityChannelIndex {
	fn validate(&self) -> bool {
		self.zone >= 0.0 && self.zone <= 8.0
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period" => self.period = value.parse().unwrap(),
			"zone" => self.zone = value.parse().unwrap(),
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

impl<T: OHLC> IndicatorInitializer<T> for CommodityChannelIndex {
	type Instance = CommodityChannelIndexInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let invert_length = (cfg.period as ValueType).recip();
		let value = candle.source(cfg.source);

		Self::Instance {
			last_cci: 0.,
			last_signal: 0,
			dev_sum: 0.,
			sma: SMA::new(cfg.period, value),
			window: Window::new(cfg.period, 0.),

			invert_length,
			cfg,
		}
	}
}

impl Default for CommodityChannelIndex {
	fn default() -> Self {
		Self {
			period: 18,
			zone: 1.0,
			source: Source::Close,
			scale: 1.5,
		}
	}
}

//period=20, zone=1.0, #from 0.0 to ~7.0
//source='close'
#[derive(Debug)]
pub struct CommodityChannelIndexInstance {
	cfg: CommodityChannelIndex,

	invert_length: ValueType,
	last_cci: ValueType,
	last_signal: i8,
	dev_sum: ValueType,
	sma: SMA,
	window: Window<ValueType>,
}

impl CommodityChannelIndexInstance {
	fn dev(&mut self, value: ValueType, ma: ValueType) -> ValueType {
		let d = (value - ma).abs();

		let past_d = self.window.push(d);
		self.dev_sum += (d - past_d) * self.invert_length;
		self.dev_sum
	}

	fn cci(&mut self, value: ValueType) -> ValueType {
		let ma = self.sma.next(value);
		let dev = self.dev(value, ma);

		(value - ma) / (dev * self.cfg.scale)
	}
}

impl<T: OHLC> IndicatorInstance<T> for CommodityChannelIndexInstance {
	type Config = CommodityChannelIndex;

	fn name(&self) -> &str {
		"CommodityChannelIndex"
	}

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let value = candle.source(self.cfg.source);

		let cci = self.cci(value);

		// let mut t_signal = 0;
		// let mut signal = 0;

		// if cci > self.cfg.zone && self.last_cci <= self.cfg.zone {
		// 	t_signal += 1;
		// }

		// if cci < -self.cfg.zone && self.last_cci >= -self.cfg.zone {
		// 	t_signal -= 1;
		// }

		let t_signal = (cci > self.cfg.zone && self.last_cci <= self.cfg.zone) as i8
			- (cci < -self.cfg.zone && self.last_cci >= -self.cfg.zone) as i8;

		// if t_signal != 0 && self.last_signal != t_signal {
		// 	signal = t_signal;
		// }

		let signal = (t_signal != 0 && self.last_signal != t_signal) as i8 * t_signal;

		self.last_cci = cci;
		self.last_signal = signal;

		IndicatorResult::new(&[cci], &[Action::from(signal)])
	}
}
