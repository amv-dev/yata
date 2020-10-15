#![warn(missing_docs, missing_debug_implementations)]

//! Commonly used methods for manipulating timeseries.
//! Every method implements [`Method`](crate::core::Method) trait.
//!
//! To create a method instance use [`Method::new`](crate::core::Method::new).
//! To get new output value over the input value use [`Method::next`](crate::core::Method::next).
//!
//! ```
//! // creating Weighted Moving Average of length 5
//! use yata::prelude::*;
//! use yata::methods::WMA;
//!
//! let mut wma = WMA::new(5, 20.0).unwrap();
//!
//! let input_value = 34.51;
//! let output_value = wma.next(input_value);
//! ```
//!
//! # Examples
//!
//! ```
//! use yata::prelude::*;
//! use yata::methods::SMA;
//!
//! let mut sma = SMA::new(3, 5.0).unwrap();
//! sma.next(5.0);
//! sma.next(4.0);
//! assert_eq!(sma.next(6.0), 5.0);
//! assert_eq!(sma.next(2.0), 4.0);
//! assert_eq!(sma.next(-2.0), 2.0);
//! ```

mod sma;
pub use sma::*;
mod wma;
pub use wma::*;
mod ema;
pub use ema::*;
mod wsma;
pub use wsma::*;
mod rma;
pub use rma::*;
mod smm;
pub use smm::*;
mod hma;
pub use hma::*;
mod lin_reg;
pub use lin_reg::*;
mod swma;
pub use swma::*;
mod conv;
pub use conv::*;
mod vwma;
pub use vwma::*;
mod trima;
pub use trima::*;
//
mod derivative;
pub use derivative::*;
mod integral;
pub use integral::*;
mod momentum;
pub use momentum::*;
mod rate_of_change;
pub use rate_of_change::*;
mod st_dev;
pub use st_dev::*;
mod volatility;
pub use volatility::*;
mod cci;
pub use cci::*;
mod mean_abs_dev;
pub use mean_abs_dev::*;
mod median_abs_dev;
pub use median_abs_dev::*;

mod cross;
pub use cross::*;
mod reverse;
pub use reverse::*;
mod highest_lowest;
pub use highest_lowest::*;
mod adi;
mod highest_lowest_index;
pub use adi::*;
pub use highest_lowest_index::*;
mod past;
pub use past::*;

#[cfg(test)]
mod tests {
	use crate::core::{Method, ValueType};
	use std::fmt::Debug;

	pub(super) fn test_const<P, I: Copy, O: Copy + Debug + PartialEq>(
		method: &mut dyn Method<Params = P, Input = I, Output = O>,
		input: I,
		output: O,
	) {
		for _ in 0..100 {
			assert_eq!(method.next(input), output);
		}
	}

	#[cfg(feature = "value_type_f32")]
	const SIGMA: ValueType = 1e-2;
	#[cfg(not(feature = "value_type_f32"))]
	const SIGMA: ValueType = 1e-7;

	pub(super) fn test_const_float<P, I: Copy>(
		method: &mut dyn Method<Params = P, Input = I, Output = ValueType>,
		input: I,
		output: ValueType,
	) {
		for _ in 0..100 {
			assert!((method.next(input) - output).abs() < SIGMA);
		}
	}
}
