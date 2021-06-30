use super::{Error, Sequence};
use std::fmt;

type BoxedFnMethod<'a, M> = Box<dyn FnMut(&'a <M as Method>::Input) -> <M as Method>::Output>;

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
/// let mut ma = SMA::new(2, s[0]).unwrap();
///
/// s.iter().enumerate().for_each(|(index, &value)| {
///     assert_eq!(ma.next(value), (value + s[index.saturating_sub(1)])/2.);
/// });
/// ```
///
/// ### Get a whole new vector over the input vector
///
/// You can call method `over` any `Sequence`:
/// ```
/// use yata::methods::SMA;
/// use yata::prelude::*;
///
/// let s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
/// let mut ma = SMA::new(2, s[0]).unwrap();
///
/// let result = ma.over(s);
/// assert_eq!(result.as_slice(), &[1., 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
/// ```
///
/// Or you can provide `Method` to `Sequence`:
/// ```
/// use yata::methods::SMA;
/// use yata::prelude::*;
///
/// let s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
/// let mut ma = SMA::new(2, s[0]).unwrap();
///
/// let result = s.call(ma);
/// assert_eq!(result.as_slice(), &[1., 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
/// ```
///
/// Or you can even change `Sequence` values in-place:
/// ```
/// use yata::methods::SMA;
/// use yata::prelude::*;
///
/// let mut s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
/// let mut ma = SMA::new(2, s[0]).unwrap();
///
/// s.apply(ma);
/// assert_eq!(s.as_slice(), &[1., 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
/// ```
///
/// # Be advised
/// There is no `reset` method on the trait. If you need reset a state of the `Method` instance, you should just create a new one.
pub trait Method: fmt::Debug {
	/// Method parameters
	type Params;
	/// Input value type
	type Input: ?Sized;
	/// Output value type
	type Output;

	/// Static method for creating an instance of the method with given `parameters` and initial `value` (simply first input value)
	fn new(parameters: Self::Params, initial_value: &Self::Input) -> Result<Self, Error>
	where
		Self: Sized;

	/// Generates next output value based on the given input `value`
	fn next(&mut self, value: &Self::Input) -> Self::Output;

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

	/// Iterates the `Method` over the given `inputs` slice and returns `Vec` of output values.
	///
	/// # Guarantees
	///
	/// The length of an output `Vec` is always equal to the length of an `inputs` slice.
	/// ```
	/// use yata::methods::SMA;
	/// use yata::prelude::*;
	///
	/// let s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
	/// let mut ma = SMA::new(5, s[0]).unwrap();
	///
	/// let result = ma.over(&s);
	/// assert_eq!(result.len(), s.len());
	/// ```
	///
	/// ```
	/// use yata::methods::SMA;
	/// use yata::prelude::*;
	///
	/// let s: Vec<_> = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
	/// let mut ma = SMA::new(100, s[0]).unwrap();
	///
	/// let result = ma.over(&s);
	/// assert_eq!(result.len(), s.len());
	/// ```
	#[inline]
	fn over<S>(&mut self, inputs: S) -> Vec<Self::Output>
	where
		S: Sequence<Self::Input>,
		Self::Input: Sized,
		Self: Sized,
	{
		inputs.call(self)
	}

	/// Applies method to the sequence in-place.
	fn apply<T, S>(&mut self, sequence: &mut S)
	where
		S: Sequence<T> + AsMut<[T]> + ?Sized,
		Self: Method<Input = T, Output = T> + Sized,
	{
		sequence.apply(self);
	}

	/// Creates new `Method` instance and iterates it over the given `inputs` slice and returns `Vec` of output values.
	///
	/// # Guarantees
	///
	/// The length of an output `Vec` is always equal to the length of an `inputs` slice.
	fn new_over<S>(parameters: Self::Params, inputs: S) -> Result<Vec<Self::Output>, Error>
	where
		S: Sequence<Self::Input>,
		Self::Input: Sized,
		Self: Sized,
	{
		match inputs.get_initial_value() {
			Some(v) => {
				let mut method = Self::new(parameters, v)?;
				Ok(inputs.call(&mut method))
			}
			None => Ok(Vec::new()),
		}
	}

	/// Creates new `Method` instance and applies it to the `sequence`.
	fn new_apply<T, S>(parameters: Self::Params, sequence: &mut S) -> Result<(), Error>
	where
		S: Sequence<T> + AsMut<[T]>,
		Self: Method<Input = T, Output = T> + Sized,
	{
		let initial_value = {
			// Why do we need to get immutable reference to get initial value?
			// If try to remove it, then compile error occured.
			// Looks like some Rust type system bug?
			let seq = &*sequence;

			match seq.get_initial_value() {
				Some(v) => v,
				None => return Ok(()),
			}
		};

		let mut m = Self::new(parameters, initial_value)?;
		sequence.apply(&mut m);
		Ok(())
	}

	/// Creates a function from the `Method` instance
	fn into_fn<'a>(mut self) -> BoxedFnMethod<'a, Self>
	where
		Self: Sized + 'static,
		Self::Input: 'static,
	{
		let f = move |x| self.next(x);
		Box::new(f)
	}

	/// Creates new function based on the method
	fn new_fn(
		params: Self::Params,
		initial_value: &Self::Input,
	) -> Result<BoxedFnMethod<Self>, Error>
	where
		Self: Sized + 'static,
	{
		let instance = Self::new(params, initial_value)?;

		Ok(instance.into_fn())
	}
}
