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
/// `length` should be > `0`
///
/// # Input type
///
/// Input type is any `T: Copy + std::fmt::Debug`
///
/// # Output type
///
/// Output type is the same as input type
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::Past;
///
/// // Move of length=3
/// let mut past = Past::new(3, &1.0).unwrap();
///
/// past.next(&1.0);
/// past.next(&2.0);
/// past.next(&3.0);
///
/// assert_eq!(past.next(&4.0), 1.0);
/// assert_eq!(past.next(&5.0), 2.0);
/// assert_eq!(past.next(&6.0), 3.0);
/// ```
///
/// # Performance
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
	T: Clone + fmt::Debug;

impl<T> Method for Past<T>
where
	T: Clone + fmt::Debug,
{
	type Params = PeriodType;
	type Input = T;
	type Output = T;

	fn new(length: Self::Params, value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self(Window::new(length, value.clone()))),
		}
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> T {
		self.0.push(value.clone())
	}
}

#[cfg(test)]
mod tests {
	use super::{Method, Past as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[test]
	fn test_past_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, &output);
		}
	}

	#[test]
	fn test_past1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first()).unwrap();

		let mut prev = None;
		candles.take(100).for_each(|x| {
			let q = ma.next(&x);
			let p = prev.unwrap_or(x);

			assert_eq!(p, q);
			prev = Some(x);
		});
	}

	#[test]
	fn test_past() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();
			src.iter().enumerate().for_each(|(i, x)| {
				assert_eq_float(src[i.saturating_sub(length as usize)], ma.next(x))
			});
		});
	}
}
