use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, MovingAverage, MovingAverageConstructor, PeriodType, ValueType};
use crate::methods::{
	LinReg, Vidya, DEMA, DMA, EMA, HMA, RMA, SMA, SMM, SWMA, TEMA, TMA, TRIMA, WMA, WSMA,
};

/// Default moving average constructor
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

/// Default moving average instance for constructor
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
pub enum MAInstance {
	/// [Simple Moving Average](crate::methods::SMA)
	SMA(SMA),

	/// [Weighed Moving Average](crate::methods::WMA)
	WMA(WMA),

	/// [Hull Moving Average](crate::methods::HMA)
	HMA(HMA),

	/// [Running Moving Average](crate::methods::RMA)
	RMA(RMA),

	/// [Exponential Moving Average](crate::methods::EMA)
	EMA(EMA),

	/// [Double Exponential Moving Average](crate::methods::DMA)
	DMA(DMA),

	/// Another type of [Double Exponential Moving Average](crate::methods::DEMA)
	DEMA(DEMA),

	/// [Triple Exponential Moving Average](crate::methods::TMA)
	TMA(TMA),

	/// Another type of [Triple Exponential Moving Average](crate::methods::DEMA)
	TEMA(TEMA),

	/// [Wilder's smoothing average](crate::methods::WSMA)
	WSMA(WSMA),

	/// [Simple Moving Median](crate::methods::SMM)
	SMM(SMM),

	/// [Symmetrically Weighted Moving Average](crate::methods::SWMA)
	SWMA(SWMA),

	/// [Triangular Moving Average](crate::methods::TRIMA)
	TRIMA(TRIMA),

	/// [Linear regression](crate::methods::LinReg)
	#[cfg_attr(feature = "serde", serde(rename = "lin_reg"))]
	LinReg(LinReg),

	/// [Variable Index Dynamic Average](crate::methods::Vidya)
	Vidya(Vidya),
}

impl Method for MAInstance {
	type Input = ValueType;
	type Output = ValueType;
	type Params = std::convert::Infallible;

	fn new(_: Self::Params, _: &Self::Input) -> Result<Self, Error>
	where
		Self: Sized,
	{
		Err(Error::Other("`MAInstance` cannot be constructed directly. You should use `MA::init` to instantiate it.".into()))
	}

	#[inline]
	fn next(&mut self, value: &Self::Input) -> Self::Output {
		match self {
			Self::SMA(i) => i.next(value),
			Self::WMA(i) => i.next(value),
			Self::HMA(i) => i.next(value),
			Self::RMA(i) => i.next(value),
			Self::EMA(i) => i.next(value),
			Self::DMA(i) => i.next(value),
			Self::DEMA(i) => i.next(value),
			Self::TMA(i) => i.next(value),
			Self::TEMA(i) => i.next(value),
			Self::WSMA(i) => i.next(value),
			Self::SMM(i) => i.next(value),
			Self::SWMA(i) => i.next(value),
			Self::TRIMA(i) => i.next(value),
			Self::LinReg(i) => i.next(value),
			Self::Vidya(i) => i.next(value),
		}
	}
}

impl MovingAverage for MAInstance {}

impl MovingAverageConstructor for MA {
	type Type = u8;
	type Instance = MAInstance;

	fn init(&self, value: ValueType) -> Result<Self::Instance, Error> {
		match *self {
			Self::SMA(length) => {
				let instance = SMA::new(length, &value)?;
				Ok(Self::Instance::SMA(instance))
			}
			Self::WMA(length) => {
				let instance = WMA::new(length, &value)?;
				Ok(Self::Instance::WMA(instance))
			}
			Self::HMA(length) => {
				let instance = HMA::new(length, &value)?;
				Ok(Self::Instance::HMA(instance))
			}
			Self::RMA(length) => {
				let instance = RMA::new(length, &value)?;
				Ok(Self::Instance::RMA(instance))
			}
			Self::EMA(length) => {
				let instance = EMA::new(length, &value)?;
				Ok(Self::Instance::EMA(instance))
			}
			Self::DMA(length) => {
				let instance = DMA::new(length, &value)?;
				Ok(Self::Instance::DMA(instance))
			}
			Self::DEMA(length) => {
				let instance = DEMA::new(length, &value)?;
				Ok(Self::Instance::DEMA(instance))
			}
			Self::TMA(length) => {
				let instance = TMA::new(length, &value)?;
				Ok(Self::Instance::TMA(instance))
			}
			Self::TEMA(length) => {
				let instance = TEMA::new(length, &value)?;
				Ok(Self::Instance::TEMA(instance))
			}
			Self::WSMA(length) => {
				let instance = WSMA::new(length, &value)?;
				Ok(Self::Instance::WSMA(instance))
			}
			Self::SMM(length) => {
				let instance = SMM::new(length, &value)?;
				Ok(Self::Instance::SMM(instance))
			}
			Self::SWMA(length) => {
				let instance = SWMA::new(length, &value)?;
				Ok(Self::Instance::SWMA(instance))
			}
			Self::TRIMA(length) => {
				let instance = TRIMA::new(length, &value)?;
				Ok(Self::Instance::TRIMA(instance))
			}
			Self::LinReg(length) => {
				let instance = LinReg::new(length, &value)?;
				Ok(Self::Instance::LinReg(instance))
			}
			Self::Vidya(length) => {
				let instance = Vidya::new(length, &value)?;
				Ok(Self::Instance::Vidya(instance))
			}
		}
	}

	#[allow(clippy::unnested_or_patterns)]
	fn ma_period(&self) -> PeriodType {
		match self {
			Self::SMA(length)
			| Self::WMA(length)
			| Self::HMA(length)
			| Self::RMA(length)
			| Self::EMA(length)
			| Self::DMA(length)
			| Self::TMA(length)
			| Self::DEMA(length)
			| Self::TEMA(length)
			| Self::WSMA(length)
			| Self::SMM(length)
			| Self::SWMA(length)
			| Self::TRIMA(length)
			| Self::LinReg(length)
			| Self::Vidya(length) => *length,
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
		let (method, period) = s.split_once('-').ok_or(Error::MovingAverageParse)?;

		let length: PeriodType = period.parse().or(Err(Error::MovingAverageParse))?;

		match method {
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
