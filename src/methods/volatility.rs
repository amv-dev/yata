use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Calculate moving linear volatility for last `length` values of type [`ValueType`]
///
/// LV = Σ\[abs([`Derivative`]\(1\))\]
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
/// Output is always positive or `0.0`
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::LinearVolatility;
///
/// // volatility over 3 periods
/// let mut vol = LinearVolatility::new(3, 1.0).unwrap();
/// vol.next(1.0);
/// vol.next(2.0);
/// assert_eq!(vol.next(3.0), 2.0);
/// assert_eq!(vol.next(1.0), 4.0);
/// ```
///
/// # Performance
///
/// O(1)
///
/// [`Derivative`]: crate::methods::Derivative
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LinearVolatility {
	window: Window<ValueType>,
	prev_value: ValueType,
	volatility: ValueType,
}

impl Method for LinearVolatility {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				window: Window::new(length, 0.),
				prev_value: value,
				volatility: 0.,
			}),
		}
	}

	#[inline]
	fn next(&mut self, &value: &Self::Input) -> Self::Output {
		let derivative = (value - self.prev_value).abs();
		self.prev_value = value;

		let past_derivative = self.window.push(derivative);

		self.volatility += derivative - past_derivative;

		self.volatility
	}
}

#[cfg(test)]
mod tests {
	use super::{LinearVolatility as TestingMethod, Method};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;
	use crate::methods::Derivative;

	#[test]
	fn test_volatility_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, &input).unwrap();

			test_const(&mut method, &input, 0.0);
		}
	}

	#[test]
	fn test_volatility1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();
		let mut der = Derivative::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			let v1 = der.next(&x.close).abs();
			let v2 = ma.next(&x.close);

			assert_eq_float(v1, v2);
		});
	}

	#[test]
	fn test_linear_volatility() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|ma_length| {
			let mut ma = TestingMethod::new(ma_length, &src[0]).unwrap();
			let ma_length = ma_length as usize;

			src.iter().enumerate().for_each(|(i, x)| {
				let mut s = 0.;
				for j in 0..ma_length {
					let d = src[i.saturating_sub(j)] - src[i.saturating_sub(j + 1)];
					s += d.abs();
				}

				let value = ma.next(x);
				let value2 = s;

				assert_eq_float(value2, value);
			});
		});
	}
}
