use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};
use crate::methods::SMA;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Moving [Standart Deviation](https://en.wikipedia.org/wiki/Standard_deviation) over the window of size `length` for timeseries of type [`ValueType`]
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
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::StDev;
///
/// // StDev over the window with length=3
/// let mut stdev = StDev::new(3, 1.0).unwrap();
///
/// stdev.next(1.0);
/// stdev.next(2.0);
///
/// assert_eq!(stdev.next(3.0), 1.0);
/// assert_eq!(stdev.next(4.0), 1.0);
/// ```
///
/// # Performance
///
/// O(1)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StDev {
	val_sum: ValueType,
	sq_val_sum: ValueType,
	k: ValueType,
	window: Window<ValueType>,
	ma: SMA,
}

impl Method for StDev {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 | 1 => Err(Error::WrongMethodParameters),
			length => {
				let k = ((length - 1) as ValueType).recip();

				let float_length = length as ValueType;
				let val_sum = value * float_length;
				Ok(Self {
					val_sum,
					sq_val_sum: value * val_sum,
					k,
					window: Window::new(length, *value),
					ma: SMA::new(length, value)?,
				})
			}
		}
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		let prev_value = self.window.push(*value);
		self.sq_val_sum += value * value - prev_value * prev_value;
		self.val_sum += value - prev_value;
		let ma_value = self.ma.next(value);

		// let sum = self.sq_val_sum - ma_value * self.val_sum;
		let sum = self.val_sum.mul_add(-ma_value, self.sq_val_sum);

		(sum.abs() * self.k).sqrt()
	}
}

#[cfg(test)]
#[allow(clippy::suboptimal_flops)]
mod tests {
	use super::{Method, StDev as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const_float;

	#[test]
	fn test_st_dev_const() {
		for i in 2..255 {
			let input = &((i as ValueType + 56.0) / 16.3251);
			let mut method = TestingMethod::new(i, input).unwrap();

			test_const_float(&mut method, input, 0.0);
		}
	}

	#[test]
	fn test_st_dev() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles
			.take(300)
			.enumerate()
			.map(|(i, x)| x.close * if i % 2 == 0 { 1.0 } else { -1.0 })
			.collect();

		(2..255).for_each(|ma_length| {
			let mut ma = TestingMethod::new(ma_length, &src[0]).unwrap();
			let ma_length = ma_length as usize;

			src.iter().enumerate().for_each(|(i, x)| {
				let mut avg = 0.;
				for j in 0..ma_length {
					avg += src[i.saturating_sub(j)] / ma_length as ValueType;
				}

				let mut diff_sq_sum = 0.;
				for j in 0..ma_length {
					diff_sq_sum += (src[i.saturating_sub(j)] - avg).powi(2);
				}

				let value = ma.next(x);
				let value2 = (diff_sq_sum / (ma_length - 1) as ValueType).sqrt();
				assert_eq_float(value, value2);
			});
		});
	}
}
