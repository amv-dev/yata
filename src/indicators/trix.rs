use crate::core::{
	Error, IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult, Method,
	PeriodType, Source, ValueType, OHLC,
};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::{Change, Cross, TMA};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Как идея: сигнал на покупку/продажу при пересекании графиком определённой зоны
// Такой сигнал не может служить как основной, но может служить как усиливающий
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Trix {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub method2: RegularMethods,
	pub source: Source,
}

impl IndicatorConfig for Trix {
	const NAME: &'static str = "Trix";

	fn validate(&self) -> bool {
		self.period1 > 2 && self.period2 > 1
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
		(1, 3)
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
				prev_value: 0.0,

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
	// <T: OHLC> {
	cfg: Trix,

	tma: TMA,
	sig: RegularMethod,
	change: Change,
	// pivot:       Option<ReverseSignal>,
	cross1: Cross,
	cross2: Cross,
	prev_value: ValueType,
	// phantom:    PhantomData<T>,
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
		// let signal1;
		// if value > self.prev_value {
		// 	signal1 = Action::BUY_ALL;
		// } else if value < self.prev_value {
		// 	signal1 = Action::SELL_ALL;
		// } else {
		// 	signal1 = Action::None;
		// }
		let signal1 = (value > self.prev_value) as i8 - (value < self.prev_value) as i8;

		let sigline = self.sig.next(value);

		let signal2 = self.cross1.next((value, sigline));
		let signal3 = self.cross2.next((value, 0.));

		IndicatorResult::new(&[value], &[signal1.into(), signal2, signal3])
	}
}
