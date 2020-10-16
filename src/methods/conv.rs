use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Convolution Moving Average with specified `weights` for timeseries of [`ValueType`].
///
/// # Parameters
///
/// Has a single parameter `weights`: Vec<[`ValueType`]>
///
/// `weights` vector's length must be > 0
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
/// O(length(`weights`))
///
/// This method is relatively slow compare to the other methods.
///
/// # See also
///
/// [`WMA`](crate::methods::WMA), [`SWMA`](crate::methods::SWMA)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Conv {
	weights: Vec<ValueType>,
	window: Window<ValueType>,
	wsum_invert: ValueType,

	initialized: bool,
}

impl Method for Conv {
	type Params = Vec<ValueType>;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(weights: Self::Params, value: Self::Input) -> Result<Self, Error> {
		if let 0 = weights.len() {
			Err(Error::WrongMethodParameters)
		} else {
			let wsum_invert = weights.iter().sum::<ValueType>().recip();

			Ok(Self {
				window: Window::new(weights.len() as PeriodType, value),
				weights,
				wsum_invert,

				initialized: false,
			})
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.window.push(value);
		self.window
			.iter()
			.zip(self.weights.iter())
			.map(|(value, &weight)| value * weight)
			.sum::<ValueType>()
			* self.wsum_invert
	}
}

#[cfg(test)]
mod tests {
	// #![allow(unused_imports)]
	use super::{Conv as TestingMethod, Method};
	use crate::core::{PeriodType, ValueType};
	use crate::helpers::RandomCandles;

	// #[allow(dead_code)]
	const SIGMA: ValueType = 1e-6;

	fn get_weights(length: PeriodType) -> Vec<ValueType> {
		(0..length)
			.map(|i| {
				let i_f = i as ValueType;
				i_f.sin().abs() * i_f + 1.0
			})
			.collect()
	}

	#[test]
	fn test_conv_const() {
		use super::*;
		use crate::core::Method;
		use crate::methods::tests::test_const_float;

		for i in 1..30 {
			let weights = get_weights(i);
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(weights, input).unwrap();

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_conv1() {
		let mut candles = RandomCandles::default();

		let weights = get_weights(1);
		let mut ma = TestingMethod::new(weights, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_conv() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..30).for_each(|weights_count| {
			let mut weights = get_weights(weights_count);
			let wsum: ValueType = weights.iter().sum();

			let mut ma = TestingMethod::new(weights.clone(), src[0]).unwrap();
			weights.reverse();

			src.iter().enumerate().for_each(|(i, &x)| {
				let wcv = weights
					.iter()
					.enumerate()
					.fold(0.0, |sum, (j, &w)| sum + w * src[i.saturating_sub(j)]);

				let value = ma.next(x);
				let value2 = wcv / wsum;
				assert!(
					(value2 - value).abs() < SIGMA,
					"{}, {}, {:?}",
					value,
					value2,
					&weights
				);
			});
		});
	}
}
