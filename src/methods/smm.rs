use crate::core::Method;
use crate::core::{PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///
/// [Simle Moving Median](https://en.wikipedia.org/wiki/Moving_average#Moving_median) of specified `length` for timeseries of type [`ValueType`]
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
/// use yata::methods::SMM;
///
/// // SMM of length=3
/// let mut smm = SMM::new(3, 1.0);
///
/// smm.next(1.0);
/// smm.next(2.0);
///
/// assert_eq!(smm.next(3.0), 2.0);
/// assert_eq!(smm.next(100.0), 3.0);
/// ```
///
/// # Perfomance
///
/// O(`length` x log(`length`))
///
/// This method is relatively very slow compare to the other methods.
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SMM {
	is_even: bool,
	half: PeriodType,
	half_m1: PeriodType,
	window: Window<ValueType>,
	slice: Vec<ValueType>,
}

impl Method for SMM {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "SMM: length should be > 0");

		let half = length / 2;

		Self {
			is_even: length % 2 == 0,
			half,
			half_m1: half.saturating_sub(1),
			window: Window::new(length, value),
			slice: vec![value; length as usize],
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let old_value = self.window.push(value);

		// self.slice.copy_from_slice(self.window.as_slice());

		// self.slice
		// 	.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

		let mut old_index = self.slice.len() - 1;
		let mut index = self.slice.len() - 1;

		for (i, &v) in self.slice.iter().enumerate() {
			// it is safe to compare f64s here
			if v > old_value {
				old_index = i.saturating_sub(1);
			}

			if v > value {
				index = i.saturating_sub(1);
			}
		}

		println!(
			"Inserting {} at {}, removing {} from {} into {:?}",
			value, index, old_value, old_index, self.slice
		);
		if index > old_index {
			self.slice.copy_within((old_index + 1)..=index, old_index);
		} else if index < old_index {
			self.slice.copy_within(index..old_index, index + 1);
		}

		self.slice[index] = value;

		println!(
			"Result {:?}\n----------------------------------------------------",
			self.slice
		);

		if self.is_even {
			(self.slice[self.half as usize] + self.slice[self.half_m1 as usize]) * 0.5
		} else {
			self.slice[self.half as usize]
		}
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused_imports)]
	use super::{Method, SMM as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::RandomCandles;

	#[allow(dead_code)]
	const SIGMA: ValueType = 1e-8;

	#[test]
	fn test_smm_const() {
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
	fn test_smm1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_smm0() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|ma_length| {
			let mut ma = TestingMethod::new(ma_length, src[0]);
			let ma_length = ma_length as usize;

			src.iter().enumerate().for_each(|(i, &x)| {
				let value = ma.next(x);
				let slice_from = i.saturating_sub(ma_length - 1);
				let slice_to = i;
				let mut slice = Vec::with_capacity(ma_length);

				src.iter()
					.skip(slice_from)
					.take(slice_to - slice_from + 1)
					.for_each(|&x| slice.push(x));
				while slice.len() < ma_length {
					slice.push(src[0]);
				}

				slice.sort_by(|a, b| a.partial_cmp(b).unwrap());

				let value2;

				if ma_length % 2 == 0 {
					value2 = (slice[ma_length / 2] + slice[ma_length / 2 - 1]) / 2.0;
				} else {
					value2 = slice[ma_length / 2];
				}
				assert!((value2 - value).abs() < SIGMA);
			});
		});
	}
}
