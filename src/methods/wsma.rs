use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType};
use crate::methods::EMA;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Wilder’s Smoothing Average](http://etfhq.com/blog/2010/08/19/wilders-smoothing/) of specified `length` for timeseries of type [`ValueType`]
///
/// It is actually a simple EMA over `length*2-1` periods
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 0 and < `PeriodType::MAX`/2
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::WSMA;
///
/// // WSMA of length=3
/// let mut wsma = WSMA::new(4, 2.0).unwrap();
///
/// wsma.next(3.0);
/// wsma.next(6.0);
///
/// assert_eq!(wsma.next(9.0), 4.640625);
/// assert_eq!(wsma.next(12.0), 6.48046875);
/// ```
/// # Performance
///
/// O(1)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WSMA(EMA);

const MAX_PERIOD: PeriodType = PeriodType::MAX / 2;

impl Method for WSMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		if length > MAX_PERIOD {
			return Err(Error::WrongMethodParameters);
		}

		Ok(Self(EMA::new(length * 2 - 1, value)?))
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.0.next(value)
	}
}

#[cfg(test)]
mod tests {
	use crate::core::Method;
	use crate::core::ValueType;
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const_float;

	use super::WSMA as TestingMethod;

	#[test]
	fn test_wsma_const() {
		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_wsma1() {
		let mut candles = RandomCandles::default();
		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(x.close));
		});
	}

	#[test]
	fn test_wsma() {
		let candles = RandomCandles::default();
		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]).unwrap();

			let mut prev_value = src[0];
			src.iter().for_each(|&x| {
				let value = ma.next(x);

				let value2 = prev_value + (x - prev_value) / length as ValueType;

				prev_value = value2;

				assert_eq_float(value2, value);
			});
		});
	}
}
