use crate::core::{Method, MovingAverage};
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Simple Moving Average](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_average) of specified `length` for timeseries of type [`ValueType`]
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
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::SMA;
///
/// // SMA of length=3
/// let mut sma = SMA::new(3, 1.0).unwrap();
///
/// sma.next(1.0);
/// sma.next(2.0);
///
/// assert_eq!(sma.next(3.0), 2.0);
/// assert_eq!(sma.next(4.0), 3.0);
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
pub struct SMA {
	divider: ValueType,
	value: ValueType,
	window: Window<ValueType>,
}

impl SMA {
	/// Returns inner [`Window`](crate::core::Window). Useful for implementing in other methods and indicators.
	#[inline]
	#[must_use]
	pub const fn get_window(&self) -> &Window<ValueType> {
		&self.window
	}

	/// Returns 1/`length`. Useful for implementing in other methods and indicators.
	#[inline]
	#[must_use]
	pub const fn get_divider(&self) -> ValueType {
		self.divider
	}

	/// Returns last result value. Useful for implementing in other methods and indicators.
	#[inline]
	#[must_use]
	pub const fn get_last_value(&self) -> ValueType {
		self.value
	}
}

impl Method for SMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				divider: (length as ValueType).recip(),
				value,
				window: Window::new(length, value),
			}),
		}
	}

	#[inline]
	fn next(&mut self, &value: &Self::Input) -> Self::Output {
		let prev_value = self.window.push(value);
		self.value += (value - prev_value) * self.divider;

		self.value
	}
}

impl MovingAverage for SMA {}

#[cfg(test)]
mod tests {
	use super::{Method, SMA as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[allow(dead_code)]
	const SIGMA: ValueType = 1e-5;

	#[test]
	fn test_sma_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, &output);
		}
	}

	#[test]
	fn test_sma1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&x.close));
		});
	}

	#[test]
	fn test_sma() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|sma_length| {
			let mut sma = TestingMethod::new(sma_length, &src[0]).unwrap();

			src.iter().enumerate().for_each(|(i, x)| {
				let value = sma.next(x);
				let slice_from = i.saturating_sub((sma_length - 1) as usize);
				let slice_to = i;
				let slice = &src[slice_from..=slice_to];
				let mut sum: ValueType = slice.iter().sum();
				if slice.len() < sma_length as usize {
					sum += (sma_length as usize - slice.len()) as ValueType * src.first().unwrap();
				}

				assert_eq_float(sum / sma_length as ValueType, value);
			});
		});
	}
}
