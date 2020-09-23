use super::Conv;
use crate::core::Method;
use crate::core::{PeriodType, ValueType};

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
/// O(`length`)
///
/// This method is relatively slower compare to the most of the other methods.
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
pub struct SWMA(Conv);

impl Method for SWMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "SWMA: length must be > 0");

		let ln2 = length / 2;

		let k_sum =
			(ln2 * (ln2 + 1) + if length % 2 == 1 { (length + 1) / 2 } else { 0 }) as ValueType;

		let mut weights = vec![0.; length as usize];
		(0..(length + 1) / 2).for_each(|i| {
			let q = (i + 1) as ValueType / k_sum;
			weights[i as usize] = q;
			weights[(length - i - 1) as usize] = q;
		});

		Self(Conv::new(weights, value))
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.0.next(value)
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused_imports)]
	use super::{Conv, Method, SWMA as TestingMethod};
	use crate::core::{PeriodType, ValueType};
	use crate::helpers::RandomCandles;

	#[allow(dead_code)]
	const SIGMA: ValueType = 1e-6;

	#[test]
	fn test_swma_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input);

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_swma1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

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
		];

		weights.iter().for_each(|weights| {
			let wsum: ValueType = weights.iter().sum();
			let length = weights.len();

			let mut ma = TestingMethod::new(length as PeriodType, src[0]);
			let mut conv = Conv::new(weights.clone(), src[0]);

			src.iter().enumerate().for_each(|(i, &x)| {
				let wcv = weights
					.iter()
					.enumerate()
					.fold(0.0, |sum, (j, &w)| sum + w * src[i.saturating_sub(j)]);

				let value = ma.next(x);
				let value2 = wcv / wsum;
				let value3 = conv.next(x);

				assert!((value2 - value).abs() < SIGMA);
				assert!((value3 - value).abs() < SIGMA);
			});
		});
	}
}
