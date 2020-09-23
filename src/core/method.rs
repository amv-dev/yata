use super::Sequence;
use std::fmt;

/// Trait for creating methods for timeseries
///
/// # Regular methods usage
///
/// ### Iterate over vector's values
///
/// ```
/// use yata::methods::SMA;
/// use yata::prelude::*;
///
/// let s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
/// let mut ma = SMA::new(2, s[0]);
///
/// s.iter().enumerate().for_each(|(index, &value)| {
/// 	assert_eq!(ma.next(value), (value + s[index.saturating_sub(1)])/2.);
/// });
/// ```
///
/// ### Get a whole new vector over the input vector
///
/// ```
/// use yata::methods::SMA;
/// use yata::prelude::*;
///
/// let s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
/// let mut ma = SMA::new(2, s[0]);
///
/// let result = ma.over(s.iter().copied());
/// assert_eq!(result.as_slice(), &[1., 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
/// ```
///
/// ### Change vector values using method
///
/// ```
/// use yata::core::Sequence;
/// use yata::methods::SMA;
/// use yata::prelude::*;
///
/// let mut s: Sequence<_> = Sequence::from(vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]);
/// let mut ma = SMA::new(2, s[0]);
///
/// s.apply(&mut ma);
/// assert_eq!(s.as_slice(), &[1., 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
/// ```
///
/// # Be advised
/// There is no `reset` method on the trait. If you need reset a state of the `Method` instance, you should just create a new one.
pub trait Method: fmt::Debug {
	/// Method parameters
	type Params;
	/// Input value type
	type Input: Copy;
	/// Output value type
	type Output: Copy; // = Self::Input;

	/// Static method for creating an instance of the method with given `parameters` and initial `value` (simply first input value)
	fn new(parameters: Self::Params, initial_value: Self::Input) -> Self
	where
		Self: Sized;

	/// Generates next output value based on the given input `value`
	fn next(&mut self, value: Self::Input) -> Self::Output;

	/// Returns a name of the method
	fn name(&self) -> &str {
		let parts = std::any::type_name::<Self>().split("::");
		parts.last().unwrap_or_default()
	}

	/// Returns memory size of the method `(size, align)`
	fn memsize(&self) -> (usize, usize)
	where
		Self: Sized,
	{
		(std::mem::size_of::<Self>(), std::mem::align_of::<Self>())
	}

	/// Creates an `iterator` which produces values by the `Method` over given input data `Iterator`
	fn iter_data<I>(&mut self, input: I) -> MethodOverIterator<Self, I>
	where
		I: Iterator<Item = Self::Input>,
		Self: Sized,
	{
		MethodOverIterator::new(self, input)
	}

	/// Iterates the `Method` over the given `Iterator` and returns timeserie of output values
	///
	/// # Guarantees
	///
	/// The length of an output `Sequence` is always equal to the length of input one
	/// ```
	/// use yata::methods::SMA;
	/// use yata::prelude::*;
	///
	/// let s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
	/// let mut ma = SMA::new(5, s[0]);
	///
	/// let result = ma.over(s.iter().copied());
	/// assert_eq!(result.len(), s.len());
	/// ```
	///
	/// ```
	/// use yata::methods::SMA;
	/// use yata::prelude::*;
	///
	/// let s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
	/// let mut ma = SMA::new(100, s[0]);
	///
	/// let result = ma.over(s.iter().copied());
	/// assert_eq!(result.len(), s.len());
	/// ```
	#[inline]
	fn over<I>(&mut self, sequence: I) -> Sequence<Self::Output>
	where
		I: Iterator<Item = Self::Input>,
		Self: Sized,
	{
		sequence.map(|x| self.next(x)).collect()
	}
}

#[derive(Debug)]
pub struct MethodOverIterator<'a, T: Method, I: Iterator<Item = T::Input>> {
	method: &'a mut T,
	over: I,
}

impl<'a, T: Method, I: Iterator<Item = T::Input>> MethodOverIterator<'a, T, I> {
	pub fn new(method: &'a mut T, over: I) -> Self {
		Self { method, over }
	}
}

impl<'a, T: Method, I: Iterator<Item = T::Input>> Iterator for MethodOverIterator<'a, T, I> {
	type Item = T::Output;

	fn next(&mut self) -> Option<Self::Item> {
		let input = self.over.next()?;
		let output = self.method.next(input);
		Some(output)
	}
}
