use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Linear regression](https://en.wikipedia.org/wiki/Linear_regression) moving average for last `length` values of timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > `1`
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// # Performance
///
/// O(1)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LinReg {
	length: PeriodType,
	s_xy: ValueType,
	s_y: ValueType,
	s_x: ValueType,
	float_length: ValueType,
	length_invert: ValueType,
	divider: ValueType,
	window: Window<ValueType>,
}

impl Method<'_> for LinReg {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		match length {
			0 | 1 => Err(Error::WrongMethodParameters),
			length => {
				let l64 = length as usize;
				let float_length = length as ValueType;
				let length_invert = -float_length.recip();

				let n_1 = l64 - 1;
				let s_x = l64 * n_1 / 2;
				let s_x2 = s_x * (2 * n_1 + 1) / 3;

				let divider = ((l64 * s_x2 - s_x * s_x) as ValueType).recip();

				let s_x = -(s_x as ValueType);
				Ok(Self {
					length,
					float_length,
					length_invert,
					divider,
					s_x,
					s_y: -value * float_length,
					s_xy: value * s_x,
					window: Window::new(length, value),
				})
			}
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let past_value = self.window.push(value);

		self.s_xy += past_value.mul_add(self.float_length, self.s_y);
		self.s_y += past_value - value;

		// y = kx + b, x=0
		let k = self.s_xy.mul_add(self.float_length, self.s_x * self.s_y) * self.divider;
		self.s_x.mul_add(k, self.s_y) * self.length_invert
	}
}

#[cfg(test)]
#[allow(clippy::suboptimal_flops)]
mod tests {
	use super::{LinReg as TestingMethod, Method};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const_float;

	#[test]
	fn test_lin_reg_const() {
		for i in 2..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_lin_reg() {
		#![allow(clippy::similar_names)]

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(2..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]).unwrap();
			let length = length as usize;

			let n = length as ValueType;
			let s_x: usize = (0..length).sum();
			let s_x2: usize = (0..length).map(|x| x * x).sum();

			let s_x = s_x as ValueType;
			let s_x2 = s_x2 as ValueType;

			src.iter().enumerate().for_each(|(i, &x)| {
				let ma_value = ma.next(x);

				let s_xy =
					(0..length).fold(0.0, |s, j| s + j as ValueType * src[i.saturating_sub(j)]);
				let s_y: ValueType = (0..length).map(|j| src[i.saturating_sub(j)]).sum();

				let a = (n * s_xy - s_x * s_y) / (n * s_x2 - s_x * s_x);
				let b = (s_y - a * s_x) / n;

				assert_eq_float(b, ma_value);
			});
		});
	}
}
