use crate::core::Method;
use crate::core::{ValueType, OHLCV};
use crate::helpers::Merge;

/// Implements some methods for sequence manipulations.
pub trait Sequence<T>: AsRef<[T]> {
	/// Validates the sequence.
	fn validate(&self) -> bool;

	/// Calls [`Method`](crate::core::Method) over the slice and returns `Vec` of result values.
	#[inline]
	fn call<M>(&self, method: &mut M) -> Vec<M::Output>
	where
		M: Method<Input = T>,
	{
		self.as_ref().iter().map(|x| method.next(x)).collect()
	}

	/// Applies [`Method`](crate::core::Method) on the slice in-place.
	#[inline]
	fn apply<M>(&mut self, method: &mut M)
	where
		M: Method<Input = T, Output = T>,
		Self: AsMut<[T]>,
	{
		self.as_mut().iter_mut().for_each(|x| *x = method.next(x));
	}

	/// Returns a reference to the first value in the sequence or `None` if it's empty.
	#[inline]
	fn get_initial_value(&self) -> Option<&T> {
		self.as_ref().first()
	}

	/// Returns a reference to the first value in the sequence or `None` if it's empty.
	#[inline]
	fn get_initial_value_mut(&mut self) -> Option<&mut T>
	where
		Self: AsMut<[T]>,
	{
		self.as_mut().first_mut()
	}

	/// Converts timeframe of the series
	///
	/// See also [`CollapseTimeframe`](crate::methods::CollapseTimeframe) method.
	fn collapse_timeframe(&self, size: usize, continuous: bool) -> Vec<T>
	where
		T: OHLCV + Merge<T> + Copy,
	{
		fn fold<T: OHLCV + Merge<T>>(folded: T, next: &T) -> T {
			folded.merge(next)
		}

		fn window<T: OHLCV + Merge<T> + Copy>(window: &[T]) -> T {
			let first = window[0];
			window.iter().skip(1).fold(first, fold)
		}

		self.as_ref()
			.windows(size)
			.step_by(if continuous { 1 } else { size })
			.map(window)
			.collect()
	}
}

impl<Q: AsRef<[ValueType]>> Sequence<ValueType> for Q {
	#[inline]
	fn validate(&self) -> bool {
		self.as_ref().iter().copied().all(ValueType::is_finite)
	}
}

impl<T: OHLCV, Q: AsRef<[T]>> Sequence<T> for Q {
	#[inline]
	fn validate(&self) -> bool {
		self.as_ref().iter().all(OHLCV::validate)
	}
}
