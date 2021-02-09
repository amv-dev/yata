use crate::core::{Candle, Error, Method, OHLCV};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Converting between different timeframes.
///
/// # Parameters
///
/// Has a single parameter `period`: [`usize`]
///
/// `period` must be > `0`
///
/// # Input type
///
/// Input type is reference to [`OHLCV`]
///
/// # Output type
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::CollapseTimeframe;
///
/// let timeframe = [
/// //   open  high  low  close volume
///     (10.0, 15.0, 5.0, 12.0, 1000.0),
///     (12.1, 17.0, 6.0, 13.0, 2000.0),
/// ];
///
/// let mut collapser = CollapseTimeframe::new(2, &timeframe[0]).unwrap();
///
/// assert_eq!(collapser.next(&timeframe[0]), None);
///
/// let collapsed = collapser.next(&timeframe[1]).unwrap();
/// assert_eq!(collapsed.open(), 10.0);
/// assert_eq!(collapsed.high(), 17.0);
/// assert_eq!(collapsed.low(), 5.0);
/// assert_eq!(collapsed.close(), 13.0);
/// assert_eq!(collapsed.volume(), 3000.0);
/// ```
///
/// Output type is [`Candle`]
///
/// # Performance
///
/// O(1)
///
/// See also Sequence's [`collapse_timeframe`](crate::core::Sequence::collapse_timeframe) function.
///
/// [`ValueType`]: crate::core::ValueType
/// [`Candle`]: crate::core::Candle
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CollapseTimeframe {
	current: Option<Candle>,
	index: usize,
	period: usize,
}

impl<'a> Method<'a> for CollapseTimeframe {
	type Params = usize;
	type Input = &'a dyn OHLCV;
	type Output = Option<Candle>;

	fn new(period: Self::Params, _candle: Self::Input) -> Result<Self, Error> {
		if period == 0 {
			return Err(Error::WrongMethodParameters);
		}

		Ok(Self {
			period,
			..Self::default()
		})
	}

	fn next(&mut self, candle: Self::Input) -> Self::Output {
		let current = self.current.map_or(candle.into(), |c2| Candle {
			high: c2.high.max(candle.high()),
			low: c2.low.min(candle.low()),
			close: candle.close(),
			volume: c2.volume + candle.volume(),
			..c2
		});

		self.current = Some(current);

		self.index += 1;

		if self.index == self.period {
			self.index = 0;
			self.current.take()
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Candle, CollapseTimeframe as TestingMethod, Method, OHLCV};
	use crate::helpers::{assert_eq_float, RandomCandles};

	#[test]
	fn test_timeframe_collapse() {
		let candles = RandomCandles::new().take(100).collect::<Vec<_>>();

		for length in 2..10 {
			let mut method = TestingMethod::new(length, &candles[0]).unwrap();

			let converted = candles.iter().map(|x| method.next(x)).filter_map(|x| x);

			candles
				.windows(length)
				.step_by(length)
				.map(|window| {
					let first = window.first().unwrap();

					Candle {
						open: first.open(),
						high: window
							.iter()
							.map(OHLCV::high)
							.fold(first.high(), |a, b| a.max(b)),
						low: window
							.iter()
							.map(OHLCV::low)
							.fold(first.low(), |a, b| a.min(b)),
						close: window.last().map(OHLCV::close).unwrap(),
						volume: window.iter().map(OHLCV::volume).sum(),
					}
				})
				.zip(converted)
				.for_each(|(a, b)| {
					assert_eq_float(a.open, b.open);
					assert_eq_float(a.high, b.high);
					assert_eq_float(a.low, b.low);
					assert_eq_float(a.close, b.close);
					assert_eq_float(a.volume, b.volume);
				});
		}
	}

	#[test]
	fn test_timeframe_collapse1() {
		let candles = RandomCandles::new().take(100).collect::<Vec<_>>();

		let mut method = TestingMethod::new(1, &candles[0]).unwrap();

		let converted = candles.iter().map(|x| method.next(x)).map(Option::unwrap);

		candles.iter().zip(converted).for_each(|(a, b)| {
			assert_eq_float(a.open, b.open);
			assert_eq_float(a.high, b.high);
			assert_eq_float(a.low, b.low);
			assert_eq_float(a.close, b.close);
			assert_eq_float(a.volume, b.volume);
		});
	}

	#[test]
	#[should_panic]
	fn test_timeframe_collapse_fail() {
		let candles = RandomCandles::new().take(1).collect::<Vec<_>>();
		TestingMethod::new(0, &candles[0]).unwrap();
	}
}
