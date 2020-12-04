use crate::core::{
	Error, IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult, Method,
	PeriodType, Source, OHLC,
};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Change, Cross, ReversalSignal, TMA};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// TRIX (extended)
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Trix_(technical_analysis)>
///
/// # 2 values
///
/// * `main` value
///
/// Range is \(`-inf`; `+inf`\)
///
/// * `signal line` value
///
/// Range is \(`-inf`; `+inf`\)
///
/// # 3 signals
///
/// * When `main` value changes direction upwards, returns full buy signal.
/// When `main` value changes direction downwards, returns full sell signal.
/// Otherwise returns no signal.
///
/// * When `main` value crosses `signal line` value upwards, returns full buy signal.
/// When `main` value crosses `signal line` value downwards, returns full sell signal.
/// Otherwise returns no signal.
///
/// * When `main` value crosses zero line upwards, returns full buy signal.
/// When `main` value crosses zero line downwards, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Trix {
	/// TRIX period. Default is `18`.
	///
	/// Range in \[`3`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period1: PeriodType,

	/// Signal line period. Default is `6`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period2: PeriodType,

	/// Signal line moving average method. Default is [`EMA`](crate::methods::EMA).
	pub method2: RegularMethods,

	/// Source type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for Trix {
	const NAME: &'static str = "Trix";

	fn validate(&self) -> bool {
		self.period1 > 2 && self.period2 > 1
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"source" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},
			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(2, 3)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for Trix {
	type Instance = TRIXInstance;

	fn init(self, candle: T) -> Result<Self::Instance, Error> {
		if self.validate() {
			let src = candle.source(self.source);

			Ok(Self::Instance {
				tma: TMA::new(self.period1, src)?,
				sig: method(self.method2, self.period2, src)?,
				change: Change::new(1, src)?,
				cross1: Cross::new((), (src, src))?,
				cross2: Cross::new((), (src, src))?,
				reverse: ReversalSignal::new(1, 1, 0.0)?,

				cfg: self,
				// phantom: PhantomData::default(),
			})
		} else {
			Err(Error::WrongConfig)
		}
	}
}

impl Default for Trix {
	fn default() -> Self {
		Self {
			period1: 18,
			period2: 6, // TODO: find recommended value here
			method2: RegularMethods::EMA,
			source: Source::Close,
		}
	}
}

// https://en.wikipedia.org/wiki/Trix_(technical_analysis)
#[derive(Debug)]
pub struct TRIXInstance {
	cfg: Trix,

	tma: TMA,
	sig: RegularMethod,
	change: Change,
	cross1: Cross,
	cross2: Cross,
	reverse: ReversalSignal,
}

impl<T: OHLC> IndicatorInstance<T> for TRIXInstance {
	type Config = Trix;

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
		let tma = self.tma.next(src);
		let value = self.change.next(tma);

		let signal1 = self.reverse.next(value);

		let sigline = self.sig.next(value);

		let signal2 = self.cross1.next((value, sigline));
		let signal3 = self.cross2.next((value, 0.));

		IndicatorResult::new(&[value, sigline], &[signal1, signal2, signal3])
	}
}
