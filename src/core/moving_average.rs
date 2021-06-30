use std::str::FromStr;

use super::{Error, Method, PeriodType, ValueType};

/// A shortcut for dynamically generated moving averages
///
/// Moving average is a [`Method`] which has parameters of single [`PeriodType`], input is single [`ValueType`] and output is single [`ValueType`].
///
/// # See also
///
/// [Default regular methods list](RegularMethods)
///
/// [`Method`]: crate::core::Method
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
pub type DynMovingAverage = Box<dyn MovingAverage>;

/// Marker trait for any moving average
pub trait MovingAverage: Method<Params = PeriodType, Input = ValueType, Output = ValueType> {}

/// Trait for dynamically creation of moving average instances based on it's type and period
///
/// This trait plays the same role for moving averages as [`IndicatorConfig`] plays for indicators.
///
/// [`IndicatorConfig`]: crate::core::indicator::IndicatorConfig
pub trait MovingAverageConstructor: Clone + FromStr {
    /// Used for comparing MA types
    type Type: Eq;

    /// Creates moving average instance with the `initial_value`
    fn init(&self, initial_value: ValueType) -> Result<DynMovingAverage, Error>;

    /// Returns period length of 
    fn ma_period(&self) -> PeriodType;

    /// Returns moving average type
    fn ma_type(&self) -> Self::Type;

    /// Checks two moving average constructors for the same moving averagee type
    fn is_similar_to(&self, other: &Self) -> bool {
        self.ma_type() == other.ma_type()
    }
}

// impl<T: MovingAverageConstructor> MovingAverageConstructor for Rc<T> {
//     fn init(&self, value: ValueType) -> Result<DynMovingAverage, Error> {
//         MovingAverageConstructor::init(&**self, value)
//     }

//     fn len(&self) -> PeriodType {
//         MovingAverageConstructor::len(&**self)
//     }
// }

// impl<T: MovingAverageConstructor> MovingAverageConstructor for Arc<T> {
//     fn init(&self, value: ValueType) -> Result<DynMovingAverage, Error> {
//         MovingAverageConstructor::init(&**self, value)
//     }

//     fn len(&self) -> PeriodType {
//         MovingAverageConstructor::len(&**self)
//     }
// }

// impl<T: MovingAverageConstructor> MovingAverageConstructor for Box<T> {
//     fn init(&self, value: ValueType) -> Result<DynMovingAverage, Error> {
//         MovingAverageConstructor::init(&**self, value)
//     }

//     fn len(&self) -> PeriodType {
//         MovingAverageConstructor::len(&**self)
//     }
// }