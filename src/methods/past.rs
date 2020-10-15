use crate::core::Method;
use crate::core::{Error, PeriodType, Window};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Moves timeseries by `length` items forward
///
/// It's just a simple method-like wrapper for [`Window<T>`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 0
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::Past;
///
/// // Move of length=3
/// let mut past = Past::new(3, 1.0).unwrap();
///
/// past.next(1.0);
/// past.next(2.0);
/// past.next(3.0);
///
/// assert_eq!(past.next(4.0), 1.0);
/// assert_eq!(past.next(5.0), 2.0);
/// assert_eq!(past.next(6.0), 3.0);
/// ```
///
/// # Perfomance
///
/// O(1)
///
/// # See also
///
/// [`Window<T>`]
///
/// [`Window<T>`]: crate::core::Window
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Past<T>(Window<T>)
where
	T: Sized + Copy + Default + fmt::Debug;

impl<T> Method for Past<T>
where
	T: Sized + Copy + Default + fmt::Debug,
{
	type Params = PeriodType;
	type Input = T;
	type Output = T;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self(Window::new(length, value))),
		}
	}

	#[inline]
	fn next(&mut self, value: T) -> T {
		self.0.push(value)
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused_imports)]
	use super::{Method, Past as TestingMethod};
	use crate::core::{Candle, ValueType};
	use crate::helpers::RandomCandles;
	use crate::methods::tests::test_const;

	#[allow(dead_code)]
	const SIGMA: ValueType = 1e-8;

	#[test]
	fn test_past_const() {
		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_past1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first()).unwrap();

		let mut prev = None;
		candles.take(100).for_each(|x| {
			let q = ma.next(x);
			let p = prev.unwrap_or(x);
			assert_abs_diff_eq!(p.close, q.close);
			assert_abs_diff_eq!(p.volume, q.volume);
			assert_abs_diff_eq!(p.high, q.high);
			prev = Some(x);
		});
	}

	#[test]
	fn test_past() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]).unwrap();
			src.iter().enumerate().for_each(|(i, &x)| {
				assert_abs_diff_eq!(src[i.saturating_sub(length as usize)], ma.next(x))
			});
		});
	}
}
