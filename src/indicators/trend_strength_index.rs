#![allow(clippy::similar_names)]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, Source, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{CrossAbove, CrossUnder, ReverseSignal, WMA};

/// Trend Strength Index
///
/// There are bunch of different indicators named "Trend Strength Index" on the internet.
///
/// This particular one was seen somewhere a long time ago. I can't even tell where. It produces an oscillator which may move in range \[`-1.0`; `1.0`\].
///
/// # 1 value
///
/// * `Main value`
///
/// Range in \[`-1.0`; `1.0`\]
///
/// # 2 signals
///
/// * When `main value` crosses upper `zone` downwards, gives full negative #1 signal.
/// When `main value` crosses lower `zone` upwards, gives full positive #1 signal.
///
/// * When `main value` is below lower `zone` and changes direction upwards, gives full positive #2 signal
/// When `main value` is above upper `zone` and changes direction downwards, gives full negative #2 signal
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrendStrengthIndex {
	/// Main period length. Default is `14`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period: PeriodType,

	/// Zone value determines when signal #2 appears. Default is `0.75`.
	///
	/// Range in \[`0.0`; `1.0`\).
	pub zone: ValueType,

	/// Reverse period
	///
	/// Range in \[`1`; [`PeriodType::MAX`](crate::core::PeriodType)/`2`\].
	pub reverse_offset: PeriodType,

	/// Source type of values. Default is [`Close`](crate::core::Source::Close).
	pub source: Source,
}

impl IndicatorConfig for TrendStrengthIndex {
	const NAME: &'static str = "TrendStrengthIndex";

	fn validate(&self) -> bool {
		self.period > 1
			&& self.zone >= 0.0
			&& self.zone < 1.0
			&& self.reverse_offset > 0
			&& self.reverse_offset <= self.period
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
			"reverse_offset" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.reverse_offset = value,
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
		(1, 2)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for TrendStrengthIndex {
	type Instance = TrendStrengthIndexInstance;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if self.validate() {
			let cfg = self;

			let inverted_period = (cfg.period as ValueType).recip();
			let src = candle.source(cfg.source);

			let period = cfg.period as usize;
			let sx = (period + 1) * period / 2;
			let sx2 = (sx * (2 * period + 1)) as ValueType / 3.0;

			let inv_sx = ((period + 1) * sx) as ValueType * 0.5;
			let k = sx2 - inv_sx;
			let sy = src * cfg.period as ValueType;
			let sy2 = src * src * cfg.period as ValueType;

			Ok(Self::Instance {
				window: Window::new(cfg.period, src),
				period: period as ValueType,
				inverted_period,
				sx: sx as ValueType,
				sy2,
				k,
				wma: WMA::new(cfg.period, src)?,
				cross_under: CrossUnder::new((), (0.0, cfg.zone))?,
				cross_above: CrossAbove::new((), (0.0, -cfg.zone))?,
				reverse: ReverseSignal::new(1, 2, 0.0)?,
				sy,

				cfg,
			})
		} else {
			Err(Error::WrongConfig)
		}
	}
}

impl Default for TrendStrengthIndex {
	fn default() -> Self {
		Self {
			period: 14,
			zone: 0.75,
			reverse_offset: 2,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct TrendStrengthIndexInstance {
	cfg: TrendStrengthIndex,
	period: ValueType,
	inverted_period: ValueType,
	sx: ValueType,
	sy: ValueType,
	sy2: ValueType,
	k: ValueType,
	wma: WMA,
	cross_under: CrossUnder,
	cross_above: CrossAbove,
	reverse: ReverseSignal,
	window: Window<ValueType>,
}

impl<T: OHLC> IndicatorInstance<T> for TrendStrengthIndexInstance {
	type Config = TrendStrengthIndex;

	fn config(&self) -> &Self::Config
	where
		Self: Sized,
	{
		&self.cfg
	}

	#[inline]
	fn next(&mut self, candle: T) -> IndicatorResult
	where
		Self: Sized,
	{
		let src = candle.source(self.cfg.source);
		let past_src = self.window.push(src);

		self.sy += src - past_src;

		self.sy2 += src * src - past_src * past_src;

		let sma = self.inverted_period * self.sy;
		let p = (self.wma.next(src) - sma) * self.sx;

		// sy2 is always greater than sma * sy, so q is always positive
		let q = self.k * (self.sy2 - sma * self.sy);

		let value = p / q.sqrt();

		let cross_signal = self.cross_under.next((value, self.cfg.zone))
			- self.cross_above.next((value, -self.cfg.zone));
		let reverse = self.reverse.next(value).analog();

		let is_upper_signal = reverse < 0 && self.window[self.cfg.reverse_offset] >= self.cfg.zone;
		let is_lower_signal = reverse > 0 && self.window[self.cfg.reverse_offset] <= -self.cfg.zone;
		let reverse_signal = is_upper_signal as i8 - is_lower_signal as i8;

		IndicatorResult::new(&[value], &[cross_signal, reverse_signal.into()])
	}
}
