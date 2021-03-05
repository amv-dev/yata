use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window};
use std::{cmp::Ordering, slice::SliceIndex};

#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!!!!! USE WITH CAUTION !!!!!!
//
// When `unsafe_performance` feature is enabled, this function may produce UB,
// when tying to get slice item outside it's bounds.
//
// !!!!!! USE WITH CAUTION !!!!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
#[inline]
#[cfg(feature = "unsafe_performance")]
#[allow(unsafe_code)]
fn get<T>(slice: &[ValueType], index: T) -> &T::Output
where
	T: SliceIndex<[ValueType]>,
{
	unsafe { slice.get_unchecked(index) }
}

#[inline]
#[cfg(not(feature = "unsafe_performance"))]
fn get<T>(slice: &[ValueType], index: T) -> &T::Output
where
	T: SliceIndex<[ValueType]>,
{
	&slice[index]
}

#[inline]
fn next_half(
	value: ValueType,
	slice: &[ValueType],
	padding: usize,
	f: fn(value: ValueType, slice: &[ValueType], padding: usize) -> usize,
) -> usize {
	let half = slice.len() / 2;

	// It's not a mistake. We really need a bit-to-bit comparison of float values here
	// Also it is not a good idea to use `match value.partial_cmp(slice[half]): it is slower.
	if value.to_bits() == get(slice, half).to_bits() {
		padding + half
	} else if &value > get(slice, half) {
		f(value, get(slice, (half + 1)..), padding + half + 1)
	} else {
		f(value, get(slice, ..half), padding)
	}
}

// find current value index
#[inline]
fn find_index(value: ValueType, slice: &[ValueType], padding: usize) -> usize {
	if slice.len() < 2 {
		return padding + 1 - slice.len();
	}

	next_half(value, slice, padding, find_index)
}

// find new value insert index at
#[inline]
fn find_insert_index(value: ValueType, slice: &[ValueType], padding: usize) -> usize {
	if slice.is_empty() {
		return padding;
	}

	next_half(value, slice, padding, find_insert_index)
}

///
/// [Simple Moving Median](https://en.wikipedia.org/wiki/Moving_average#Moving_median) of specified `length` for timeseries of type [`ValueType`]
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
/// use yata::methods::SMM;
///
/// // SMM of length=3
/// let mut smm = SMM::new(3, 1.0).unwrap();
///
/// smm.next(1.0);
/// smm.next(2.0);
///
/// assert_eq!(smm.next(3.0), 2.0);
/// assert_eq!(smm.next(100.0), 3.0);
/// ```
///
/// # Performance
///
/// O(log(`length`))
///
/// This method is relatively slower compare to the most of the other methods.
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
pub struct SMM {
	half: PeriodType,
	half_m1: PeriodType,
	window: Window<ValueType>,
	slice: Box<[ValueType]>,
}

impl SMM {
	/// Returns inner [`Window`](crate::core::Window). Useful for implementing in other methods and indicators.
	#[inline]
	#[must_use]
	pub const fn get_window(&self) -> &Window<ValueType> {
		&self.window
	}

	/// Returns last result value. Useful for implementing in other methods and indicators.
	#[inline]
	#[must_use]
	pub fn get_last_value(&self) -> ValueType {
		(get(&self.slice, self.half as usize) + get(&self.slice, self.half_m1 as usize)) * 0.5
	}
}

impl Method<'_> for SMM {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		if !value.is_finite() {
			return Err(Error::InvalidCandles);
		}

		match length {
			0 => Err(Error::WrongMethodParameters),
			length => {
				let half = length / 2;

				let is_even = length % 2 == 0;
				Ok(Self {
					half,
					half_m1: half.saturating_sub(is_even as PeriodType),
					window: Window::new(length, value),
					slice: vec![value; length as usize].into(),
				})
			}
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		assert!(
			value.is_finite(),
			"SMM method cannot operate with NAN values"
		);

		let old_value = self.window.push(value);

		let old_index = find_index(old_value, &self.slice, 0);
		let index = find_insert_index(value, &self.slice, 0);

		// if the old index is before current, then we should offset current value by 1 back
		let index = index - (old_index < index) as usize;

		if cfg!(feature = "unsafe_performance") {
			if index != old_index {
				let is_after = (index > old_index) as usize;
				let start = (old_index + 1) * is_after + index * (1 - is_after);
				let dest = old_index * is_after + (index + 1) * (1 - is_after);

				let count = index.saturating_sub(old_index) * is_after
					+ old_index.saturating_sub(index) * (1 - is_after);

				#[allow(unsafe_code)]
				unsafe {
					std::ptr::copy(
						self.slice.as_ptr().add(start),
						self.slice.as_mut_ptr().add(dest),
						count,
					);
				}
			}

			#[allow(unsafe_code)]
			unsafe {
				let q = self.slice.get_unchecked_mut(index);
				*q = value;
			}
		} else {
			// moving values inside the sorted slice
			match index.cmp(&old_index) {
				Ordering::Greater => self.slice.copy_within((old_index + 1)..=index, old_index),
				Ordering::Less => self.slice.copy_within(index..old_index, index + 1),
				Ordering::Equal => {}
			};

			// inserting new value
			self.slice[index] = value;
		}

		self.get_last_value()
	}
}

#[cfg(feature = "serde")]
impl Serialize for SMM {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut s = serializer.serialize_struct("SMM", 2)?;
		s.serialize_field("window", &self.window)?;
		s.serialize_field("slice", &self.slice)?;
		s.end()
	}
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SMM {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		struct DeserializedSMM {
			window: Window<ValueType>,
			slice: Box<[ValueType]>,
		}

		let de = DeserializedSMM::deserialize(deserializer)?;

		let window = de.window;
		let slice = de.slice;

		if window.len() as usize != slice.len() {
			return Err(serde::de::Error::custom(
				"Window's and slice's lengths must be equal.",
			));
		}

		if window.is_empty() {
			return Err(serde::de::Error::custom("SMM must have non-zero length."));
		}

		let half = window.len() / 2;
		let is_even = window.len() % 2 == 0;

		let smm = Self {
			half,
			half_m1: half.saturating_sub(is_even as PeriodType),
			window,
			slice,
		};

		Ok(smm)
	}
}

#[cfg(test)]
mod tests {
	use super::{Method, SMM as TestingMethod};
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[test]
	fn test_smm_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_smm1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(x.close));
		});
	}

	#[test]
	fn test_smm() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(3000).map(|x| x.close).collect();

		[1, 2, 3, 5, 11, 23, 51, 100, 150, 203, 254]
			.iter()
			.for_each(|&ma_length| {
				let mut ma = TestingMethod::new(ma_length, src[0]).unwrap();
				let ma_length = ma_length as usize;

				src.iter().enumerate().for_each(|(i, &x)| {
					let value = ma.next(x);
					let slice_from = i.saturating_sub(ma_length - 1);
					let slice_to = i;
					let mut slice = Vec::with_capacity(ma_length);

					src.iter()
						.skip(slice_from)
						.take(slice_to - slice_from + 1)
						.for_each(|&x| slice.push(x));

					while slice.len() < ma_length {
						slice.push(src[0]);
					}

					slice.sort_by(|a, b| a.partial_cmp(b).unwrap());

					assert_eq!(slice.len(), ma.slice.len());

					slice
						.iter()
						.zip(ma.slice.iter())
						.for_each(|(&a, &b)| assert_eq!(a.to_bits(), b.to_bits()));

					let value2 = if ma_length % 2 == 0 {
						(slice[ma_length / 2] + slice[ma_length / 2 - 1]) / 2.0
					} else {
						slice[ma_length / 2]
					};
					assert_eq_float(value2, value);
				});
			});
	}
}
