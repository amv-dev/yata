use super::{IndicatorInstance, IndicatorResult};
use crate::core::{Error, OHLC};

/// Each indicator has it's own **Configuration** with parameters
///
/// Each that config should implement `IndicatorConfig` trait
///
/// See example with [`Example Indicator`](crate::indicators::example)
// Config cannot be Copy because it might consist ov Vec-s. F.e. if indicator using Conv method with custom weights.
pub trait IndicatorConfig: Clone {
	/// Name of an indicator
	const NAME: &'static str;

	/// Validates if **Configuration** is OK
	fn validate(&self) -> bool;

	/// Sets dynamically **Configuration** parameters
	fn set(&mut self, name: &str, value: String) -> Option<Error>;

	/// Should return `true` if indicator uses *volume* data
	fn is_volume_based(&self) -> bool {
		false
	}

	/// Returns a name of the indicator
	fn name(&self) -> &'static str {
		Self::NAME
	}

	/// Returns an [`IndicatorResult`](crate::core::IndicatorResult) size processing by the indicator `(count of raw value, count of signals)`
	fn size(&self) -> (u8, u8);
}

/// To initialize an indicator's **State** indicator should implement `IndicatorInitializer`
pub trait IndicatorInitializer<T: OHLC> {
	/// Type of **State**
	type Instance: IndicatorInstance<T>;

	/// Initializes the **State** based on current **Configuration**
	fn init(self, initial_value: T) -> Result<Self::Instance, Error>;

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
	fn over(self, over_slice: &[T]) -> Result<Vec<IndicatorResult>, Error>
	where
		Self: Sized,
	{
		if over_slice.is_empty() {
			return Ok(Vec::new());
		}

		let mut state = self.init(over_slice[0])?;
		Ok(state.over(over_slice))
	}
}

// pub trait IndicatorConfigDyn<T: OHLC + 'static>: IndicatorConfig<T> {
// 	fn validate(&self) -> bool;

// 	fn set(&mut self, name: &str, value: String);

// 	fn init(&self, initial_value: T) -> Box<dyn IndicatorInstanceDyn<T>>;
// }
