#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unsafe_derive_deserialize)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::shadow_unrelated)]
#![allow(clippy::copy_iterator)]
#![deny(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::suboptimal_flops)]
/*

Copyright 2020 AMvDev (amv-dev@protonmail.com)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/
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
//! `YaTa` implements most common technical analysis [methods](crate::methods) and [indicators](crate::indicators)
//!
//! It also provides you an interface to create your own indicators.
//!
//! ## Some commonly used methods:
//!
//! * [ADI](crate::methods::ADI) Accumulation-distribution index;
//! * [Cross](crate::methods::Cross) / [CrossAbove](crate::methods::CrossAbove) / [CrossUnder](crate::methods::CrossUnder);
//! * [Derivative](crate::methods::Derivative) (differential);
//! * [Highest](crate::methods::Highest) / [Lowest](crate::methods::Lowest) / [Highest - Lowest Delta](crate::methods::HighestLowestDelta);
//! * [HMA](crate::methods::HMA) Hull moving average;
//! * [Integral](crate::methods::Integral) (sum);
//! * [LinReg](crate::methods::LinReg) Linear regression moving average;
//! * [Momentum](crate::methods::Momentum);
//! * [Reverse points](crate::methods::ReverseSignal);
//! * [SMA](crate::methods::SMA) Simple moving average;
//! * [WMA](crate::methods::WMA) Weighted moving average;
//! * [VWMA](crate::methods::VWMA) Volume weighted moving average;
//! * [EMA](crate::methods::EMA), [DMA](crate::methods::DMA), [TMA](crate::methods::TMA), [DEMA](crate::methods::DEMA), [TEMA](crate::methods::TEMA) Exponential moving average family;
//! * [SWMA](crate::methods::SWMA) Symmetrically weighted moving average.
//!
//! And many others: [See Full list](crate::methods#structs)
//!
//! ## Some commonly used **indicators**:
//!
//! - Average Directional Index;
//! - Awesome Oscillator;
//! - Bollinger Bands;
//! - Commodity Channel Index;
//! - Detrended Price Oscillator;
//! - Ease Of Movement;
//! - Elders Force Index;
//! - Envelopes;
//! - Fisher Transform;
//! - Ichimoku Cloud;
//! - Keltner Channels;
//! - Moving Average Convergence Divergence (MACD);
//! - Money Flow Index;
//! - Price Channel Strategy;
//! - Relative Strength Index (RSI);
//! - Stochastic Oscillator;
//! - Trix;
//! - Woodies CCI;
//!
//! And many others: [See Full list](crate::indicators#structs)
//!
//! ## Method usage example
//!
//! ```
//! use yata::prelude::*;
//! use yata::methods::EMA;
//!
//! // EMA of length=3
//! let mut ema = EMA::new(3, 3.0).unwrap();
//!
//! ema.next(3.0);
//! ema.next(6.0);
//!
//! assert_eq!(ema.next(9.0), 6.75);
//! assert_eq!(ema.next(12.0), 9.375);
//! ```
//!
//! ## Indicator usage example
//!
//! ```
//! use yata::helpers::{RandomCandles, RegularMethods};
//! use yata::indicators::MACD;
//! use yata::prelude::*;
//! use std::convert::TryInto;
//!
//! let mut candles = RandomCandles::new();
//! let mut macd = MACD::default();
//!
//! macd.period3 = 4; // setting signal period MA to 4
//!
//! macd.method1 = "sma".try_into().unwrap(); // one way of defining methods inside indicators
//!
//! macd.method3 = RegularMethods::TEMA; // another way of defining methods inside indicators
//!
//! let mut macd = macd.init(candles.first()).unwrap();
//!
//! for candle in candles.take(10) {
//!     let result = macd.next(candle);
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! ## Current usafe status
//!
//! Currently there is no `unsafe` code in the crate.
//!
//! ## Suggestions
//!
//! You are welcome to give any suggestions about new indicators and methods
//!
//! # Say thanks
//!
//! If you like this library and you want to say thanks, you can do it also by donating to bitcoin address _1P3gTnaTK9LKSYx2nETrKe2zjP4HMkdhvK_

pub mod core;
pub mod helpers;
pub mod indicators;
pub mod methods;

/// Contains main traits you need to start using this library
pub mod prelude {
	pub use super::core::{
		Candle, Error, IndicatorConfig, IndicatorInitializer, IndicatorInstance, Method, OHLC,
		OHLCV,
	};
}
