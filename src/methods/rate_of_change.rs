use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Just an alias for [RateOfChange](RateOfChange) method
pub type ROC = RateOfChange;

/// [Rate of change](https://en.wikipedia.org/wiki/Momentum_(technical_analysis)) calculates relative difference between current
/// value and n-th value back, where n = `length`
///
/// `ROC` = (`value` - `n_th_value`) / `n_th_value`
///
/// `ROC` = [`Momentum`] / `n_th_value`
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
/// Input value should always be greater than `0.0.` (`value` > `0.0`)
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// # Performance
///
/// O(1)
///
/// # See Also
///
/// [`Momentum`], [`Derivative`](crate::methods::Derivative)
///
/// [`Momentum`]: crate::methods::Momentum
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RateOfChange(Window<ValueType>);

impl Method for RateOfChange {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self(Window::new(length, value))),
		}
	}

	#[inline]
	fn next(&mut self, &value: &Self::Input) -> Self::Output {
		let prev_value = self.0.push(value);

		(value - prev_value) / prev_value
	}
}

#[cfg(test)]
mod tests {
	use super::{Method, ROC as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;
	use crate::methods::{Derivative, Past};

	#[test]
	fn test_rate_of_change_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, output);
		}
	}

	#[test]
	fn test_rate_of_change1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();
		let mut der = Derivative::new(1, &candles.first().close).unwrap();
		let mut mv = Past::new(1, &candles.first().close).unwrap();

		candles.take(100).map(|x| x.close).for_each(|x| {
			let value = ma.next(&x);
			let value2 = der.next(&x) / mv.next(&x);
			assert_eq_float(value2, value);
		});
	}

	#[test]
	fn test_rate_of_change() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();

			src.iter().enumerate().for_each(|(i, x)| {
				let value = ma.next(x);
				let left_value = src[i.saturating_sub(length as usize)];

				let value2 = (x - left_value) / left_value;

				assert_eq_float(value2, value);
			});
		});
	}
}
