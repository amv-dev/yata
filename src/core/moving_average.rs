use std::str::FromStr;

use super::{Error, Method, PeriodType, ValueType};

/// Marker trait for any moving average
///
/// Moving average is a [`Method`] which has parameters of single [`PeriodType`], input is single [`ValueType`] and output is single [`ValueType`].
///
/// # See also
///
/// [Default moving averages methods list](crate::helpers::MA)
///
/// [`Method`]: crate::core::Method
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
pub trait MovingAverage: Method<Input = ValueType, Output = ValueType> {}

/// Trait for dynamically creation of moving average instances based on it's type and period
///
/// This trait plays the same role for moving averages as [`IndicatorConfig`] plays for indicators.
///
/// [`IndicatorConfig`]: crate::core::IndicatorConfig
pub trait MovingAverageConstructor: Send + Clone + FromStr {
	/// Used for comparing MA types
	type Type: Eq;

	/// `MovingAverage` Instance type
	type Instance: MovingAverage;

	/// Creates moving average instance with the `initial_value`
	fn init(&self, initial_value: ValueType) -> Result<Self::Instance, Error>;

	/// Returns period length of
	fn ma_period(&self) -> PeriodType;

	/// Returns moving average type
	fn ma_type(&self) -> Self::Type;

	/// Checks two moving average constructors for the same moving averagee type
	fn is_similar_to(&self, other: &Self) -> bool {
		self.ma_type() == other.ma_type()
	}
}
