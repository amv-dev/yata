#[allow(unused_imports)]
use super::Method;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;
use std::ops::Deref;
use std::ops::DerefMut;

/// Wrapper for timeseries data vectors
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sequence<T: Copy>(Vec<T>);

impl<T: Copy> Sequence<T> {
	/// Creates an empty `Sequence` instance
	pub fn empty() -> Self {
		Self(Vec::new())
	}

	/// Creates an empty `Sequence` instance with pre-allocated memory for `len` elements
	pub fn new(len: usize) -> Self {
		Self(Vec::with_capacity(len))
	}

	/// Changes vector values using method
	///
	/// # Examples
	///
	/// ```
	/// use yata::core::Sequence;
	/// use yata::methods::SMA;
	/// use yata::prelude::*;
	///
	/// let mut s: Sequence<_> = Sequence::from(vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]);
	/// let mut ma = SMA::new(2, s[0]).unwrap();
	///
	/// s.apply(&mut ma);
	/// assert_eq!(s.as_slice(), &[1., 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
	/// ```
	///
	/// ```
	/// use yata::core::Sequence;
	/// use yata::helpers::method;
	/// use std::convert::TryInto;
	///
	/// let mut s: Sequence<_> = Sequence::from(vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]);
	/// let mut ma = method("sma".try_into().unwrap(), 2, s[0]).unwrap();
	///
	/// s.apply(ma.as_mut());
	/// assert_eq!(s.as_slice(), &[1., 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
	/// ```
	pub fn apply<P>(&mut self, method: &mut dyn Method<Params = P, Input = T, Output = T>) {
		self.iter_mut().for_each(|x| {
			*x = method.next(*x);
		});
	}

	/// Evaluates given `method` over this `sequence` and returns new `sequence` filled with method's output values
	pub fn eval<P, O: Copy>(
		&self,
		method: &mut dyn Method<Params = P, Input = T, Output = O>,
	) -> Sequence<O> {
		self.iter().map(|&x| method.next(x)).collect()
	}
}

impl<T: Copy> Deref for Sequence<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T: Copy> DerefMut for Sequence<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T: Copy> From<Vec<T>> for Sequence<T> {
	fn from(v: Vec<T>) -> Self {
		Self(v)
	}
}

impl<T: Copy> From<Sequence<T>> for Vec<T> {
	fn from(v: Sequence<T>) -> Self {
		v.0
	}
}

impl<T: Copy> FromIterator<T> for Sequence<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let v: Vec<T> = iter.into_iter().collect();
		Self(v)
	}
}
