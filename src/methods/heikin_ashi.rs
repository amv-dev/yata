use crate::core::{Candle, Error, Method, ValueType, OHLCV};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Converts default `OHLCV`s into [Heikin Ashi](https://en.wikipedia.org/wiki/Candlestick_chart#Heikin-Ashi_candlesticks) `OHLCV`s
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeikinAshi {
	next_open: ValueType,
}

impl Method for HeikinAshi {
	type Params = ();
	type Input = dyn OHLCV;
	type Output = Candle;

	fn new((): Self::Params, value: &Self::Input) -> Result<Self, Error> {
		// It is not so obvious how we should correctly initialize first candle
		// The reason why `ohlc4` is used is because it's the only way we can achieve stable results
		// when constant input value is provided, which is one of requiremets for all the methods
		Ok(Self {
			next_open: value.ohlc4(),
		})
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		let open = self.next_open;
		let close = value.ohlc4();

		self.next_open = (open + close) * 0.5;

		Candle {
			open,
			high: value.high().max(open),
			low: value.low().min(open),
			close,
			volume: value.volume(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{HeikinAshi, OHLCV};
	use crate::core::{Candle, Method, ValueType};
	use crate::helpers::{assert_eq_float, RandomCandles};

	#[test]
	#[allow(clippy::inspect_for_each)]
	fn test_heikin_ashi() {
		let mut candles = RandomCandles::default();

		let first = candles.first();
		let mut heikin_ashi = HeikinAshi::new((), &first).unwrap();

		let mut prev = Candle {
			open: first.ohlc4(),
			high: ValueType::NAN,
			low: ValueType::NAN,
			close: first.ohlc4(),
			volume: ValueType::NAN,
		};

		candles
			.take(100)
			.map(|candle| {
				let open = (prev.open() + prev.close()) / 2.0;
				let close = (candle.open() + candle.high() + candle.low() + candle.close()) / 4.0;

				let tested = Candle {
					open,
					high: candle.high().max(open).max(close),
					low: candle.low().min(open).min(close),
					close,
					..candle
				};

				prev = tested;

				(tested, heikin_ashi.next(&candle))
			})
			.inspect(|(original, ha)| assert_eq_float(original.close(), ha.close()))
			.inspect(|(original, ha)| assert_eq_float(original.high(), ha.high()))
			.inspect(|(original, ha)| assert_eq_float(original.low(), ha.low()))
			.inspect(|(original, ha)| assert_eq_float(original.close(), ha.close()))
			.for_each(|(original, ha)| assert_eq_float(original.volume(), ha.volume()));
	}
}
