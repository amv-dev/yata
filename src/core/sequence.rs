use crate::core::Method;
use crate::core::{ValueType, OHLCV};
use crate::prelude::Candle;

/// Implements some methods for sequence manipulations.
pub trait Sequence<T>: AsRef<[T]> {
	/// Validates the sequence.
	fn validate(&self) -> bool;

	/// Calls [`Method`](crate::core::Method) over the slice and returns `Vec` of result values.
	fn call<'a, M>(&self, method: M) -> Vec<M::Output>
	where
		M: Method<'a, Input = T>;

	/// Applies [`Method`](crate::core::Method) on the slice in-place.
	#[inline]
	fn apply<'a, M>(&'a mut self, mut method: M)
	where
		M: Method<'a, Input = T, Output = T>,
		Self: AsMut<[T]>,
		T: Clone,
	{
		self.as_mut()
			.iter_mut()
			.for_each(|x| *x = method.next(x.clone()));
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
	fn collapse_timeframe(&self, size: usize, continuous: bool) -> Vec<Candle>
	where
		T: OHLCV,
	{
		fn fold<T: OHLCV>(folded: Candle, next: &T) -> Candle {
			Candle {
				high: folded.high.max(next.high()),
				low: folded.low.min(next.low()),
				close: next.close(),
				volume: folded.volume + next.volume(),
				..folded
			}
		}

		fn window<T: OHLCV>(window: &[T]) -> Candle {
			let first = window.first().unwrap();
			let initial = Candle {
				open: first.open(),
				high: first.high(),
				low: first.low(),
				close: first.close(),
				volume: first.volume(),
			};

			window.iter().skip(1).fold(initial, fold)
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

	#[inline]
	fn call<'a, M>(&self, mut method: M) -> Vec<M::Output>
	where
		M: Method<'a, Input = ValueType>,
	{
		self.as_ref().iter().map(|&x| method.next(x)).collect()
	}
}

impl<T: OHLCV + Clone, Q: AsRef<[T]>> Sequence<T> for Q {
	#[inline]
	fn validate(&self) -> bool {
		self.as_ref().iter().all(OHLCV::validate)
	}

	#[inline]
	fn call<'a, M>(&self, mut method: M) -> Vec<M::Output>
	where
		M: Method<'a, Input = T>,
	{
		self.as_ref()
			.iter()
			.cloned()
			.map(|x| method.next(x))
			.collect()
	}
}
