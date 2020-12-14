use crate::core::Method;
use crate::core::{ValueType, OHLCV};
use std::borrow::BorrowMut;

/// Implements some methods for sequence manipulations.
pub trait Sequence<T>: AsRef<[T]> {
	/// Validates the sequence.
	fn validate(&self) -> bool;

	/// Applies [`Method`](crate::core::Method) on the slice in-place.
	fn apply<'a, M>(&'a mut self, method: M)
	where
		M: Method<'a, Input = T, Output = T> + BorrowMut<M> + 'a,
		Self: AsMut<[T]>,
		T: Copy;

	/// Calls [`Method`](crate::core::Method) over the slice and returns `Vec` of result values.
	fn call<'a, M>(&self, method: M) -> Vec<M::Output>
	where
		M: Method<'a, Input = T> + BorrowMut<M> + 'a;

	/// Returns a reference to the first value in the sequence or `None` if it's empty.
	fn get_initial_value(&self) -> Option<&T>;

	/// Clones current `Sequence` of `OHLCV`s into [Heikin Ashi](https://en.wikipedia.org/wiki/Candlestick_chart#Heikin-Ashi_candlesticks) `OHLCV`s
	fn clone_to_heiken_ashi(&self) -> Vec<(ValueType, ValueType, ValueType, ValueType, ValueType)>
	where
		T: OHLCV,
	{
		let last = self
			.as_ref()
			.windows(2)
			.map(|x| (&x[0], &x[1]))
			.map(|(prev, current)| {
				let open = current.ha_open(prev);
				let close = current.ha_close(prev);

				(
					open,
					current.high().max(open).max(close),
					current.low().min(open).max(close),
					close,
					current.volume(),
				)
			});

		let first = self.as_ref().iter().take(1).map(|x| {
			(
				(x.open() + x.close()) * 0.5,
				x.high(),
				x.low(),
				x.ohlc4(),
				x.volume(),
			)
		});

		first.chain(last).collect()
	}
}

impl<Q: AsRef<[ValueType]>> Sequence<ValueType> for Q {
	fn validate(&self) -> bool {
		self.as_ref().iter().copied().all(ValueType::is_finite)
	}

	fn apply<'a, M>(&'a mut self, mut method: M)
	where
		M: Method<'a, Input = ValueType, Output = ValueType> + BorrowMut<M> + 'a,
		Self: AsMut<[ValueType]>,
	{
		let borrowed = method.borrow_mut();
		let input = self.as_mut();
		input.iter_mut().for_each(|x| *x = borrowed.next(*x));
	}

	fn call<'a, M>(&self, mut method: M) -> Vec<M::Output>
	where
		M: Method<'a, Input = ValueType> + BorrowMut<M> + 'a,
	{
		let method = method.borrow_mut();
		let inputs = self.as_ref();

		inputs.iter().map(|x| method.next(*x)).collect()
	}

	fn get_initial_value(&self) -> Option<&ValueType> {
		self.as_ref().first()
	}
}

impl<T: OHLCV + Clone, Q: AsRef<[T]>> Sequence<T> for Q {
	fn validate(&self) -> bool {
		self.as_ref().iter().all(OHLCV::validate)
	}

	fn apply<'a, M>(&'a mut self, mut method: M)
	where
		M: Method<'a, Input = T, Output = T> + BorrowMut<M> + 'a,
		Self: AsMut<[T]>,
		T: Copy,
	{
		self.as_mut().iter_mut().for_each(|x| *x = method.next(*x));
	}

	fn call<'a, M>(&self, mut method: M) -> Vec<M::Output>
	where
		M: Method<'a, Input = T> + BorrowMut<M> + 'a,
	{
		let method = method.borrow_mut();
		let input = self.as_ref();

		input.iter().map(|x| method.next(x.clone())).collect()
	}

	fn get_initial_value(&self) -> Option<&T> {
		self.as_ref().first()
	}
}
