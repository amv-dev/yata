use crate::core::Method;
use crate::core::{PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Simple Moving Average](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_average) of specified `length` for timeseries of type [`ValueType`]
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
/// use yata::methods::SMA;
///
/// // SMA of length=3
/// let mut sma = SMA::new(3, 1.0);
///
/// sma.next(1.0);
/// sma.next(2.0);
///
/// assert_eq!(sma.next(3.0), 2.0);
/// assert_eq!(sma.next(4.0), 3.0);
/// ```
///
/// # Perfomance
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

impl Method for SMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "SMA: length should be > 0");

		Self {
			divider: (length as ValueType).recip(),
			value,
			window: Window::new(length, value),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let prev_value = self.window.push(value);
		self.value += (value - prev_value) * self.divider;

		self.value
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused_imports)]
	use super::{Method, SMA as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::RandomCandles;

	#[allow(dead_code)]
	const SIGMA: ValueType = 1e-8;

	#[test]
	fn test_sma_const() {
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
	fn test_sma1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_sma() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|sma_length| {
			let mut sma = TestingMethod::new(sma_length, src[0]);

			src.iter().enumerate().for_each(|(i, &x)| {
				let value = sma.next(x);
				let slice_from = i.saturating_sub((sma_length - 1) as usize);
				let slice_to = i;
				let slice = &src[slice_from..=slice_to];
				let mut sum: ValueType = slice.iter().sum();
				if slice.len() < sma_length as usize {
					sum += (sma_length as usize - slice.len()) as ValueType * src.first().unwrap();
				}

				assert!((sum / sma_length as ValueType - value).abs() < SIGMA);
			});
		});
	}
}
