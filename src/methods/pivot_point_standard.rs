//! The Pivot Point indicator helps traders and analysts identify potential support and resistance levels for a given
//! trading day or time period. Pivot Points are particularly popular in day trading and short-term trading strategies.
//!

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Method, ValueType, OHLCV};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// The Pivot Point indicator is calculated based on the high, low, and close prices of the previous trading
/// period (such as a day, week, or month) and provides several support and resistance levels.
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Pivot_point_(technical_analysis)>
///
pub struct PivotPointStandard {
	close: ValueType,
	high: ValueType,
	low: ValueType,
}

impl Method for PivotPointStandard {
	type Params = ();
	type Input = dyn OHLCV;
	type Output = PivotPointTraditionalOutput;

	fn new((): Self::Params, initial_value: &Self::Input) -> Result<Self, crate::core::Error>
	where
		Self: Sized,
	{
		Ok(Self {
			close: initial_value.close(),
			high: initial_value.high(),
			low: initial_value.low(),
		})
	}

	fn next(&mut self, value: &Self::Input) -> Self::Output {
		let result = PivotPointTraditionalOutput::new(self.high, self.low, self.close);
		self.close = value.close();
		self.high = value.high();
		self.low = value.low();
		result
	}
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// The Pivot Point Traditional method generates pivot points using the Traditional
/// Pivot Point method as described [TradingView](https://www.tradingview.com/support/solutions/43000521824-pivot-points-standard/).
pub struct PivotPointTraditionalOutput {
	/// PP = (HIGHprev + LOWprev + CLOSEprev) / 3
	pub pp: ValueType,
	/// R1 = PP * 2 - LOWprev
	pub r1: ValueType,
	/// S1 = PP * 2 - HIGHprev
	pub s1: ValueType,
	/// R2 = PP + (HIGHprev - LOWprev)
	pub r2: ValueType,
	/// S2 = PP - (HIGHprev - LOWprev)
	pub s2: ValueType,
	/// R3 = PP * 2 + (HIGHprev - 2 * LOWprev)
	pub r3: ValueType,
	/// S3 = PP * 2 - (2 * HIGHprev - LOWprev)
	pub s3: ValueType,
	/// R4 = PP * 3 + (HIGHprev - 3 * LOWprev)
	pub r4: ValueType,
	/// S4 = PP * 3 - (3 * HIGHprev - LOWprev)
	pub s4: ValueType,
	/// R5 = PP * 4 + (HIGHprev - 4 * LOWprev)
	pub r5: ValueType,
	/// S5 = PP * 4 - (4 * HIGHprev - LOWprev)
	pub s5: ValueType,
}

impl PivotPointTraditionalOutput {
	#[rustfmt::skip]
	#[allow(clippy::suboptimal_flops)]
	fn new(high: ValueType, low: ValueType, close: ValueType) -> Self {
		// PP = (HIGHprev + LOWprev + CLOSEprev) / 3
		let pp: ValueType = (high + low + close) / 3.0;
		// R1 = PP * 2 - LOWprev
		let r1: ValueType = pp.mul_add(2.0, -low);
		// S1 = PP * 2 - HIGHprev
		let s1: ValueType = pp.mul_add(2.0, -high);
		// R2 = PP + (HIGHprev - LOWprev)
		let r2: ValueType = pp + (high - low);
		// S2 = PP - (HIGHprev - LOWprev)
		let s2: ValueType = pp - (high - low);
		// R3 = PP * 2 + (HIGHprev - 2 * LOWprev)
		let r3: ValueType = pp.mul_add(2.0, high - 2.0 * low);
		// S3 = PP * 2 - (2 * HIGHprev - LOWprev)
		let s3: ValueType = pp * 2.0 - (2.0 * high - low);
		// R4 = PP * 3 + (HIGHprev - 3 * LOWprev)
		let r4: ValueType = pp * 3.0 + (high - 3.0 * low);
		// S4 = PP * 3 - (3 * HIGHprev - LOWprev)
		let s4: ValueType = pp * 3.0 - (3.0 * high - low);
		// R5 = PP * 4 + (HIGHprev - 4 * LOWprev)
		let r5: ValueType = pp.mul_add(4.0, high - 4.0 * low);
		// S5 = PP * 4 - (4 * HIGHprev - LOWprev)
		let s5: ValueType = pp * 4.0 - (4.0 * high - low);

		Self { pp, r1, s1, r2, s2, r3, s3, r4, s4, r5, s5 }
	}
}

#[cfg(test)]
mod test {
	use crate::helpers::assert_eq_float;

	use super::{Method, PivotPointStandard};

	#[test]
	fn test_pivot_point_standard() {
		// OHLCV: (open, high, low, close, volume)
		let candle = (2.0, 200.29, 195.21, 198.45, 10.0);
		let mut instance = PivotPointStandard::new((), &candle).unwrap();
		let next = instance.next(&candle);
		assert_eq_float(197.983, next.pp);
		assert_eq_float(200.756, next.r1);
		assert_eq_float(195.676, next.s1);
		assert_eq_float(203.063, next.r2);
		assert_eq_float(192.903, next.s2);
		assert_eq_float(205.836, next.r3);
		assert_eq_float(190.596, next.s3);
		assert_eq_float(208.609, next.r4);
		assert_eq_float(188.289, next.s4);
		assert_eq_float(211.383, next.r5);
		assert_eq_float(185.983, next.s5);
	}
}
