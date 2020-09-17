#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::str::FromStr;

use crate::core::{Sequence, ValueType, OHLC, OHLCV};

/// Source enum represents common parts of a *Candle*
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Source {
	/// *Close* part of a candle
	#[cfg_attr(feature = "serde", serde(rename = "close"))]
	Close,

	/// *Open* part of a candle
	#[cfg_attr(feature = "serde", serde(rename = "open"))]
	Open,

	/// *High* part of a candle
	#[cfg_attr(feature = "serde", serde(rename = "high"))]
	High,

	/// *Low* part of a candle
	#[cfg_attr(feature = "serde", serde(rename = "low"))]
	Low,

	/// (*High*+*Low*)/2 part of a candle
	#[cfg_attr(feature = "serde", serde(rename = "hl2"))]
	HL2,

	/// Typical price of a candle
	#[cfg_attr(feature = "serde", serde(rename = "tp"))]
	TP,

	/// *Volume* part of a candle
	#[cfg_attr(feature = "serde", serde(rename = "volume"))]
	Volume,
}

impl FromStr for Source {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().trim() {
			"close" => Ok(Self::Close),
			"high" => Ok(Self::High),
			"low" => Ok(Self::Low),
			"volume" => Ok(Self::Volume),
			"tp" => Ok(Self::TP),
			"hl2" => Ok(Self::HL2),
			"open" => Ok(Self::Open),

			_ => Err(format!("Unknown source {}", s)),
		}
	}
}

impl From<&str> for Source {
	fn from(s: &str) -> Self {
		Self::from_str(s).unwrap()
	}
}

impl From<String> for Source {
	fn from(s: String) -> Self {
		Self::from_str(s.as_str()).unwrap()
	}
}

/// Simple Candlestick structure for implementing [OHLC] and [OHLCV]
///
/// Can be also used by an alias [Candlestick]
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Candle {
	/// *Open* value of the candle
	#[cfg_attr(feature = "serde", serde(rename = "open"))]
	pub open: ValueType,

	/// *High* value of the candle
	#[cfg_attr(feature = "serde", serde(rename = "high"))]
	pub high: ValueType,

	/// *Low* value of the candle
	#[cfg_attr(feature = "serde", serde(rename = "low"))]
	pub low: ValueType,

	/// *Close* value of the candle
	#[cfg_attr(feature = "serde", serde(rename = "close"))]
	pub close: ValueType,

	/// *Volume* value of the candle
	#[cfg_attr(feature = "serde", serde(rename = "volume"))]
	pub volume: ValueType,
}

/// Just an alias for [Candle]
pub type Candlestick = Candle;

impl OHLC for Candle {
	#[inline]
	fn open(&self) -> ValueType {
		self.open
	}

	#[inline]
	fn high(&self) -> ValueType {
		self.high
	}

	#[inline]
	fn low(&self) -> ValueType {
		self.low
	}

	#[inline]
	fn close(&self) -> ValueType {
		self.close
	}
}

impl OHLCV for Candle {
	#[inline]
	fn volume(&self) -> ValueType {
		self.volume
	}
}

/// Just an alias for the Sequence of any `T`
pub type Candles<T> = Sequence<T>;
