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
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_precision_loss)]
#![allow(renamed_and_removed_lints)] // workaround clippy unknown lints when rust stable 1.50. May be removed in the future
#![allow(clippy::unknown_clippy_lints)] // workaround clippy unknown lints when rust stable 1.50. May be removed in the future
#![allow(unknown_lints)] // workaround clippy unknown lints when rust stable 1.50. May be removed in the future
#![allow(clippy::upper_case_acronyms)]
#![deny(clippy::nursery)]
#![allow(clippy::use_self)]
#![cfg_attr(feature = "period_type_u64", allow(clippy::cast_possible_truncation))]

//! Yet Another Technical Analysis library
//!
//! `YaTa` implements most common technical analysis [methods] and [indicators]
//!
//! It also provides you an interface to create your own indicators.
//!
//! ## Available **moving averages**:
//!
//! - [Simple moving average (SMA)](crate::methods::SMA);
//! - [Weighted moving average (WMA)](crate::methods::WMA);
//! - Exponential moving average family: [EMA](crate::methods::EMA), [DMA](crate::methods::DMA), [TMA](crate::methods::TMA),
//! [DEMA](crate::methods::DEMA), [TEMA](crate::methods::TEMA);
//! - [Simple moving median (SMM)](crate::methods::SMM);
//! - [Linear regression moving average (LSMA)](crate::methods::LinReg);
//! - [Volume weighted moving average (VWMA)](crate::methods::VWMA);
//! - [Symmetrically weighted moving average (SWMA)](crate::methods::SWMA);
//! - [Hull moving average (HMA)](crate::methods::HMA);
//! - [Running Moving Average (RMA)](crate::methods::RMA);
//! - [Triangular Moving Average (TRIMA)](crate::methods::TRIMA);
//! - [Wilder’s Smoothing Average (WSMA)](crate::methods::WSMA);
//! - [Kaufman Adaptive Moving Average (KAMA)](crate::indicators::Kaufman);
//! - [Convolution Moving Average](crate::methods::Conv);
//! - [Variable Index Dynamic Average (Vidya)](crate::methods::Vidya);
//!
//! [See all](crate::methods#structs)
//!
//! ## Timeseries conversion
//!
//! - [Timeframe Collapsing](crate::methods::CollapseTimeframe);
//! - [Heikin Ashi](crate::methods::HeikinAshi);
//! - [Renko](crate::methods::Renko);
//!
//! ## Some commonly used **methods**:
//!
//! - [Accumulation-distribution index](crate::methods::ADI);
//! - [Commodity channel index](crate::methods::CCI);
//! - [`Cross`](crate::methods::Cross) / [`CrossAbove`](crate::methods::CrossAbove) / [`CrossUnder`](crate::methods::CrossUnder);
//! - [Derivative](crate::methods::Derivative) (differential);
//! - [Highest](crate::methods::Highest) / [Lowest](crate::methods::Lowest) / [Highest-Lowest Delta](crate::methods::HighestLowestDelta);
//! - [Highest Index](crate::methods::HighestIndex) / [Lowest Index](crate::methods::LowestIndex);
//! - [Integral](crate::methods::Integral) (sum);
//! - [Mean absolute deviation](crate::methods::MeanAbsDev);
//! - [Median absolute deviation](crate::methods::MedianAbsDev);
//! - [Momentum](crate::methods::Momentum);
//! - [Past](crate::methods::Past);
//! - [Rate Of Change](crate::methods::RateOfChange) (ROC);
//! - [Reversal points](crate::methods::ReversalSignal);
//! - [Standard Deviation](crate::methods::StDev);
//! - [True Range](crate::methods::TR);
//! - [True Strength Index](crate::methods::TSI);
//! - [Volatility](crate::methods::LinearVolatility);
//!
//! [See all](crate::methods#structs)
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
//! let mut ema = EMA::new(3, &3.0).unwrap();
//!
//! ema.next(&3.0);
//! ema.next(&6.0);
//!
//! assert_eq!(ema.next(&9.0), 6.75);
//! assert_eq!(ema.next(&12.0), 9.375);
//! ```
//!
//! ## Indicator usage example
//!
//! ```
//! use yata::helpers::{RandomCandles, MA};
//! use yata::indicators::MACD;
//! use yata::prelude::*;
//!
//! let mut candles = RandomCandles::new();
//! let mut macd = MACD::default();
//!
//! macd.ma1 = "sma-4".parse().unwrap(); // one way of defining methods inside indicators
//!
//! macd.signal = MA::TEMA(5); // another way of defining methods inside indicators
//!
//! let mut macd = macd.init(&candles.first()).unwrap();
//!
//! for candle in candles.take(10) {
//!     let result = macd.next(&candle);
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! ## Current usafe status
//!
//! By default, there is no `unsafe` code in the crate. But you can optionally enable `unsafe_performance` feature throw you `Cargo.toml` or by `--feature` flag in your CLI.
//!
//! `usafe_performance` enables some unsafe code blocks, most of them are unsafe access to a vector's elements. For some methods it may increase performance by ~5-10%.
//!
//! ## Suggestions
//!
//! You are welcome to give any suggestions about new indicators and methods
//!
//! # Say thanks
//!
//! If you like this library and you want to say thanks, you can do it also by donating to bitcoin address `1P3gTnaTK9LKSYx2nETrKe2zjP4HMkdhvK`

pub mod core;
pub mod helpers;
pub mod indicators;
pub mod methods;

/// Contains main traits you need to start using this library
pub mod prelude {
	pub use super::core::{
		Candle, Error, IndicatorConfig, IndicatorInstance, Method, Sequence, OHLCV,
	};

	pub use super::helpers::{Buffered, Peekable};

	/// Dynamically dispatchable traits for indicators creation
	pub mod dd {
		pub use crate::core::{IndicatorConfigDyn, IndicatorInstanceDyn};
	}
}
