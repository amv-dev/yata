use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType};
use crate::methods::SMA;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Mean absolute deviation](https://en.wikipedia.org/wiki/Average_absolute_deviation) of specified `length` for timeseries of type [`ValueType`]
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
/// # Performance
///
/// O(`length`)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MeanAbsDev(SMA);

impl MeanAbsDev {
	/// Returns reference to inner SMA. Useful for implementing in other methods and indicators.
	#[must_use]
	pub const fn get_sma(&self) -> &SMA {
		&self.0
	}
}

impl Method for MeanAbsDev {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self(SMA::new(length, value)?)),
		}
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		let mean = self.0.next(value);

		self.0
			.get_window()
			.as_slice()
			.iter()
			.map(|x| x - mean)
			.map(ValueType::abs)
			.sum::<ValueType>()
			* self.0.get_divider()
	}
}

#[cfg(test)]
mod tests {
	use super::{MeanAbsDev as TestingMethod, Method};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};

	#[test]
	fn test_mean_abs_dev_const() {
		for i in 2..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, &input).unwrap();

			let output = method.next(&input);
			assert_eq_float(0.0, output);
		}
	}

	#[test]
	fn test_mean_abs_dev1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(0.0, ma.next(&x.close));
		});
	}

	#[test]
	fn test_mean_abs_dev0() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(2..255).for_each(|length| {
			let mut method = TestingMethod::new(length, &src[0]).unwrap();

			src.iter().enumerate().for_each(|(i, x)| {
				let mut sum = 0.0;

				for j in 0..length {
					sum += src[i.saturating_sub(j as usize)];
				}

				let sma = sum / length as ValueType;

				let mut sum2 = 0.0;
				for j in 0..length {
					sum2 += (sma - src[i.saturating_sub(j as usize)]).abs();
				}

				let q = sum2 / length as ValueType;

				let value = method.next(x);
				assert_eq_float(q, value);
			});
		});
	}
}
