use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType};
use crate::methods::SMM;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Median absolute deviation](https://en.wikipedia.org/wiki/Average_absolute_deviation) of specified `length` for timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 1
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
/// O(`length`)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MedianAbsDev {
	smm: SMM,
	divider: ValueType,
}

impl MedianAbsDev {
	/// Returns reference to inner SMA. Useful for implementing in other methods and indicators.
	pub fn get_smm(&self) -> &SMM {
		&self.smm
	}
}

impl Method for MedianAbsDev {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		match length {
			0 | 1 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				smm: SMM::new(length, value)?,
				divider: (length as ValueType).recip(),
			}),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let smm = self.smm.next(value);

		self.smm
			.get_window()
			.as_slice()
			.iter()
			.map(|x| (x - smm).abs())
			.sum::<ValueType>()
			* self.divider
	}
}

#[cfg(test)]
mod tests {
	use super::{MedianAbsDev as TestingMethod, Method};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use std::cmp::Ordering;

	#[test]
	fn test_median_abs_dev_const() {
		for i in 2..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			assert_eq_float(0.0, output);
		}
	}

	#[test]
	#[should_panic]
	fn test_median_abs_dev1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(0.0, ma.next(x.close));
		});
	}

	#[test]
	fn test_median_abs_dev0() {
		#![allow(clippy::similar_names)]
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(2..20).for_each(|length| {
			let mut method = TestingMethod::new(length, src[0]).unwrap();

			src.iter().enumerate().for_each(|(i, &x)| {
				let mut smm_slice = Vec::with_capacity(length as usize);

				for j in 0..length {
					smm_slice.push(src[i.saturating_sub(j as usize)]);
				}

				smm_slice.sort_unstable_by(|a, b| {
					if a < b {
						Ordering::Less
					} else if a > b {
						Ordering::Greater
					} else {
						Ordering::Equal
					}
				});

				let smm = (smm_slice[(length as usize) / 2]
					+ smm_slice[((length - 1) as usize) / 2])
					/ 2.0;

				let mut sum = 0.0;
				for j in 0..length {
					sum += (smm - src[i.saturating_sub(j as usize)]).abs();
				}

				let q = sum / length as ValueType;

				let value = method.next(x);
				assert_eq_float(q, value);
			});
		});
	}
}
