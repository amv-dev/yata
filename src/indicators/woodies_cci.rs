#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::commodity_channel_index::CommodityChannelIndexInstance;
use super::CommodityChannelIndex;
use crate::core::{Action, Error, Method, PeriodType, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::signi;
use crate::methods::{Cross, CrossAbove, CrossUnder, SMA};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WoodiesCCI {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub signal1_period: PeriodType,
	pub signal2_bars_count: PeriodType,
	pub signal3_zone: ValueType,
}

impl IndicatorConfig for WoodiesCCI {
	const NAME: &'static str = "WoodiesCCI";

	fn validate(&self) -> bool {
		self.period1 > self.period2
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"period1" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period1 = value,
			},
			"period2" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period2 = value,
			},
			"signal1_period" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.signal1_period = value,
			},
			"signal1_bars_count" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.signal2_bars_count = value,
			},
			"signal3_zone" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.signal3_zone = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(2, 3)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for WoodiesCCI {
	type Instance = WoodiesCCIInstance;

	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;

		let mut cci1 = CommodityChannelIndex::default();
		cci1.period = cfg.period1;
		let mut cci2 = CommodityChannelIndex::default();
		cci2.period = cfg.period2;

		Ok(Self::Instance {
			cci1: cci1.init(candle)?,
			cci2: cci2.init(candle)?,
			sma: SMA::new(cfg.signal1_period, 0.)?,
			cross1: Cross::default(),
			cross2: Cross::default(),
			s2_sum: 0,
			s3_sum: 0.,
			s3_count: 0,
			window: Window::new(cfg.signal2_bars_count, 0),
			cross_above: CrossAbove::default(),
			cross_under: CrossUnder::default(),
			cfg,
		})
	}
}

impl Default for WoodiesCCI {
	fn default() -> Self {
		Self {
			period1: 14,
			period2: 6,
			signal1_period: 9,
			signal2_bars_count: 6,
			signal3_zone: 0.2,
		}
	}
}

#[derive(Debug)]
pub struct WoodiesCCIInstance {
	cfg: WoodiesCCI,

	cci1: CommodityChannelIndexInstance,
	cci2: CommodityChannelIndexInstance,
	sma: SMA,
	cross1: Cross,
	cross2: Cross,
	s2_sum: isize,
	s3_sum: ValueType,
	s3_count: PeriodType,
	window: Window<i8>,
	cross_above: CrossAbove,
	cross_under: CrossUnder,
}

impl<T: OHLC> IndicatorInstance<T> for WoodiesCCIInstance {
	type Config = WoodiesCCI;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let cci1 = self.cci1.next(candle).value(0);
		let cci2 = self.cci2.next(candle).value(0);

		let cci1_sign = signi(cci1);
		let d_cci = cci1 - cci2;
		let sma = self.sma.next(d_cci);
		let s1 = self.cross1.next((sma, 0.));

		let s0 = self.cross2.next((cci1, 0.));
		self.s2_sum += (cci1_sign - self.window.push(cci1_sign)) as isize;

		// let s2;
		// if self.s2_sum >= self.cfg.signal2_bars_count {
		// 	s2 = 1;
		// } else if self.s2_sum <= -self.cfg.signal2_bars_count {
		// 	s2 = -1;
		// } else {
		// 	s2 = 0;
		// }
		let s2 = (self.s2_sum >= isize::from(self.cfg.signal2_bars_count)) as i8
			- (self.s2_sum <= -isize::from(self.cfg.signal2_bars_count)) as i8;

		// if s0.is_some() {
		// 	self.s3_sum = 0.;
		// 	self.s3_count = 0;
		// }

		let is_none = s0.is_none();
		self.s3_sum *= is_none as i8 as ValueType;
		self.s3_count *= is_none as PeriodType;

		self.s3_sum += cci1;
		self.s3_count += 1;

		let s3v = self.s3_sum / self.s3_count as ValueType;
		let s3 = self.cross_above.next((s3v, self.cfg.signal3_zone))
			- self.cross_under.next((s3v, -self.cfg.signal3_zone));

		IndicatorResult::new(&[cci1, cci2], &[s1, Action::from(s2), s3])
	}
}
