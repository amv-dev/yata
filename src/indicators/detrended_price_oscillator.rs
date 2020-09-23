#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::core::{Method, PeriodType, Source, ValueType, Window, OHLC};
use crate::helpers::{method, RegularMethod, RegularMethods};
use crate::methods::Cross;
// https://www.investopedia.com/terms/d/detrended-price-oscillator-dpo.asp
// The Formula for the Detrended Price Oscillator (DPO) is

// <br>DPO=Price from X2+1 periods ago−X period SMA
// <br>where:
// <br>X = Number of periods used for the look-back period
// <br>SMA = Simple Moving Average<br>\begin{aligned}
// <br>&DPO=Price~from~\frac{X}{2}+1~periods~ago-X~period~SMA\\
// <br>&\textbf{where:}\\
// <br>&\text{X = Number of periods used for the look-back period}\\
// <br>&\text{SMA = Simple Moving Average}\\
// <br>\end{aligned}
// <br><br><br><br><br>​DPO=Price from 2
// X​+1 periods ago−X period SMAwhere:X = Number of periods used for the look-back periodSMA = Simple Moving Average​

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DetrendedPriceOscillator {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub period3: PeriodType,
	pub method1: RegularMethods,
	pub method2: RegularMethods,
	pub method3: RegularMethods,
	pub source: Source,
}

impl IndicatorConfig for DetrendedPriceOscillator {
	fn validate(&self) -> bool {
		self.period1 > 1 && self.period2 >= 1 && self.period3 >= 1
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"period3" => self.period3 = value.parse().unwrap(),
			"method1" => self.method1 = value.parse().unwrap(),
			"method2" => self.method2 = value.parse().unwrap(),
			"method3" => self.method2 = value.parse().unwrap(),
			"source" => self.source = value.parse().unwrap(),

			_ => {
				dbg!(format!(
					"Unknown attribute `{:}` with value `{:}` for `{:}`",
					name,
					value,
					std::any::type_name::<Self>(),
				));
			}
		};
	}

	fn size(&self) -> (u8, u8) {
		(2, 2)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for DetrendedPriceOscillator {
	type Instance = DetrendedPriceOscillatorInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let src = candle.source(cfg.source);
		Self::Instance {
			sma: method(cfg.method1, cfg.period1, src),
			window: Window::new(cfg.period1 / 2 + 1, src),
			ma2: method(cfg.method2, cfg.period2, 0.),
			ma3: method(cfg.method3, cfg.period3, 0.),
			cross_over1: Cross::default(),
			cross_over2: Cross::default(),

			cfg,
		}
	}
}

impl Default for DetrendedPriceOscillator {
	fn default() -> Self {
		Self {
			period1: 21,
			period2: 21,
			period3: 21,
			method1: RegularMethods::SMA,
			method2: RegularMethods::SMA,
			method3: RegularMethods::SMA,
			source: Source::Close,
		}
	}
}

#[derive(Debug)]
pub struct DetrendedPriceOscillatorInstance {
	cfg: DetrendedPriceOscillator,

	sma: RegularMethod,
	window: Window<ValueType>,
	ma2: RegularMethod,
	ma3: RegularMethod,
	cross_over1: Cross,
	cross_over2: Cross,
}

impl<T: OHLC> IndicatorInstance<T> for DetrendedPriceOscillatorInstance {
	type Config = DetrendedPriceOscillator;

	fn name(&self) -> &str {
		"DetrendedPriceOscillator"
	}

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	#[allow(unreachable_code, unused_variables)]
	fn next(&mut self, candle: T) -> IndicatorResult {
		todo!("Проверить дефолтные значения");
		// Возможно стоит вернуть индикатор к оригинальному виду

		let src = candle.source(self.cfg.source);

		let sma = self.sma.next(src);
		let left_src = self.window.push(src);

		let dpo = left_src - sma;
		let q = self.ma2.next(dpo);
		let ma_q = self.ma3.next(q);

		let signal1 = self.cross_over1.next((q, 0.));
		let signal2 = self.cross_over2.next((q, ma_q));

		IndicatorResult::new(&[q, ma_q], &[signal1, signal2])
	}
}
