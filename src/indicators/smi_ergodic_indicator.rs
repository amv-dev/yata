#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, MovingAverageConstructor, OHLCV, PeriodType, Source, ValueType};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
use crate::methods::{Cross, TSI};

/// SMI Ergodic Indicator
///
/// ## Links
///
/// * <http://www.motivewave.com/studies/smi_ergodic_indicator.htm>
/// * <https://en.wikipedia.org/wiki/Ergodic_theory>
///
/// # 3 value
///
/// * `SMI` main value
///
/// Range in \[`-1.0`; `1.0`\]
///
/// * `Signal line` value
///
/// Range in \[`-1.0`; `1.0`\]
///
/// * `Oscillator` value
///
/// Range in \[`-2.0`; `2.0`\]
///
/// # 1 signals
///
/// * Signal #1 on `SMI` crosses `Signal`
///
/// When `Signal line` value is below `-zone` and `SMI` value crosses `Signal line` upwards, returns full buy signal.
/// When `Signal line` value is above `+zone` and `SMI` value crosses `Signal line` downwards, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SMIErgodicIndicator<M: MovingAverageConstructor = MA> {
	/// Long TSI period. Default is `20`.
	///
	/// Range in \[`period2`, [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period1: PeriodType,

	/// Short TSI period. Default is `5`.
	///
	/// Range in \(`2`, `period1`\].
	pub period2: PeriodType,
	/*
	/// Signal line MA period. Default is `5`.
	///
	/// Range in \[`2`, [`PeriodType::MAX`](crate::core::PeriodType)\).
	pub period3: PeriodType,

	/// Signal line MA method. Default is [`EMA`](crate::methods::EMA).
	pub method: RegularMethods,
	*/
	pub signal: M,
	/// Signal zone size. Default is `0.2`.
	///
	/// Range in \[`0.0`; `1.0`]
	pub zone: ValueType,

	/// Source type of values. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl<M: MovingAverageConstructor> IndicatorConfig for SMIErgodicIndicator<M> {
	type Instance = SMIErgodicIndicatorInstance<M>;

	const NAME: &'static str = "SMIErgodicIndicator";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let src = candle.source(cfg.source);

		Ok(Self::Instance {
			tsi: TSI::new(cfg.period2, cfg.period1, src)?,
			ma: cfg.signal.init(0.)?, // method(cfg.method, cfg.period3, 0.)?,
			cross: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period2 > 1
			&& self.period2 <= self.period1
			&& self.period1 < PeriodType::MAX
			&& self.signal.ma_period() > 1
			&& self.signal.ma_period() < PeriodType::MAX
			&& self.zone >= 0.
			&& self.zone <= 1.
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"signal" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.signal = value,
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
		(3, 1)
	}
}

impl Default for SMIErgodicIndicator {
	fn default() -> Self {
		Self {
			period1: 20,
			period2: 5,
			signal: MA::EMA(5),
			// period3: 5,
			// method: RegularMethods::EMA,
			zone: 0.2,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct SMIErgodicIndicatorInstance<M: MovingAverageConstructor = MA> {
	cfg: SMIErgodicIndicator<M>,

	tsi: TSI,
	ma: M::Instance,
	cross: Cross,
}

impl<M: MovingAverageConstructor> IndicatorInstance for SMIErgodicIndicatorInstance<M> {
	type Config = SMIErgodicIndicator<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);
		let tsi = self.tsi.next(&src);

		let sig: ValueType = self.ma.next(&tsi);

		let cross = self.cross.next(&(tsi, sig)).analog();
		let s1 =
			(cross > 0 && sig < -self.cfg.zone) as i8 - (cross < 0 && sig > self.cfg.zone) as i8;

		IndicatorResult::new(&[tsi, sig, tsi - sig], &[s1.into()])
	}
}
