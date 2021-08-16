#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Error, Method, MovingAverageConstructor, OHLCV, PeriodType, ValueType};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::MA;
use crate::methods::{Cross, SMA, SWMA};

/// Relative Vigor Index
///
/// ## Links:
///
/// * <https://www.investopedia.com/terms/r/relative_vigor_index.asp>
///
/// # 2 values
///
/// * `main` value
///
/// Range in \[`-0.5`; `0.5`\]
///
/// * `signal line` value
///
/// Range in \[`-0.5`; `0.5`\]
///
/// # 2 signals
///
/// * Signal #1 on `main` value crosses `signal line` value.
///
/// When main value crosses signal line upwards, returns full buy signal.
/// When main value crosses signal line downwards, returns full sell signal.
/// Otherwise returns no signal.
///
/// * Signal #2 on `main` value crosses `signal line` value outside safe zone.
///
/// When main value is below `-zone` and crosses signal line upwards, returns full buy signal.
/// When main value is above `+zone` and crosses signal line downwards, returns full sell signal.
/// Otherwise returns no signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelativeVigorIndex<M: MovingAverageConstructor = MA> {
	/// Summarize period. Default is `10`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period1: PeriodType,

	/// SWMA period. Default is `4`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period2: PeriodType,
	/*
	/// Signal line MA period. Default is `4`.
	///
	/// Range in \[`2`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub period3: PeriodType,

	/// Signal line MA method. Default is [`SWMA`](crate::methods::SWMA).
	pub method: RegularMethods,
	*/
	pub signal: M,
	/// Signal zone filter. Default is `0.25`.
	///
	/// Range in \[`0.0`; `0.5`\)
	pub zone: ValueType,
}

impl<M: MovingAverageConstructor> IndicatorConfig for RelativeVigorIndex<M> {
	type Instance = RelativeVigorIndexInstance<M>;

	const NAME: &'static str = "RelativeVigorIndex";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let d_close = &0.0; // candle.close() - candle.open();
		let d_hl = &(candle.high() - candle.low());
		let rvi = 0.0; // if d_hl == 0. { 0. } else { d_close / d_hl };

		Ok(Self::Instance {
			prev_close: candle.open(),
			swma1: SWMA::new(cfg.period2, d_close)?,
			sma1: SMA::new(cfg.period1, d_close)?,
			swma2: SWMA::new(cfg.period2, d_hl)?,
			sma2: SMA::new(cfg.period1, d_hl)?,
			ma: cfg.signal.init(rvi)?, // method(cfg.method, cfg.period3, rvi)?,
			cross: Cross::default(),
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.period1 >= 2
			&& self.zone >= 0.
			&& self.zone < 0.5
			&& self.period2 > 1
			&& self.signal.ma_period() > 1
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
			"zone" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.zone = value,
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

impl Default for RelativeVigorIndex {
	fn default() -> Self {
		Self {
			period1: 10,
			period2: 4,
			signal: MA::SWMA(4),
			// period3: 4,
			// method: RegularMethods::SWMA,
			zone: 0.25,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelativeVigorIndexInstance<M: MovingAverageConstructor = MA> {
	cfg: RelativeVigorIndex<M>,

	prev_close: ValueType,
	swma1: SWMA,
	sma1: SMA,
	swma2: SWMA,
	sma2: SMA,
	ma: M::Instance,
	cross: Cross,
}

impl<M: MovingAverageConstructor> IndicatorInstance for RelativeVigorIndexInstance<M> {
	type Config = RelativeVigorIndex<M>;

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	#[allow(clippy::similar_names)]
	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let close_open = candle.close() - self.prev_close;
		let high_low = candle.high() - candle.low();

		self.prev_close = candle.close();

		let swma1 = self.swma1.next(&close_open);
		let sma1 = self.sma1.next(&swma1);
		let swma2 = self.swma2.next(&high_low);
		let sma2 = self.sma2.next(&swma2);

		let rvi = if sma2 == 0. { 0. } else { sma1 / sma2 };
		let sig: ValueType = self.ma.next(&rvi);

		let s1 = self.cross.next(&(rvi, sig)).analog();

		// if s1.sign().unwrap_or_default() < 0 && rvi > self.cfg.zone && sig > self.cfg.zone {
		// 	s2 = 1;
		// } else if s1.sign().unwrap_or_default() > 0 && rvi < -self.cfg.zone && sig < -self.cfg.zone
		// {
		// 	s2 = -1;
		// } else {
		// 	s2 = 0;
		// }

		let s2 = (s1 < 0 && rvi > self.cfg.zone && sig > self.cfg.zone) as i8
			- (s1 > 0 && rvi < -self.cfg.zone && sig < -self.cfg.zone) as i8;

		IndicatorResult::new(&[rvi, sig], &[s1.into(), s2.into()])
	}
}
