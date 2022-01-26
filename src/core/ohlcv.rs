use super::{Source, ValueType};
// use std::fmt::Debug;

/// Basic trait for implementing [Open-High-Low-Close-Volume timeseries data](https://en.wikipedia.org/wiki/Candlestick_chart).
///
/// It has already implemented for tuple of 5 float values:
/// ```
/// use yata::prelude::OHLCV;
/// //         open high low  close, volume
/// let row = (2.0, 5.0, 1.0,  4.0,   10.0 );
/// assert_eq!(row.open(), row.0);
/// assert_eq!(row.high(), row.1);
/// assert_eq!(row.low(), row.2);
/// assert_eq!(row.close(), row.3);
/// assert_eq!(row.volume(), row.4);
/// ```
///
/// See also [Candle](crate::prelude::Candle).
pub trait OHLCV: 'static {
	/// Should return an *open* value of the period
	fn open(&self) -> ValueType;

	/// Should return an *highest* value of the period
	fn high(&self) -> ValueType;

	/// Should return an *lowest* value of the period
	fn low(&self) -> ValueType;

	/// Should return an *close* value of the candle
	fn close(&self) -> ValueType;

	/// Should return *volume* value for the period
	fn volume(&self) -> ValueType;

	/// Calculates [Typical price](https://en.wikipedia.org/wiki/Typical_price).
	/// It's just a simple \(`High` + `Low` + `Close`\) / `3`
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

	/// Calculates arithmetic average of `high`, `low`, `open` and `close` values of the candle
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
	///     open: 2.0,
	///     close: 3.0,
	///     ..Candle::default()
	/// };
	///
	/// assert_eq!(candle.ohlc4(), 5.0);
	/// ```
	fn ohlc4(&self) -> ValueType {
		(self.high() + self.low() + self.close() + self.open()) * 0.25
	}

	/// CLV = \[\(close - low\) - \(high - close\)\] / \(high - low\)
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
		// we need to check division by zero, so we can really just check if `high` is equal to `low` without using any kind of round error checks
		#[allow(clippy::float_cmp)]
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
	fn tr(&self, prev_candle: &dyn OHLCV) -> ValueType {
		self.tr_close(prev_candle.close())
	}

	/// Calculates [True Range](https://en.wikipedia.org/wiki/Average_true_range) over last two candles using `close` price from the previous candle.
	#[inline]
	fn tr_close(&self, prev_close: ValueType) -> ValueType {
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
		self.high().max(prev_close) - self.low().min(prev_close)
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
	/// assert!(!candle1.validate());
	/// assert!(!candle2.validate());
	/// ```
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
			&& (self.volume().is_nan() || self.volume() >= 0.0)
	}

	/// Returns [`Source`] field value of the candle.
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
			Source::Volume => self.volume(),
			Source::VolumedPrice => self.volumed_price(),
			Source::Open => self.open(),
		}
	}

	/// Volumed price
	///
	/// Same as [`OHLCV::tp()`] * [`OHLCV::volume()`]
	fn volumed_price(&self) -> ValueType {
		self.tp() * self.volume()
	}

	/// Checks if candle is "rising": it's close value greater than open value
	fn is_rising(&self) -> bool {
		self.close() > self.open()
	}

	/// Checks if candle is "falling": it's close value smaller than open value
	fn is_falling(&self) -> bool {
		self.close() < self.open()
	}
}

// impl<T: OHLCV + Copy> Sequence<T> {
// 	/// Validates a whole sequence
// 	///
// 	/// Returns `true` if every candle validates OK
// 	pub fn validate(&self) -> bool {
// 		self.iter().all(T::validate)
// 	}
// }

impl OHLCV for (ValueType, ValueType, ValueType, ValueType, ValueType) {
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

	#[inline]
	fn volume(&self) -> ValueType {
		self.4
	}
}

impl OHLCV for [ValueType; 5] {
	#[inline]
	fn open(&self) -> ValueType {
		self[0]
	}

	#[inline]
	fn high(&self) -> ValueType {
		self[1]
	}

	#[inline]
	fn low(&self) -> ValueType {
		self[2]
	}

	#[inline]
	fn close(&self) -> ValueType {
		self[3]
	}

	#[inline]
	fn volume(&self) -> ValueType {
		self[4]
	}
}

// impl<T: OHLCV> OHLCV for &T {
// 	#[inline]
// 	fn open(&self) -> ValueType {
// 		(**self).open()
// 	}

// 	#[inline]
// 	fn high(&self) -> ValueType {
// 		(**self).high()
// 	}

// 	#[inline]
// 	fn low(&self) -> ValueType {
// 		(**self).low()
// 	}

// 	#[inline]
// 	fn close(&self) -> ValueType {
// 		(**self).close()
// 	}

// 	#[inline]
// 	fn volume(&self) -> ValueType {
// 		(**self).volume()
// 	}
// }

