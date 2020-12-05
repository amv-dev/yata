use super::{IndicatorConfig, IndicatorResult};
use crate::core::OHLC;

/// Base trait for implementing indicators **State**
pub trait IndicatorInstance {
	// type Config: IndicatorConfig + IndicatorInitializer<T>;
	/// Type of Indicator **Configuration**
	type Config: IndicatorConfig;

	/// Returns a reference to the indicator **Configuration**
	fn config(&self) -> &Self::Config;

	// fn config(&self) -> &dyn IndicatorConfig<T>;

	/// Preceed given candle and returns [`IndicatorResult`](crate::core::IndicatorResult)
	fn next(&mut self, candle: &dyn OHLC) -> IndicatorResult;

	/// Evaluates the **State** over the given sequence of candles and returns sequence of `IndicatorResult`s.
	/// ```
	/// use yata::prelude::*;
	/// use yata::helpers::{RandomCandles};
	/// use yata::indicators::Trix;
	///
	/// let candles: Vec<_> = RandomCandles::new().take(10).collect();
	/// let trix = Trix::default();
	/// let mut state = trix.init(candles[0]).unwrap();
	///
	/// let results = state.over(&candles);
	/// println!("{:?}", results);
	/// ```
	#[inline]
	fn over<T: OHLC>(&mut self, candles: &[T]) -> Vec<IndicatorResult>
	where
		Self: Sized,
	{
		candles.iter().map(|x| self.next(x)).collect()
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
	/// See more at [`IndicatorConfig`](crate::core::IndicatorConfig#tymethod.size)
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