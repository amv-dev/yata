use crate::core::Method;
use crate::core::{ValueType, OHLCV};
use std::borrow::BorrowMut;

/// Implements some methods for sequence manipulations.
pub trait Sequence<T>: AsRef<[T]> {
	/// Validates the sequence.
	fn validate(&self) -> bool;

	/// Applies [`Method`](crate::core::Method) on the slice in-place.
	fn apply<'a, M>(&'a mut self, method: &mut M)
	where
		M: Method<'a, Input = T, Output = T> + ?Sized,
		Self: AsMut<[T]>;

	/// Calls [`Method`](crate::core::Method) over the slice and returns `Vec` of result values.
	fn call<'a, M>(&self, method: &mut M) -> Vec<M::Output>
	where
		M: Method<'a, Input = T>;

	/// Returns a reference to the first value in the sequence or `None` if it's empty.
	fn get_initial_value(&self) -> Option<T> where T: Copy;
}

impl<Q: AsRef<[ValueType]>> Sequence<ValueType> for Q {
	fn validate(&self) -> bool {
		self.as_ref().iter().copied().all(ValueType::is_finite)
	}

	fn apply<'a, M>(&'a mut self, method: &mut M)
	where
		M: Method<'a, Input = ValueType, Output = ValueType> + ?Sized,
		Self: AsMut<[ValueType]>,
	{
		self.as_mut().iter_mut().for_each(|x| *x = method.next(*x));
	}

	fn call<'a, M>(&self, method: &mut M) -> Vec<M::Output>
	where
		M: Method<'a, Input = ValueType>,
	{
		let method = method.borrow_mut();
		let inputs = self.as_ref();

		inputs.iter().map(|x| method.next(*x)).collect()
	}

	fn get_initial_value(&self) -> Option<ValueType> {
		self.as_ref().first().copied()
	}
}

impl<T: OHLCV + Copy, Q: AsRef<[T]>> Sequence<T> for Q {
	fn validate(&self) -> bool {
		self.as_ref().iter().all(OHLCV::validate)
	}

	fn apply<'a, M>(&'a mut self, method: &mut M)
	where
		M: Method<'a, Input = T, Output = T> + ?Sized,
		Self: AsMut<[T]>,
	{
		self.as_mut().iter_mut().for_each(|x| *x = method.next(*x));
	}

	fn call<'a, M>(&self, method: &mut M) -> Vec<M::Output>
	where
		M: Method<'a, Input = T>,
	{
		let method = method.borrow_mut();
		let input = self.as_ref();

		input.iter().map(|x| method.next(*x)).collect()
	}

	fn get_initial_value(&self) -> Option<T> {
		self.as_ref().first().copied()
	}
}
