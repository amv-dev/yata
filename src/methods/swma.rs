use crate::core::{Error, Method, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Symmetrically Weighted Moving Average of specified `length` for timeseries of [`ValueType`].
///
/// F.e. if `length` = 4, then weights are: [ 1.0, 2.0, 2.0, 1.0 ].
///
/// If `length` = 5, then weights are: [ 1.0, 2.0, 3.0, 2.0, 1.0 ].
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
/// # Perfomance
///
/// O(1)
///
/// # See also
///
/// [`WMA`](crate::methods::WMA)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SWMA {
	right_total: ValueType,
	right_float_length: ValueType,
	right_window: Window<ValueType>,

	left_total: ValueType,
	left_float_length: ValueType,
	left_window: Window<ValueType>,

	invert_sum: ValueType,
	numerator: ValueType,
}

impl Method for SWMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => {
				let left_length = (length + 1) / 2;
				let right_length = length / 2;

				let right_length2 = right_length as usize;
				let left_length2 = left_length as usize;

				let sum = ((left_length2 * (left_length2 + 1)) / 2
					+ (right_length2 * (right_length2 + 1) / 2)) as ValueType;

				let right_float_length = -(right_length as ValueType);
				let left_float_length = left_length as ValueType;

				Ok(Self {
					left_total: -value * left_length2 as ValueType,
					left_float_length,
					left_window: Window::new(left_length, value),

					right_total: value * right_length2 as ValueType,
					right_float_length,
					right_window: Window::new(right_length, value),

					invert_sum: sum.recip(),

					numerator: value * sum,
				})
			}
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		if self.right_window.is_empty() {
			return value;
		}

		let prev_value = self.right_window.push(value);
		self.right_total += value - prev_value;
		self.numerator += prev_value.mul_add(self.right_float_length, self.right_total);

		let value = prev_value;
		let prev_value = self.left_window.push(value);
		self.numerator += value.mul_add(self.left_float_length, self.left_total);
		self.left_total += prev_value - value;

		self.numerator * self.invert_sum
	}
}

#[cfg(test)]
mod tests {
	use super::{Method, SWMA as TestingMethod};
	use crate::core::{PeriodType, ValueType};
	use crate::helpers::RandomCandles;
	use crate::methods::tests::test_const;
	use crate::methods::Conv;

	const SIGMA: ValueType = 1e-5;

	#[test]
	fn test_swma_const() {
		for i in 2..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_swma1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_swma() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		let weights: Vec<Vec<ValueType>> = vec![
			vec![1.0],
			vec![1.0, 1.0],
			vec![1.0, 2.0, 1.0],
			vec![1.0, 2.0, 2.0, 1.0],
			vec![1.0, 2.0, 3.0, 2.0, 1.0],
			vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0],
			vec![1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0],
			vec![1.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0],
			vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0],
			vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 4.0, 3.0, 2.0, 1.0],
			vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
			vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
			vec![
				1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
			],
		];

		weights.iter().for_each(|weights| {
			let wsum: ValueType = weights.iter().sum();
			let length = weights.len();

			let mut ma = TestingMethod::new(length as PeriodType, src[0]).unwrap();
			let mut conv = Conv::new(weights.clone(), src[0]).unwrap();

			src.iter().enumerate().for_each(|(i, &x)| {
				let wcv = weights
					.iter()
					.enumerate()
					.fold(0.0, |sum, (j, &w)| sum + w * src[i.saturating_sub(j)]);

				let value = ma.next(x);
				let value2 = wcv / wsum;
				let value3 = conv.next(x);

				assert!(
					(value2 - value).abs() < SIGMA,
					"Got {}, should {} at {} with length {}",
					value2,
					value,
					i,
					length
				);
				assert!((value3 - value).abs() < SIGMA);
			});
		});
	}
}
