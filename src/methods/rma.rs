use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Running Moving Average](https://en.wikipedia.org/wiki/Moving_average#Modified_moving_average) of specified `length` for timeseries of type [`ValueType`]
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
/// use yata::methods::RMA;
///
/// // RMA of length=3
/// let mut rma = RMA::new(3, &1.0).unwrap();
///
/// rma.next(&1.0);
/// rma.next(&2.0);
///
/// assert!((rma.next(&3.0)-1.8888888).abs() < 1e-5);
/// assert!((rma.next(&4.0)-2.5925925925).abs() < 1e-5);
/// ```
///
/// # Performance
///
/// O(1)
///
/// # See also
///
/// [`EMA`](crate::methods::EMA)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
///
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RMA {
	alpha: ValueType,
	alpha_rev: ValueType,
	prev_value: ValueType,
}

/// Just an alias for RMA
pub type MMA = RMA;

/// Just an alias for RMA
pub type SMMA = RMA;

impl Method<'_> for RMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => {
				let alpha = (length as ValueType).recip();
				Ok(Self {
					alpha,
					alpha_rev: 1. - alpha,
					prev_value: value,
				})
			}
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let value = self.alpha.mul_add(value, self.alpha_rev * self.prev_value);
		self.prev_value = value;

		value
	}
}

#[cfg(test)]
#[allow(clippy::suboptimal_flops)]
mod tests {
	use super::{Method, RMA as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};

	#[test]
	fn test_rma_const() {
		use crate::methods::tests::test_const_float;

		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_rma1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(x.close));
		});
	}

	#[test]
	fn test_rma() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]).unwrap();

			let mut value2 = src[0];
			src.iter().for_each(|&x| {
				let value = ma.next(x);

				value2 = (x + (length - 1) as ValueType * value2) / (length as ValueType);

				assert_eq_float(value2, value);
			});
		});
	}
}
