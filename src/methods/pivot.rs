use crate::core::Method;
use crate::core::{Action, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Searches for [Pivot Points](https://en.wikipedia.org/wiki/Pivot_point_(technical_analysis)) over last `left`+`right`+1 values of type [`ValueType`]
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
/// use yata::methods::PivotSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   1,   0,   -1,  0,   1 ];
///
/// let mut pivot = PivotSignal::new(2, 2, s[0]);
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
/// [PivotHighSignal], [PivotLowSignal]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PivotSignal {
	high: PivotHighSignal,
	low: PivotLowSignal,
}

impl PivotSignal {
	/// Constructs new instanceof PivotSignal
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` touple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Self {
		debug_assert!(
			left > 0 && right > 0,
			"PivotSignal: left and right should be >= 1"
		);

		Self {
			high: Method::new((left, right), value),
			low: Method::new((left, right), value),
		}
	}
}

impl Method for PivotSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Self
	where
		Self: Sized,
	{
		let (left, right) = params;

		debug_assert!(
			left >= 1 && right >= 1,
			"PivotSignal: left and right should be >= 1"
		);

		Self {
			high: Method::new((left, right), value),
			low: Method::new((left, right), value),
		}
	}

	#[inline]
	fn next(&mut self, value: ValueType) -> Self::Output {
		self.low.next(value) - self.high.next(value)
	}
}

/// Searches for high [Pivot Points](https://en.wikipedia.org/wiki/Pivot_point_(technical_analysis)) over last `left`+`right`+1 values of type [`ValueType`]
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
/// use yata::methods::PivotHighSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   0,   0,   1,   0,   0 ];
///
/// let mut pivot = PivotHighSignal::new(2, 2, s[0]);
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
/// [PivotSignal], [PivotLowSignal]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
///
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PivotHighSignal {
	left: PeriodType,
	right: PeriodType,

	max_value: ValueType,
	max_index: PeriodType,
	index: PeriodType,
	window: Window<ValueType>,
}

impl PivotHighSignal {
	/// Constructs new instanceof PivotHighSignal
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` touple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Self {
		Method::new((left, right), value)
	}
}

impl Method for PivotHighSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Self {
		debug_assert!(
			params.0 >= 1 && params.1 >= 1,
			"PivotHighSignal: left and right should be >= 1"
		);

		let (left, right) = params;

		Self {
			left,
			right,
			max_value: value,
			max_index: 0,
			index: 0,
			window: Window::new(left + right + 1, value),
		}
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

/// Searches for low [Pivot Points](https://en.wikipedia.org/wiki/Pivot_point_(technical_analysis)) over last `left`+`right`+1 values of type [`ValueType`]
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
/// use yata::methods::PivotHighSignal;
///
/// let s = [1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0];
/// let r = [ 0,   0,   1,   0,   0,   0,   1 ];
///
/// let mut pivot = PivotHighSignal::new(2, 2, s[0]);
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
/// [PivotSignal], [PivotHighSignal]
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`Action`]: crate::core::Action
///
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PivotLowSignal {
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

impl PivotLowSignal {
	/// Constructs new instanceof PivotLowSignal
	/// It's just an alias for `Method::new((left, right), value)` but without parentheses of `Input` touple
	pub fn new(left: PeriodType, right: PeriodType, value: ValueType) -> Self {
		Method::new((left, right), value)
	}
}

impl Method for PivotLowSignal {
	type Params = (PeriodType, PeriodType);
	type Input = ValueType;
	type Output = Action;

	fn new(params: Self::Params, value: Self::Input) -> Self {
		debug_assert!(
			params.0 >= 1 && params.1 >= 1,
			"PivotLowSignal: left and right should be >= 1"
		);

		let (left, right) = params;

		Self {
			left,
			right,
			min_value: value,
			min_index: 0,
			index: 0,
			window: Window::new(left + right + 1, value),
		}
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
	#[allow(unused_imports)]
	use super::*;

	#[test]
	fn test_pivot_low_const() {
		use super::*;
		use crate::core::Method;
		use crate::methods::tests::test_const;

		for i in 1..10 {
			for j in 1..10 {
				let input = (i as ValueType + 56.0) / 16.3251;
				let mut method = PivotLowSignal::new(i, j, input);

				let output = method.next(input);
				test_const(&mut method, input, output);
			}
		}
	}

	#[test]
	#[rustfmt::skip]
	fn test_pivot_low() {
		let v: Vec<f64> = vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0, 1.0, 2.0, 1.0, 2.0, 3.0];
		let r: Vec<i8> =  vec![ 0,   0,   0,   1,   0,   0,   0,   0,   1,   0,   0,   1,   0,   0,   0,   0,   1 ];

		let mut pivot = PivotLowSignal::new(2, 2, v[0]);

		let r2: Vec<i8> = v.iter().map(|&x| pivot.next(x).analog()).collect();
		assert_eq!(r, r2);
	}

	#[test]
	fn test_pivot_high_const() {
		use super::*;
		use crate::core::Method;
		use crate::methods::tests::test_const;

		for i in 1..10 {
			for j in 1..10 {
				let input = (i as ValueType + 56.0) / 16.3251;
				let mut method = PivotHighSignal::new(i, j, input);

				let output = method.next(input);
				test_const(&mut method, input, output);
			}
		}
	}

	#[test]
	#[rustfmt::skip]
	fn test_pivot_high() {
		let v: Vec<f64> = vec![2.0, 1.0, 2.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 4.0, 1.0, 2.0, 1.0, 2.0, 3.0];
		let r: Vec<i8> =  vec![ 0,   0,   0,   0,   0,   0,   1,   0,   0,   0,   0,   0,   0,   1,   0,   0,   0 ];

		let mut pivot = PivotHighSignal::new(2, 2, v[0]);

		let r2: Vec<i8> = v.iter().map(|&x| pivot.next(x).analog()).collect();
		assert_eq!(r, r2);
	}
}
