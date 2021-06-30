use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, DynMovingAverage, MovingAverageConstructor, PeriodType, ValueType};
use crate::methods::{LinReg, Vidya, DEMA, DMA, EMA, HMA, RMA, SMA, SMM, SWMA, TEMA, TMA, TRIMA, WMA, WSMA};

/// Default moving average constructor
#[allow(clippy::pub_enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
pub enum MA {
	/// [Simple Moving Average](crate::methods::SMA)
	SMA(PeriodType),

	/// [Weighed Moving Average](crate::methods::WMA)
	WMA(PeriodType),

	/// [Hull Moving Average](crate::methods::HMA)
	HMA(PeriodType),

	/// [Running Moving Average](crate::methods::RMA)
	RMA(PeriodType),

	/// [Exponential Moving Average](crate::methods::EMA)
	EMA(PeriodType),

	/// [Double Exponential Moving Average](crate::methods::DMA)
	DMA(PeriodType),

	/// Another type of [Double Exponential Moving Average](crate::methods::DEMA)
	DEMA(PeriodType),

	/// [Triple Exponential Moving Average](crate::methods::TMA)
	TMA(PeriodType),

	/// Another type of [Triple Exponential Moving Average](crate::methods::DEMA)
	TEMA(PeriodType),

	/// [Wilder's smoothing average](crate::methods::WSMA)
	WSMA(PeriodType),

	/// [Simple Moving Median](crate::methods::SMM)
	SMM(PeriodType),

	/// [Symmetrically Weighted Moving Average](crate::methods::SWMA)
	SWMA(PeriodType),

	/// [Triangular Moving Average](crate::methods::TRIMA)
	TRIMA(PeriodType),

	/// [Linear regression](crate::methods::LinReg)
	#[cfg_attr(feature = "serde", serde(rename = "lin_reg"))]
	LinReg(PeriodType),

	/// [Variable Index Dynamic Average](crate::methods::Vidya)
	Vidya(PeriodType),
}

impl MovingAverageConstructor for MA {
	type Type = u8;

	fn init(&self, value: ValueType) -> Result<DynMovingAverage, Error> {
		match *self {
			Self::SMA(length) => Ok(Box::new(SMA::new(length, &value)?)),
			Self::WMA(length) => Ok(Box::new(WMA::new(length, &value)?)),
			Self::HMA(length) => Ok(Box::new(HMA::new(length, &value)?)),
			Self::RMA(length) => Ok(Box::new(RMA::new(length, &value)?)),
			Self::EMA(length) => Ok(Box::new(EMA::new(length, &value)?)),
			Self::DMA(length) => Ok(Box::new(DMA::new(length, &value)?)),
			Self::TMA(length) => Ok(Box::new(TMA::new(length, &value)?)),
			Self::DEMA(length) => Ok(Box::new(DEMA::new(length, &value)?)),
			Self::TEMA(length) => Ok(Box::new(TEMA::new(length, &value)?)),
			Self::WSMA(length) => Ok(Box::new(WSMA::new(length, &value)?)),
			Self::SMM(length) => Ok(Box::new(SMM::new(length, &value)?)),
			Self::SWMA(length) => Ok(Box::new(SWMA::new(length, &value)?)),
			Self::TRIMA(length) => Ok(Box::new(TRIMA::new(length, &value)?)),
			Self::LinReg(length) => Ok(Box::new(LinReg::new(length, &value)?)),
			Self::Vidya(length) => Ok(Box::new(Vidya::new(length, &value)?)),
		}
	}

	#[allow(clippy::unnested_or_patterns)]
	fn ma_period(&self) -> PeriodType {
		match self {
			Self::SMA(length)|Self::WMA(length)|Self::HMA(length)|Self::RMA(length)|
			Self::EMA(length)|Self::DMA(length)|Self::TMA(length)|Self::DEMA(length)|
			Self::TEMA(length)|Self::WSMA(length)|Self::SMM(length)|Self::SWMA(length)|
			Self::TRIMA(length)|Self::LinReg(length)|Self::Vidya(length) => *length,
		}
	}

	fn ma_type(&self) -> Self::Type {
		match *self {
			Self::SMA(_) => 0,
			Self::WMA(_) => 1,
			Self::HMA(_) => 2,
			Self::RMA(_) => 3,
			Self::EMA(_) => 4,
			Self::DMA(_) => 5,
			Self::TMA(_) => 6,
			Self::DEMA(_) => 7,
			Self::TEMA(_) => 8,
			Self::WSMA(_) => 9,
			Self::SMM(_) => 10,
			Self::SWMA(_) => 11,
			Self::TRIMA(_) => 12,
			Self::LinReg(_) => 13,
			Self::Vidya(_) => 14,
		}
	}
}

impl FromStr for MA {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts = s.split_once('-').ok_or(Error::MovingAverageParse)?;

		let length: PeriodType = parts.1.parse().or(Err(Error::MovingAverageParse))?;

		match parts.0 {
			"sma" => Ok(Self::SMA(length)),
			"wma" => Ok(Self::WMA(length)),
			"hma" => Ok(Self::HMA(length)),
			"rma" => Ok(Self::RMA(length)),
			"ema" => Ok(Self::EMA(length)),
			"dma" => Ok(Self::DMA(length)),
			"tma" => Ok(Self::TMA(length)),
			"dema" => Ok(Self::DEMA(length)),
			"tema" => Ok(Self::TEMA(length)),
			"wsma" => Ok(Self::WSMA(length)),
			"smm" => Ok(Self::SMM(length)),
			"swma" => Ok(Self::SWMA(length)),
			"trima" => Ok(Self::TRIMA(length)),
			"linreg" => Ok(Self::LinReg(length)),
			"vidya" => Ok(Self::Vidya(length)),
			_ => Err(Error::MovingAverageParse),
		}
	}
}