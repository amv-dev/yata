use crate::core::{Error, PeriodType, ValueType};
use crate::core::{Method, MovingAverage};
use crate::helpers::Peekable;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Exponential Moving Average](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average) of specified `length` for timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > `0`
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
/// use yata::methods::EMA;
///
/// // EMA of length=3
/// let mut ema = EMA::new(3, &3.0).unwrap();
///
/// ema.next(&3.0);
/// ema.next(&6.0);
///
/// assert_eq!(ema.next(&9.0), 6.75);
/// assert_eq!(ema.next(&12.0), 9.375);
/// ```
/// # Performance
///
/// O(1)
///
/// # See also
///
/// [DMA], [DEMA], [TMA], [TEMA], [RMA](crate::methods::RMA)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EMA {
	alpha: ValueType,
	value: ValueType,
}

impl Method for EMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, &value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => {
				let alpha = 2. / ((length + 1) as ValueType);
				Ok(Self { alpha, value })
			}
		}
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		self.value = (value - self.value).mul_add(self.alpha, self.value);

		self.value
	}
}

impl MovingAverage for EMA {}

impl Peekable<<Self as Method>::Output> for EMA {
	fn peek(&self) -> <Self as Method>::Output {
		self.value
	}
}

/// Simple shortcut for [EMA] over [EMA]
///
/// # See also
///
/// [EMA], [DEMA], [TMA], [TEMA], [RMA](crate::methods::RMA)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DMA {
	ema: EMA,
	dma: EMA,
}

impl Method for DMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				ema: EMA::new(length, value)?,
				dma: EMA::new(length, value)?,
			}),
		}
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		self.dma.next(&self.ema.next(value))
	}
}

impl MovingAverage for DMA {}

impl Peekable<<Self as Method>::Output> for DMA {
	fn peek(&self) -> <Self as Method>::Output {
		self.dma.value
	}
}

/// Simple shortcut for [EMA] over [EMA] over [EMA] (or [EMA] over [DMA], or [DMA] over [EMA])
///
/// # See also
///
/// [EMA], [DMA], [DEMA], [TEMA], [RMA](crate::methods::RMA)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TMA {
	dma: DMA,
	tma: EMA,
}

impl Method for TMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				dma: DMA::new(length, value)?,
				tma: EMA::new(length, value)?,
			}),
		}
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		self.tma.next(&self.dma.next(value))
	}
}

impl MovingAverage for TMA {}

impl Peekable<<Self as Method>::Output> for TMA {
	fn peek(&self) -> <Self as Method>::Output {
		self.tma.value
	}
}

/// [Double Exponential Moving Average](https://en.wikipedia.org/wiki/Double_exponential_moving_average) of specified `length` for timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 0
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
/// use yata::core::Method;
/// use yata::methods::DEMA;
///
/// // DEMA of length=3
/// let mut dema = DEMA::new(3, &1.0).unwrap();
///
/// dema.next(&1.0);
/// dema.next(&2.0);
///
/// assert_eq!(dema.next(&3.0), 2.75);
/// assert_eq!(dema.next(&4.0), 3.8125);
/// ```
///
/// # Performance
///
/// O(1)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
///
/// # See also
///
/// [EMA], [DMA], [TMA], [TEMA], [RMA](crate::methods::RMA)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEMA {
	ema: EMA,
	dma: EMA,
}

impl Method for DEMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				ema: EMA::new(length, value)?,
				dma: EMA::new(length, value)?,
			}),
		}
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		let e_ma = self.ema.next(value);
		self.dma.next(&e_ma);

		self.peek()
	}
}

impl MovingAverage for DEMA {}

impl Peekable<<Self as Method>::Output> for DEMA {
	fn peek(&self) -> <Self as Method>::Output {
		let e_ma = self.ema.value;
		let d_ma = self.dma.value;

		// 2. * ema - dma
		e_ma.mul_add(2., -d_ma)
	}
}

/// [Triple Exponential Moving Average](https://en.wikipedia.org/wiki/Triple_exponential_moving_average) of specified `length` for timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` should be > 0
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
/// use yata::core::Method;
/// use yata::methods::TEMA;
///
/// // TEMA of length=3
/// let mut tema = TEMA::new(3, &1.0).unwrap();
///
/// tema.next(&1.0);
/// tema.next(&2.0);
///
/// assert_eq!(tema.next(&3.0), 2.9375);
/// assert_eq!(tema.next(&4.0), 4.0);
/// ```
///
/// # Performance
///
/// O(1)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
///
/// # See also
///
/// [EMA], [DMA], [DEMA], [TMA], [RMA](crate::methods::RMA)
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TEMA {
	ema: EMA,
	dma: EMA,
	tma: EMA,
}

impl Method for TEMA {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, value: &Self::Input) -> Result<Self, Error> {
		match length {
			0 => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				ema: EMA::new(length, value)?,
				dma: EMA::new(length, value)?,
				tma: EMA::new(length, value)?,
			}),
		}
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		let e_ma = self.ema.next(value);
		let d_ma = self.dma.next(&e_ma);
		self.tma.next(&d_ma);

		self.peek()
	}
}

impl MovingAverage for TEMA {}

impl Peekable<<Self as Method>::Output> for TEMA {
	fn peek(&self) -> <Self as Method>::Output {
		let e_ma = self.ema.value;
		let d_ma = self.dma.value;
		let t_ma = self.tma.value;

		// 3. * (ema - dma) + tma
		(e_ma - d_ma).mul_add(3., t_ma)
	}
}

#[cfg(test)]
#[allow(clippy::suboptimal_flops)]
mod tests {
	#![allow(unused_imports)]
	use super::{DEMA, DMA, EMA, TEMA, TMA};
	use crate::core::{Method, ValueType};
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const_float;

	#[test]
	fn test_ema_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = EMA::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const_float(&mut method, &input, output);
		}
	}

	#[test]
	fn test_ema1() {
		use super::EMA as TestingMethod;
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&x.close));
		});
	}

	#[test]
	fn test_ema() {
		use super::EMA as TestingMethod;
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value = src[0];
			for &x in &src {
				let value = ma.next(&x);

				let value2 = alpha * x + (1. - alpha) * prev_value;

				prev_value = value2;

				assert_eq_float(value2, value);
			}
		});
	}

	#[test]
	fn test_dma_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = DMA::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const_float(&mut method, &input, output);
		}
	}

	#[test]
	fn test_dma1() {
		use super::DMA as TestingMethod;
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&x.close));
		});
	}

	#[test]
	fn test_dma() {
		use super::DMA as TestingMethod;
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value1 = src[0];
			let mut prev_value2 = src[0];

			for &x in &src {
				let value = ma.next(&x);

				let ema1 = alpha * x + (1. - alpha) * prev_value1;
				let ema2 = alpha * ema1 + (1. - alpha) * prev_value2;

				prev_value1 = ema1;
				prev_value2 = ema2;

				let value2 = ema2;

				assert_eq_float(value2, value);
			}
		});
	}

	#[test]
	fn test_dema_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = DEMA::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const_float(&mut method, &input, output);
		}
	}

	#[test]
	fn test_dema1() {
		use super::DEMA as TestingMethod;
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&x.close));
		});
	}

	#[test]
	fn test_dema() {
		use super::DEMA as TestingMethod;
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value1 = src[0];
			let mut prev_value2 = src[0];

			for &x in &src {
				let value = ma.next(&x);

				let ema1 = alpha * x + (1. - alpha) * prev_value1;
				let ema2 = alpha * ema1 + (1. - alpha) * prev_value2;

				prev_value1 = ema1;
				prev_value2 = ema2;

				let value2 = 2. * ema1 - ema2;

				assert_eq_float(value2, value);
			}
		});
	}

	#[test]
	fn test_tma_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TMA::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const_float(&mut method, &input, output);
		}
	}

	#[test]
	fn test_tma1() {
		use super::TMA as TestingMethod;
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&x.close));
		});
	}

	#[test]
	fn test_tma() {
		use super::TMA as TestingMethod;
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value1 = src[0];
			let mut prev_value2 = src[0];
			let mut prev_value3 = src[0];

			for &x in &src {
				let value = ma.next(&x);

				let ema1 = alpha * x + (1. - alpha) * prev_value1;
				let ema2 = alpha * ema1 + (1. - alpha) * prev_value2;
				let ema3 = alpha * ema2 + (1. - alpha) * prev_value3;

				prev_value1 = ema1;
				prev_value2 = ema2;
				prev_value3 = ema3;

				let value2 = ema3;

				assert_eq_float(value2, value);
			}
		});
	}

	#[test]
	fn test_tema_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TEMA::new(i, &input).unwrap();

			let output = method.next(&input);
			test_const_float(&mut method, &input, output);
		}
	}

	#[test]
	fn test_tema1() {
		use super::{Method, TEMA as TestingMethod};
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, &candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(&x.close));
		});
	}

	#[test]
	fn test_tema() {
		use super::{Method, TEMA as TestingMethod};
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		(1..255).for_each(|length| {
			let mut ma = TestingMethod::new(length, &src[0]).unwrap();

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value1 = src[0];
			let mut prev_value2 = src[0];
			let mut prev_value3 = src[0];

			for &x in &src {
				let value = ma.next(&x);

				let e_ma = alpha * x + (1. - alpha) * prev_value1;
				let d_ma = alpha * e_ma + (1. - alpha) * prev_value2;
				let t_ma = alpha * d_ma + (1. - alpha) * prev_value3;

				prev_value1 = e_ma;
				prev_value2 = d_ma;
				prev_value3 = t_ma;

				let value2 = 3. * e_ma - 3. * d_ma + t_ma;

				assert_eq_float(value2, value);
			}
		});
	}
}
