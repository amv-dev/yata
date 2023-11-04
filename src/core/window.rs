#![allow(clippy::cast_possible_truncation)]
#![allow(unsafe_code)]
use super::PeriodType;
use std::mem;
use std::vec;

#[cfg(feature = "serde")]
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

/// Window is a [circular buffer](https://en.wikipedia.org/wiki/Circular_buffer) where both
/// `start` and `end` pointers always point to a single element.
///
/// When push new value into it, it remembers that value and returns the oldest pushed value.
///
/// Also you can [iterate](Window::iter) over remembered values inside the `Window`
///
/// # Examples
/// ```
/// use yata::core::Window;
///
/// let mut w = Window::new(3, 1); // [1, 1, 1]
///
/// assert_eq!(w.push(2), 1); // [1, 1, 2]
/// assert_eq!(w.push(3), 1); // [1, 2, 3]
/// assert_eq!(w.push(4), 1); // [2, 3, 4]
/// assert_eq!(w.push(5), 2); // [3, 4, 5]
/// assert_eq!(w.push(6), 3); // [4, 5, 6]
/// ```
///
/// ```
/// use yata::core::Window;
///
/// let mut w = Window::new(3, 0);
///
/// w.push(1);
/// w.push(2);
/// assert_eq!(w[0], 2);
/// assert_eq!(w[1], 1);
/// assert_eq!(w[2], 0);
///
/// w.push(3);
/// assert_eq!(w[0], 3);
/// assert_eq!(w[1], 2);
/// assert_eq!(w[2], 1);
///
/// w.push(4);
/// assert_eq!(w[0], 4);
/// assert_eq!(w[1], 3);
/// assert_eq!(w[2], 2);
/// ```
///
/// # See also
///
/// [`Past`](crate::methods::Past)
///
/// [`Windows`](std::slice::Windows)
#[derive(Debug, Clone)]
pub struct Window<T> {
	buf: Box<[T]>,
	index: PeriodType,
	size: PeriodType,
	s_1: PeriodType,
}

impl<T> Window<T> {
	/// Creates new `Window` object of size `size` with filled values `value`
	///
	/// # Panics
	///
	/// When in development mode, this method may panic if `size` is equal to [`PeriodType::MAX`]
	///
	/// [`PeriodType::MAX`]: crate::core::PeriodType
	#[must_use]
	pub fn new(size: PeriodType, value: T) -> Self
	where
		T: Clone,
	{
		debug_assert!(size <= (PeriodType::MAX - 1), "PeriodType overflow");
		Self {
			buf: vec![value; size as usize].into(),
			index: 0,
			size,
			s_1: size.saturating_sub(1),
		}
	}

	/// Creates new `Window` object from raw slice and index of the oldest inserted element in that slice.
	///
	/// `index` must be an index of the most oldest value in the slice. In most cases it should be zero.
	///
	/// # Panics
	///
	/// This method will panic if length of the slice is greater or equal to [`PeriodType::MAX`].
	/// This method will also panic if provided `index` is greater or equal to slice's length.
	///
	/// [`PeriodType::MAX`]: crate::core::PeriodType
	#[must_use]
	pub fn from_parts(slice: Box<[T]>, index: PeriodType) -> Self {
		let size = slice.len() as PeriodType;

		assert!(
			slice.len() < PeriodType::MAX as usize,
			"The length of the slice is too large"
		);
		assert!(
			slice.len() > index as usize,
			"Index is out of slice's range"
		);

		Self {
			buf: slice,
			index,
			size,
			s_1: size.saturating_sub(1),
		}
	}

	/// Creates an empty `Window` instance (no buffer allocated)
	#[must_use]
	pub fn empty() -> Self {
		Self {
			buf: Vec::new().into(),
			index: 0,
			size: 0,
			s_1: 0,
		}
	}

	/// Pushes the `value` into the `Window`.
	///
	/// Returns an oldest pushed value.
	///
	/// # Panics
	///
	/// This method panics if try to push into empty `Window` (when `size` = `0`).
	#[inline]
	pub fn push(&mut self, value: T) -> T {
		debug_assert!(!self.is_empty(), "Trying to use an empty window");

		let refer = if cfg!(feature = "unsafe_performance") {
			unsafe { self.buf.get_unchecked_mut(self.index as usize) }
		} else {
			&mut self.buf[self.index as usize]
		};

		let old_value = mem::replace(refer, value);

		// Next string is branchless version of the code:
		// if self.index == self.size - 1 {
		//	self.index = 0;
		// } else {
		//	self.index += 1;
		// }
		self.index = (self.index != self.s_1) as PeriodType * (self.index + 1);

		old_value
	}

	/// Returns an iterator over the `Window`'s values (by copy) (from the newest to the oldest).
	///
	/// # Examples
	///
	/// ```
	/// use yata::core::Window;
	///
	/// let mut w = Window::new(3, 1);
	///
	/// w.push(2);
	/// w.push(3);
	/// w.push(4);
	/// w.push(5);
	///
	/// let p: Vec<_> = w.iter().copied().collect();
	/// assert_eq!(p, [5, 4, 3]);
	/// ```
	#[inline]
	#[must_use]
	pub const fn iter(&self) -> WindowIterator<T> {
		WindowIterator::new(self)
	}

	/// Returns a reversed iterator over the `Window`'s values (by copy) (from the oldest value to the newest).
	///
	/// # Examples
	///
	/// ```
	/// use yata::core::Window;
	///
	/// let mut w = Window::new(3, 1);
	///
	/// w.push(2);
	/// w.push(3);
	/// w.push(4);
	/// w.push(5);
	///
	/// let p: Vec<_> = w.iter_rev().copied().collect();
	/// assert_eq!(p, [3, 4, 5]);
	/// ```
	#[inline]
	#[must_use]
	pub const fn iter_rev(&self) -> ReversedWindowIterator<T> {
		ReversedWindowIterator::new(self)
	}

	/// Returns a last pushed value
	///
	/// # Examples
	///
	/// ```
	/// use yata::core::Window;
	/// let mut w = Window::new(3, 1);
	///
	/// assert_eq!(w.newest(), &1);
	/// w.push(2);
	/// assert_eq!(w.newest(), &2);
	/// w.push(3);
	/// assert_eq!(w.newest(), &3);
	/// w.push(4);
	/// assert_eq!(w.newest(), &4);
	/// w.push(5);
	/// assert_eq!(w.newest(), &5);
	/// w.push(6);
	/// assert_eq!(w.newest(), &6);
	/// ```
	#[inline]
	#[must_use]
	pub fn newest(&self) -> &T {
		let index = self.index.checked_sub(1).unwrap_or(self.s_1);

		if cfg!(feature = "unsafe_performance") {
			unsafe { self.buf.get_unchecked(index as usize) }
		} else {
			&self.buf[index as usize]
		}
	}

	/// Returns an oldest value
	#[inline]
	#[must_use]
	pub fn oldest(&self) -> &T {
		if cfg!(feature = "unsafe_performance") {
			unsafe { self.buf.get_unchecked(self.index as usize) }
		} else {
			&self.buf[self.index as usize]
		}
	}

	/// Checks if `Window` is empty (`length` == 0). Returns `true` if `Window` is empty or false otherwise.
	#[must_use]
	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.buf.is_empty()
	}

	/// Casts `Window` as a raw slice of `T`
	///
	/// ## Important!
	///
	/// The sequence of elements is not preserved.
	#[must_use]
	#[inline]
	pub const fn as_slice(&self) -> &[T] {
		&self.buf
	}

	/// Returns the length (elements count) of the `Window`
	#[must_use]
	#[inline]
	pub const fn len(&self) -> PeriodType {
		self.size
	}

	/// Returns an element at `index` starting from the newest
	#[must_use]
	#[inline]
	pub fn get(&self, index: PeriodType) -> Option<&T> {
		let buf_index = self.slice_index(index)?;
		self.buf.get(buf_index as usize)
	}

	#[must_use]
	#[inline]
	fn slice_index(&self, index: PeriodType) -> Option<PeriodType> {
		let index = self.s_1.checked_sub(index)?;
		let saturated = self.index.saturating_add(index);
		let overflow = (saturated >= self.size) as PeriodType;
		let s = self.size - self.index;
		Some(overflow * index.saturating_sub(s) + (1 - overflow) * saturated)
	}
}

impl<T> AsRef<[T]> for Window<T> {
	fn as_ref(&self) -> &[T] {
		&self.buf
	}
}

impl<T> Default for Window<T> {
	fn default() -> Self {
		Self::empty()
	}
}

impl<T> std::ops::Index<PeriodType> for Window<T> {
	type Output = T;

	fn index(&self, index: PeriodType) -> &Self::Output {
		let buf_index = self
			.slice_index(index)
			.unwrap_or_else(|| panic!("Window index {index} is out of range")) as usize;

		if cfg!(feature = "unsafe_performance") {
			unsafe { self.buf.get_unchecked(buf_index) }
		} else {
			&self.buf[buf_index]
		}
	}
}

impl<T> From<Box<[T]>> for Window<T> {
	#[inline]
	fn from(slice: Box<[T]>) -> Self {
		Self::from_parts(slice, 0)
	}
}

impl<T> From<Vec<T>> for Window<T> {
	#[inline]
	fn from(v: Vec<T>) -> Self {
		Self::from_parts(v.into_boxed_slice(), 0)
	}
}

impl<'a, T> IntoIterator for &'a Window<T> {
	type Item = &'a T;
	type IntoIter = WindowIterator<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

// impl<T> std::ops::Deref for Window<T>
// 	where T: Sized + Copy + Default
// {
// 	type Target = Vec<T>;

// 	fn deref(&self) -> &Self::Target {
// 		&self.buf
// 	}
// }

#[derive(Debug)]
pub struct WindowIterator<'a, T> {
	window: &'a Window<T>,
	index: PeriodType,
	size: PeriodType,
}

impl<'a, T> WindowIterator<'a, T> {
	pub const fn new(window: &'a Window<T>) -> Self {
		Self {
			window,
			index: window.index,
			size: window.size,
		}
	}
}

impl<'a, T> Iterator for WindowIterator<'a, T> {
	type Item = &'a T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.size == 0 {
			return None;
		}

		self.size -= 1;

		let at_start = (self.index == 0) as PeriodType;
		self.index = self.index.saturating_sub(1) + at_start * self.window.s_1;

		let value = if cfg!(feature = "unsafe_performance") {
			unsafe { self.window.buf.get_unchecked(self.index as usize) }
		} else {
			&self.window.buf[self.index as usize]
		};

		Some(value)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let size = self.size as usize;
		(size, Some(size))
	}

	fn count(self) -> usize {
		self.size as usize
	}

	fn last(self) -> Option<Self::Item> {
		Some(self.window.oldest())
	}
}

impl<'a, T> ExactSizeIterator for WindowIterator<'a, T> {}
impl<'a, T> std::iter::FusedIterator for WindowIterator<'a, T> {}

#[derive(Debug)]
pub struct ReversedWindowIterator<'a, T> {
	window: &'a Window<T>,
	index: PeriodType,
	size: PeriodType,
}

impl<'a, T> ReversedWindowIterator<'a, T> {
	pub const fn new(window: &'a Window<T>) -> Self {
		Self {
			window,
			index: window.index,
			size: window.size,
		}
	}
}

impl<'a, T> Iterator for ReversedWindowIterator<'a, T> {
	type Item = &'a T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if self.size == 0 {
			return None;
		}

		let value = if cfg!(feature = "unsafe_performance") {
			unsafe { self.window.buf.get_unchecked(self.index as usize) }
		} else {
			&self.window.buf[self.index as usize]
		};

		self.size -= 1;

		let not_at_the_end = (self.index != self.window.s_1) as PeriodType;
		self.index = (self.index + 1) * not_at_the_end;

		Some(value)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let size = self.size as usize;
		(size, Some(size))
	}

	fn count(self) -> usize {
		self.size as usize
	}

	fn last(self) -> Option<Self::Item> {
		Some(self.window.newest())
	}
}

impl<'a, T> ExactSizeIterator for ReversedWindowIterator<'a, T> {}
impl<'a, T> std::iter::FusedIterator for ReversedWindowIterator<'a, T> {}

#[derive(Deserialize)]
#[cfg(feature = "serde")]
struct SerializableWindow<T> {
	buf: Box<[T]>,
	index: PeriodType,
}

#[cfg(feature = "serde")]
impl<T> Serialize for Window<T>
where
	T: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut s = serializer.serialize_struct("Window", 2)?;
		s.serialize_field("buf", &self.buf)?;
		s.serialize_field("index", &self.index)?;
		s.end()
	}
}

#[cfg(feature = "serde")]
use serde::de::Error as SerdeError;

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for Window<T>
where
	T: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let w = SerializableWindow::deserialize(deserializer)?;

		let buf = w.buf;
		let index = w.index;

		if buf.len() > PeriodType::MAX as usize - 1 {
			let max_length = PeriodType::MAX as usize - 1;
			let error = SerdeError::custom(format!(
				"Length of window's buffer cannot be more than {max_length}.",
			));
			return Err(error);
		}

		if (buf.len() as PeriodType) <= index {
			let error =
				SerdeError::custom(format!("Index {index} is out of window's buffer bounds."));
			return Err(error);
		}

		Ok(Self::from_parts(buf, index))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::helpers::RandomCandles;

	#[test]
	fn test_push() {
		let data: Vec<_> = RandomCandles::new().take(300).collect();

		for length in 1..255 {
			let mut w = Window::new(length, data[0]);

			data.iter().enumerate().for_each(|(i, &c)| {
				let left = data[i.saturating_sub(length as usize)];
				assert_eq!(left, w.push(c));
			});
		}
	}

	#[test]
	fn test_oldest() {
		let data: Vec<_> = RandomCandles::new().take(300).collect();

		for length in 1..255 {
			let mut w = Window::new(length, data[0]);

			data.iter().enumerate().for_each(|(i, &c)| {
				let first = data[i.saturating_sub(length.saturating_sub(1) as usize)];
				w.push(c);
				assert_eq!(first, *w.oldest());
			});
		}
	}

	#[test]
	fn test_newest() {
		let data: Vec<_> = RandomCandles::new().take(300).collect();

		for length in 1..255 {
			let mut w = Window::new(length, data[0]);

			for &c in &data {
				w.push(c);
				assert_eq!(c, *w.newest());
			}
		}
	}

	#[test]
	fn test_iterator() {
		let data: Vec<_> = RandomCandles::new().take(600).collect();

		for length in 1..255 {
			let mut w = Window::new(length, data[0]);

			data.iter().enumerate().for_each(|(i, &c)| {
				w.push(c);

				if i >= length as usize {
					let iterated: Vec<_> = w.iter().copied().collect();

					let original_slice: Vec<_> = {
						let from = i.saturating_sub((length - 1) as usize);
						let to = i;
						data[from..=to].iter().rev().copied().collect()
					};

					assert_eq!(iterated, original_slice);
				}

				assert_eq!(
					data[i.saturating_sub((length - 1) as usize)],
					w.iter().last().copied().unwrap()
				);
			});

			assert_eq!(
				w.iter().size_hint(),
				(length as usize, Some(length as usize))
			);
			assert_eq!(w.iter().count(), length as usize);
		}
	}

	#[test]
	fn test_rev_iterator() {
		let data: Vec<_> = RandomCandles::new().take(600).collect();

		for length in 1..255 {
			let mut w = Window::new(length, data[0]);

			data.iter().enumerate().for_each(|(i, &c)| {
				w.push(c);

				if i >= length as usize {
					let iterated: Vec<_> = w.iter_rev().copied().collect();

					let original_slice = {
						let from = i.saturating_sub((length - 1) as usize);
						let to = i;
						&data[from..=to]
					};
					assert_eq!(iterated.as_slice(), original_slice);
				}

				// assert_eq!(data[i.saturating_sub((length - 1) as usize)], w.iter_rev().last().unwrap());
			});

			assert_eq!(
				w.iter().size_hint(),
				(length as usize, Some(length as usize))
			);
			assert_eq!(w.iter().count(), length as usize);
		}
	}

	#[test]
	fn test_index() {
		let data: Vec<_> = RandomCandles::new().take(300).collect();

		for length in 1..255 {
			let mut w = Window::new(length, data[0]);

			data.iter().enumerate().for_each(|(i, &c)| {
				w.push(c);
				assert_eq!(w[0], c);

				if i >= length as usize {
					let from = i.saturating_sub((length - 1) as usize);
					let to = i;
					let slice = &data[from..=to];
					for j in 0..length {
						assert_eq!(slice[(length - 1 - j) as usize], w[j]);
					}
				}
			});
		}
	}
}
