#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use std::str::FromStr;

use crate::core::{Error, Sequence, ValueType, OHLC, OHLCV};

/// Source enum represents common parts of a *Candle*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Source {
	/// *Close* part of a candle
	Close,

	/// *Open* part of a candle
	Open,

	/// *High* part of a candle
	High,

	/// *Low* part of a candle
	Low,

	/// (*High*+*Low*)/2 part of a candle
	HL2,

	/// Typical price of a candle
	TP,

	/// *Volume* part of a candle
	Volume,

	/// Same as `typical price * volume`
	VolumedPrice,
}

impl FromStr for Source {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().trim() {
			"close" => Ok(Self::Close),
			"high" => Ok(Self::High),
			"low" => Ok(Self::Low),
			"volume" => Ok(Self::Volume),
			"tp" => Ok(Self::TP),
			"hl2" => Ok(Self::HL2),
			"open" => Ok(Self::Open),
			"volumed_price" => Ok(Self::VolumedPrice),

			value => Err(Error::SourceParse(value.to_string())),
		}
	}
}

impl TryFrom<&str> for Source {
	type Error = Error;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Self::from_str(s)
	}
}

impl TryFrom<String> for Source {
	type Error = Error;

	fn try_from(s: String) -> Result<Self, Self::Error> {
		Self::from_str(s.as_str())
	}
}

/// Simple Candlestick structure for implementing [OHLC] and [OHLCV]
///
/// Can be also used by an alias [Candlestick]
///
/// You may convert simple tuples of 4 or 5 float values into Candle:
/// ```
/// use yata::prelude::Candle;
/// //               open  high  low  close
/// let my_candle = (3.0,  5.0,  2.0, 4.0  );
/// let converted: Candle = my_candle.into();
/// println!("{:?}", converted);
/// ```
///
/// ```
/// use yata::prelude::Candle;
/// //               open  high  low  close  volume
/// let my_candle = (3.0,  5.0,  2.0, 4.0  ,  50.0 );
/// let converted: Candle = my_candle.into();
/// println!("{:?}", converted);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Candle {
	/// *Open* value of the candle
	pub open: ValueType,

	/// *High* value of the candle
	pub high: ValueType,

	/// *Low* value of the candle
	pub low: ValueType,

	/// *Close* value of the candle
	pub close: ValueType,

	/// *Volume* value of the candle
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

impl From<(ValueType, ValueType, ValueType, ValueType)> for Candle {
	fn from(value: (ValueType, ValueType, ValueType, ValueType)) -> Self {
		Self {
			open: value.0,
			high: value.1,
			low: value.2,
			close: value.3,
			volume: 0.0,
		}
	}
}

impl From<(ValueType, ValueType, ValueType, ValueType, ValueType)> for Candle {
	fn from(value: (ValueType, ValueType, ValueType, ValueType, ValueType)) -> Self {
		Self {
			open: value.0,
			high: value.1,
			low: value.2,
			close: value.3,
			volume: value.4,
		}
	}
}

impl PartialEq for Candle {
	fn eq(&self, other: &Self) -> bool {
		self.open.to_bits() == other.open.to_bits()
			&& self.high.to_bits() == other.high.to_bits()
			&& self.low.to_bits() == other.low.to_bits()
			&& self.close.to_bits() == other.close.to_bits()
			&& self.volume.to_bits() == other.volume.to_bits()
	}
}

impl Eq for Candle {}

/// Just an alias for the Sequence of any `T`
pub type Candles<T> = Sequence<T>;
