use crate::core::Method;
use crate::core::{ValueType, OHLCV};

/// Implements some methods for sequence manipulations.
pub trait Sequence<T> {
	/// Validates the sequence.
	fn validate(&self) -> bool;

	/// Applies [`Method`](crate::core::Method) on the slice in-place.
	fn apply<M>(&mut self, method: &mut M)
	where
		M: Method<Input = T, Output = T> + ?Sized;

	/// Returns a reference to the first value in the sequence or `None` if it's empty.
	fn get_initial_value(&self) -> Option<&T>;
}

impl<T: OHLCV + Sized> Sequence<T> for [T] {
	fn validate(&self) -> bool {
		self.iter().all(OHLCV::validate)
	}

	fn apply<M>(&mut self, method: &mut M)
	where
		M: Method<Input = T, Output = T> + ?Sized,
	{
		self.iter_mut().for_each(|x| *x = method.next(x));
	}

	fn get_initial_value(&self) -> Option<&T> {
		self.first()
	}
}

impl Sequence<ValueType> for [ValueType] {
	fn validate(&self) -> bool {
		self.iter().copied().all(ValueType::is_finite)
	}

	fn apply<M>(&mut self, method: &mut M)
	where
		M: Method<Input = ValueType, Output = ValueType> + ?Sized,
	{
		self.iter_mut().for_each(|x| *x = method.next(x));
	}

	fn get_initial_value(&self) -> Option<&ValueType> {
		self.first()
	}
}
