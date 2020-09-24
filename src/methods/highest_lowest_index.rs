use crate::core::Method;
use crate::core::{PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Returns highest value index over the last `length` values for timeseries of type [`ValueType`]
///
/// If period has more than one maximum values, then returns the index of the newest value (e.g. the smallest index)
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
/// Output type is [`PeriodType`]
///
/// # Examples
///
/// ```
/// use yata::core::Method;
/// use yata::methods::HighestIndex;
///
/// let values = [1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 2.0, 3.0];
/// let r      = [ 0,   0,   0,   1,   2,   2,   0,   0 ];
///
/// let mut highest_index = HighestIndex::new(3, values[0]);
///
/// (0..values.len()).for_each(|i| {
/// 	let v = highest_index.next(values[i]);
/// 	assert_eq!(v, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(`length`)
///
/// This method is relatively slow compare to the other methods.
///
/// # See also
///
/// [HighestLowestDelta], [Highest], [Lowest]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HighestIndex {
	index: PeriodType,
	value: ValueType,
	window: Window<ValueType>,
}

impl Method for HighestIndex {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = PeriodType;

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(
			length > 0 && length < Self::Params::MAX,
			"Highest: length should be > 0 and < {}",
			Self::Params::MAX,
		);

		Self {
			window: Window::new(length, value),
			index: 0,
			value,
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let left_value = self.window.push(value);
		self.index += 1;

		if value >= self.value {
			self.value = value;
			self.index = 0;
		} else if left_value == self.value || self.index == self.window.len() {
			let (index, value) = self.window.iter().enumerate().fold(
				(self.window.len() as usize - 1, value),
				|a, b| {
					if b.1 >= a.1 {
						b
					} else {
						a
					}
				},
			);

			self.index = self.window.len() - index as PeriodType - 1;
			self.value = value;
		}

		self.index
	}
}

/// Returns lowest value index over the last `length` values for timeseries of type [`ValueType`]
///
/// If period has more than one minimum values, then returns the index of the newest value (e.g. the smallest index)
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
/// Output type is [`PeriodType`]
///
/// # Examples
///
/// ```
/// use yata::core::Method;
/// use yata::methods::LowestIndex;
///
/// let values = [1.0, 2.0, 3.0, 2.0, 1.0, 0.5, 2.0, 3.0];
/// let r      = [ 0,   1,   2,   0,   0,   0,   1,   2 ];
///
/// let mut lowest_index = LowestIndex::new(3, values[0]);
///
/// (0..values.len()).for_each(|i| {
/// 	let v = lowest_index.next(values[i]);
/// 	assert_eq!(v, r[i]);
/// });
/// ```
///
/// # Perfomance
///
/// O(`length`)
///
/// This method is relatively slow compare to the other methods.
///
/// # See also
///
/// [HighestLowestDelta], [Highest], [Lowest]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LowestIndex {
	index: PeriodType,
	value: ValueType,
	window: Window<ValueType>,
}

impl Method for LowestIndex {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = PeriodType;

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(
			length > 0 && length < Self::Params::MAX,
			"Highest: length should be > 0 and < {}",
			Self::Params::MAX,
		);

		Self {
			window: Window::new(length, value),
			index: 0,
			value,
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let left_value = self.window.push(value);
		self.index += 1;

		if value <= self.value {
			self.value = value;
			self.index = 0;
		} else if left_value == self.value || self.index == self.window.len() {
			let (index, value) = self.window.iter().enumerate().fold(
				(self.window.len() as usize - 1, value),
				|a, b| {
					if b.1 <= a.1 {
						b
					} else {
						a
					}
				},
			);

			self.index = self.window.len() - index as PeriodType - 1;
			self.value = value;
		}

		self.index
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::core::Method;
	use crate::core::{PeriodType, ValueType};
	use crate::helpers::RandomCandles;
	use crate::methods::tests::test_const;

	#[test]
	fn test_highest_index_const() {
		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = HighestIndex::new(i, input);

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_highest_index1() {
		use super::{HighestIndex as TestingMethod, Method};

		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert_eq!(0, ma.next(x.close));
		});
	}

	#[test]
	fn test_highest_index() {
		use super::{HighestIndex as TestingMethod, Method};

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, &x)| {
				let mut max_value = x;
				let mut max_index = 0;

				for j in 0..length {
					let v = src[i.saturating_sub(j)];

					if v > max_value {
						max_value = v;
						max_index = j;
					}
				}

				assert_eq!(
					max_index as PeriodType,
					ma.next(x),
					"{}, {:?}, {:?}",
					length,
					&src[i.saturating_sub(length)..=i],
					ma
				);
			});
		});
	}

	#[test]
	fn test_lowest_index_const() {
		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = LowestIndex::new(i, input);

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_lowest_index1() {
		use super::{LowestIndex as TestingMethod, Method};

		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert_eq!(0, ma.next(x.close));
		});
	}

	#[test]
	fn test_lowest_index() {
		use super::{LowestIndex as TestingMethod, Method};

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);
			let length = length as usize;

			src.iter().enumerate().for_each(|(i, &x)| {
				let mut max_value = x;
				let mut max_index = 0;

				for j in 0..length {
					let v = src[i.saturating_sub(j)];

					if v < max_value {
						max_value = v;
						max_index = j;
					}
				}

				assert_eq!(
					max_index as PeriodType,
					ma.next(x),
					"{}, {:?}, {:?}",
					length,
					&src[i.saturating_sub(length)..=i],
					ma
				);
			});
		});
	}
}
