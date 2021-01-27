use crate::core::{Error, Method, ValueType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Converts timeseries to [Renko](https://en.wikipedia.org/wiki/Renko_chart) timeseries
///
/// Renko is very different from the simple timeseries. On each step it may generate any amount of blocks or not genereate it at all.
/// That's why it needs to be implement throw three different structures:
///
/// * [`Renko`] method itself.
/// 
/// When call [`Method::next`] on `Renko`, it always returns `RenkoOutput`.
/// * [`RenkoOutput`] which is `Renko`'s method output type.
/// 
/// It implements an [`Iterator`](std::iter::Iterator) trait for generating [`RenkoBlock`]s after each step of calling [`Method::next`] on [`Renko`].
/// `RenkoOutput` may produce any amount of `RenkoBlock`s or may not produce it at all.
/// 
/// * [`RenkoBlock`] is final entity of Renko chart.
/// 
/// It has `open` and `close` values which are similar to corresponding [`OHLCV`](crate::core::OHLCV)'s values.
///
/// So the final workflow is like that:
///
/// 1. Call [`Renko`]'s [`Method::next`] on some [`ValueType`] and get [`RenkoOutput`].
/// 2. Iterate over taken [`RenkoOutput`] to get some (or none) [`RenkoBlock`]s.
/// 3. Use produced [`RenkoBlock`]s on your own.
///
/// # Parameters
///
/// Has a single parameter `size`: [`ValueType`]. It represents relative block size.
///
/// `size` must be in range (`0.0`; `1.0`)
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::Renko;
/// let first_timeseries_value = 123.456;
/// let renko = Renko::new(0.01, first_timeseries_value); // creates a Renko method with relative block size of 1%.
/// ```
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Input type is [`RenkoOutput`]
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::Renko;
///
/// let inputs = [100.0, 100.5, 101.506, 105.0, 102.0, 101.4, 100.0];
/// let mut renko = Renko::new(0.01, inputs[0]).unwrap(); // renko with relative block size of 1%
///
/// assert!(renko.next(inputs[0]).is_empty());
/// assert!(renko.next(inputs[1]).is_empty());
/// assert_eq!(renko.next(inputs[2]).len(), 1);
/// let blocks = renko.next(inputs[3]);
/// assert_eq!(blocks.len(), 3);
/// blocks.for_each(|block| { println!("{:?}", &block); });
/// assert_eq!(renko.next(inputs[4]).len(), 1);
/// assert_eq!(renko.next(inputs[5]).len(), 1);
/// assert_eq!(renko.next(inputs[6]).len(), 1);
/// ```
///
/// # Performance
///
/// O(1)
///
/// # See also
///
/// * [`HeikinAshi`](crate::methods::HeikinAshi)
///
/// [`ValueType`]: crate::core::ValueType

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Renko {
	last_block_upper: ValueType,
	last_block_lower: ValueType,
	next_block_upper: ValueType,
	next_block_lower: ValueType,
	brick_size: ValueType,
}

/// Single unit for [`Renko`] charts
///
/// May be produced by [`RenkoOutput`] iterator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenkoBlock {
	/// Current block's open value
	pub open: ValueType,

	/// Current block's close value
	pub close: ValueType,
}

impl RenkoBlock {
	/// Returns upper bound of the Renko's block
	#[must_use]
	pub fn upper_bound(&self) -> ValueType {
		self.open.max(self.close)
	}

	/// Returns lower bound of the Renko's block
	#[must_use]
	pub fn lower_bound(&self) -> ValueType {
		self.open.min(self.close)
	}

	/// Returns sign of the Renko's block
	#[must_use]
	pub fn sign(&self) -> i8 {
		1 - (self.close < self.open) as i8 * 2
	}
}

/// [`Renko`]'s method [output type](crate::core::Method::Output)
///
/// Implements [`Iterator`](std::iter::Iterator) trait for generating [`RenkoBlock`]s.
#[derive(Debug, Clone)]
#[allow(missing_copy_implementations)]
pub struct RenkoOutput {
	len: usize,
	pos: usize,
	brick_size: ValueType,
	base_line: ValueType,
}

impl RenkoOutput {
	/// Checks if there is no generated blocks
	///
	/// Returns `true` if there is no generated blocks.
	/// Otherwise returns `false`.
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.len == 0
	}
}

impl Iterator for RenkoOutput {
	type Item = RenkoBlock;

	fn next(&mut self) -> Option<Self::Item> {
		if self.pos == self.len {
			return None;
		}

		let block = RenkoBlock {
			// open: (1. + self.pos as ValueType * self.brick_size) * self.base_line,
			open: self.brick_size.mul_add(self.pos as ValueType, 1.) * self.base_line,
			// close: (1. + (self.pos + 1) as ValueType * self.brick_size) * self.base_line,
			close: self.brick_size.mul_add((self.pos + 1) as ValueType, 1.) * self.base_line,
		};

		self.pos += 1;

		Some(block)
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let size = self.len - self.pos;
		(size, Some(size))
	}

	#[inline]
	fn count(self) -> usize {
		self.len - self.pos
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.pos += n;
		self.next()
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item> {
		if self.pos == self.len {
			None
		} else {
			self.pos = self.len - 1;
			self.next()
		}
	}
}

impl ExactSizeIterator for RenkoOutput {
	#[inline]
	fn len(&self) -> usize {
		self.len - self.pos
	}
}

impl std::iter::FusedIterator for RenkoOutput {}

impl Method<'_> for Renko {
	type Params = ValueType;
	type Input = ValueType;
	type Output = RenkoOutput;

	fn new(brick_size: Self::Params, value: Self::Input) -> Result<Self, Error> {
		if (ValueType::EPSILON..1.0).contains(&brick_size) {
			let half_size = value * brick_size * 0.5;
			Ok(Self {
				brick_size,
				last_block_upper: value + half_size,
				last_block_lower: value - half_size,
				next_block_upper: (value + half_size) * (1. + brick_size),
				next_block_lower: (value - half_size) * (1. - brick_size),
			})
		} else {
			Err(Error::WrongMethodParameters)
		}
	}

	#[inline]
	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	#[allow(clippy::suboptimal_flops)]
	#[allow(clippy::assign_op_pattern)]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		if value >= self.next_block_upper {
			let len = ((value - self.last_block_upper) / self.last_block_upper / self.brick_size)
				as usize;
			let base_line = self.last_block_upper;

			self.last_block_upper = base_line * (1. + self.brick_size * len as ValueType);
			self.last_block_lower = base_line * (1. + self.brick_size * (len - 1) as ValueType);

			self.next_block_upper = self.last_block_upper * (1. + self.brick_size);
			self.next_block_lower = self.last_block_lower * (1. - self.brick_size);

			RenkoOutput {
				len,
				pos: 0,
				brick_size: self.brick_size,
				base_line,
			}
		} else if value <= self.next_block_lower {
			let len = ((self.last_block_lower - value) / self.last_block_lower / self.brick_size)
				as usize;
			let base_line = self.last_block_lower;

			self.last_block_upper = base_line * (1. - self.brick_size * (len - 1) as ValueType);
			self.last_block_lower = base_line * (1. - self.brick_size * len as ValueType);

			self.next_block_upper = self.last_block_upper * (1. + self.brick_size);
			self.next_block_lower = self.last_block_lower * (1. - self.brick_size);

			RenkoOutput {
				len,
				pos: 0,
				brick_size: -self.brick_size,
				base_line,
			}
		} else {
			RenkoOutput {
				len: 0,
				pos: 0,
				brick_size: ValueType::NAN,
				base_line: ValueType::NAN,
			}
		}

		// if value >= self.next_block_upper {
		// 	let blocks_count = ((value - self.last_block_upper) / self.last_block_upper / self.brick_size) as usize;

		// 	let mut blocks = Vec::with_capacity(blocks_count);

		// 	let base_line = self.last_block_upper;
		// 	for i in 0..blocks_count {
		// 		let block = RenkoBlock {
		// 			// lower_bound: base_line * (1. + self.brick_size * (i-1) as ValueType),
		// 			open: base_line * self.brick_size.mul_add(i as ValueType, 1.0),
		// 			// upper_bound: base_line * (1. + self.brick_size * i as ValueType),
		// 			close: base_line * self.brick_size.mul_add((i + 1) as ValueType, 1.0),
		// 		};

		// 		self.last_block_upper = block.close;
		// 		self.last_block_lower = block.open;
		// 		self.next_block_upper =
		// 			block.close * (1. + self.brick_size);
		// 		self.next_block_lower =
		// 			block.open * (1. - self.brick_size);

		// 		blocks.push(block);
		// 	}

		// 	blocks
		// } else if value <= self.next_block_lower {
		// 	let blocks_count = ((self.last_block_lower - value) / self.last_block_lower / self.brick_size) as usize;

		// 	let mut blocks = Vec::with_capacity(blocks_count);

		// 	let base_line = self.last_block_lower;
		// 	for i in 0..blocks_count {
		// 		let block = RenkoBlock {
		// 			close: base_line * (1. - self.brick_size * (i+1) as ValueType),
		// 			open: base_line * (1. - self.brick_size * i as ValueType),
		// 		};

		// 		self.last_block_upper = block.open;
		// 		self.last_block_lower = block.close;
		// 		self.next_block_upper =
		// 			block.open * (1. + self.brick_size);
		// 		self.next_block_lower =
		// 			block.close * (1. - self.brick_size);

		// 		blocks.push(block);
		// 	}

		// 	blocks
		// } else {
		// 	Vec::new()
		// }
	}
}

#[cfg(test)]
mod tests {
	use super::{Method, Renko};

	#[test]
	#[allow(clippy::match_same_arms)]
	#[allow(clippy::clone_on_copy)]
	fn test_renko() {
		//						0	   1	   2	   3	  4	     5	    6
		let inputs = [100.0, 100.5, 101.506, 105.0, 102.0, 101.4, 100.0];

		let mut renko = Renko::new(0.01, inputs[0]).unwrap();
		inputs
			.iter()
			.copied()
			.map(|x| (renko.clone(), renko.next(x), renko.clone()))
			.enumerate()
			.for_each(|(i, (r1, x, r2))| match i {
				0 => assert!(x.is_empty(), "{:?} => {:?} ::: {:?}", r1, r2, x),
				1 => assert!(x.is_empty(), "{:?} => {:?} ::: {:?}", r1, r2, x),
				2 => assert_eq!(x.len(), 1, "{:?} => {:?} ::: {:?}", r1, r2, x),
				3 => assert_eq!(x.len(), 3, "{:?} => {:?} ::: {:?}", r1, r2, x),
				4 => assert_eq!(x.len(), 1, "{:?} => {:?} ::: {:?}", r1, r2, x),
				5 => assert_eq!(x.len(), 1, "{:?} => {:?} ::: {:?}", r1, r2, x),
				6 => assert_eq!(x.len(), 1, "{:?} => {:?} ::: {:?}", r1, r2, x),
				_ => panic!("Expected match arm for index {}", i),
			});
	}
}
