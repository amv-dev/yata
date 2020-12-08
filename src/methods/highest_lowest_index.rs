use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};

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
/// `length` should be > `0`
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
/// let mut highest_index = HighestIndex::new(3, &values[0]).unwrap();
///
/// (0..values.len()).for_each(|i| {
///     let v = highest_index.next(&values[i]);
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
/// [`LowestIndex`], [`Highest`], [`Lowest`], [`HighestLowestDelta`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Highest`]: crate::methods::Highest
/// [`Lowest`]: crate::methods::Lowest
/// [`HighestLowestDelta`]: crate::methods::HighestLowestDelta
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HighestIndex {
	index: PeriodType,
	value: ValueType,
	window: Window<ValueType>,
}

impl Method<'_> for HighestIndex {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = PeriodType;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		if !value.is_finite() {
			return Err(Error::InvalidCandles);
		}

		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				window: Window::new(length, value),
				index: 0,
				value,
			}),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		assert!(
			value.is_finite(),
			"HighestIndex method cannot operate with NAN values"
		);

		self.window.push(value);
		self.index += 1;

		#[allow(clippy::cast_possible_truncation)]
		if value >= self.value {
			self.value = value;
			self.index = 0;
		} else if self.index == self.window.len() {
			let (index, value) =
				self.window.iter().enumerate().fold(
					(0, value),
					|a, b| {
						if b.1 > a.1 {
							b
						} else {
							a
						}
					},
				);

			self.index = index as PeriodType; // self.window.len() - index as PeriodType - 1;
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
/// let mut lowest_index = LowestIndex::new(3, &values[0]).unwrap();
///
/// (0..values.len()).for_each(|i| {
///     let v = lowest_index.next(&values[i]);
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
/// [`HighestIndex`], [`Highest`], [`Lowest`], [`HighestLowestDelta`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Highest`]: crate::methods::Highest
/// [`Lowest`]: crate::methods::Lowest
/// [`HighestLowestDelta`]: crate::methods::HighestLowestDelta
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LowestIndex {
	index: PeriodType,
	value: ValueType,
	window: Window<ValueType>,
}

impl Method<'_> for LowestIndex {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = PeriodType;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		if !value.is_finite() {
			return Err(Error::InvalidCandles);
		}

		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				window: Window::new(length, value),
				index: 0,
				value,
			}),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		assert!(
			value.is_finite(),
			"LowestIndex method cannot operate with NAN values"
		);

		self.window.push(value);
		self.index += 1;

		#[allow(clippy::cast_possible_truncation)]
		if value <= self.value {
			self.value = value;
			self.index = 0;
		} else if self.index == self.window.len() {
			let (index, value) =
				self.window.iter().enumerate().fold(
					(0, value),
					|a, b| {
						if b.1 < a.1 {
							b
						} else {
							a
						}
					},
				);

			self.index = index as PeriodType; // self.window.len() - index as PeriodType - 1;
			self.value = value;
		}

		self.index
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::core::Method;
	use crate::core::ValueType;
	use crate::helpers::RandomCandles;
	use crate::methods::tests::test_const;

	#[test]
	fn test_highest_index_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = HighestIndex::new(i, input).unwrap();

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_highest_index1() {
		use super::HighestIndex as TestingMethod;

		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq!(0, ma.next(x.close));
		});
	}

	#[test]
	fn test_highest_index() {
		use super::HighestIndex as TestingMethod;

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]).unwrap();
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
					max_index,
					ma.next(x) as usize,
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
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = LowestIndex::new(i, input).unwrap();

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_lowest_index1() {
		use super::LowestIndex as TestingMethod;

		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq!(0, ma.next(x.close));
		});
	}

	#[test]
	fn test_lowest_index() {
		use super::LowestIndex as TestingMethod;

		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]).unwrap();
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
					max_index,
					ma.next(x) as usize,
					"{}, {:?}, {:?}",
					length,
					&src[i.saturating_sub(length)..=i],
					ma
				);
			});
		});
	}
}
