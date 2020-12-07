use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Momentum](https://en.wikipedia.org/wiki/Momentum_(technical_analysis)) calculates difference between current
/// value and n-th value back, where n = `length`
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > `0`
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
/// use yata::methods::Momentum;
///
/// let mut change = Momentum::new(3, &1.0).unwrap(); // a.k.a. Change => let mut change = Change::new(3);
/// change.next(&1.0);
/// change.next(&2.0);
/// assert_eq!(change.next(&3.0), 2.0);
/// assert_eq!(change.next(&4.0), 3.0);
/// assert_eq!(change.next(&2.0), 0.0);
/// ```
///
/// ### At `length`=1 Momentum is the same as Derivative with `length`=1
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::Momentum;
/// use yata::methods::Derivative;
///
/// let mut change = Momentum::new(1, &1.0).unwrap();
/// let mut derivative = Derivative::new(1, &1.0).unwrap();
/// change.next(&1.0); derivative.next(&1.0);
/// change.next(&2.0); derivative.next(&2.0);
/// assert_eq!(change.next(&3.0), derivative.next(&3.0));
/// assert_eq!(change.next(&4.0), derivative.next(&4.0));
/// assert_eq!(change.next(&2.0), derivative.next(&2.0));
/// ```
///
/// # Performance
///
/// O(1)
///
/// # See Also
///
/// [`Rate of Change`](crate::methods::RateOfChange), [`Derivative`](crate::methods::Derivative)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
///

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Momentum {
	window: Window<ValueType>,
	last_value: ValueType,
}

/// Just an alias for [Momentum] method
pub type Change = Momentum;

/// Just an alias for [Momentum] method
pub type MTM = Momentum;

impl Method for Momentum {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				window: Window::new(length, value),
				last_value: value,
			}),
		}
	}

	#[inline]
	fn next(&mut self, &value: &Self::Input) -> Self::Output {
		value - self.window.push(value)
	}
}

#[cfg(test)]
mod tests {
	use super::{Method, Momentum as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[test]
	fn test_momentum_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, output);
		}
	}

	#[test]
	fn test_momentum1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		let mut prev = None;
		candles.take(100).map(|x| x.close).for_each(|x| {
			let q = ma.next(&x);
			let p = prev.unwrap_or(x);
			// assert_eq!(q, x - p);
			assert_eq_float(x - p, q);
			prev = Some(x);
		});
	}

	#[test]
	fn test_momentum() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();
			src.iter().enumerate().for_each(|(i, x)| {
				assert_eq_float(x - src[i.saturating_sub(length as usize)], ma.next(x));
			});
		});
	}
}
