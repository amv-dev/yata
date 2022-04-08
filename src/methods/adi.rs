use crate::core::Method;
use crate::core::{Error, PeriodType, ValueType, Window, OHLCV};
use crate::helpers::Peekable;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Accumulation Distribution Index](https://en.wikipedia.org/wiki/Accumulation/distribution_index) of specified `length` for timeseries of [`OHLCV`]
///
/// [`CLV`] ranges from `-1.0` when the close is the low of the day, to `+1.0` when it's the high.
/// For instance if the close is `3/4` the way up the range then [`CLV`] is `+0.5`.
/// The accumulation/distribution index adds up volume multiplied by the [`CLV`] factor, i.e.
///
/// ADI = `ADI_prev` + [`CLV`] * [`volume`]
///
/// The name accumulation/distribution comes from the idea that during accumulation buyers are in control
/// and the price will be bid up through the day, or will make a recovery if sold down, in either case
/// more often finishing near the day's high than the low. The opposite applies during distribution.
///
/// The accumulation/distribution index is similar to on balance volume, but acc/dist is based on the close
/// within the day's range, instead of the close-to-close up or down that the latter uses.
///
/// Can be used by a shortcut [`ADI`]
///
/// Used in indicators: [`Chaikin Money Flow`](crate::indicators::ChaikinMoneyFlow), [`Chaikin Oscillator`](crate::indicators::ChaikinOscillator)
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// When `length == 0`, `ADI` becomes windowless. That means full `ADI` value accumulation over time.
///
/// When `length > 0`, `ADI` will be calculated over the last `length` values.
///
/// # Input type
/// Input type is reference to [`OHLCV`]
///
/// # Output type
/// Output type is [`ValueType`]
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::ADI;
/// use yata::helpers::RandomCandles;
///
/// let mut candles = RandomCandles::default();
/// let mut windowless = ADI::new(0, &candles.first()).unwrap();
/// let mut windowed = ADI::new(3, &candles.first()).unwrap(); // <----- Window size 3
///
/// let candle = candles.next().unwrap();
/// assert_ne!(windowless.next(&candle), windowed.next(&candle));
///
/// let candle = candles.next().unwrap();
/// assert_ne!(windowless.next(&candle), windowed.next(&candle));
///
/// let candle = candles.next().unwrap();
/// assert!((windowless.next(&candle)-windowed.next(&candle)).abs() < 1e-5); // Must be equal here
/// ```
///
/// # Performance
///
/// O(1)
///
/// # See also
///
/// [ADI]
///
/// [`OHLCV`]: crate::core::OHLCV
/// [`volume`]: crate::core::OHLCV::volume
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
/// [`CLV`]: crate::core::OHLCV::clv
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ADI {
	cmf_sum: ValueType,
	window: Window<ValueType>,
}

impl ADI {
	/// Returns last calculated value
	#[must_use]
	#[deprecated(since = "0.6.0", note = "Use `Peekable::peek` instead")]
	pub const fn get_value(&self) -> ValueType {
		self.cmf_sum
	}
}

impl Method for ADI {
	type Params = PeriodType;
	type Input = dyn OHLCV;
	type Output = ValueType;

	fn new(length: Self::Params, candle: &Self::Input) -> Result<Self, Error> {
		let mut cmf_sum = 0.0;
		let window = if length > 0 {
			let clvv = candle.clv() * candle.volume();
			cmf_sum = clvv * length as ValueType;
			Window::new(length, clvv)
		} else {
			Window::empty()
		};

		Ok(Self { cmf_sum, window })
	}

	#[inline]
	fn next(&mut self, candle: &Self::Input) -> Self::Output {
		let clvv = candle.clv() * candle.volume();
		self.cmf_sum += clvv;

		if !self.window.is_empty() {
			self.cmf_sum -= self.window.push(clvv);
		}

		self.peek()
	}
}

impl Peekable<<Self as Method>::Output> for ADI {
	fn peek(&self) -> <Self as Method>::Output {
		self.cmf_sum
	}
}

#[cfg(test)]
#[allow(clippy::suboptimal_flops)]
mod tests {
	use super::ADI;
	use crate::core::OHLCV;
	use crate::core::{Candle, Method};
	use crate::helpers::RandomCandles;
	use crate::helpers::{assert_eq_float, assert_neq_float};
	use crate::methods::tests::test_const;

	#[test]
	fn test_adi_const() {
		let candle = Candle {
			open: 121.0,
			high: 133.0,
			low: 49.0,
			close: 70.0,
			volume: 531.0,
		};

		for i in 1..30 {
			let mut adi = ADI::new(i, &candle).unwrap();
			let output = adi.next(&candle);

			test_const(&mut adi, &candle, &output);
		}
	}

	#[test]
	#[should_panic]
	fn test_adi_windowless_const() {
		let candle = Candle {
			open: 121.0,
			high: 133.0,
			low: 49.0,
			close: 70.0,
			volume: 531.0,
		};

		let mut adi = ADI::new(0, &candle).unwrap();
		let output = adi.next(&candle);

		test_const(&mut adi, &candle, &output);
	}

	#[test]
	fn test_adi() {
		let mut candles = RandomCandles::default();
		let first_candle = candles.first();
		let mut adi = ADI::new(0, &first_candle).unwrap();

		candles.take(100).fold(0., |s, candle| {
			assert_eq_float(adi.next(&candle), s + candle.clv() * candle.volume());
			s + candle.clv() * candle.volume()
		});
	}

	#[test]
	fn test_adi_windowed() {
		let mut candles = RandomCandles::default();
		let first = candles.first();
		let mut adi = ADI::new(0, &first).unwrap();
		let mut adiw = [
			ADI::new(1, &first).unwrap(),
			ADI::new(2, &first).unwrap(),
			ADI::new(3, &first).unwrap(),
			ADI::new(4, &first).unwrap(),
			ADI::new(5, &first).unwrap(),
		];

		candles
			.take(adiw.len())
			.enumerate()
			.for_each(|(i, candle)| {
				let v1 = adi.next(&candle);

				adiw.iter_mut().enumerate().for_each(|(j, adiw)| {
					let v2 = adiw.next(&candle);
					if i == j {
						assert_eq_float(v1, v2);
					} else {
						assert_neq_float(v1, v2);
					}
				});
			});
	}
}
