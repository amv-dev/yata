#![allow(unused_imports)]
use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};
use crate::methods::MeanAbsDev;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Commodity channel index](https://en.wikipedia.org/wiki/Commodity_channel_index) of specified `length` for time series of type [`ValueType`]
///
/// In the original formula there is constant coefficient K = 1/0.015. This implementation does not include this coefficient (tl;dr it is 1.0).
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
/// # Performance
///
/// O(`length`)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CCI(MeanAbsDev);

impl Method for CCI {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self(MeanAbsDev::new(length, value)?)),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let mean = self.0.next(value);
		let ma = self.0.get_sma().get_last_value();

		if mean > 0.0 {
			(value - ma) / mean
		} else {
			0.
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Method, CCI as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};

	#[test]
	fn test_cci_const() {
		for i in 2..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			assert_eq_float(output, 0.0);
		}
	}

	#[test]
	fn test_cci1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(0., ma.next(x.close));
		});
	}

	#[test]
	fn test_cci() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(2..20).for_each(|length| {
			let mut method = TestingMethod::new(length, src[0]).unwrap();

			src.iter().enumerate().for_each(|(i, &x)| {
				let mut sum = 0.0;
				for j in 0..length {
					sum += src[i.saturating_sub(j as usize)];
				}

				let ma = sum / length as ValueType;
				let mut dev_sum = 0.0;
				for j in 0..length {
					dev_sum += (src[i.saturating_sub(j as usize)] - ma).abs();
				}

				let mean_dev = dev_sum / length as ValueType;

				let q = if mean_dev == 0.0 {
					0.0
				} else {
					(x - ma) / mean_dev
				};

				let value = method.next(x);
				assert_eq_float(q, value);
			});
		});
	}
}
