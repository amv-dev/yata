use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Volume Weighed Moving Average](https://en.wikipedia.org/wiki/Moving_average#Weighted_moving_average) of specified `length`
/// for timeseries of type ([`ValueType`], [`ValueType`]) which represents pair of values (`value`, `volume`)
///
/// # Parameters
///
/// `length` should be > `0`
///
/// Has a single parameter `length`: [`PeriodType`]
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
/// use yata::methods::VWMA;
///
/// // VWMA of length=3
/// let mut vwma = VWMA::new(3, (3.0, 1.0)).unwrap();
///
/// // input value is a pair of f64 (value, weight)
/// vwma.next((3.0, 1.0));
/// vwma.next((6.0, 1.0));
///
/// assert_eq!(vwma.next((9.0, 2.0)), 6.75);
/// assert!((vwma.next((12.0, 0.5))- 8.571428571428571).abs() < 1e-10);
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
pub struct VWMA {
	sum: ValueType,
	vol_sum: ValueType,
	window: Window<(ValueType, ValueType)>,
}

impl Method for VWMA {
	type Params = PeriodType;
	type Input = (ValueType, ValueType);
	type Output = ValueType;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				sum: value.0 * value.1 * length as ValueType,
				vol_sum: value.1 * length as ValueType,
				window: Window::new(length, value),
			}),
		}
	}

	#[inline]
	fn next(&mut self, &value: &Self::Input) -> Self::Output {
		let past_value = self.window.push(value);

		self.vol_sum += value.1 - past_value.1;
		self.sum += value.0.mul_add(value.1, -past_value.0 * past_value.1);

		self.sum / self.vol_sum
	}
}

#[cfg(test)]
#[allow(clippy::suboptimal_flops)]
mod tests {
	use super::{Method, VWMA as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[test]
	fn test_vwma_const() {
		for i in 1..255 {
			let input = ((i as ValueType + 56.0) / 16.3251, 3.55);
			let mut method = TestingMethod::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, &output);
		}
	}

	#[test]
	fn test_vwma1() {
		let mut candles = RandomCandles::default();

		let mut ma =
			TestingMethod::new(1, &(candles.first().close, candles.first().volume)).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&(x.close, x.volume)));
		});
	}

	#[test]
	fn test_vwma() {
		let candles = RandomCandles::default();

		let src: Vec<(ValueType, ValueType)> =
			candles.take(300).map(|x| (x.close, x.volume)).collect();

		(1..255).for_each(|ma_length| {
			let mut ma = TestingMethod::new(ma_length, &src[0]).unwrap();
			let ma_length = ma_length as usize;

			src.iter().enumerate().for_each(|(i, x)| {
				let mut slice: Vec<(ValueType, ValueType)> = Vec::with_capacity(ma_length);
				for x in 0..ma_length {
					slice.push(src[i.saturating_sub(x)]);
				}

				let sum = slice
					.iter()
					.fold(0.0, |s, (close, volume)| s + close * volume);
				let vol_sum = slice.iter().fold(0.0, |s, (_close, vol)| s + vol);

				let value2 = sum / vol_sum;

				assert_eq_float(value2, ma.next(x));
			});
		});
	}
}
