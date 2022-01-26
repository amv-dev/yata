#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use std::str::FromStr;

use crate::core::{Error, ValueType, OHLCV};
use crate::helpers::Merge;

/// Source enum represents common parts of a *Candle*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
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

	/// [Typical price](https://en.wikipedia.org/wiki/Typical_price) of a candle
	TP,

	/// *Volume* part of a candle
	Volume,

	/// Same as `typical price * volume`
	#[cfg_attr(feature = "serde", serde(rename = "volumed_price"))]
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
			"tp" | "hlc3" => Ok(Self::TP),
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

impl From<Source> for &'static str {
	fn from(value: Source) -> Self {
		match value {
			Source::Close => "close",
			Source::High => "high",
			Source::Low => "low",
			Source::Open => "open",
			Source::TP => "tp",
			Source::HL2 => "hl2",
			Source::Volume => "volume",
			Source::VolumedPrice => "volumed_price",
		}
	}
}

impl From<Source> for String {
	fn from(value: Source) -> Self {
		let s: &str = value.into();
		s.to_string()
	}
}

/// Simple Candlestick structure for implementing [`OHLCV`]
///
/// Can be also used by an alias [`Candlestick`]
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

impl Candle {
	/// Creates `Candle` from any another `OHLCV`-object.
	pub fn from<T: OHLCV + ?Sized>(src: &T) -> Self {
		Self {
			open: src.open(),
			high: src.high(),
			low: src.low(),
			close: src.close(),
			volume: src.volume(),
		}
	}
}

/// Just an alias for [Candle]
pub type Candlestick = Candle;

impl OHLCV for Candle {
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

	#[inline]
	fn volume(&self) -> ValueType {
		self.volume
	}
}

impl Merge<Candle> for Candle {
	fn merge(&self, other: &Candle) -> Candle {
		Candle {
			high: self.high.max(other.high()),
			low: self.low.min(other.low()),
			close: other.close(),
			volume: self.volume + other.volume(),
			..*self
		}
	}
}

impl From<&dyn OHLCV> for Candle {
	fn from(src: &dyn OHLCV) -> Self {
		Self::from(src)
	}
}

impl From<(ValueType, ValueType, ValueType, ValueType)> for Candle {
	fn from(value: (ValueType, ValueType, ValueType, ValueType)) -> Self {
		Self {
			open: value.0,
			high: value.1,
			low: value.2,
			close: value.3,
			volume: ValueType::NAN,
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

#[cfg(test)]
mod tests {
	use super::Source;

	#[test]
	fn test_source_to_string_str() {
		let values = [
			Source::Open,
			Source::High,
			Source::Low,
			Source::Close,
			Source::Volume,
			Source::VolumedPrice,
			Source::TP,
			Source::HL2,
		];

		for &v in &values {
			let r1: String = v.into();
			let r2: &str = v.into();

			assert_eq!(r1, r2);

			match v {
				Source::Open => assert_eq!("open", r1),
				Source::High => assert_eq!("high", r1),
				Source::Low => assert_eq!("low", r1),
				Source::Close => assert_eq!("close", r1),
				Source::Volume => assert_eq!("volume", r1),
				Source::VolumedPrice => assert_eq!("volumed_price", r1),
				Source::TP => assert_eq!("tp", r1),
				Source::HL2 => assert_eq!("hl2", r1),
			}
		}
	}

	#[test]
	fn test_source_from_string() {
		let values = [
			"oPeN",
			"HIGH",
			"low",
			"cLose",
			"volume",
			"vOluMeD_prIcE",
			"tP",
			"hlc3",
			"Hl2",
		];

		values.iter().enumerate().for_each(|(i, s)| {
			let r: Source = s.parse().unwrap();
			match i {
				0 => assert_eq!(Source::Open, r),
				1 => assert_eq!(Source::High, r),
				2 => assert_eq!(Source::Low, r),
				3 => assert_eq!(Source::Close, r),
				4 => assert_eq!(Source::Volume, r),
				5 => assert_eq!(Source::VolumedPrice, r),
				6 | 7 => assert_eq!(Source::TP, r),
				8 => assert_eq!(Source::HL2, r),
				_ => panic!("Wow. You cannot be here."),
			}
		});

		let src: Result<Source, _> = "some other string".parse();

		assert!(src.is_err());
	}
}
