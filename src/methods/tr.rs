use crate::core::{Error, Method, ValueType, OHLCV};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [True Range](https://en.wikipedia.org/wiki/Average_true_range)
///
/// # Parameters
///
/// Has no parameters
///
/// # Input type
///
/// Input type is [`OHLCV`]
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// # Performance
///
/// O(1)
///
/// # See also
///
/// [`OHLCV::tr`]
///
/// [`OHLCV::tr_close`]
///
/// [`ValueType`]: crate::core::ValueType
/// [`OHLCV`]: crate::core::OHLCV
/// [`OHLCV::tr`]: crate::core::OHLCV::tr
/// [`OHLCV::tr_close`]: crate::core::OHLCV::tr_close
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TR {
	prev_close: ValueType,
}

impl<'a> TR {
	/// Creates new TR method instance
	/// It's a simple shortcut for [`Method::new`](crate::core::Method::new) method.
	#[allow(clippy::needless_pass_by_value)]
	pub fn new(value: <Self as Method>::Input) -> Result<Self, Error> {
		Method::new((), value)
	}
}

impl<'a> Method<'a> for TR {
	type Params = ();
	type Input = &'a dyn OHLCV;
	type Output = ValueType;

	fn new(_: Self::Params, value: Self::Input) -> Result<Self, Error> {
		Ok(Self {
			prev_close: value.close(),
		})
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let result = value.tr_close(self.prev_close);
		self.prev_close = value.close();

		result
	}
}

#[cfg(test)]
mod tests {
	use super::{Method, OHLCV, TR as TestingMethod};
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[test]
	fn test_tr_const() {
		for _ in 1..10 {
			let input = RandomCandles::default().first();
			let mut method = TestingMethod::new(&input).unwrap();

			let output = method.next(&input);
			test_const(&mut method, &input, output);
		}
	}

	#[test]
	fn test_tr() {
		let candles = RandomCandles::default();

		let src: Vec<_> = candles.take(50).collect();

		let mut tr = TestingMethod::new(&src[0]).unwrap();
		let mut prev_close = src[0].close;

		for c in &src {
			let value = (c.high - c.low)
				.max((c.high - prev_close).abs())
				.max((c.low - prev_close).abs());

			let value2 = tr.next(c);
			let value3 = c.tr_close(prev_close);

			prev_close = c.close;
			assert_eq_float(value, value2);
			assert_eq_float(value, value3);
		}
	}
}
