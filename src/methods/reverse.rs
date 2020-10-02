use crate::core::Method;
use crate::core::{Action, Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Searches for reverse points over last `left`+`right`+1 values of type [`ValueType`]
///
/// # Parameters
///
/// Has a tuple of 2 parameters (`left`: [`PeriodType`], `right`: [`PeriodType`])
///
/// `left` should be > 0 and `right` should be > 0
///
/// There is an additional restriction on parameters: `left`+`right`+1 should be <= [`PeriodType`]::MAX.
/// So if your [`PeriodType`] is default `u8`, then `left`+`right`+1 should be <= 255
///
/// [Read more about `PeriodType`][`PeriodType`]
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`Action`]
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::ReverseSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   1,   0,   -1,  0,   1 ];
///
/// let mut pivot = ReverseSignal::new(2, 2, s[0]).unwrap();
/// let r2: Vec<i8> = s.iter().map(|&v| pivot.next(v).analog()).collect();
///
/// assert_eq!(r2, r2);
/// ```
///
/// # Perfomance
///
/// O(`left`+`right`)
///
/// # See also
///
/// [ReverseHighSignal], [ReverseLowSignal]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReverseSignal {
	high: ReverseHighSignal,
	low: ReverseLowSignal,
}

impl ReverseSignal {
	/// Constructs new instanceof ReverseSignal
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` touple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Result<Self, Error> {
		Method::new((left, right), value)
	}
}

impl Method for ReverseSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Result<Self, Error>
	where
		Self: Sized,
	{
		let (left, right) = params;

		if left == 0 || right == 0 {
			return Err(Error::WrongMethodParameters);
		}

		Ok(Self {
			high: Method::new((left, right), value).unwrap(),
			low: Method::new((left, right), value).unwrap(),
		})
	}

	#[inline]
	fn next(&mut self, value: ValueType) -> Self::Output {
		self.low.next(value) - self.high.next(value)
	}
}

/// Searches for high Reverse points over last `left`+`right`+1 values of type [`ValueType`]
///
/// # Parameters
///
/// Has a tuple of 2 parameters (`left`: [`PeriodType`], `right`: [`PeriodType`])
///
/// `left` should be > 0 and `right` should be > 0
///
/// There is an additional restriction on parameters: `left`+`right`+1 should be <= [`PeriodType`]::MAX.
/// So if your [`PeriodType`] is default `u8`, then `left`+`right`+1 should be <= 255
///
/// [Read more about `PeriodType`][`PeriodType`]
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`Action`]
///
/// ```
/// use yata::core::Method;
/// use yata::methods::ReverseHighSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   0,   0,   1,   0,   0 ];
///
/// let mut pivot = ReverseHighSignal::new(2, 2, s[0]).unwrap();
/// let r2: Vec<i8> = s.iter().map(|&v| pivot.next(v).analog()).collect();
///
/// assert_eq!(r2, r2);
/// ```
///
/// # Perfomance
///
/// O(`left`+`right`)
///
/// # See also
///
/// [ReverseSignal], [ReverseLowSignal]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReverseHighSignal {
	left: PeriodType,
	right: PeriodType,

	max_value: ValueType,
	max_index: PeriodType,
	index: PeriodType,
	window: Window<ValueType>,
}

impl ReverseHighSignal {
	/// Constructs new instanceof ReverseHighSignal
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` touple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Result<Self, Error> {
		Method::new((left, right), value)
	}
}

impl Method for ReverseHighSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Result<Self, Error> {
		let (left, right) = params;

		if left == 0 || right == 0 {
			return Err(Error::WrongMethodParameters);
		}

		Ok(Self {
			left,
			right,
			max_value: value,
			max_index: 0,
			index: 0,
			window: Window::new(left + right + 1, value),
		})
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.window.push(value);

		let first_index = (self.index + 1).saturating_sub(self.window.len());

		if self.max_index < first_index {
			let mut max_index = first_index;
			let mut max_value = self.window.first();

			self.window
				.iter()
				.zip(first_index..)
				.skip(1)
				.for_each(|(x, i)| {
					if x >= max_value {
						max_value = x;
						max_index = i;
					}
				});
			self.max_value = max_value;
			self.max_index = max_index;
		} else {
			if value >= self.max_value {
				self.max_value = value;
				self.max_index = self.index;
			}
		}

		let s;
		if self.index >= self.right && self.max_index == self.index.saturating_sub(self.right) {
			s = Action::BUY_ALL;
		} else {
			s = Action::None;
		}

		self.index += 1;
		s
	}
}

/// Searches for low reverse points over last `left`+`right`+1 values of type [`ValueType`]
///
/// # Parameters
///
/// Has a tuple of 2 parameters (`left`: [`PeriodType`], `right`: [`PeriodType`])
///
/// `left` should be > 0 and `right` should be > 0
///
/// There is an additional restriction on parameters: `left`+`right`+1 should be <= [`PeriodType`]::MAX.
/// So if your [`PeriodType`] is default `u8`, then `left`+`right`+1 should be <= 255
///
/// [Read more about `PeriodType`][`PeriodType`]
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`Action`]
///
/// ```
/// use yata::core::Method;
/// use yata::methods::ReverseHighSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   1,   0,   0,   0,   1 ];
///
/// let mut pivot = ReverseHighSignal::new(2, 2, s[0]).unwrap();
/// let r2: Vec<i8> = s.iter().map(|&v| pivot.next(v).analog()).collect();
///
/// assert_eq!(r2, r2);
/// ```
///
/// # Perfomance
///
/// O(`left`+`right`)
///
/// # See also
///
/// [ReverseSignal], [ReverseHighSignal]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReverseLowSignal {
	left: PeriodType,
	right: PeriodType,

	// value:	ValueType,
	// before:	usize,
	// after:	usize,
	min_value: ValueType,
	min_index: PeriodType,
	index: PeriodType,
	window: Window<ValueType>,
}

impl ReverseLowSignal {
	/// Constructs new instanceof ReverseLowSignal
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` touple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Result<Self, Error> {
		Method::new((left, right), value)
	}
}

impl Method for ReverseLowSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Result<Self, Error> {
		let (left, right) = params;

		if left == 0 || right == 0 {
			return Err(Error::WrongMethodParameters);
		}

		Ok(Self {
			left,
			right,
			min_value: value,
			min_index: 0,
			index: 0,
			window: Window::new(left + right + 1, value),
		})
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.window.push(value);

		let first_index = (self.index + 1).saturating_sub(self.window.len());

		if self.min_index < first_index {
			let mut min_index = first_index;
			let mut min_value = self.window.first();

			self.window
				.iter()
				.zip(first_index..)
				.skip(1)
				.for_each(|(x, i)| {
					if x <= min_value {
						min_value = x;
						min_index = i;
					}
				});
			self.min_value = min_value;
			self.min_index = min_index;
		} else {
			if value <= self.min_value {
				self.min_value = value;
				self.min_index = self.index;
			}
		}

		let s;
		if self.index >= self.right && self.min_index == self.index.saturating_sub(self.right) {
			s = Action::BUY_ALL;
		} else {
			s = Action::None;
		}

		self.index += 1;
		s
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::methods::tests::test_const;

	#[test]
	fn test_pivot_low_const() {
		for i in 1..10 {
			for j in 1..10 {
				let input = (i as ValueType + 56.0) / 16.3251;
				let mut method = ReverseLowSignal::new(i, j, input).unwrap();

				let output = method.next(input);
				test_const(&mut method, input, output);
			}
		}
	}

	#[test]
	#[rustfmt::skip]
	fn test_pivot_low() {
		let v: Vec<ValueType> = vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0, 1.0, 2.0, 1.0, 2.0, 3.0];
		let r: Vec<i8> =  vec![ 0,   0,   0,   1,   0,   0,   0,   0,   1,   0,   0,   1,   0,   0,   0,   0,   1 ];

		let mut pivot = ReverseLowSignal::new(2, 2, v[0]).unwrap();

		let r2: Vec<i8> = v.iter().map(|&x| pivot.next(x).analog()).collect();
		assert_eq!(r, r2);
	}

	#[test]
	fn test_pivot_high_const() {
		for i in 1..10 {
			for j in 1..10 {
				let input = (i as ValueType + 56.0) / 16.3251;
				let mut method = ReverseHighSignal::new(i, j, input).unwrap();

				let output = method.next(input);
				test_const(&mut method, input, output);
			}
		}
	}

	#[test]
	#[rustfmt::skip]
	fn test_pivot_high() {
		let v: Vec<ValueType> = vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0, 1.0, 2.0, 1.0, 2.0, 3.0];
		let r: Vec<i8> =  vec![ 0,   0,   0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   1,   0,   0,   0 ];

		let mut pivot = ReverseHighSignal::new(2, 2, v[0]).unwrap();

		let r2: Vec<i8> = v.iter().map(|&x| pivot.next(x).analog()).collect();
		assert_eq!(r, r2);
	}
}
