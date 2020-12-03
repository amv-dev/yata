#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Error, Method, PeriodType, Source, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::methods::{Cross, Highest, Lowest};

/// Ichimoku cloud
///
/// ## Links
///
/// * <https://en.wikipedia.org/wiki/Ichimoku_Kink%C5%8D_Hy%C5%8D>
///
/// # 4 values
///
/// * `Tenkan Sen`
/// * `Kijun Sen`
/// * `Senkou Span A`
/// * `Senkou Span B`
///
/// # 2 signals
///
/// * When `Tenkan Sen` crosses `Kijun Sen` upwards and `source` value is greter than both `Senkou Span A and B` and when `Senkou Span A` is greter than `Senkou Span B`,
/// returns full buy signal.
/// When `Tenkan Sen` crosses `Kijun Sen` downwards and `source` value is lower than both `Senkou Span A and B` and when `Senkou Span A` is lower than `Senkou Span B`,
/// returns full sell signal.
/// 
/// * When `source` value crosses `Kijun Sen` upwards and `source` value is greter than both `Senkou Span A and B` and when `Senkou Span A` is greter than `Senkou Span B`,
/// returns full buy signal.
/// When `source` value crosses `Kijun Sen` downwards and `source` value is lower than both `Senkou Span A and B` and when `Senkou Span A` is lower than `Senkou Span B`,
/// returns full sell signal.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IchimokuCloud {
	/// `Tenkan Sen` period. Default is 9.
	/// 
	/// Range in \[1; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub l1: PeriodType,

	/// `Kijun Sen` period. Default is 26.
	/// 
	/// Range in \[1; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub l2: PeriodType,

	/// Senkou Span B period. Default is 52.
	/// 
	/// Range in \[1; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub l3: PeriodType,

	/// Move `Senkou Span A and B` forward by this period. Default is 26.
	/// 
	/// Range in \[1; [`PeriodType::MAX`](crate::core::PeriodType)\)
	pub m: PeriodType,

	/// Source type. Default is [`Close`](crate::core::Source::Close)
	pub source: Source,
}

impl IndicatorConfig for IchimokuCloud {
	const NAME: &'static str = "IchimokuCloud";

	fn validate(&self) -> bool {
		self.l1 < self.l2 && self.l2 < self.l3
	}

	fn set(&mut self, name: &str, value: String) -> Option<Error> {
		match name {
			"l1" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.l1 = value,
			},
			"l2" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.l2 = value,
			},
			"l3" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.l3 = value,
			},
			"m" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.m = value,
			},
			"source" => match value.parse() {
				Err(_) => return Some(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.source = value,
			},

			_ => {
				return Some(Error::ParameterParse(name.to_string(), value));
			}
		};

		None
	}

	fn size(&self) -> (u8, u8) {
		(4, 2)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for IchimokuCloud {
	type Instance = IchimokuCloudInstance;
	fn init(self, candle: T) -> Result<Self::Instance, Error>
	where
		Self: Sized,
	{
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		Ok(Self::Instance {
			highest1: Highest::new(cfg.l1, candle.high())?,
			highest2: Highest::new(cfg.l2, candle.high())?,
			highest3: Highest::new(cfg.l3, candle.high())?,
			lowest1: Lowest::new(cfg.l1, candle.low())?,
			lowest2: Lowest::new(cfg.l2, candle.low())?,
			lowest3: Lowest::new(cfg.l3, candle.low())?,
			window1: Window::new(cfg.m, candle.hl2()),
			window2: Window::new(cfg.m, candle.hl2()),
			cross1: Cross::default(),
			cross2: Cross::default(),
			cfg,
		})
	}
}

impl Default for IchimokuCloud {
	fn default() -> Self {
		Self {
			l1: 9,
			l2: 26,
			l3: 52,
			m: 26,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct IchimokuCloudInstance {
	cfg: IchimokuCloud,

	highest1: Highest,
	highest2: Highest,
	highest3: Highest,
	lowest1: Lowest,
	lowest2: Lowest,
	lowest3: Lowest,
	window1: Window<ValueType>,
	window2: Window<ValueType>,
	cross1: Cross,
	cross2: Cross,
}

impl<T: OHLC> IndicatorInstance<T> for IchimokuCloudInstance {
	type Config = IchimokuCloud;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let src = candle.source(self.cfg.source);
		let (high, low) = (candle.high(), candle.low());
		let (highest1, lowest1) = (self.highest1.next(high), self.lowest1.next(low));
		let (highest2, lowest2) = (self.highest2.next(high), self.lowest2.next(low));
		let (highest3, lowest3) = (self.highest3.next(high), self.lowest3.next(low));

		let tenkan_sen = (highest1 + lowest1) * 0.5;
		let kijun_sen = (highest2 + lowest2) * 0.5;

		let senkou_span_a = self.window1.push((tenkan_sen + kijun_sen) * 0.5);
		let senkou_span_b = self.window2.push((highest3 + lowest3) * 0.5);

		let s1_cross = self.cross1.next((tenkan_sen, kijun_sen));
		let s2_cross = self.cross2.next((src, kijun_sen));

		let green: bool = senkou_span_a > senkou_span_b;
		let red: bool = senkou_span_a < senkou_span_b;

		// if src > senkou_span_a && src > senkou_span_b && green && s1_cross == Action::BUY_ALL {
		// 	s1 += 1;
		// } else if src < senkou_span_a && src < senkou_span_b && red && s1_cross == Action::SELL_ALL
		// {
		// 	s1 -= 1;
		// }

		// if src > senkou_span_a && src > senkou_span_b && green && s2_cross == Action::BUY_ALL {
		// 	s2 += 1;
		// } else if src < senkou_span_a && src < senkou_span_b && red && s2_cross == Action::SELL_ALL
		// {
		// 	s2 -= 1;
		// }

		let s1 = (src > senkou_span_a
			&& src > senkou_span_b
			&& green && s1_cross == Action::BUY_ALL) as i8
			- (src < senkou_span_a && src < senkou_span_b && red && s1_cross == Action::SELL_ALL)
				as i8;
		let s2 = (src > senkou_span_a
			&& src > senkou_span_b
			&& green && s2_cross == Action::BUY_ALL) as i8
			- (src < senkou_span_a && src < senkou_span_b && red && s2_cross == Action::SELL_ALL)
				as i8;

		IndicatorResult::new(
			&[tenkan_sen, kijun_sen, senkou_span_a, senkou_span_b],
			&[Action::from(s1), Action::from(s2)],
		)
	}
}
