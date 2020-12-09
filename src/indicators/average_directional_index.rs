#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::HLC;
use crate::core::{Error, PeriodType, ValueType, Window, OHLCV};
use crate::core::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, RegularMethod, RegularMethods};

/// Average Directional Index
///
/// ## Links:
///
/// * <https://school.stockcharts.com/doku.php?id=technical_indicators:average_directional_index_adx>
/// * <https://www.investopedia.com/terms/a/adx.asp>
/// * <https://primexbt.com/blog/average-directional-index/>
///
/// # 3 values
///
/// * `ADX`
///
/// Range in \[`0.0`; `1.0`\]
///
/// * `+DI`
///
/// Range in \[`0.0`; `1.0`\]
///
/// * `-DI`
///
/// Range in \[`0.0`; `1.0`\]
///
/// # 2 signals
///
/// * `BUY_ALL` when `ADX` over `zone` and `+DI` > `-DI`, `SELL_ALL` when `ADX` over `zone` and `-DI` > `+DI`. Otherwise - no signal.
/// * Digital signal by difference between `+DI` and `-DI`
///
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AverageDirectionalIndex {
	/// Default is [`RMA`](crate::methods::RMA)
	pub method1: RegularMethods,
	/// Default is `14`.
	///
	/// Range in \(`period1`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub di_length: PeriodType,

	/// Default is [`RMA`](crate::methods::RMA)
	pub method2: RegularMethods,
	/// Default is `14`
	///
	/// Range in \(`period1`; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub adx_smoothing: PeriodType,

	/// Default is `1`
	///
	/// Range in \[`1`; `min(di_length, adx_smoothing)`\)
	pub period1: PeriodType,
	/// Default is `0.2`
	///
	/// Range in \[`0.0`; `1.0`\]
	pub zone: ValueType,
}

impl IndicatorConfig for AverageDirectionalIndex {
	type Instance = AverageDirectionalIndexInstance;

	const NAME: &'static str = "AverageDirectionalIndex";

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let tr = candle.tr(candle);

		Ok(Self::Instance {
			window: Window::new(cfg.period1, HLC::from(candle)),
			prev_close: candle.close(),
			tr_ma: method(cfg.method1, cfg.di_length, tr)?,
			plus_di: method(cfg.method1, cfg.di_length, 0.0)?,
			minus_di: method(cfg.method1, cfg.di_length, 0.0)?,
			ma2: method(cfg.method2, cfg.adx_smoothing, 0.0)?,
			cfg,
		})
	}

	fn validate(&self) -> bool {
		self.di_length >= 1
			&& self.di_length < PeriodType::MAX
			&& self.adx_smoothing >= 1
			&& self.adx_smoothing < PeriodType::MAX
			&& self.zone >= 0.
			&& self.zone <= 1.
			&& self.period1 >= 1
			&& self.period1 < self.di_length
			&& self.period1 < self.adx_smoothing
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"method1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method1 = value,
			},
			"di_length" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.di_length = value,
			},

			"method2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method2 = value,
			},
			"adx_smoothing" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.adx_smoothing = value,
			},

			"period1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
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
		(3, 2)
	}
}

impl Default for AverageDirectionalIndex {
	fn default() -> Self {
		Self {
			method1: RegularMethods::RMA,
			di_length: 14,
			method2: RegularMethods::RMA,
			adx_smoothing: 14,
			period1: 1,
			zone: 0.2,
		}
	}
}

#[derive(Debug)]
pub struct AverageDirectionalIndexInstance {
	cfg: AverageDirectionalIndex,

	window: Window<HLC>,
	prev_close: ValueType,
	tr_ma: RegularMethod,
	plus_di: RegularMethod,
	minus_di: RegularMethod,
	ma2: RegularMethod,
}

impl AverageDirectionalIndexInstance {
	fn dir_mov(&mut self, candle: HLC) -> (ValueType, ValueType) {
		let prev_candle = self.window.push(candle);
		let true_range = self.tr_ma.next(candle.tr_close(self.prev_close));
		self.prev_close = candle.close();

		let (du, dd) = (
			candle.high() - prev_candle.high(),
			prev_candle.low() - candle.low(),
		);

		let plus_dm = du * (du > dd && du > 0.) as u8 as ValueType; // +DM
		let minus_dm = dd * (dd > du && dd > 0.) as u8 as ValueType; // -DM

		let plus_di_value = self.plus_di.next(plus_dm); // +DI
		let minus_di_value = self.minus_di.next(minus_dm); // -DI

		(plus_di_value / true_range, minus_di_value / true_range)
	}

	fn adx(&mut self, plus: ValueType, minus: ValueType) -> ValueType {
		let s = plus + minus;

		if s == 0. {
			return self.ma2.next(0.);
		}

		let t = (plus - minus).abs() / s;
		self.ma2.next(t)
	}
}

impl IndicatorInstance for AverageDirectionalIndexInstance {
	type Config = AverageDirectionalIndex;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let (plus, minus) = self.dir_mov(HLC::from(candle));
		let adx = self.adx(plus, minus);

		let signal1 = (adx > self.cfg.zone) as i8 * ((plus > minus) as i8 - (plus < minus) as i8);
		let signal2 = plus - minus;

		let values = [adx, plus, minus];

		IndicatorResult::new(&values, &[signal1.into(), signal2.into()])
	}
}
