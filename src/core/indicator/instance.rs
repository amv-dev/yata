use super::{IndicatorConfig, IndicatorResult};
use crate::core::OHLC;

/// Base trait for implementing indicators **State**
pub trait IndicatorInstance<T: OHLC> {
	// type Config: IndicatorConfig + IndicatorInitializer<T>;
	/// Type of Indicator **Configuration**
	type Config: IndicatorConfig;

	/// Returns a reference to the indicator **Configuration**
	fn config(&self) -> &Self::Config
	where
		Self: Sized;

	// fn config(&self) -> &dyn IndicatorConfig<T>;

	/// Preceed given candle and returns [`IndicatorResult`](crate::core::IndicatorResult)
	fn next(&mut self, candle: T) -> IndicatorResult
	where
		Self: Sized;

	/// Returns a name of the indicator
	fn name(&self) -> &str {
		let parts = std::any::type_name::<Self>().split("::");
		parts.last().unwrap_or_default()
	}

	/// Evaluates the **State** over the given sequence of candles and returns sequence of `IndicatorResult`.
	#[inline]
	fn over<S>(&mut self, candles: &S) -> Vec<IndicatorResult>
	where
		S: AsRef<Vec<T>>,
		Self: Sized,
	{
		candles.as_ref().iter().map(|&x| self.next(x)).collect()
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
