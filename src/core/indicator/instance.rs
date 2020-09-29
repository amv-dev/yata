use super::{IndicatorConfig, IndicatorResult};
use crate::core::OHLC;

/// Base trait for implementing indicators **State**
pub trait IndicatorInstance<T: OHLC> {
	// type Config: IndicatorConfig + IndicatorInitializer<T>;
	/// Type of Indicator **Configuration**
	type Config: IndicatorConfig;

	/// Returns a reference to the indicator **Configuration**
	fn config(&self) -> &Self::Config;

	// fn config(&self) -> &dyn IndicatorConfig<T>;

	/// Preceed given candle and returns [`IndicatorResult`](crate::core::IndicatorResult)
	fn next(&mut self, candle: T) -> IndicatorResult
	where
		Self: Sized;

	/// Evaluates the **State** over the given sequence of candles and returns sequence of `IndicatorResult`s.
	/// ```
	/// use yata::prelude::*;
	/// use yata::helpers::{RandomCandles};
	/// use yata::indicators::Trix;
	///
	/// let candles: Vec<_> = RandomCandles::new().take(10).collect();
	/// let trix = Trix::default();
	/// let mut state = trix.init(candles[0]);
	///
	/// let results = state.over(&candles);
	/// println!("{:?}", results);
	/// ```
	#[inline]
	fn over(&mut self, candles: &[T]) -> Vec<IndicatorResult>
	where
		Self: Sized,
	{
		candles.iter().map(|&x| self.next(x)).collect()
	}

	/// Returns true if indicator is using volume data
	fn is_volume_based(&self) -> bool
	where
		Self: Sized,
	{
		self.config().is_volume_based()
	}

	/// Returns count of indicator's raw values and count of indicator's signals.
	///
	/// See more at [IndicatorConfig](crate::core::IndicatorConfig#tymethod.size)
	fn size(&self) -> (u8, u8)
	where
		Self: Sized,
	{
		self.config().size()
	}

	/// Returns a name of the indicator
	fn name(&self) -> &'static str {
		self.config().name()
	}
}

// pub trait IndicatorInstanceDyn<T: OHLC>: Debug {
// 	fn config(&self) -> &dyn IndicatorConfigDyn<T>;

// 	fn next(&mut self, candle: T) -> IndicatorResult;

// 	fn name(&self) -> &str {
// 		let parts = std::any::type_name::<Self>().split("::");
// 		parts.last().unwrap_or_default()
// 	}

// 	fn is_volume_based(&self) -> bool { false }

// 	#[inline]
// 	fn over(&mut self, candles: &Sequence<T>) -> Vec<IndicatorResult> {
// 		candles.iter().map(|&x| self.next(x)).collect()
// 	}

// 	fn size(&self) -> (u8, u8);
// }
