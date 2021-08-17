use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Calculates absolute difference between highest and lowest values over the last `length` values for timeseries of type [`ValueType`]
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
/// Output value is always >= `0.0`
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::HighestLowestDelta;
///
///
/// let values = [1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 2.0, 3.0];
/// let r      = [0.0, 1.0, 2.0, 1.0, 2.0, 1.5, 1.5, 2.5];
/// let mut hld = HighestLowestDelta::new(3, &values[0]).unwrap();
///
/// (0..values.len()).for_each(|i| {
///     let v = hld.next(&values[i]);
///     assert_eq!(v, r[i]);
/// });
/// ```
///
/// # Performance
///
/// O(`length`)
///
/// This method is relatively very slow compare to the other methods.
///
/// # See also
///
/// [`Highest`], [`Lowest`], [`HighestIndex`], [`LowestIndex`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`HighestIndex`]: crate::methods::HighestIndex
/// [`LowestIndex`]: crate::methods::LowestIndex
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HighestLowestDelta {
	// highest: Highest,
	// lowest: Lowest,
	highest: ValueType,
	lowest: ValueType,
	window: Window<ValueType>,
}

impl Method for HighestLowestDelta {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error>
	where
		Self: Sized,
	{
		if !value.is_finite() {
			return Err(Error::InvalidCandles);
		}

		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				window: Window::new(length, value),
				highest: value,
				lowest: value,
			}),
		}
	}

	#[inline]
	fn next(&mut self, &value: &Self::Input) -> ValueType {
		let left_value = self.window.push(value);

		let mut search = false;
		if value >= self.highest {
			self.highest = value;
		// It's not a mistake. We really need a bit-to-bit comparison of float values here
		} else if left_value.to_bits() == self.highest.to_bits() {
			search = true;
		}

		if value <= self.lowest {
			self.lowest = value;
		// It's not a mistake. We really need a bit-to-bit comparison of float values here
		} else if left_value.to_bits() == self.lowest.to_bits() {
			search = true;
		}

		if search {
			let (min, max) = self
				.window
				.iter()
				.fold((value, value), |(min, max), &v| (min.min(v), max.max(v)));
			self.highest = max;
			self.lowest = min;
		}

		self.highest - self.lowest
	}
}

/// Returns highest value over the last `length` values for timeseries of type [`ValueType`]
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
/// use yata::core::Method;
/// use yata::methods::Highest;
///
/// let values = [1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 2.0, 3.0];
/// let r      = [1.0, 2.0, 3.0, 3.0, 3.0, 2.0, 2.0, 3.0];
///
/// let mut highest = Highest::new(3, &values[0]).unwrap();
///
/// (0..values.len()).for_each(|i| {
///     let v = highest.next(&values[i]);
///     assert_eq!(v, r[i]);
/// });
/// ```
///
/// # Performance
///
/// O(`length`)
///
/// This method is relatively slow compare to the other methods.
///
/// # See also
///
/// [`HighestLowestDelta`], [`Lowest`], [`HighestIndex`], [`LowestIndex`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`HighestIndex`]: crate::methods::HighestIndex
/// [`LowestIndex`]: crate::methods::LowestIndex
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Highest {
	value: ValueType,
	window: Window<ValueType>,
}

impl Method for Highest {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error> {
		if !value.is_finite() {
			return Err(Error::InvalidCandles);
		}

		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				window: Window::new(length, value),
				value,
			}),
		}
	}

	#[inline]
	fn next(&mut self, &value: &Self::Input) -> ValueType {
		assert!(
			value.is_finite(),
			"Highest method cannot operate with NAN values"
		);

		let left_value = self.window.push(value);

		if value >= self.value {
			self.value = value;
		// It's not a mistake. We really need a bit-to-bit comparison of float values here
		} else if left_value.to_bits() == self.value.to_bits() {
			self.value = self.window.iter().fold(value, |a, &b| a.max(b));
		}

		self.value
	}
}

/// Returns lowest value over the last `length` values for timeseries of type [`ValueType`]
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
/// use yata::core::Method;
/// use yata::methods::Lowest;
///
/// let values = [1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 2.0, 3.0];
/// let r      = [1.0, 1.0, 1.0, 2.0, 1.0, 0.5, 0.5, 0.5];
///
/// let mut lowest = Lowest::new(3, &values[0]).unwrap();
///
/// (0..values.len()).for_each(|i| {
///     let v = lowest.next(&values[i]);
///     assert_eq!(v, r[i]);
/// });
/// ```
///
/// # Performance
///
/// O(`length`)
///
/// This method is relatively slow compare to the other methods.
///
/// # See also
///
/// [`HighestLowestDelta`], [`Highest`], [`HighestIndex`], [`LowestIndex`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`HighestIndex`]: crate::methods::HighestIndex
/// [`LowestIndex`]: crate::methods::LowestIndex
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lowest {
	value: ValueType,
	window: Window<ValueType>,
}

impl Method for Lowest {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error> {
		if !value.is_finite() {
			return Err(Error::InvalidCandles);
		}

		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				window: Window::new(length, value),
				value,
			}),
		}
	}

	#[inline]
	fn next(&mut self, &value: &Self::Input) -> ValueType {
		assert!(
			value.is_finite(),
			"Lowest method cannot operate with NAN values"
		);

		let left_value = self.window.push(value);

		if value <= self.value {
			self.value = value;
		// It's not a mistake. We really need a bit-to-bit comparison of float values here
		} else if left_value.to_bits() == self.value.to_bits() {
			self.value = self.window.iter().fold(value, |a, &b| a.min(b));
		}

		self.value
	}
}

#[cfg(test)]
mod tests {
	use super::{Highest, HighestLowestDelta, Lowest};
	use crate::core::{Method, ValueType};
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[test]
	fn test_highest_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = Highest::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, &output);
		}
	}

	#[test]
	fn test_highest1() {
		use super::Highest as TestingMethod;

		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&x.close));
		});
	}

	#[test]
	fn test_highest() {
		use super::Highest as TestingMethod;

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(2..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, x)| {
				let value1 = ma.next(x);
				let value2 = (0..length).fold(src[i], |m, j| m.max(src[i.saturating_sub(j)]));
				assert_eq_float(value2, value1);
			});
		});
	}

	#[test]
	fn test_lowest_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = Lowest::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, &output);
		}
	}

	#[test]
	fn test_lowest1() {
		use super::Lowest as TestingMethod;
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&x.close));
		});
	}

	#[test]
	fn test_lowest() {
		use super::Lowest as TestingMethod;
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(2..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, x)| {
				let value1 = ma.next(x);
				let value2 = (0..length).fold(src[i], |m, j| m.min(src[i.saturating_sub(j)]));
				assert_eq_float(value2, value1);
			});
		});
	}

	#[test]
	fn test_highest_lowest_delta_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = HighestLowestDelta::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, &output);
		}
	}

	#[test]
	fn test_highest_lowest_delta1() {
		use super::HighestLowestDelta as TestingMethod;
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(0.0, ma.next(&x.close));
		});
	}

	#[test]
	fn test_highest_lowest_delta() {
		use super::HighestLowestDelta as TestingMethod;
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(2..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, x)| {
				let value1 = ma.next(x);
				let min = (0..length).fold(src[i], |m, j| m.min(src[i.saturating_sub(j)]));
				let max = (0..length).fold(src[i], |m, j| m.max(src[i.saturating_sub(j)]));
				assert_eq_float(max - min, value1);
			});
		});
	}
}
