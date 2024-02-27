use crate::prelude::{Error, Method};

/// Trait for picking the very last value for methods and indicators
pub trait Peekable<V> {
	/// Peeks the very last value, produced by method or indicator
	fn peek(&self) -> V;
}

/// Trait for picking historical values for methods and indicators
pub trait Buffered<V> {
	/// Picks value at `index` position, starting from the newest value
	fn get(&self, index: usize) -> Option<V>;
}

/// Wrapper for holding historical data
#[derive(Debug, Clone)]
pub struct WithHistory<T: ?Sized, V> {
	history: Vec<V>,
	instance: T,
}

impl<T: ?Sized, V: Clone> WithHistory<T, V> {
	/// Picks value at `index` position, starting from the newest value
	pub fn get(&self, index: usize) -> Option<V> {
		Buffered::get(self, index)
	}

	/// Iterate over historical values, starting from the oldest value
	pub fn iter(&self) -> impl Iterator<Item = &V> {
		self.history.iter()
	}
}

impl<T: ?Sized, V: Clone> Buffered<V> for WithHistory<T, V> {
	fn get(&self, index: usize) -> Option<V> {
		let index = self.history.len().checked_sub(index + 1)?;
		self.history.get(index).cloned()
	}
}

impl<T> Method for WithHistory<T, T::Output>
where
	T: Method,
	T::Output: std::fmt::Debug + Clone,
{
	type Params = T::Params;
	type Input = T::Input;
	type Output = T::Output;

	fn new(parameters: Self::Params, initial_value: &Self::Input) -> Result<Self, Error> {
		Ok(Self {
			instance: T::new(parameters, initial_value)?,
			history: Vec::new(),
		})
	}

	fn next(&mut self, value: &Self::Input) -> Self::Output {
		let next_value = self.instance.next(value);
		self.history.push(next_value.clone());

		next_value
	}
}

impl<'a, T, V> IntoIterator for &'a WithHistory<T, V> {
	type Item = &'a V;
	type IntoIter = std::slice::Iter<'a, V>;

	fn into_iter(self) -> Self::IntoIter {
		self.history.iter()
	}
}

impl<T, V> IntoIterator for WithHistory<T, V> {
	type Item = V;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.history.into_iter()
	}
}

/// Wrapper for keeping last produced value
#[derive(Debug, Clone)]
pub struct WithLastValue<T: ?Sized, V> {
	last_value: V,
	instance: T,
}

impl<T> Method for WithLastValue<T, T::Output>
where
	T: Method,
	T::Output: std::fmt::Debug + Clone,
{
	type Params = T::Params;
	type Input = T::Input;
	type Output = T::Output;

	fn new(parameters: Self::Params, initial_value: &Self::Input) -> Result<Self, Error> {
		let mut instance = T::new(parameters, initial_value)?;
		let last_value = instance.next(initial_value);

		Ok(Self {
			last_value,
			instance,
		})
	}

	fn next(&mut self, value: &Self::Input) -> Self::Output {
		let next_value = self.instance.next(value);
		self.last_value = next_value.clone();
		next_value
	}
}

impl<T, V: Clone> Peekable<V> for WithLastValue<T, V> {
	fn peek(&self) -> V {
		self.last_value.clone()
	}
}

impl<V: Clone, T: Peekable<V>> Peekable<V> for &T {
	fn peek(&self) -> V {
		(*self).peek()
	}
}

impl<V: Clone, T: Buffered<V>> Buffered<V> for &T {
	fn get(&self, index: usize) -> Option<V> {
		(*self).get(index)
	}
}
