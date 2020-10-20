use super::{Sequence, Source, ValueType};
use std::fmt::Debug;

/// Basic trait for implementing [Open-High-Low-Close timeseries data](https://en.wikipedia.org/wiki/Candlestick_chart).
///
/// It has already implemented for tuple of 4 and 5 float values:
/// ```
/// use yata::prelude::OHLC;
/// //         open high low  close
/// let row = (2.0, 5.0, 1.0,  4.0 );
/// assert_eq!(row.open(), row.0);
/// assert_eq!(row.high(), row.1);
/// assert_eq!(row.low(), row.2);
/// assert_eq!(row.close(), row.3);
/// ```
///
/// See also [Candle](crate::prelude::Candle).
pub trait OHLC: Copy + Debug + Default {
	/// Should return an *open* value of the period
	fn open(&self) -> ValueType;

	/// Should return an *highest* value of the period
	fn high(&self) -> ValueType;

	/// Should return an *lowest* value of the period
	fn low(&self) -> ValueType;

	/// Should return an *close* value of the candle
	fn close(&self) -> ValueType;

	/// Calculates [Typical price](https://en.wikipedia.org/wiki/Typical_price).
	/// It's just a simple (High + Low + Close) / 3
	///
	/// # Examples
	///
	/// ```
	/// use yata::prelude::*;
	/// use yata::core::Candle;
	///
	/// let candle = Candle {
	///     high: 10.0,
	///     low: 5.0,
	///     close: 9.0,
	///     ..Candle::default()
	/// };
	///
	/// assert_eq!(candle.tp(), 8.0);
	/// ```
	#[inline]
	fn tp(&self) -> ValueType {
		(self.high() + self.low() + self.close()) / 3.
	}

	/// Calculates arithmetic average of `high` and `low` values of the candle
	///
	/// # Examples
	///
	/// ```
	/// use yata::prelude::*;
	/// use yata::core::Candle;
	///
	/// let candle = Candle {
	///     high: 10.0,
	///     low: 5.0,
	///     ..Candle::default()
	/// };
	///
	/// assert_eq!(candle.hl2(), 7.5);
	/// ```
	#[inline]
	fn hl2(&self) -> ValueType {
		(self.high() + self.low()) * 0.5
	}

	/// CLV = \[(close - low) - (high - close)\] / (high - low)
	///
	/// # Examples
	///
	/// ```
	/// use yata::prelude::*;
	/// use yata::core::Candle;
	/// let candle = Candle {
	///     high: 5.0,
	///     low: 2.0,
	///     close: 4.0,
	///     ..Candle::default()
	/// };
	///
	/// assert_eq!(candle.clv(), ((candle.close()-candle.low()) - (candle.high() - candle.close()))/(candle.high() - candle.low()));
	/// assert_eq!(candle.clv(), ((4. - 2.) - (5. - 4.))/(5. - 2.));
	/// ```
	#[inline]
	fn clv(&self) -> ValueType {
		if self.high() == self.low() {
			0.
		} else {
			(2. * self.close() - self.low() - self.high()) / (self.high() - self.low())
		}
	}

	/// Calculates [True Range](https://en.wikipedia.org/wiki/Average_true_range) over last two candles
	///
	/// # Examples
	///
	/// ```
	/// use yata::prelude::*;
	/// use yata::core::Candle;
	///
	/// let candle1 = Candle {
	///     close: 70.0,
	///     ..Candle::default()
	/// };
	///
	/// let candle2 = Candle {
	///     high: 100.0,
	///     low: 50.0,
	///     ..Candle::default()
	/// };
	///
	/// let tr = candle2.tr(&candle1);
	/// assert_eq!(tr, 50.);
	/// ```
	///
	/// ```
	/// use yata::prelude::*;
	/// use yata::core::Candle;
	///
	/// let candle1 = Candle {
	///     close: 30.0,
	///     ..Candle::default()
	/// };
	///
	/// let candle2 = Candle {
	///     high: 100.0,
	///     low: 50.0,
	///     ..Candle::default()
	/// };
	///
	/// let tr = candle2.tr(&candle1);
	/// assert_eq!(tr, 70.);
	/// ```
	#[inline]
	fn tr(&self, prev_candle: &Self) -> ValueType {
		// Original formula

		// let (a, b, c) = (
		//     self.high() - self.low(),
		//     (self.high() - prev_candle.close()).abs(),
		//     (prev_candle.close() - self.low()).abs(),
		// );

		// a.max(b).max(c)

		// -----------------------
		// more performance
		// only 1 subtract operation instead of 3
		self.high().max(prev_candle.close()) - self.low().min(prev_candle.close())
	}

	/// Validates candle attributes
	///
	/// Returns `true` if validates OK
	///
	/// # Examples
	///
	/// ```
	/// use yata::prelude::*;
	/// use yata::core::Candle;
	///
	/// let candle1 = Candle {
	///     open: 7.0,
	///     high: 10.0,
	///     low: 7.0,
	///     close: 11.0, // cannot be more than high
	///
	///     ..Candle::default()
	/// };
	/// let candle2 = Candle {
	///     open: 10.0,
	///     high: 10.0,
	///     low: 11.0, // low cannot be more than any other value of the candle
	///     close: 10.0,
	///
	///     ..Candle::default()
	/// };
	///
	/// assert!(!OHLC::validate(&candle1));
	/// assert!(!OHLC::validate(&candle2));
	/// ```
	#[inline]
	fn validate(&self) -> bool {
		!(self.close() > self.high() || self.close() < self.low() || self.high() < self.low())
			&& self.close() > 0.
			&& self.open() > 0.
			&& self.high() > 0.
			&& self.low() > 0.
			&& self.close().is_finite()
			&& self.open().is_finite()
			&& self.high().is_finite()
			&& self.low().is_finite()
	}

	/// Returns [Source] field value of the candle.
	///
	/// # Examples
	///
	/// ```
	/// use yata::prelude::*;
	/// use yata::core::{Candle, Source};
	/// let candle = Candle {
	///     open: 12.0,
	///     high: 15.0,
	///     low: 7.0,
	///     close: 10.0,
	///     ..Candle::default()
	/// };
	/// assert_eq!(OHLCV::source(&candle, Source::Low), 7.0);
	/// assert_eq!(OHLCV::source(&candle, "close".to_string().parse().unwrap()), 10.0);
	/// ```
	#[inline]
	fn source(&self, source: Source) -> ValueType {
		match source {
			Source::Close => self.close(),
			Source::High => self.high(),
			Source::Low => self.low(),
			Source::TP => self.tp(),
			Source::HL2 => self.hl2(),
			Source::Open => self.open(),
			Source::Volume => panic!("Volume is not implemented for OHLC"),
		}
	}
}

/// Basic trait for implementing [Open-High-Low-Close-Volume timeseries data](https://en.wikipedia.org/wiki/Candlestick_chart).
///
/// It has already implemented for tuple of 5 float values:
/// ```
/// use yata::prelude::{OHLC, OHLCV};
/// //         open high low  close volume
/// let row = (2.0, 5.0, 1.0,  4.0,  10.0 );
/// assert_eq!(row.open(), row.0);
/// assert_eq!(row.high(), row.1);
/// assert_eq!(row.low(), row.2);
/// assert_eq!(row.close(), row.3);
/// assert_eq!(row.volume(), row.4);
/// ```
///
/// See also [Candle](crate::prelude::Candle).
pub trait OHLCV: OHLC {
	/// Should return *volume* value for the period
	fn volume(&self) -> ValueType;

	/// Validates candle attributes
	///
	/// See more at [OHLC#method.validate].
	#[inline]
	fn validate(&self) -> bool {
		OHLC::validate(self) && self.volume() >= 0. && self.volume().is_finite()
	}

	/// Returns [Source] field value of the candle.
	///
	/// See more at [OHLC#method.source].
	#[inline]
	fn source(&self, source: Source) -> ValueType {
		match source {
			Source::Volume => self.volume(),
			_ => OHLC::source(self, source),
		}
	}
}

impl<T: OHLC> Sequence<T> {
	/// Validates a whole sequence
	///
	/// Returns `true` if every candle validates OK
	pub fn validate(&self) -> bool {
		self.iter().all(T::validate)
	}
}

impl OHLC for (ValueType, ValueType, ValueType, ValueType) {
	#[inline]
	fn open(&self) -> ValueType {
		self.0
	}

	#[inline]
	fn high(&self) -> ValueType {
		self.1
	}

	#[inline]
	fn low(&self) -> ValueType {
		self.2
	}

	#[inline]
	fn close(&self) -> ValueType {
		self.3
	}
}

impl OHLC for (ValueType, ValueType, ValueType, ValueType, ValueType) {
	#[inline]
	fn open(&self) -> ValueType {
		self.0
	}

	#[inline]
	fn high(&self) -> ValueType {
		self.1
	}

	#[inline]
	fn low(&self) -> ValueType {
		self.2
	}

	#[inline]
	fn close(&self) -> ValueType {
		self.3
	}
}

impl OHLCV for (ValueType, ValueType, ValueType, ValueType, ValueType) {
	#[inline]
	fn volume(&self) -> ValueType {
		self.4
	}
}
