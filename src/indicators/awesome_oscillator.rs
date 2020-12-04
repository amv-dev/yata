#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Cross, ReversalSignal};

/// Awesome Oscillator
///
/// ## Links
///
/// * <https://www.tradingview.com/scripts/awesomeoscillator/>
///
/// # 1 value
///
/// * Absolute difference between fast and slow periods MA
///
/// Range in \(`-inf`; `+inf`\)
///
/// # 2 signals
///
/// * "Twin Peaks". When `value` is below zero line and we got `conseq_peaks` lower peaks, then returns full positive signal
/// When `value` is above zero line and we got `conseq_peaks` higher peaks, then returns full negative signal.
/// Otherwise gives no signal.
/// * Gives signal when `values` crosses zero line
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AwesomeOscillator {
	/// Default is `34`.
	///
	/// Range in \(`period2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period1: PeriodType,
	/// Default is `5`.
	///
	/// Range in \[`3`; `period1`\).
	pub period2: PeriodType,
	/// Default is [`SMA`](crate::methods::SMA).
	pub method: RegularMethods,
	/// Default is [`HL2`](crate::core::Source::HL2).
	pub source: Source,
	/// Default is `1`.
	///
	/// Range in \[1; [`PeriodType::MAX`](crate::core::PeriodType)-`right`\).
	pub left: PeriodType,
	/// Default is `1`.
	///
	/// Range in \[1; [`PeriodType::MAX`](crate::core::PeriodType)-`left`\).
	pub right: PeriodType,
	/// Default is `2`.
	///
	/// Range in \[1; [`PeriodType::MAX`](crate::core::PeriodType)\].
	pub conseq_peaks: u8,
}

impl IndicatorConfig for AwesomeOscillator {
	const NAME: &'static str = "AwesomeOscillator";

	fn validate(&self) -> bool {
		self.period1 > 2
			&& self.period1 < PeriodType::MAX
			&& self.period1 > self.period2
			&& self.period2 > 1
			&& self.left > 0
			&& self.right > 0
			&& self.conseq_peaks > 0
			&& PeriodType::MAX
				.saturating_sub(self.left)
				.saturating_sub(self.right)
				> 0
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"method" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},
			"left" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.left = value,
			},
			"right" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.right = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(1, 2)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for AwesomeOscillator {
	type Instance = AwesomeOscillatorInstance;

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
			ma1: method(cfg.method, cfg.period1, src)?,
			ma2: method(cfg.method, cfg.period2, src)?,
			cross_over: Cross::default(),
			reverse: Method::new((cfg.left, cfg.right), 0.0)?,
			low_peaks: 0,
			high_peaks: 0,
			cfg,
		})
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
	reverse: ReversalSignal,
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
		// should do it after actual signals calculating
		self.high_peaks *= (value >= 0.0) as u8;
		self.low_peaks *= (value <= 0.0) as u8;

		let values = [value];
		let signals = [s1.into(), s2];

		IndicatorResult::new(&values, &signals)
	}
}
