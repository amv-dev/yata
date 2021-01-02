use crate::core::Method;
use crate::core::{Action, Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Searches for reversal points over last `left`+`right`+1 values of type [`ValueType`]
///
/// # Parameters
///
/// Has a tuple of 2 parameters (`left`: [`PeriodType`], `right`: [`PeriodType`])
///
/// `left` should be > `0` and `right` should be > `0`
///
/// There is an additional restriction on parameters: `left`+`right`+1 should be <= [`PeriodType`]::MAX.
/// So if your [`PeriodType`] is default `u8`, then `left`+`right`+1 should be <= `255`
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
/// use yata::methods::ReversalSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   1,   0,   -1,  0,   0 ];
///
/// let mut pivot = ReversalSignal::new(2, 2, s[0]).unwrap();
/// let r2: Vec<i8> = s.iter().map(|&v| pivot.next(v).analog()).collect();
///
/// assert_eq!(r2, r);
/// ```
///
/// # Performance
///
/// O(`left`+`right`)
///
/// # See also
///
/// [`UpperReversalSignal`], [`LowerReversalSignal`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReversalSignal {
	high: UpperReversalSignal,
	low: LowerReversalSignal,
}

impl ReversalSignal {
	/// Constructs new instanceof `ReversalSignal`
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` tuple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Result<Self, Error> {
		Method::new((left, right), value)
	}
}

impl Method<'_> for ReversalSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Result<Self, Error>
	where
		Self: Sized,
	{
		Ok(Self {
			high: Method::new(params, value)?,
			low: Method::new(params, value)?,
		})
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.low.next(value) - self.high.next(value)
	}
}

/// Searches for upper reversal points over last `left`+`right`+1 values of type [`ValueType`]
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
/// use yata::methods::UpperReversalSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   0,   0,   1,   0,   0 ];
///
/// let mut pivot = UpperReversalSignal::new(2, 2, s[0]).unwrap();
/// let r2: Vec<i8> = s.iter().map(|&v| pivot.next(v).analog()).collect();
///
/// assert_eq!(r2, r);
/// ```
///
/// # Performance
///
/// O(`left`+`right`)
///
/// # See also
///
/// [`ReversalSignal`], [`LowerReversalSignal`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpperReversalSignal {
	left: PeriodType,
	right: PeriodType,

	max_value: ValueType,
	max_index: PeriodType,
	index: PeriodType,
	window: Window<ValueType>,
}

impl UpperReversalSignal {
	/// Constructs new instanceof `UpperReversalSignal`
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` tuple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Result<Self, Error> {
		Method::new((left, right), value)
	}
}

impl Method<'_> for UpperReversalSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Result<Self, Error> {
		let (left, right) = params;

		if left == 0 || right == 0 || left.saturating_add(right) == PeriodType::MAX {
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

		let first_index = self
			.index
			.saturating_add(1)
			.saturating_sub(self.window.len());

		if self.max_index < first_index {
			let mut max_index = first_index;
			let mut max_value = self.window.oldest();

			self.window
				.iter_rev()
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
		} else if value >= self.max_value {
			self.max_value = value;
			self.max_index = self.index;
		}

		let s = if self.index >= self.right
			&& self.max_index == self.index.saturating_sub(self.right)
		{
			Action::BUY_ALL
		} else {
			Action::None
		};

		self.index = self.index.saturating_add(1);
		s
	}
}

/// Searches for lower reversal points over last `left`+`right`+1 values of type [`ValueType`]
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
/// use yata::methods::UpperReversalSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   0,   0,   1,   0,   0 ];
///
/// let mut pivot = UpperReversalSignal::new(2, 2, s[0]).unwrap();
/// let r2: Vec<i8> = s.iter().map(|&v| pivot.next(v).analog()).collect();
///
/// assert_eq!(r2, r);
/// ```
///
/// # Performance
///
/// O(`left`+`right`)
///
/// # See also
///
/// [`ReversalSignal`], [`UpperReversalSignal`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LowerReversalSignal {
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

impl LowerReversalSignal {
	/// Constructs new instanceof `LowerReversalSignal`
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` tuple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Result<Self, Error> {
		Method::new((left, right), value)
	}
}

impl Method<'_> for LowerReversalSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Result<Self, Error> {
		let (left, right) = params;

		if left == 0 || right == 0 || left.saturating_add(right) == PeriodType::MAX {
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

		let first_index = self
			.index
			.saturating_add(1)
			.saturating_sub(self.window.len());

		if self.min_index < first_index {
			let mut min_index = first_index;
			let mut min_value = self.window.oldest();

			self.window
				.iter_rev()
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
		} else if value <= self.min_value {
			self.min_value = value;
			self.min_index = self.index;
		}

		let s = if self.index >= self.right
			&& self.min_index == self.index.saturating_sub(self.right)
		{
			Action::BUY_ALL
		} else {
			Action::None
		};

		self.index = self.index.saturating_add(1);
		s
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::methods::tests::test_const;

	#[test]
	fn test_reverse_low_const() {
		for i in 1..254 {
			for j in 1..(254 - i) {
				let input = (i as ValueType + 56.0) / 16.3251;
				let mut method = LowerReversalSignal::new(i, j, input).unwrap();

				let output = method.next(input);
				test_const(&mut method, input, output);
			}
		}
	}

	#[test]
	#[rustfmt::skip]
	fn test_reverse_low() {
		let v: Vec<ValueType> = vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0, 1.0, 2.0, 1.0, 2.0, 3.0];
		let r: Vec<i8> =  vec![ 0,   0,   0,   1,   0,   0,   0,   0,   1,   0,   0,   1,   0,   0,   0,   0,   1 ];

		let mut pivot = LowerReversalSignal::new(2, 2, v[0]).unwrap();

		let r2: Vec<i8> = v.iter().map(|&x| pivot.next(x).analog()).collect();
		assert_eq!(r, r2);
	}

	#[test]
	fn test_reverse_high_const() {
		for i in 1..254 {
			for j in 1..(254 - i) {
				let input = (i as ValueType + 56.0) / 16.3251;
				let mut method = UpperReversalSignal::new(i, j, input).unwrap();

				let output = method.next(input);
				test_const(&mut method, input, output);
			}
		}
	}

	#[test]
	#[rustfmt::skip]
	fn test_reverse_high() {
		let v: Vec<ValueType> = vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0, 1.0, 2.0, 1.0, 2.0, 3.0];
		let r: Vec<i8> =  vec![ 0,   0,   0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   1,   0,   0,   0 ];

		let mut pivot = UpperReversalSignal::new(2, 2, v[0]).unwrap();

		let r2: Vec<i8> = v.iter().map(|&x| pivot.next(x).analog()).collect();
		assert_eq!(r, r2);
	}
}
