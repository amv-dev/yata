use crate::core::Method;
use crate::core::{PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Derivative](https://en.wikipedia.org/wiki/Derivative) of specified window `length` for timeseries of [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 0
///
/// Default is 1
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
/// use yata::methods::Derivative;
///
/// let s = vec![0.0, 1.0, 3.0, 0.5, 2.0, -10.0];
///	let r = vec![0.0, 1.0, 2.0,-2.5, 1.5, -12.0];
///
/// let mut derivative = Derivative::new(1, s[0]);
///
/// (0..s.len()).for_each(|i| {
/// 	let der = derivative.next(s[i]);
/// 	assert_eq!(der, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(1)
///
/// # See also
///
/// [`Integral`](crate::methods::Integral), [`Rate of Change`](crate::methods::RateOfChange), [`Momentum`](crate::methods::Momentum)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Derivative {
	divider: ValueType,
	window: Window<ValueType>,
	initialized: bool,
}

/// Just an alias for Derivative
pub type Differential = Derivative;

impl Method for Derivative {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Self {
		Self {
			divider: (length as ValueType).recip(),
			window: Window::new(length, value),
			initialized: false,
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let prev_value = self.window.push(value);
		(value - prev_value) * self.divider
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused_imports)]
	use super::{Derivative as TestingMethod, Method};
	use crate::core::ValueType;
	use crate::helpers::RandomCandles;

	#[allow(dead_code)]
	const SIGMA: ValueType = 1e-8;

	#[test]
	fn test_derivative_const() {
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input);

			test_const(&mut method, input, 0.0);
		}
	}

	#[test]
	fn test_derivative1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);
		let mut prev = None;

		candles.take(100).map(|x| x.close).for_each(|x| {
			assert!(((x - prev.unwrap_or(x)) - ma.next(x)).abs() < SIGMA);
			prev = Some(x);
		});
	}

	#[test]
	fn test_derivative() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);

			let mut value2 = src[0];
			src.iter().enumerate().for_each(|(i, &x)| {
				let value = ma.next(x);

				value2 = (x - src[i.saturating_sub(length as usize)]) / (length as ValueType);

				assert!(
					(value2 - value).abs() < SIGMA,
					"{}, {} at index {} with length {}",
					value2,
					value,
					i,
					length
				);
			});
		});
	}
}
