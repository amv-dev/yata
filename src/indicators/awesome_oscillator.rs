#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{DynMovingAverage, Error, Method, MovingAverageConstructor, OHLCV, PeriodType, Source};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
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
pub struct AwesomeOscillator<M: MovingAverageConstructor = MA> {
	pub ma1: M,
	pub ma2: M,
	/*
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
	*/
	/// Default is [`HL2`](crate::core::Source::HL2).
	pub source: Source,
	/// Default is `1`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)-`right`\).
	pub left: PeriodType,
	/// Default is `1`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)-`left`\).
	pub right: PeriodType,
	/// Default is `2`.
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)\].
	pub conseq_peaks: u8,
}

impl<M: MovingAverageConstructor> IndicatorConfig for AwesomeOscillator<M> {
	type Instance = AwesomeOscillatorInstance<M>;

	const NAME: &'static str = "AwesomeOscillator";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			ma1: cfg.ma1.init(src)?,
			ma2: cfg.ma2.init(src)?,
			cross_over: Cross::default(),
			reverse: Method::new((cfg.left, cfg.right), &0.0)?,
			low_peaks: 0,
			high_peaks: 0,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.ma1.ma_period() > 2
			&& self.ma1.is_similar_to(&self.ma2)
			&& self.ma1.ma_period() < PeriodType::MAX
			&& self.ma1.ma_period() > self.ma2.ma_period()
			&& self.ma2.ma_period() > 1
			&& self.left > 0
			&& self.right > 0
			&& self.conseq_peaks > 0
			&& self.left.saturating_add(self.right) < PeriodType::MAX
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"ma1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma1 = value,
			},
			"ma2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma2 = value,
			},
			"source" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},
			"left" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.left = value,
			},
			"right" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.right = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(1, 2)
	}
}

impl Default for AwesomeOscillator<MA> {
	fn default() -> Self {
		Self {
			ma1: MA::SMA(34),
			ma2: MA::SMA(5),
			source: Source::HL2,
			left: 1,
			right: 1,
			conseq_peaks: 2,
		}
	}
}

#[derive(Debug)]
pub struct AwesomeOscillatorInstance<M: MovingAverageConstructor = MA> {
	cfg: AwesomeOscillator<M>,

	ma1: DynMovingAverage,
	ma2: DynMovingAverage,
	cross_over: Cross,
	reverse: ReversalSignal,
	low_peaks: u8,
	high_peaks: u8,
}

impl<M: MovingAverageConstructor> IndicatorInstance for AwesomeOscillatorInstance<M> {
	type Config = AwesomeOscillator<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);

		let ma1 = &mut self.ma1;
		let ma2 = &mut self.ma2;
		let value = ma2.next(&src) - ma1.next(&src);

		let reverse: i8 = self.reverse.next(&value).into();

		self.high_peaks = self.high_peaks.saturating_add((reverse > 0) as u8);
		self.low_peaks = self.low_peaks.saturating_add((reverse < 0) as u8);

		let s1 = (reverse < 0 && self.low_peaks >= self.cfg.conseq_peaks) as i8
			- (reverse > 0 && self.high_peaks >= self.cfg.conseq_peaks) as i8;
		let s2 = self.cross_over.next(&(value, 0.));

		// need to reset high/low peaks counter if value got lower/higher 0.0
		// should do it after actual signals calculating
		self.high_peaks *= (value >= 0.0) as u8;
		self.low_peaks *= (value <= 0.0) as u8;

		let values = [value];
		let signals = [s1.into(), s2];

		IndicatorResult::new(&values, &signals)
	}
}
