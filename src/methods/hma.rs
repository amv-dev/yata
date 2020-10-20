use super::WMA;
use crate::core::{Error, Method, PeriodType, ValueType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Hull Moving Average](https://www.tradingview.com/scripts/hullma/) for last `length` values for timeseries of type [`ValueType`]
///
/// HMA = [`WMA`] from (2*[`WMA`] over `length`/2 − [`WMA`] over `length`) over sqrt(`length`))
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 1
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
/// use yata::methods::HMA;
/// use yata::helpers::RandomCandles;
///
/// let mut candles = RandomCandles::default();
///
/// let mut hma = HMA::new(5, candles.first().close).unwrap();
///
/// candles.take(5).enumerate().for_each(|(index, candle)| {
///     println!("HMA at #{} is {}", index, hma.next(candle.close));
/// });
///
/// ```
///
/// # Performance
///
/// O(1)
///
/// # See also
///
/// [Weighted Moving Average][`WMA`]
///
/// [`WMA`]: crate::methods::WMA
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HMA {
	wma1: WMA,
	wma2: WMA,
	wma3: WMA,
}

impl Method for HMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: Self::Input) -> Result<Self, Error> {
		#[allow(clippy::cast_possible_truncation)]
		match length {
			0 | 1 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				wma1: WMA::new(length / 2, value)?,
				wma2: WMA::new(length, value)?,
				wma3: WMA::new((length as ValueType).sqrt() as PeriodType, value)?,
			}),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let w1 = self.wma1.next(value);
		let w2 = self.wma2.next(value);

		self.wma3.next(w1.mul_add(2., -w2))
	}
}

#[cfg(test)]
mod tests {
	use super::{HMA as TestingMethod, WMA};
	use crate::core::Method;
	use crate::core::{PeriodType, ValueType};
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const_float;

	#[test]
	fn test_hma_const() {
		for i in 2..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_hma() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		#[allow(clippy::cast_possible_truncation)]
		(2..20).for_each(|length| {
			let mut wma1 = WMA::new(length, src[0]).unwrap();
			let mut wma2 = WMA::new(length / 2, src[0]).unwrap();
			let mut wma3 = WMA::new((length as ValueType).sqrt() as PeriodType, src[0]).unwrap();

			let mut ma = TestingMethod::new(length, src[0]).unwrap();

			src.iter().for_each(|&x| {
				let value1 = ma.next(x);
				let value2 = wma3.next(2. * wma2.next(x) - wma1.next(x));
				assert_eq_float(value2, value1);
			});
		});
	}
}
