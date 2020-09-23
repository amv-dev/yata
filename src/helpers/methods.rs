#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Method, PeriodType, ValueType};
use crate::methods::*;

use std::convert::TryFrom;
use std::str::FromStr;
/// A shortcut for dynamically (runtime) generated regular methods
///
/// Regular method is a method which has parameters of single [`PeriodType`], input is single [`ValueType`] and output is single [`ValueType`].
///
/// # See also
///
/// [Default regular methods list](RegularMethods)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
pub type RegularMethod =
	Box<dyn Method<Params = PeriodType, Input = ValueType, Output = ValueType>>;

/// Regular methods dictionary
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RegularMethods {
	/// [Simple Moving Average](crate::methods::SMA)
	#[cfg_attr(feature = "serde", serde(rename = "sma"))]
	SMA,

	/// [Weighed Moving Average](crate::methods::WMA)
	#[cfg_attr(feature = "serde", serde(rename = "wma"))]
	WMA,

	/// [Hull Moving Average](crate::methods::HMA)
	#[cfg_attr(feature = "serde", serde(rename = "hma"))]
	HMA,

	/// [Running Moving Average](crate::methods::RMA)
	#[cfg_attr(feature = "serde", serde(rename = "rma"))]
	RMA,

	/// [Exponential Moving Average](crate::methods::EMA)
	#[cfg_attr(feature = "serde", serde(rename = "ema"))]
	EMA,

	/// [Double Exponential Moving Average](crate::methods::DMA)
	#[cfg_attr(feature = "serde", serde(rename = "dma"))]
	DMA,

	/// Another type of [Double Exponential Moving Average](crate::methods::DEMA)
	#[cfg_attr(feature = "serde", serde(rename = "dema"))]
	DEMA,

	/// [Triple Exponential Moving Average](crate::methods::TMA)
	#[cfg_attr(feature = "serde", serde(rename = "tma"))]
	TMA,

	/// Another type of [Triple Exponential Moving Average](crate::methods::DEMA)
	#[cfg_attr(feature = "serde", serde(rename = "tema"))]
	TEMA,

	/// [Simle Moving Median](crate::methods::SMM)
	#[cfg_attr(feature = "serde", serde(rename = "smm"))]
	SMM,

	/// [Symmetrically Weighted Moving Average](crate::methods::SWMA)
	#[cfg_attr(feature = "serde", serde(rename = "swma"))]
	SWMA,

	/// [Triangular Moving Average](crate::methods::TRIMA)
	#[cfg_attr(feature = "serde", serde(rename = "trima"))]
	TRIMA,

	/// [Linear regression](crate::methods::LinReg)
	#[cfg_attr(feature = "serde", serde(rename = "lin_reg"))]
	LinReg,

	/// [Past](crate::methods::Past) moves timeseries forward
	#[cfg_attr(feature = "serde", serde(rename = "move"))]
	Past,

	/// Just an alias for `Past`
	#[cfg_attr(feature = "serde", serde(rename = "move"))]
	Move,

	/// [Derivative](crate::methods::Derivative)
	#[cfg_attr(feature = "serde", serde(rename = "derivative"))]
	Derivative,

	/// [Integral](crate::methods::Integral)
	#[cfg_attr(feature = "serde", serde(rename = "integral"))]
	Integral,

	/// [Standart deviation](crate::methods::StDev)
	#[cfg_attr(feature = "serde", serde(rename = "st_dev"))]
	StDev,

	/// [Momentum](crate::methods::Momentum)
	#[cfg_attr(feature = "serde", serde(rename = "momentum"))]
	Momentum,

	/// [Change](crate::methods::Change)
	#[cfg_attr(feature = "serde", serde(rename = "momentum"))]
	Change,

	/// [Rate Of Change](crate::methods::RateOfChange)
	#[cfg_attr(feature = "serde", serde(rename = "rate_of_change"))]
	RateOfChange,

	/// Just an alias for [Rate Of Change](crate::methods::RateOfChange)
	#[cfg_attr(feature = "serde", serde(rename = "rate_of_change"))]
	ROC,

	/// [Highest](crate::methods::Highest)
	#[cfg_attr(feature = "serde", serde(rename = "highest"))]
	Highest,

	/// [Lowest](crate::methods::Lowest)
	#[cfg_attr(feature = "serde", serde(rename = "lowest"))]
	Lowest,

	/// [HighestLowestDelta](crate::methods::HighestLowestDelta)
	#[cfg_attr(feature = "serde", serde(rename = "highest_lowest_delta"))]
	HighestLowestDelta,
}

impl FromStr for RegularMethods {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().trim() {
			"sma" => Ok(Self::SMA),
			"wma" => Ok(Self::WMA),
			"hma" => Ok(Self::HMA),
			"rma" => Ok(Self::RMA),
			"ema" => Ok(Self::EMA),
			"dma" => Ok(Self::DMA),
			"dema" => Ok(Self::DEMA),
			"tma" => Ok(Self::TMA),
			"tema" => Ok(Self::TEMA),
			"smm" => Ok(Self::SMM),
			"swma" => Ok(Self::SWMA),
			"trima" => Ok(Self::TRIMA),
			"lin_reg" | "linreg" => Ok(Self::LinReg),

			"past" | "move" => Ok(Self::Past),
			"derivative" => Ok(Self::Derivative),
			"integral" => Ok(Self::Integral),
			"st_dev" | "stdev" => Ok(Self::StDev),
			"momentum" | "change" => Ok(Self::Momentum),
			"rate_of_change" | "rateofchange" | "roc" => Ok(Self::RateOfChange),
			"highest" => Ok(Self::Highest),
			"lowest" => Ok(Self::Lowest),
			"highest_lowest_delta" => Ok(Self::HighestLowestDelta),

			_ => Err(format!("Unknown regular method name {}", s)),
		}
	}
}

impl TryFrom<&str> for RegularMethods {
	type Error = String;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Self::from_str(s)
	}
}

impl TryFrom<String> for RegularMethods {
	type Error = String;

	fn try_from(s: String) -> Result<Self, Self::Error> {
		Self::from_str(s.as_str())
	}
}

/// Returns a heap-allocated [RegularMethod] for timeseries by given `name` and window `length`.
/// These methods are always gets an input value of type f64 and the same output value type
///
/// Available methods:
/// * `sma` - [simple moving average](SMA)
/// * `wma` - [weighed moving average](WMA)
/// * `hma` - [hull moving average](HMA)
/// * `ema` - [exponential moving average](EMA)
/// * `rma` - [running moving average](RMA)
/// * `dma` - [double exponential moving average](DMA)
/// * `dema` - [another double exponential moving average](DEMA)
/// * `tma` - [triple exponential moving average](TMA)
/// * `tema` - [another triple exponential moving average](TEMA)
/// * `smm` - [simple moving median](SMM)
/// * `swma` - [symmetrically weighted moving average](SWMA)
/// * `lin_reg` - [linear regression moving average](LinReg)
/// * `trima` - [triangular moving average](TRIMA)
/// * `past`, `move` - [moves timeseries forward](Past)
/// * `derivative` - [derivative](Derivative)
/// * `st_dev` - [standart deviation](StDev)
/// * `momentum`, `change` - [absolute change of values](Momentum)
/// * `rate_of_change` - [relative change of values](RateOfChange)
/// * [`highest`](Highest), [`lowest`](Lowest), [`highest_lowest_delta`](HighestLowestDelta)
///
/// # Examples
///
/// ```
/// use yata::helpers::{method, RegularMethods};
///
/// let mut m = method(RegularMethods::SMA, 3, 1.0);
///
/// m.next(1.0);
/// m.next(2.0);
///
/// assert_eq!(m.next(3.0), 2.0);
/// assert_eq!(m.next(4.0), 3.0);
/// ```
///
/// ```
/// use yata::core::Sequence;
/// use yata::helpers::{method, RegularMethods};
/// use std::convert::TryInto;
///
/// let mut s:Sequence<_> = Sequence::from(vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]);
/// let mut ma = method("sma".try_into().unwrap(), 2, s[0]);
///
/// s.apply(ma.as_mut());
/// assert_eq!(s.as_slice(), &[1., 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
/// ```
///
/// ```
/// use yata::prelude::*;
/// use yata::core::{Sequence, ValueType};
/// use yata::methods::WMA;
/// use yata::helpers::method;
/// use std::convert::TryInto;
///
/// let my_method = String::from("wma");
/// let mut s:Sequence<_> = Sequence::from(vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]);
/// let mut wma1 = method(my_method.try_into().unwrap(), 4, s[0]);
/// let mut wma2 = WMA::new(4, s[0]);
///
/// let s1:Vec<ValueType> = s.iter().map(|&x| wma1.next(x)).collect();
/// let s2:Vec<ValueType> = wma2.iter_data(s.iter().copied()).collect();
/// assert_eq!(s1.as_slice(), s2.as_slice());
/// ```
///
/// # See also
///
/// [Default regular methods list](RegularMethods)

pub fn method(
	method: RegularMethods,
	length: PeriodType,
	initial_value: ValueType,
) -> RegularMethod {
	match method {
		RegularMethods::SMA => Box::new(SMA::new(length, initial_value)),
		RegularMethods::WMA => Box::new(WMA::new(length, initial_value)),
		RegularMethods::HMA => Box::new(HMA::new(length, initial_value)),
		RegularMethods::RMA => Box::new(RMA::new(length, initial_value)),
		RegularMethods::EMA => Box::new(EMA::new(length, initial_value)),
		RegularMethods::DMA => Box::new(DMA::new(length, initial_value)),
		RegularMethods::DEMA => Box::new(DEMA::new(length, initial_value)),
		RegularMethods::TMA => Box::new(TMA::new(length, initial_value)),
		RegularMethods::TEMA => Box::new(TEMA::new(length, initial_value)),
		RegularMethods::SMM => Box::new(SMM::new(length, initial_value)),
		RegularMethods::SWMA => Box::new(SWMA::new(length, initial_value)),
		RegularMethods::LinReg => Box::new(LinReg::new(length, initial_value)),
		RegularMethods::TRIMA => Box::new(TRIMA::new(length, initial_value)),

		RegularMethods::Past | RegularMethods::Move => Box::new(Past::new(length, initial_value)),
		RegularMethods::Derivative => Box::new(Derivative::new(length, initial_value)),
		RegularMethods::Integral => Box::new(Integral::new(length, initial_value)),
		RegularMethods::StDev => Box::new(StDev::new(length, initial_value)),
		RegularMethods::Momentum | RegularMethods::Change => {
			Box::new(Momentum::new(length, initial_value))
		}
		RegularMethods::RateOfChange | RegularMethods::ROC => {
			Box::new(RateOfChange::new(length, initial_value))
		}
		RegularMethods::Highest => Box::new(Highest::new(length, initial_value)),
		RegularMethods::Lowest => Box::new(Lowest::new(length, initial_value)),
		RegularMethods::HighestLowestDelta => {
			Box::new(HighestLowestDelta::new(length, initial_value))
		}
	}
}
