#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, PeriodType, ValueType};
use crate::methods::{
	Derivative, Highest, HighestLowestDelta, Integral, LinReg, Lowest, MeanAbsDev, MedianAbsDev,
	Momentum, Past, RateOfChange, StDev, CCI, DEMA, DMA, EMA, HMA, RMA, SMA, SMM, SWMA, TEMA, TMA,
	TRIMA, WMA, WSMA,
};

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
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum RegularMethods {
	/// [Simple Moving Average](crate::methods::SMA)
	SMA,

	/// [Weighed Moving Average](crate::methods::WMA)
	WMA,

	/// [Hull Moving Average](crate::methods::HMA)
	HMA,

	/// [Running Moving Average](crate::methods::RMA)
	RMA,

	/// [Exponential Moving Average](crate::methods::EMA)
	EMA,

	/// [Double Exponential Moving Average](crate::methods::DMA)
	DMA,

	/// Another type of [Double Exponential Moving Average](crate::methods::DEMA)
	DEMA,

	/// [Triple Exponential Moving Average](crate::methods::TMA)
	TMA,

	/// Another type of [Triple Exponential Moving Average](crate::methods::DEMA)
	TEMA,

	/// [Wilder's smoothing average](crate::methods::WSMA)
	WSMA,

	/// [Simple Moving Median](crate::methods::SMM)
	SMM,

	/// [Symmetrically Weighted Moving Average](crate::methods::SWMA)
	SWMA,

	/// [Triangular Moving Average](crate::methods::TRIMA)
	TRIMA,

	/// [Linear regression](crate::methods::LinReg)
	#[cfg_attr(feature = "serde", serde(rename = "lin_reg"))]
	LinReg,

	/// [Past](crate::methods::Past) moves timeseries forward
	Past,

	/// Just an alias for `Past`
	Move,

	/// [Derivative](crate::methods::Derivative)
	Derivative,

	/// [Integral](crate::methods::Integral)
	Integral,

	/// [Mean Absolute Deviation](crate::methods::MeanAbsDev)
	#[cfg_attr(feature = "serde", serde(rename = "mean_abs_dev"))]
	MeanAbsDev,

	/// [Median Absolute Deviation](crate::methods::MedianAbsDev)
	#[cfg_attr(feature = "serde", serde(rename = "median_abs_dev"))]
	MedianAbsDev,

	/// [Standard Deviation](crate::methods::StDev)
	#[cfg_attr(feature = "serde", serde(rename = "st_dev"))]
	StDev,

	/// [Commodity channel index](crate::methods::CCI)
	CCI,

	/// [Momentum](crate::methods::Momentum)
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
	Highest,

	/// [Lowest](crate::methods::Lowest)
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
			"wsma" => Ok(Self::WSMA),
			"smm" => Ok(Self::SMM),
			"swma" => Ok(Self::SWMA),
			"trima" => Ok(Self::TRIMA),
			"lin_reg" | "linreg" => Ok(Self::LinReg),

			"past" | "move" => Ok(Self::Past),
			"derivative" => Ok(Self::Derivative),
			"integral" => Ok(Self::Integral),
			"mean_abs_dev" => Ok(Self::MeanAbsDev),
			"median_abs_dev" => Ok(Self::MedianAbsDev),
			"st_dev" | "stdev" => Ok(Self::StDev),
			"cci" => Ok(Self::CCI),
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

/// Returns a heap-allocated [`RegularMethod`] for timeseries by given `name` and window `length`.
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
/// * `wsma` - [Wilder's smoothing average](WSMA)
/// * `smm` - [simple moving median](SMM)
/// * `swma` - [symmetrically weighted moving average](SWMA)
/// * `lin_reg` - [linear regression moving average](LinReg)
/// * `trima` - [triangular moving average](TRIMA)
/// * `past`, `move` - [moves timeseries forward](Past)
/// * `derivative` - [derivative](Derivative)
/// * `mean_abs_dev` - [mead absolute deviation](MeanAbsDev)
/// * `median_abs_dev` - [median absolute deviation](MedianAbsDev)
/// * `st_dev` - [standart deviation](StDev)
/// * `cci` - [Commodity channel index](CCI)
/// * `momentum`, `change` - [absolute change of values](Momentum)
/// * `rate_of_change` - [relative change of values](RateOfChange)
/// * [`highest`](Highest), [`lowest`](Lowest), [`highest_lowest_delta`](HighestLowestDelta)
///
/// # Examples
///
/// ```
/// use yata::helpers::{method, RegularMethods};
///
/// let mut m = method(RegularMethods::SMA, 3, 1.0).unwrap();
///
/// m.next(&1.0);
/// m.next(&2.0);
///
/// assert_eq!(m.next(&3.0), 2.0);
/// assert_eq!(m.next(&4.0), 3.0);
/// ```
///
/// # See also
///
/// [Default regular methods list](RegularMethods)

pub fn method(
	method: RegularMethods,
	length: PeriodType,
	initial_value: ValueType,
) -> Result<RegularMethod, Error> {
	match method {
		RegularMethods::SMA => Ok(Box::new(SMA::new(length, &initial_value)?)),
		RegularMethods::WMA => Ok(Box::new(WMA::new(length, &initial_value)?)),
		RegularMethods::HMA => Ok(Box::new(HMA::new(length, &initial_value)?)),
		RegularMethods::RMA => Ok(Box::new(RMA::new(length, &initial_value)?)),
		RegularMethods::EMA => Ok(Box::new(EMA::new(length, &initial_value)?)),
		RegularMethods::DMA => Ok(Box::new(DMA::new(length, &initial_value)?)),
		RegularMethods::DEMA => Ok(Box::new(DEMA::new(length, &initial_value)?)),
		RegularMethods::TMA => Ok(Box::new(TMA::new(length, &initial_value)?)),
		RegularMethods::TEMA => Ok(Box::new(TEMA::new(length, &initial_value)?)),
		RegularMethods::WSMA => Ok(Box::new(WSMA::new(length, &initial_value)?)),
		RegularMethods::SMM => Ok(Box::new(SMM::new(length, &initial_value)?)),
		RegularMethods::SWMA => Ok(Box::new(SWMA::new(length, &initial_value)?)),
		RegularMethods::LinReg => Ok(Box::new(LinReg::new(length, &initial_value)?)),
		RegularMethods::TRIMA => Ok(Box::new(TRIMA::new(length, &initial_value)?)),

		RegularMethods::Past | RegularMethods::Move => {
			Ok(Box::new(Past::new(length, &initial_value)?))
		}
		RegularMethods::Derivative => Ok(Box::new(Derivative::new(length, &initial_value)?)),
		RegularMethods::Integral => Ok(Box::new(Integral::new(length, &initial_value)?)),
		RegularMethods::MeanAbsDev => Ok(Box::new(MeanAbsDev::new(length, &initial_value)?)),
		RegularMethods::MedianAbsDev => Ok(Box::new(MedianAbsDev::new(length, &initial_value)?)),
		RegularMethods::StDev => Ok(Box::new(StDev::new(length, &initial_value)?)),
		RegularMethods::CCI => Ok(Box::new(CCI::new(length, &initial_value)?)),
		RegularMethods::Momentum | RegularMethods::Change => {
			Ok(Box::new(Momentum::new(length, &initial_value)?))
		}
		RegularMethods::RateOfChange | RegularMethods::ROC => {
			Ok(Box::new(RateOfChange::new(length, &initial_value)?))
		}
		RegularMethods::Highest => Ok(Box::new(Highest::new(length, &initial_value)?)),
		RegularMethods::Lowest => Ok(Box::new(Lowest::new(length, &initial_value)?)),
		RegularMethods::HighestLowestDelta => {
			Ok(Box::new(HighestLowestDelta::new(length, &initial_value)?))
		}
	}
}
