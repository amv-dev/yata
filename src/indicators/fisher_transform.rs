#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, MovingAverageConstructor, PeriodType, Source, ValueType, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
use crate::methods::{Cross, Highest, Lowest};

// FT = 1/2 * ln((1+x)/(1-x)) = arctanh(x)
// x - transformation of price to a level between -1 and 1 for N periods

/// Fisher transform
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Fisher_transformation>
/// * <https://www.investopedia.com/terms/f/fisher-transform.asp>
///
/// # 2 values
///
/// * FT `main value`
///
/// Range in \(`-inf`; `+inf`\).
///
/// * `signal value` line
///
/// Range in \(`-inf`; `+inf`\).
///
/// # 2 signals
///
/// * Signal 1 appears when `main value` crosses zero line.
/// When `main value` changes direction, returns signal corresponds to relative position of `main value` in `zone`
/// * Signal 2 appears when `main value` crosses `signal line` and after signal 1 appears
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FisherTransform<M: MovingAverageConstructor = MA> {
	/// Main period for max/min values calculation. Default is `9`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period1: PeriodType,
	/// Zone size for signals. Default is `1.5`.
	///
	/// Range in \(`0.0`; `+inf`\)
	pub zone: ValueType,
	/// Signal line moving average type.
	///
	/// Default is [`SMA(2)`](crate::methods::SMA).
	///
	/// Period range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub signal: M,
	/// Source type of values. Default is [`TP`](crate::core::Source::TP)
	pub source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for FisherTransform<M> {
	type Instance = FisherTransformInstance<M>;

	const NAME: &'static str = "FisherTransform";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = &candle.source(cfg.source);

		Ok(Self::Instance {
			ma1: cfg.signal.init(0.)?, // method(cfg.method, cfg.period2, 0.)?,
			highest: Highest::new(cfg.period1, src)?,
			lowest: Lowest::new(cfg.period1, src)?,
			cross: Cross::default(),
			cross_ma: Cross::default(),
			prev_value: 0.,
			last_reverse: 0,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period1 > 1 && self.signal.ma_period() > 1 && self.zone > 0.
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"signal" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.signal = value,
			},
			"zone" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
			},
			"source" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(2, 2)
	}
}

impl Default for FisherTransform<MA> {
	fn default() -> Self {
		Self {
			period1: 9,
			signal: MA::SMA(2),
			zone: 1.5,
			source: Source::TP,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FisherTransformInstance<M: MovingAverageConstructor = MA> {
	cfg: FisherTransform<M>,

	ma1: M::Instance,
	highest: Highest,
	lowest: Lowest,
	cross: Cross,
	cross_ma: Cross,
	prev_value: ValueType,
	last_reverse: i8,
}

const BOUND: ValueType = 0.999;

#[inline]
fn bound_value(value: ValueType) -> ValueType {
	value.min(BOUND).max(-BOUND)
}

impl<M: MovingAverageConstructor> IndicatorInstance for FisherTransformInstance<M> {
	type Config = FisherTransform<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = &candle.source(self.cfg.source);

		// first we need to find MAX and MIN values for last `period1` prices
		let highest = self.highest.next(src);
		let lowest = self.lowest.next(src);

		// we need to check division by zero, so we can really just check if `h` is equal to `l` without using any kind of round error checks
		let fisher_transform = if highest.to_bits() == lowest.to_bits() {
			0.
		} else {
			// converting `SRC` into a value in range [-1; 1]
			let x = bound_value((src - lowest) / (highest - lowest) * 2. - 1.);
			// calculating fisher transform value
			x.atanh()
		};

		let cumulative = self.prev_value.mul_add(0.5, fisher_transform);

		// We’ll take trade signals based on the following rules:
		// Long trades

		// 	Fisher Transform must be negative (i.e., the more negative the indicator is, the more “stretched” or excessively bearish price is)
		// 	Taken after a reversal of the Fisher Transform from negatively sloped to positively sloped (i.e., rate of change from negative to positive)

		// Short trades

		// 	Fisher Transform must be positive (i.e., price perceived to be excessively bullish)
		// 	Taken after a reversal in the direction of the Fisher Transform
		let reverse = self.cross.next(&(cumulative, self.prev_value)).analog();

		let s1 = cumulative / self.cfg.zone
			* ((cumulative < 0.0 && reverse > 0) || (cumulative > 0.0 && reverse < 0)) as i8
				as ValueType;

		// The Fisher Transform frequently has a signal line attached to it. This is a moving average of the Fisher Transform value,
		// so it moves slightly slower than the Fisher Transform line. When the Fisher Transform crosses the trigger line it is used
		// by some traders as a trade signal. For example, when the Fisher Transform drops below the signal line after hitting an
		// extreme high, that could be used as a signal to sell a current long position.
		let signal_line = self.ma1.next(&cumulative);
		let crossed_ma = self.cross_ma.next(&(cumulative, signal_line)).analog();

		let is_reversed = (reverse != 0) as i8;
		self.last_reverse = (1 - is_reversed) * self.last_reverse + is_reversed * reverse;

		let s2 = signal_line / self.cfg.zone
			* ((signal_line < 0.0 && self.last_reverse > 0 && crossed_ma > 0)
				|| (signal_line > 0.0 && self.last_reverse < 0 && crossed_ma < 0)) as i8 as ValueType;

		self.prev_value = cumulative;

		IndicatorResult::new(&[cumulative, signal_line], &[s1.into(), s2.into()])
	}
}
