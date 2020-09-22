use super::IndicatorInstance;
use crate::core::OHLC;

/// Each indicator has it's own **Configuration** with parameters
///
/// Each that config should implement `IndicatorConfig` trait
///
/// See example with [`Example Indicator`](crate::indicators::example)
// Config cannot be Copy because it might consist ov Vec-s. F.e. if indicator using Conv method with custom weights.
pub trait IndicatorConfig: Clone {
	/// Validates if **Configuration** is OK
	fn validate(&self) -> bool;

	/// Sets dynamically **Configuration** parameters
	fn set(&mut self, name: &str, value: String);

	/// Should return `true` if indicator uses *volume* data
	fn is_volume_based(&self) -> bool {
		false
	}

	/// Returns a name of the indicator
	fn name(&self) -> &'static str {
		let parts = std::any::type_name::<Self>().split("::");
		parts.last().unwrap_or_default()
	}

	/// Returns an [IndicatorResult](crate::core::IndicatorResult) size processing by the indicator `(count of raw value, count of signals)`
	fn size(&self) -> (u8, u8);
}

/// To initialize an indicator's **State** indicator should implement `IndicatorInitializer`
pub trait IndicatorInitializer<T: OHLC> {
	/// Type of **State**
	type Instance: IndicatorInstance<T>;

	/// Initializes the **State** based on current **Configuration**
	fn init(self, initial_value: T) -> Self::Instance;
}

// pub trait IndicatorConfigDyn<T: OHLC + 'static>: IndicatorConfig<T> {
// 	fn validate(&self) -> bool;

// 	fn set(&mut self, name: &str, value: String);

// 	fn init(&self, initial_value: T) -> Box<dyn IndicatorInstanceDyn<T>>;
// }
