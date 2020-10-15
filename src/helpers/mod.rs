#![warn(missing_docs, missing_debug_implementations)]
//! Additional helping primitives
//!

mod methods;
use crate::core::{Candle, ValueType};
pub use methods::*;

/// sign is like [f64.signum](https://doc.rust-lang.org/std/primitive.f64.html#method.signum)
/// except when value == 0.0, then sign returns 0.0
///
/// See also [signi]
///
/// # Examples
///
/// ```
/// use yata::helpers::sign;
///
/// assert_eq!(sign(4.65), 1.0);
/// assert_eq!(sign(-25.6), -1.0);
/// assert_eq!(sign(0.0), 0.0);
/// assert_eq!(sign(-0.0), 0.0);
/// assert_eq!(sign(0.000001), 1.0);
/// ```
#[inline]
pub fn sign(value: ValueType) -> ValueType {
	// if value > 0. {
	// 	1.
	// } else if value < 0. {
	// 	-1.
	// } else {
	// 	0.
	// }
	((value > 0.) as i8 - (value < 0.) as i8) as ValueType
}

/// signi is like [f64.signum](https://doc.rust-lang.org/std/primitive.f64.html#method.signum), except 2 things
/// - when value == 0.0, then signi returns 0
/// - signi always returns i8
///
/// See also [sign]
///
/// # Examples
///
/// ```
/// use yata::helpers::signi;
///
/// assert_eq!(signi(4.65), 1);
/// assert_eq!(signi(-25.6), -1);
/// assert_eq!(signi(0.0), 0);
/// assert_eq!(signi(-0.0), 0);
/// assert_eq!(signi(0.000001), 1);
/// assert_eq!(signi(-0.000001), -1);
/// ```
#[inline]
pub fn signi(value: ValueType) -> i8 {
	// if value > 0. {
	// 	1
	// } else if value < 0. {
	// 	-1
	// } else {
	// 	0
	// }

	(value > 0.) as i8 - (value < 0.) as i8
}

/// Random Candles iterator for testing purposes
#[derive(Debug, Clone, Copy)]
pub struct RandomCandles(u16);

impl RandomCandles {
	const DEFAULT_PRICE: ValueType = 1.0;
	const DEFAULT_VOLUME: ValueType = 10.0;

	/// Returns new instance of RandomCandles for testing purposes
	pub fn new() -> Self {
		Self::default()
	}

	/// Returns very first candle in the sequence
	pub fn first(&mut self) -> Candle {
		let position = self.0;
		self.0 = 0;
		let candle = self.next().unwrap();
		self.0 = position;

		candle
	}
}

impl Default for RandomCandles {
	fn default() -> Self {
		Self(0)
	}
}

impl Iterator for RandomCandles {
	type Item = Candle;

	fn next(&mut self) -> Option<Self::Item> {
		let prev_position = self.0.wrapping_sub(1) as ValueType;
		let position = self.0 as ValueType;

		let close = Self::DEFAULT_PRICE + position.sin() / 2.;
		let open = Self::DEFAULT_PRICE + prev_position.sin() / 2.;

		let high = close.max(open) + (position * 1.4).tan().abs();
		let low = close.min(open) - (position * 0.8).cos().abs() / 3.;
		let volume = Self::DEFAULT_VOLUME * (position / 2.).sin() + Self::DEFAULT_VOLUME / 2.;

		let candle = Self::Item {
			// candle: Candle {
			open,
			high,
			low,
			close,
			volume,
			// },
			// timestamp: position as i64,
			// ..Self::Item::default()
		};

		self.0 = self.0.wrapping_sub(1);
		Some(candle)
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.0 = n as u16;
		self.0 = self.0.wrapping_sub(1);

		self.next()
	}
}
