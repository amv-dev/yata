use crate::core::Method;
use crate::core::{Action, Error, ValueType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Searches for two timeseries lines of type [`ValueType`] cross each other.
///
/// If `value` crossed `base` upwards, then returns [Action::BUY_ALL](crate::core::Action::BUY_ALL)
///
/// If `value` crossed `base` downwards, then returns [Action::SELL_ALL](crate::core::Action::SELL_ALL)
///
/// Else (if series did not cross each other) returns [Action::None](crate::core::Action::None)
///
/// # Parameters
///
/// Has no parameters
///
/// # Input type
///
/// Input type is (`value`: [`ValueType`], `base`: [`ValueType`])
///
/// # Output type
///
/// Output type is [`Action`]
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::Cross;
///
/// let mut cross = Cross::default();
///
/// let t1 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
/// let t2 = vec![5.0, 3.0, 1.8, 2.9, 4.1, 5.6];
/// let r  = vec![ 0,   0,   1,   0,   -1,  0 ];
///
/// (0..t1.len()).for_each(|i| {
/// 	let value = t1[i];
/// 	let base = t2[i];
/// 	let cross_value = cross.next((value, base)).analog();
/// 	assert_eq!(cross_value, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(1)
///
/// # See also
///
/// [CrossAbove], [CrossUnder]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cross {
	up: CrossAbove,
	down: CrossUnder,
}

impl Method for Cross {
	type Params = ();
	type Input = (ValueType, ValueType);
	type Output = Action;

	fn new(_: Self::Params, value: Self::Input) -> Result<Self, Error>
	where
		Self: Sized,
	{
		Ok(Self {
			up: CrossAbove::new((), value).unwrap(),
			down: CrossUnder::new((), value).unwrap(),
		})
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let up = self.up.binary(value.0, value.1);
		let down = self.down.binary(value.0, value.1);

		((up as i8) - (down as i8)).into()
	}
}

/// Searches for `value` timeseries line crosses `base` line upwards
///
/// If `value` crossed `base` upwards, then returns [Action::BUY_ALL](crate::core::Action::BUY_ALL)
///
/// Else returns [Action::None](crate::core::Action::None)
///
/// # Parameters
///
/// Has no parameters
///
/// # Input type
///
/// Input type is (`value`: [`ValueType`], `base`: [`ValueType`])
///
/// # Output type
///
/// Output type is [`Action`]
///
/// # Examples
///
/// ```
/// use yata::core::Method;
/// use yata::methods::CrossAbove;
///
/// let mut cross_above = CrossAbove::new((), (0.0, 5.0)).unwrap();
///
/// let t1 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
/// let t2 = vec![5.0, 3.0, 1.8, 2.9, 4.1, 5.6];
/// let r  = vec![ 0,   0,   1,   0,   0,   0 ];
///
/// (0..t1.len()).for_each(|i| {
/// 	let value = t1[i];
/// 	let base = t2[i];
/// 	let cross_value = cross_above.next((value, base)).analog();
/// 	assert_eq!(cross_value, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(1)
///
/// # See also
///
/// [Cross], [CrossUnder]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`DigitalSignal`]: crate::core::DigitalSignal
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrossAbove {
	last_delta: ValueType,
}

impl CrossAbove {
	/// Returns `true` when value1 crosses `value2` timeseries upwards
	/// Otherwise returns `false`
	#[inline]
	pub fn binary(&mut self, value1: ValueType, value2: ValueType) -> bool {
		let last_delta = self.last_delta;
		let current_delta = value1 - value2;

		self.last_delta = current_delta;

		last_delta < 0. && current_delta >= 0.
	}
}

impl Method for CrossAbove {
	type Params = ();
	type Input = (ValueType, ValueType);
	type Output = Action;

	fn new(_: Self::Params, value: Self::Input) -> Result<Self, Error>
	where
		Self: Sized,
	{
		Ok(Self {
			last_delta: value.0 - value.1,
			..Self::default()
		})
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		Action::from(self.binary(value.0, value.1) as i8)
	}
}

/// Searches for `value` timeseries line crosses `base` line downwards
///
/// If `value` crossed `base` downwards, then returns [Action::BUY_ALL](crate::core::Action::BUY_ALL)
///
/// Else returns [Action::None](crate::core::Action::None)
///
/// # Parameters
///
/// Has no parameters
///
/// # Input type
///
/// Input type is (`value`: [`ValueType`], `base`: [`ValueType`])
///
/// # Output type
///
/// Output type is [`Action`]
///
/// # Examples
///
/// ```
/// use yata::core::Method;
/// use yata::methods::CrossUnder;
///
/// let mut cross_under = CrossUnder::new((), (0.0, 5.0)).unwrap();
///
/// let t1 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
/// let t2 = vec![5.0, 3.0, 1.8, 2.9, 4.1, 5.6];
/// let r  = vec![ 0,   0,   0,   0,   1,   0 ];
///
/// (0..t1.len()).for_each(|i| {
/// 	let value = t1[i];
/// 	let base = t2[i];
/// 	let cross_value = cross_under.next((value, base)).analog();
/// 	assert_eq!(cross_value, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(1)
///
/// # See also
///
/// [Cross], [CrossAbove]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`DigitalSignal`]: crate::core::DigitalSignal
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrossUnder {
	last_delta: ValueType,
}

impl CrossUnder {
	/// Returns `true` when value1 crosses `value2` timeseries downwards
	/// Otherwise returns `false`
	#[inline]
	pub fn binary(&mut self, value1: ValueType, value2: ValueType) -> bool {
		let last_delta = self.last_delta;
		let current_delta = value1 - value2;

		self.last_delta = current_delta;

		last_delta > 0. && current_delta <= 0.
	}
}

impl Method for CrossUnder {
	type Params = ();
	type Input = (ValueType, ValueType);
	type Output = Action;

	fn new(_: Self::Params, value: Self::Input) -> Result<Self, Error>
	where
		Self: Sized,
	{
		Ok(Self {
			last_delta: value.0 - value.1,
			..Self::default()
		})
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		Action::from(self.binary(value.0, value.1) as i8)
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused_imports)]
	use crate::core::{Candle, Method, ValueType};
	use crate::helpers::RandomCandles;
	use crate::methods::tests::test_const;

	#[test]
	fn test_cross_const() {
		use super::Cross as TestingMethod;

		let input = (7.0, 1.0);
		let mut cross = TestingMethod::new((), input).unwrap();
		let output = cross.next(input);

		test_const(&mut cross, input, output);
	}

	#[test]
	fn test_cross() {
		use super::Cross as TestingMethod;

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();
		let avg = src.iter().sum::<ValueType>() / src.len() as ValueType;

		let mut ma = TestingMethod::new((), (src[0], avg)).unwrap();

		src.iter().enumerate().for_each(|(i, &x)| {
			let value1 = ma.next((x, avg)).analog();

			let value2;
			if x > avg && src[i.saturating_sub(1)] < avg {
				value2 = 1;
			} else if x < avg && src[i.saturating_sub(1)] > avg {
				value2 = -1;
			} else {
				value2 = 0;
			}

			assert_eq!(value1, value2, "{}, {} at index {}", value2, value1, i);
		});
	}
	#[test]
	fn test_cross_above_const() {
		use super::CrossAbove as TestingMethod;

		let input = (7.0, 1.0);
		let mut cross = TestingMethod::new((), input).unwrap();
		let output = cross.next(input);

		test_const(&mut cross, input, output);
	}

	#[test]
	fn test_cross_above() {
		use super::CrossAbove as TestingMethod;

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();
		let avg = src.iter().sum::<ValueType>() / src.len() as ValueType;

		let mut ma = TestingMethod::new((), (src[0], avg)).unwrap();

		src.iter().enumerate().for_each(|(i, &x)| {
			let value1 = ma.next((x, avg)).analog();

			let value2;
			if x > avg && src[i.saturating_sub(1)] < avg {
				value2 = 1;
			} else {
				value2 = 0;
			}

			assert_eq!(value1, value2, "{}, {} at index {}", value2, value1, i);
		});
	}

	#[test]
	fn test_cross_under_const() {
		use super::CrossUnder as TestingMethod;

		let input = (7.0, 1.0);
		let mut cross = TestingMethod::new((), input).unwrap();
		let output = cross.next(input);

		test_const(&mut cross, input, output);
	}

	#[test]
	fn test_cross_under() {
		use super::CrossUnder as TestingMethod;

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();
		let avg = src.iter().sum::<ValueType>() / src.len() as ValueType;

		let mut ma = TestingMethod::new((), (src[0], avg)).unwrap();

		src.iter().enumerate().for_each(|(i, &x)| {
			let value1 = ma.next((x, avg)).analog();

			let value2;
			if x < avg && src[i.saturating_sub(1)] > avg {
				value2 = 1;
			} else {
				value2 = 0;
			}

			assert_eq!(value1, value2, "{}, {} at index {}", value2, value1, i);
		});
	}
}
