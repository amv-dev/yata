#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{Action, Method, PeriodType, ValueType, Window, OHLC};
use crate::core::{IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult};
use crate::helpers::{method, signi, RegularMethod, RegularMethods};
use crate::methods::{Cross, ReverseHighSignal, ReverseLowSignal};

/*
Билл Вильямс выделил три возможных варианта сигнала на покупку (+ три возможных
сигнала на продажу полностью противоположные сигналам на покупку) которые создает awesome oscillator.

1. «Блюдце» — это единственный сигнал на покупку, который возникает, когда
гистограмма awesome oscillator находится выше нулевой линии. Для образования
сигнала «Блюдце» необходимо, по крайней мере, три столбца гистограммы. «Блюдце»
образуется, когда гистограмма меняет направление с нисходящего на восходящее,
т.е. у 1-го будет большее значение чем у 2-го, у 2-го меньшее чем у 1-го (красный столбец),
у 3-го больше чем у 2-го (зеленый столбец). При этом все столбцы гистограммы
awesome oscillator должны быть выше нулевой линии.

сигнал awesome oscillator - Блюдце

2. «Пересечение нулевой линии» — сигнал на покупку образуется, когда гистограмма
awesome oscillator переходит от отрицательных значений к положительным значениям.
Это происходит тогда, когда гистограмма пересекает нулевую линию. При наличии сигнала
к покупке «Пересечение нулевой линии», сигнальный столбец гистограммы всегда будет
зеленого цвета.

сигнал awesome oscillator - Пересечение нулевой линии

3. «Два Пика» — сигнал на покупку образуется, когда у вас есть направленный вниз
пик (самый низкий минимум), находящийся ниже нулевой линии awesome oscillator,
за которым следует другой направленный вниз пик, который выше (отрицательное число,
меньшее по абсолютному значению, поэтому оно находится ближе к нулевой линии), чем
предыдущий пик, смотрящий вниз. Гистограмма должна находиться ниже нулевой линии
между двумя пиками. Если гистограмма пересекает нулевую линию между пиками, сигнал
на покупку не действует. Однако создается сигнал на покупку «Пересечение нулевой линии».
Если формируется дополнительный, более высокий пик (который ближе к нулевой линии) и
гистограмма не пересекла нулевую линию, то образуется дополнительный сигнал на покупку.
Сигнальный столбец гистограммы должен быть зеленого цвета.

Если столбец гистограммы awesome oscillator зеленого цвета, сигнала на продажу не может
быть. Если он красного цвета, то у вас не может быть сигнала на покупку по awesome
oscillator. Другой важный момент заключается в том, что если сигнал на покупку или
продажу образован, но не преодолевается текущим ценовым баром, затем столбец гистограммы
меняет цвет, этот сигнал аннулируется.
*/

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AwesomeOscillator {
	pub period1: PeriodType,
	pub period2: PeriodType,
	pub method: RegularMethods,
	pub left: PeriodType,
	pub right: PeriodType,
}

impl IndicatorConfig for AwesomeOscillator {
	fn validate(&self) -> bool {
		self.period1 > self.period2
	}

	fn set(&mut self, name: &str, value: String) {
		match name {
			"period1" => self.period1 = value.parse().unwrap(),
			"period2" => self.period2 = value.parse().unwrap(),
			"method" => self.method = value.parse().unwrap(),
			"left" => self.left = value.parse().unwrap(),
			"right" => self.right = value.parse().unwrap(),

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
		(1, 2)
	}
}

impl<T: OHLC> IndicatorInitializer<T> for AwesomeOscillator {
	type Instance = AwesomeOscillatorInstance;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		let hl2 = candle.hl2();

		Self::Instance {
			ma1: method(cfg.method, cfg.period1, hl2),
			ma2: method(cfg.method, cfg.period2, hl2),
			cross_over: Cross::default(),
			ph: Method::new((cfg.left, cfg.right), 0.0),
			pl: Method::new((cfg.left, cfg.right), 0.0),
			window: Window::new(cfg.right, 0.),
			cfg,
		}
	}
}

impl Default for AwesomeOscillator {
	fn default() -> Self {
		Self {
			period1: 34,
			period2: 5,
			method: RegularMethods::SMA,
			left: 1,
			right: 1,
		}
	}
}

#[derive(Debug)]
pub struct AwesomeOscillatorInstance {
	cfg: AwesomeOscillator,

	ma1: RegularMethod,
	ma2: RegularMethod,
	cross_over: Cross,
	ph: ReverseHighSignal,
	pl: ReverseLowSignal,
	window: Window<ValueType>,
}

impl<T: OHLC> IndicatorInstance<T> for AwesomeOscillatorInstance {
	type Config = AwesomeOscillator;

	fn name(&self) -> &str {
		"AwesomeOscillator"
	}

	#[inline]
	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next(&mut self, candle: T) -> IndicatorResult {
		let hl2 = candle.hl2();

		let ma1 = &mut self.ma1;
		let ma2 = &mut self.ma2;
		let value = ma2.next(hl2) - ma1.next(hl2);

		let s2 = self.cross_over.next((value, 0.));

		let ph: i8 = self.ph.next(value).into();
		let pl: i8 = self.pl.next(value).into();

		let last_value = self.window.push(value); //self.window.first();
		let sign = signi(last_value);

		// let mut m_up = pl * sign;
		// let mut m_down = ph * sign;

		// if m_up < 0 {
		// 	m_up = 0;
		// }

		// if m_down > 0 {
		// 	m_down = 0;
		// }

		// let s1 = m_up + m_down;

		let m_up = ((pl * sign) > 0) as i8;
		let m_down = ((ph * sign) < 0) as i8;

		let s1 = m_up - m_down;

		let values = [value];
		let signals = [Action::from(s1), s2];

		IndicatorResult::new(&values, &signals)
	}
}
