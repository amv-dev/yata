use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Integrates (summarizes) [`ValueType`] values for the given window size `length`
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// If `length` == 0, then integrates since the beginning of timeseries
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
/// use yata::methods::Integral;
///
/// // Integrates over last 3 values
/// let mut integral = Integral::new(3, 1.0).unwrap();
///
/// integral.next(1.0);
/// integral.next(2.0);
/// assert_eq!(integral.next(3.0), 6.0); // 1 + 2 + 3
/// assert_eq!(integral.next(4.0), 9.0); // 2 + 3 + 4
/// assert_eq!(integral.next(5.0), 12.0); // 3 + 4 + 5
/// ```
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::Integral;
///
/// // Integrates since the beginning
/// let mut integral = Integral::new(0, 1.0).unwrap(); // same as Integral::default()
///
/// integral.next(1.0);
/// integral.next(2.0);
/// assert_eq!(integral.next(3.0), 6.0); // 1 + 2 + 3
/// assert_eq!(integral.next(4.0), 10.0); // 1 + 2 + 3 + 4
/// assert_eq!(integral.next(5.0), 15.0); // 1 + 2 + 3 + 4 + 5
/// ```
///
/// ### Intergal is opposite method for Derivative
/// ```
/// use yata::prelude::*;
/// use yata::methods::{Integral, Derivative};
///
/// let s = [1.0, 2.0, 3.0, 3.0, 2.5, 3.5, 5.0];
/// let mut integral = Integral::default();
/// let mut derivative = Derivative::new(1, s[0]).unwrap();
///
/// (&s).iter().for_each(|&v| {
///     let integration_constant = s[0];
///     let der = derivative.next(v);
///     let integr = integral.next(der) + integration_constant;
///
///     assert_eq!(integr, v);
/// });
/// ```
///
/// # Performance
///
/// O(1)
///
/// # See also
///
/// [Derivative](crate::methods::Derivative)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Integral {
	value: ValueType,
	window: Window<ValueType>,
}

/// Just an alias for Integral
pub type Sum = Integral;

impl Method for Integral {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		Ok(Self {
			window: Window::new(length, value),
			value: value * length as ValueType,
		})
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.value += value;

		if !self.window.is_empty() {
			self.value -= self.window.push(value);
		}

		self.value
	}
}

impl Default for Integral {
	fn default() -> Self {
		Self::new(0, 0.0).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::{Integral as TestingMethod, Method};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[test]
	fn test_integral_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	#[should_panic]
	fn test_integral0_const() {
		use crate::core::Method;
		use crate::methods::tests::test_const;

		let input = (5.0 + 56.0) / 16.3251;
		let mut method = TestingMethod::new(0, input).unwrap();

		let output = method.next(input);
		test_const(&mut method, input, output);
	}

	#[test]
	fn test_integral0() {
		let src: Vec<ValueType> = RandomCandles::default()
			.take(100)
			.map(|x| x.close)
			.collect();

		let mut ma = TestingMethod::new(0, src[0]).unwrap();
		let mut q = Vec::new();

		src.iter().enumerate().for_each(|(i, &x)| {
			let value1 = ma.next(x);
			let value2 = src.iter().take(i + 1).fold(0.0, |s, &c| s + c);
			q.push(x);

			// assert_eq!(value1, value2, "at index {} with value {}: {:?}", i, x, q);
			assert_eq_float(value2, value1);
		});
	}

	#[test]
	fn test_integral1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(x.close));
		});
	}

	#[test]
	fn test_integral() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]).unwrap();
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, &x)| {
				let value1 = ma.next(x);

				let value2 = (0..length).fold(0.0, |s, j| s + src[i.saturating_sub(j)]);

				assert_eq_float(value2, value1);
			});
		});
	}
}
