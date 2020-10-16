#![allow(unsafe_code)]
use super::PeriodType;
use std::mem;
use std::vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Some kind of a stack or a buffer of fixed size for remembering timeseries values
///
/// When push new value into it, it remembers that value and returns the oldest value
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
/// assert_eq!(w[0], 0);
/// assert_eq!(w[1], 1);
/// assert_eq!(w[2], 2);
///
/// w.push(3);
/// assert_eq!(w[0], 1);
/// assert_eq!(w[1], 2);
/// assert_eq!(w[2], 3);
///
/// w.push(4);
/// assert_eq!(w[0], 2);
/// assert_eq!(w[1], 3);
/// assert_eq!(w[2], 4);
/// ```
///
/// # See also
///
/// [Past](crate::methods::Past)
///
/// [windows](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Window<T>
where
	T: Copy,
{
	buf: Vec<T>,
	index: PeriodType,
	size: PeriodType,
	s_1: PeriodType,
}

impl<T> Window<T>
where
	T: Copy,
{
	/// Creates new Window object of size `size` with filled values `value`
	pub fn new(size: PeriodType, value: T) -> Self {
		debug_assert!(size <= (PeriodType::MAX - 1), "PeriodType overflow");
		Self {
			buf: vec![value; size as usize],
			index: 0,
			size,
			s_1: size.saturating_sub(1),
		}
	}

	/// Creates an empty `Window` instance (no buffer allocated)
	pub fn empty() -> Self {
		Self {
			buf: Vec::new(),
			index: 0,
			size: 0,
			s_1: 0,
		}
	}

	/// Pushes the `value` into the `Window`.
	///
	/// Returns an oldest pushed value.
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

	/// Returns an iterator over the `Window`'s values (by copy) (from the oldest to the newest).
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
	/// let p: Vec<i32> = w.iter().collect();
	/// assert_eq!(p, [3, 4, 5]);
	/// ```
	#[inline]
	pub fn iter(&self) -> WindowIterator<T> {
		WindowIterator::new(&self)
	}

	/// Returns an oldest value
	#[inline]
	pub fn first(&self) -> T {
		if cfg!(feature = "unsafe_performance") {
			*unsafe { self.buf.get_unchecked(self.index as usize) }
		} else {
			self.buf[self.index as usize]
		}
	}

	/// Returns a last pushed value
	///
	/// # Examples
	///
	/// ```
	/// use yata::core::Window;
	/// let mut w = Window::new(3, 1);
	///
	/// assert_eq!(w.last(), 1);
	/// w.push(2);
	/// assert_eq!(w.last(), 2);
	/// w.push(3);
	/// assert_eq!(w.last(), 3);
	/// w.push(4);
	/// assert_eq!(w.last(), 4);
	/// w.push(5);
	/// assert_eq!(w.last(), 5);
	/// w.push(6);
	/// assert_eq!(w.last(), 6);
	/// ```
	#[inline]
	pub fn last(&self) -> T {
		let is_zero = self.index == 0;
		let index = !is_zero as PeriodType * self.index.saturating_sub(1)
			+ is_zero as PeriodType * self.s_1;
		// let index = if self.index > 0 {
		// 	self.index - 1
		// } else {
		// 	self.s_1
		// };
		// self.buf[index as usize]
		*unsafe { self.buf.get_unchecked(index as usize) }
	}

	/// Checks if `Window` is empty (`length` == 0). Returns `true` if `Window` is empty or false otherwise.
	pub fn is_empty(&self) -> bool {
		self.buf.is_empty()
	}

	/// Casts `Window` to a regular vector of `T`
	pub fn as_vec(&self) -> &Vec<T> {
		&self.buf
	}

	/// Casts `Window` as a slice of `T`
	pub fn as_slice(&self) -> &[T] {
		self.buf.as_slice()
	}

	/// Returns the length (elements count) of the `Window`
	pub fn len(&self) -> PeriodType {
		self.size
	}
}

impl<T> Default for Window<T>
where
	T: Copy,
{
	fn default() -> Self {
		Self::empty()
	}
}

impl<T> std::ops::Index<PeriodType> for Window<T>
where
	T: Copy,
{
	type Output = T;

	fn index(&self, index: PeriodType) -> &Self::Output {
		debug_assert!(index < self.size, "Window index {:} is out of range", index);

		let saturated = self.index.saturating_add(index);
		let overflow = (saturated >= self.size) as PeriodType;
		let s = self.size - self.index;
		let buf_index = (overflow * index.saturating_sub(s) + (1 - overflow) * saturated) as usize;

		if cfg!(feature = "unsafe_performance") {
			unsafe { self.buf.get_unchecked(buf_index) }
		} else {
			&self.buf[buf_index]
		}
	}
}

impl<'a, T> IntoIterator for &'a Window<T>
where
	T: Copy,
{
	type Item = T;
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
pub struct WindowIterator<'a, T>
where
	T: Copy,
{
	window: &'a Window<T>,
	index: PeriodType,
	size: PeriodType,
}

impl<'a, T> WindowIterator<'a, T>
where
	T: Copy,
{
	pub fn new(window: &'a Window<T>) -> Self {
		Self {
			window,
			index: window.index,
			size: window.size,
		}
	}
}

impl<'a, T> Iterator for WindowIterator<'a, T>
where
	T: Copy,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.size == 0 {
			return None;
		}

		// let value = self.window.buf[self.index as usize];
		let value = if cfg!(feature = "unsafe_performance") {
			*unsafe { self.window.buf.get_unchecked(self.index as usize) }
		} else {
			self.window.buf[self.index as usize]
		};

		self.size -= 1;
		let not_at_end = self.index != self.window.s_1;
		self.index = not_at_end as PeriodType * (self.index + 1);

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
		Some(self.window.last())
	}
}

impl<'a, T> ExactSizeIterator for WindowIterator<'a, T> where T: Copy {}
impl<'a, T> std::iter::FusedIterator for WindowIterator<'a, T> where T: Copy {}
