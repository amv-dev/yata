#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, Source, OHLC};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, ReverseSignal};

/// [Awesome Oscillator](https://www.tradingview.com/scripts/awesomeoscillator/)
/// # 1 value
///
/// * Absolute difference between fast and slow periods MA
///
/// # 2 signals
///
/// * "Twin Peaks". When `value` is below zero line and we got `conseq_peaks` lower peaks, then returns full positive signal
/// When `value` is above zero line and we got `conseq_peaks` higher peaks, then returns full negative signal.
/// Ohterwise gives no signal.
/// * Gives signal when `values` crosses zero line
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AwesomeOscillator {
	/// Default is 34
	pub period1: PeriodType,
	/// Default is 5
	pub period2: PeriodType,
	/// Default is [SMA](crate::methods::SMA)
	pub method: RegularMethods,
	/// Default is [HL2](crate::core::Source#variant.HL2)
	pub source: Source,
	/// Default is 1
	pub left: PeriodType,
	/// Default is 1
	pub right: PeriodType,
	/// Defualt is 2
	pub conseq_peaks: u8,
}

impl IndicatorConfig for AwesomeOscillator {
	fn validate(&self) -> bool {
		self.period1 > self.period2 && self.left > 0 && self.right > 0 && self.conseq_peaks > 0
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"method" => self.method = value.parse().unwrap(),
			"source" => self.source = value.parse().unwrap(),
			"left" => self.left = value.parse().unwrap(),
			"right" => self.right = value.parse().unwrap(),

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
		(1, 2)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for AwesomeOscillator {
	type Instance = AwesomeOscillatorInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let src = candle.source(cfg.source);

		Self::Instance {
			ma1: method(cfg.method, cfg.period1, src),
			ma2: method(cfg.method, cfg.period2, src),
			cross_over: Cross::default(),
			reverse: Method::new((cfg.left, cfg.right), 0.0),
			low_peaks: 0,
			high_peaks: 0,
			cfg,
		}
	}
}

impl Default for AwesomeOscillator {
	fn default() -> Self {
		Self {
			period1: 34,
			period2: 5,
			method: RegularMethods::SMA,
			source: Source::HL2,
			left: 1,
			right: 1,
			conseq_peaks: 2,
		}
	}
}

#[derive(Debug)]
pub struct AwesomeOscillatorInstance {
	cfg: AwesomeOscillator,

	ma1: RegularMethod,
	ma2: RegularMethod,
	cross_over: Cross,
	reverse: ReverseSignal,
	low_peaks: u8,
	high_peaks: u8,
}

impl<T: OHLC> IndicatorInstance<T> for AwesomeOscillatorInstance {
	type Config = AwesomeOscillator;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let ma1 = &mut self.ma1;
		let ma2 = &mut self.ma2;
		let value = ma2.next(src) - ma1.next(src);

		let reverse: i8 = self.reverse.next(value).into();

		self.high_peaks = self.high_peaks.saturating_add((reverse > 0) as u8);
		self.low_peaks = self.low_peaks.saturating_add((reverse < 0) as u8);

		let s1 = (reverse < 0 && self.low_peaks >= self.cfg.conseq_peaks) as i8
			- (reverse > 0 && self.high_peaks >= self.cfg.conseq_peaks) as i8;
		let s2 = self.cross_over.next((value, 0.));

		// need to reset high/low peaks counter if value got lower/higher 0.0
		// sould do it after actual signals calculating
		self.high_peaks = self.high_peaks * (value >= 0.0) as u8;
		self.low_peaks = self.low_peaks * (value <= 0.0) as u8;

		let values = [value];
		let signals = [s1.into(), s2];

		IndicatorResult::new(&values, &signals)
	}
}
