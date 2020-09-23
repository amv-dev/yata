use crate::core::Method;
use crate::core::{PeriodType, ValueType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Exponential Moving Average](https://en.wikipedia.org/wiki/Moving_average#Exponential_moving_average) of specified `length` for timeseries of type [`ValueType`]
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
/// use yata::prelude::*;
/// use yata::methods::EMA;
///
/// // EMA of length=3
/// let mut ema = EMA::new(3, 3.0);
///
/// ema.next(3.0);
/// ema.next(6.0);
///
/// assert_eq!(ema.next(9.0), 6.75);
/// assert_eq!(ema.next(12.0), 9.375);
/// ```
/// # Perfomance
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

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "EMA: length should be > 0");

		let alpha = 2. / ((length + 1) as ValueType);
		Self { alpha, value }
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.value = (value - self.value).mul_add(self.alpha, self.value);

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

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "DMA: length should be > 0");

		Self {
			ema: EMA::new(length, value),
			dma: EMA::new(length, value),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.dma.next(self.ema.next(value))
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

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "TMA: length should be > 0");

		Self {
			dma: DMA::new(length, value),
			tma: EMA::new(length, value),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		self.tma.next(self.dma.next(value))
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
/// let mut dema = DEMA::new(3, 1.0);
///
/// dema.next(1.0);
/// dema.next(2.0);
///
/// assert_eq!(dema.next(3.0), 2.75);
/// assert_eq!(dema.next(4.0), 3.8125);
/// ```
///
/// # Perfomance
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

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "DEMA: length should be > 0");

		Self {
			ema: EMA::new(length, value),
			dma: EMA::new(length, value),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let ema = self.ema.next(value);
		let dma = self.dma.next(ema);

		// 2. * ema - dma
		ema.mul_add(2., -dma)
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
/// let mut tema = TEMA::new(3, 1.0);
///
/// tema.next(1.0);
/// tema.next(2.0);
///
/// assert_eq!(tema.next(3.0), 2.9375);
/// assert_eq!(tema.next(4.0), 4.0);
/// ```
///
/// # Perfomance
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

	fn new(length: Self::Params, value: Self::Input) -> Self {
		debug_assert!(length > 0, "TEMA: length should be > 0");

		Self {
			ema: EMA::new(length, value),
			dma: EMA::new(length, value),
			tma: EMA::new(length, value),
		}
	}

	#[inline]
	fn next(&mut self, value: Self::Input) -> Self::Output {
		let ema = self.ema.next(value);
		let dma = self.dma.next(ema);
		let tma = self.tma.next(dma);

		// 3. * (ema - dma) + tma
		(ema - dma).mul_add(3., tma)
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused_imports)]
	use crate::core::ValueType;
	use crate::helpers::RandomCandles;

	#[allow(dead_code)]
	const SIGMA: ValueType = 1e-5;

	#[test]
	fn test_ema_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const_float;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = EMA::new(i, input);

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_ema1() {
		use super::{Method, EMA as TestingMethod};
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_ema() {
		use super::{Method, EMA as TestingMethod};
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value = src[0];
			src.iter().enumerate().for_each(|(i, &x)| {
				let value = ma.next(x);

				let value2 = alpha * x + (1. - alpha) * prev_value;

				prev_value = value2;

				assert!(
					(value2 - value).abs() < SIGMA,
					"{}, {} at index {} with length {}",
					value2,
					value,
					i,
					length
				);
			});
		});
	}

	#[test]
	fn test_dma_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const_float;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = DMA::new(i, input);

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_dma1() {
		use super::{Method, DMA as TestingMethod};
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_dma() {
		use super::{Method, DMA as TestingMethod};
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value1 = src[0];
			let mut prev_value2 = src[0];
			src.iter().enumerate().for_each(|(i, &x)| {
				let value = ma.next(x);

				let ema1 = alpha * x + (1. - alpha) * prev_value1;
				let ema2 = alpha * ema1 + (1. - alpha) * prev_value2;

				prev_value1 = ema1;
				prev_value2 = ema2;

				let value2 = ema2;

				assert!(
					(value2 - value).abs() < SIGMA,
					"{}, {} at index {} with length {}",
					value2,
					value,
					i,
					length
				);
			});
		});
	}

	#[test]
	fn test_dema_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const_float;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = DEMA::new(i, input);

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_dema1() {
		use super::{Method, DEMA as TestingMethod};
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_dema() {
		use super::{Method, DEMA as TestingMethod};
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value1 = src[0];
			let mut prev_value2 = src[0];
			src.iter().enumerate().for_each(|(i, &x)| {
				let value = ma.next(x);

				let ema1 = alpha * x + (1. - alpha) * prev_value1;
				let ema2 = alpha * ema1 + (1. - alpha) * prev_value2;

				prev_value1 = ema1;
				prev_value2 = ema2;

				let value2 = 2. * ema1 - ema2;

				assert!(
					(value2 - value).abs() < SIGMA,
					"{}, {} at index {} with length {}",
					value2,
					value,
					i,
					length
				);
			});
		});
	}

	#[test]
	fn test_tma_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const_float;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TMA::new(i, input);

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_tma1() {
		use super::{Method, TMA as TestingMethod};
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_tma() {
		use super::{Method, TMA as TestingMethod};
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value1 = src[0];
			let mut prev_value2 = src[0];
			let mut prev_value3 = src[0];
			src.iter().enumerate().for_each(|(i, &x)| {
				let value = ma.next(x);

				let ema1 = alpha * x + (1. - alpha) * prev_value1;
				let ema2 = alpha * ema1 + (1. - alpha) * prev_value2;
				let ema3 = alpha * ema2 + (1. - alpha) * prev_value3;

				prev_value1 = ema1;
				prev_value2 = ema2;
				prev_value3 = ema3;

				let value2 = ema3;

				assert!(
					(value2 - value).abs() < SIGMA,
					"{}, {} at index {} with length {}",
					value2,
					value,
					i,
					length
				);
			});
		});
	}

	#[test]
	fn test_tema_const() {
		use super::*;
		use crate::core::{Candle, Method};
		use crate::methods::tests::test_const_float;

		for i in 1..30 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TEMA::new(i, input);

			let output = method.next(input);
			test_const_float(&mut method, input, output);
		}
	}

	#[test]
	fn test_tema1() {
		use super::{Method, TEMA as TestingMethod};
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close);

		candles.take(100).for_each(|x| {
			assert!((x.close - ma.next(x.close)).abs() < SIGMA);
		});
	}

	#[test]
	fn test_tema() {
		use super::{Method, TEMA as TestingMethod};
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(100).map(|x| x.close).collect();

		(1..20).for_each(|length| {
			let mut ma = TestingMethod::new(length, src[0]);

			let alpha = 2. / (length + 1) as ValueType;

			let mut prev_value1 = src[0];
			let mut prev_value2 = src[0];
			let mut prev_value3 = src[0];
			src.iter().enumerate().for_each(|(i, &x)| {
				let value = ma.next(x);

				let ema = alpha * x + (1. - alpha) * prev_value1;
				let dma = alpha * ema + (1. - alpha) * prev_value2;
				let tma = alpha * dma + (1. - alpha) * prev_value3;

				prev_value1 = ema;
				prev_value2 = dma;
				prev_value3 = tma;

				let value2 = 3. * ema - 3. * dma + tma;

				assert!(
					(value2 - value).abs() < SIGMA,
					"{}, {} at index {} with length {}",
					value2,
					value,
					i,
					length
				);
			});
		});
	}
}
