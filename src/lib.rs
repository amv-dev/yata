#![warn(
	missing_docs,
	missing_debug_implementations,
	missing_copy_implementations,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications
)]

//! Yet Another Technical Analysis library
//!
//! YaTA implements most common technical analysis [methods](crate::methods) and [indicators](crate::indicators)
//!
//! It also provides you an iterface to create your own indicators.
//!
//! Some commonly used methods:
//! * [ADI](crate::methods::ADI) Accumulation-distribution index;
//! * [Cross](crate::methods::Cross) / [CrossAbove](crate::methods::CrossAbove) / [CrossUnder](crate::methods::CrossUnder);
//! * [Derivative](crate::methods::Derivative) (differential);
//! * [Highest](crate::methods::Highest) / [Lowest](crate::methods::Lowest) / [Highest - Lowest Delta](crate::methods::HighestLowestDelta);
//! * [HMA](crate::methods::HMA) Hull moving average;
//! * [Integral](crate::methods::Integral) (sum);
//! * [LinReg](crate::methods::LinReg) Linear regression moving average;
//! * [Momentum](crate::methods::Momentum);
//! * [Pivot points](crate::methods::PivotSignal);
//! * [SMA](crate::methods::SMA) Simple moving average;
//! * [WMA](crate::methods::WMA) Weighted moving average;
//! * [VWMA](crate::methods::VWMA) Volume weighted moving average;
//! * [EMA](crate::methods::EMA), [DMA](crate::methods::DMA), [TMA](crate::methods::TMA), [DEMA](crate::methods::DEMA), [TEMA](crate::methods::TEMA) Exponential moving average family;
//! * [SWMA](crate::methods::SWMA) Symmetrically weighted moving average.
//!
//! And many others: [See Full list](crate::methods#structs)
//!
//! # Current usafe status
//! Currently there is no `unsafe` code in the crate.

pub mod core;
pub mod helpers;
pub mod indicators;
pub mod methods;

/// Contains main traits you need to start using this library
pub mod prelude {
	pub use super::core::{
		Candle, IndicatorConfig, IndicatorInitializer, IndicatorInstance, Method, OHLC, OHLCV,
	};
}
