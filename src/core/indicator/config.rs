use super::{IndicatorInstance, IndicatorResult};
use crate::core::{Error, OHLCV};

/// Each indicator has it's own **Configuration** with parameters
///
/// Each that config should implement `IndicatorConfig` trait
///
/// See example with [`Example Indicator`](crate::indicators::example)
// Config cannot be Copy because it might consist ov Vec-s. F.e. if indicator using Conv method with custom weights.
pub trait IndicatorConfig: Clone {
	/// Type of **State**
	type Instance: IndicatorInstance<Config = Self>;

	/// Name of an indicator
	const NAME: &'static str;

	/// Validates if **Configuration** is OK
	fn validate(&self) -> bool;

	/// Dynamically sets **Configuration** parameters
	fn set(&mut self, name: &str, value: String) -> Result<(), Error>;

	/// Returns a name of the indicator
	fn name(&self) -> &'static str {
		Self::NAME
	}

	/// Returns an [`IndicatorResult`](crate::core::IndicatorResult) size processing by the indicator `(count of raw values, count of signals)`
	fn size(&self) -> (u8, u8);

	/// Initializes the **State** based on current **Configuration**
	fn init<T: OHLCV>(self, initial_value: &T) -> Result<Self::Instance, Error>;

	/// Creates an `IndicatorInstance` function from this `IndicatorConfig`.
	fn init_fn<'a, T: OHLCV>(
		self,
		initial_value: &'a T,
	) -> Result<Box<dyn FnMut(&'a T) -> IndicatorResult>, Error>
	where
		Self: 'static,
	{
		let instance = self.init(initial_value)?;

		Ok(instance.into_fn())
	}

	/// Evaluates indicator config over sequence of OHLC and returns sequence of `IndicatorResult`s
	/// ```
	/// use yata::prelude::*;
	/// use yata::helpers::{RandomCandles};
	/// use yata::indicators::Trix;
	///
	/// let candles: Vec<_> = RandomCandles::new().take(10).collect();
	/// let trix = Trix::default();
	/// let results = trix.over(&candles).unwrap();
	/// println!("{:?}", results);
	/// ```
	fn over<T, S>(self, inputs: S) -> Result<Vec<IndicatorResult>, Error>
	where
		T: OHLCV,
		S: AsRef<[T]>,
		Self: Sized,
	{
		let inputs_ref = inputs.as_ref();

		if inputs_ref.is_empty() {
			return Ok(Vec::new());
		}

		let mut state = self.init(&inputs_ref[0])?;

		Ok(IndicatorInstance::over(&mut state, inputs))
	}
}
