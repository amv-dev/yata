use super::{IndicatorConfig, IndicatorResult};
use crate::core::OHLCV;

/// Base trait for implementing indicators **State**
pub trait IndicatorInstance {
	/// Type of Indicator **Configuration**
	type Config: IndicatorConfig<Instance = Self>;

	/// Returns a reference to the indicator **Configuration**
	fn config(&self) -> &Self::Config;

	/// Evaluates given candle and returns [`IndicatorResult`](crate::core::IndicatorResult)
	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult
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
	/// let mut state = trix.init(&candles[0]).unwrap();
	///
	/// let results = state.over(&candles);
	/// println!("{:?}", results);
	/// ```
	#[inline]
	fn over<T, S>(&mut self, inputs: S) -> Vec<IndicatorResult>
	where
		T: OHLCV,
		S: AsRef<[T]>,
		Self: Sized,
	{
		let inputs_ref = inputs.as_ref();
		inputs_ref.iter().map(|x| self.next(x)).collect()
	}

	/// Returns count of indicator's raw values and count of indicator's signals.
	///
	/// See more at [`IndicatorConfig`](crate::core::IndicatorConfig::size)
	fn size(&self) -> (u8, u8) {
		self.config().size()
	}

	/// Returns a name of the indicator
	fn name(&self) -> &'static str {
		Self::Config::NAME
	}

	/// Creates a function from `IndicatorInstance`
	fn into_fn<'a, T>(mut self) -> Box<dyn FnMut(&'a T) -> IndicatorResult>
	where
		T: OHLCV,
		Self: Sized + 'static,
	{
		let f = move |x| self.next(x);

		Box::new(f)
	}
}
