#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{
	Action, IndicatorConfig, IndicatorInitializer, IndicatorInstance, IndicatorResult,
};
use crate::core::{PeriodType, ValueType, Window, OHLC};

/*
N = 14 #common values: 14, 25, с каким периодом будет сравниваться значения
signalZone = 0.3 #сигналом будет считать показания выше 0.7 (70) и ниже 0.3 (30)

При базовом отображении индикатор Aroon колеблется в диапазоне между 0 и 100.
Нахождение линий в верхней части (70-100) говорит о частом обновлении соответствующих экстремумов.
Нахождение линий в нижней части (0-30) шкалы свидетельствует о редком обновлении экстремумов.
Считается, что на рынке преобладают покупатели, если «верхняя» (зеленая) линия
находится выше 50, а «нижняя» (красная) линия находится ниже 50. При медвежьих
настроениях возникает противоположная ситуация: зеленая линия находится ниже 50,
а красная выше 50. Приближение одной из линий индикатора Aroon к 100 при падении
другой ниже 30 может быть признаком начала тренда.

Если индикатор Aroon используется в виде осциллятора, то он будет представлять
собой одну линию, колеблющуюся в диапазоне от -100 до +100. Если осциллятор выше 0,
то «верхняя» (зеленая) линия базового индикатора находится над «нижней» (красной) линией.
Отрицательные значения осциллятора будут говорить о противоположной ситуации.
Значения осциллятора можно использовать для определения силы тренда.

Индикатор Aroon для определения начала нового тренда

Базовый индикатор Aroon можно использовать для определения начала тренда.
Признаком начала нового тренда будет пересечение линий индикатора, их закрепление
по разные стороны от центральной линии и достижение соответствующей линией значения 100.
Например, для выявления начала восходящего движения должно произойти следующее:

*/

// Нерабочая версия
// Проверить и исправить
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Aroon {
	pub signal_zone: ValueType,
	pub n:           PeriodType,
}

impl IndicatorConfig for Aroon {
	fn validate(&self) -> bool { true }

	fn set(&mut self, name: &str, value: String) {
		match name {
			"signal_zone" => self.signal_zone = value.parse().unwrap(),
			"n" => self.n = value.parse().unwrap(),

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

	fn size(&self) -> (u8, u8) { (3, 3) }
}

impl<T: OHLC> IndicatorInitializer<T> for Aroon {
	type Instance = AroonInstance<T>;

	fn init(self, candle: T) -> Self::Instance
	where
		Self: Sized,
	{
		let cfg = self;
		Self::Instance {
			i: 0,
			max_value: candle.high(),
			min_value: candle.low(),
			max_index: 0,
			min_index: 0,
			candle,
			index: 0,
			n_1: cfg.n - 1,
			invert_n: (cfg.n as ValueType).recip(),
			window: Window::new(cfg.n, candle),
			cfg,
		}
	}
}

impl Default for Aroon {
	fn default() -> Self {
		Self {
			signal_zone: 0.3,
			n:           14,
		}
	}
}

#[derive(Debug, Default)]
pub struct AroonInstance<T: OHLC> {
	cfg: Aroon,

	i:         PeriodType,
	max_index: PeriodType,
	max_value: ValueType,
	min_index: PeriodType,
	min_value: ValueType,
	candle:    T,
	index:     PeriodType,
	n_1:       PeriodType,
	invert_n:  ValueType,
	window:    Window<T>,
}

impl<T: OHLC> IndicatorInstance<T> for AroonInstance<T> {
	type Config = Aroon;

	fn name(&self) -> &str { "Aroon" }

	#[inline]
	fn config(&self) -> &Self::Config { &self.cfg }

	#[allow(unreachable_code, unused_variables)]
	fn next(&mut self, candle: T) -> IndicatorResult {
		todo!("Некорректная реализация");

		self.max_index += 1;
		self.min_index += 1;

		self.window.push(candle);

		let length = self.n_1;
		if self.max_index > length || self.min_index > length {
			let first = self.window.first();
			let mut max_index = self.cfg.n - 1;
			let mut min_index = self.cfg.n - 1;
			let mut max_value = first.high();
			let mut min_value = first.low();

			self.window
				.iter()
				.enumerate() /*.skip(1)*/
				.for_each(|(i, c)| {
					let j = self.cfg.n - (i as PeriodType) - 1;
					if c.high() >= max_value {
						max_index = j;
						max_value = candle.high(); // Ошибка?????? Мб. c.high()???
					}

					if c.low() <= min_value {
						min_index = j;
						min_value = candle.low(); // Ошибка ????? мб c.low()????
					}
				});

			if self.min_index > length {
				self.min_value = min_value;
				self.min_index = min_index;
			}

			if self.max_index > length {
				self.max_value = max_value;
				self.max_index = max_index;
			}

			print!("{}:{}={}\n", self.min_index, self.i, self.min_value);
		} else {
			if candle.high() >= self.max_value {
				self.max_index = 0;
				self.max_value = candle.high();
			}

			if candle.low() <= self.min_value {
				self.min_index = 0;
				self.min_value = candle.low();
			}
		}

		let aroon_u = (self.cfg.n - self.max_index) as ValueType * self.invert_n;
		let aroon_d = (self.cfg.n - self.min_index) as ValueType * self.invert_n;
		let aroon_o = aroon_u - aroon_d;

		let (mut u, mut d) = (0i8, 0i8);

		if aroon_u > 1. - self.cfg.signal_zone {
			u += 1;
		}

		if aroon_u < self.cfg.signal_zone {
			u -= 1;
		}

		if aroon_d > 1.0 - self.cfg.signal_zone {
			d += 1;
		}

		if aroon_d < self.cfg.signal_zone {
			d -= 1;
		}

		let o = (aroon_o - 0.5).ceil() as i8;

		self.i += 1;
		let values = [aroon_u, aroon_d, aroon_o];
		let signals = [Action::from(o), Action::from(u), Action::from(d)];

		IndicatorResult::new(&values, &signals)

		// NextEntry::from([
		// 	Entry {
		// 		value:  aroon_u,
		// 		signal: o,
		// 	},
		// 	Entry {
		// 		value:  aroon_d,
		// 		signal: u,
		// 	},
		// 	Entry {
		// 		value:  aroon_o,
		// 		signal: d,
		// 	},
		// ])
	}
}

// // extern crate trading_core;
//
// use serde::{Serialize, Deserialize};
//
// //use crate::core::{ Candles, IndicatorInstance, Sequence, Signal, SignalType };
// use crate::core::{ Candles, IndicatorInstance, Value, Signal, ValueType, SignalType };
// /*
// N = 14 #common values: 14, 25, с каким периодом будет сравниваться значения
// signalZone = 0.3 #сигналом будет считать показания выше 0.7 (70) и ниже 0.3 (30)
//
// При базовом отображении индикатор Aroon колеблется в диапазоне между 0 и 100.
// Нахождение линий в верхней части (70-100) говорит о частом обновлении соответствующих экстремумов.
// Нахождение линий в нижней части (0-30) шкалы свидетельствует о редком обновлении экстремумов.
// Считается, что на рынке преобладают покупатели, если «верхняя» (зеленая) линия
// находится выше 50, а «нижняя» (красная) линия находится ниже 50. При медвежьих
// настроениях возникает противоположная ситуация: зеленая линия находится ниже 50,
// а красная выше 50. Приближение одной из линий индикатора Aroon к 100 при падении
// другой ниже 30 может быть признаком начала тренда.
//
// Если индикатор Aroon используется в виде осциллятора, то он будет представлять
// собой одну линию, колеблющуюся в диапазоне от -100 до +100. Если осциллятор выше 0,
// то «верхняя» (зеленая) линия базового индикатора находится над «нижней» (красной) линией.
// Отрицательные значения осциллятора будут говорить о противоположной ситуации.
// Значения осциллятора можно использовать для определения силы тренда.
//
// Индикатор Aroon для определения начала нового тренда
//
// Базовый индикатор Aroon можно использовать для определения начала тренда.
// Признаком начала нового тренда будет пересечение линий индикатора, их закрепление
// по разные стороны от центральной линии и достижение соответствующей линией значения 100.
// Например, для выявления начала восходящего движения должно произойти следующее:
//
// */
//
// #[derive(Debug)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// pub struct Aroon {
// 	pub signal_zone: ValueType,
// 	pub n: usize,
// }
//
// // Функцию new нельзя вынести в Trait, потому что иначе отказывается генерировать indicator через функцию get_indicator -> Box<dyn IndicatorInstance + 'a>
// impl Aroon {
// 	pub fn new() -> Self where Self: Sized {
// 		Self::default()
// 	}
// }
//
// impl Default for Aroon {
// 	fn default() -> Self where Self: Sized {
// 		Self {
// 			signal_zone: 0.3,
// 			n: 14,
// 		}
// 	}
// }
//
// impl<T: OHLC> IndicatorInstance<T> for Aroon {
// 	fn new() -> Self where Self: Sized {
// 		Self::default()
// 	}
// 	fn value(&self, candles: &Candles) -> Vec<Value> {
// 		let first_candle = match candles.first() {
// 			Some(candle) => candle,
// 			None => {
// 				return Vec::with_capacity(0);
// 			}
// 		};
//
// 		let mut max_index:isize = 0;
// 		let mut min_index:isize = 0;
// 		let mut max_value = first_candle.high();
// 		let mut min_value = first_candle.low();
//
// 		let n = self.n as isize;
// 		let n_1 = n-1;
// 		let invert_n = (n as ValueType).recip();
//
// 		let mut result = Vec::<Value>::with_capacity(3);
// 		for _ in 0..3 {
// 			result.push(Value::new(candles.len()));
// 		}
//
// 		for (index, candle) in candles.iter().enumerate() {
// 			let iindex = index as isize;
// 			let first_index = iindex - n_1; //включая текущую свечу
//
// 			if (max_index < first_index) || (min_index < first_index) {
// 				if max_index < first_index {
// 					max_index = first_index;
// 					max_value =  candles[first_index].high();
// 				}
//
// 				if min_index < first_index {
// 					min_index = first_index;
// 					min_value = candles[first_index].low();
// 				}
//
// 				for i in (first_index + 1)..(iindex+1) {
// 					if candles[i].high() >= max_value {
// 						max_index = i;
// 						max_value = candle.high();
// 					}
//
// 					if candles[i].low() <= min_value {
// 						min_index = i;
// 						min_value = candle.low();
// 					}
// 				}
// 			} else {
// 				if candle.high() >= max_value {
// 					max_index = iindex;
// 					max_value = candle.high();
// 				}
//
// 				if candle.low() <= min_value {
// 					min_index = iindex;
// 					min_value = candle.low();
// 				}
// 			}
//
// 			let aroon_u = (n-(iindex-max_index)) as ValueType * invert_n;
// 			let aroon_d = (n-(iindex-min_index)) as ValueType * invert_n;
//
// 			let aroon_o = aroon_u - aroon_d;
//
// 			// r[0][index], r[1][index], r[2][index] = AroonU, AroonD, AroonO
// 			result[0].push(aroon_u);
// 			result[1].push(aroon_d);
// 			result[2].push(aroon_o);
// 		}
//
// 		result
// 	}
//
// 	fn signal(&self, candles: &Candles) -> Vec<Signal> {
// 		let zone = self.signal_zone;
// 		let mut result = Vec::<Signal>::with_capacity(3);
//
// 		for _ in 0..3 {
// 			result.push(Signal::new(candles.len()));
// 		}
//
// 		let values = self.value(candles);
//
// 		for index in 0..candles.len() {
// 			let _u = values[0][index];
// 			let _d = values[1][index];
// 			let o = values[2][index];
//
// 			let mut u:SignalType = 0;
// 			let mut d:SignalType = 0;
//
// 			if _u > 1.-zone {
// 				u+=1;
// 			}
//
// 			if _u < zone {
// 				u-=1;
// 			}
//
// 			if _d > 1.0-zone {
// 				d+=1;
// 			}
//
// 			if _d < zone {
// 				d-=1;
// 			}
//
// 			result[0].push( (o - 0.5).ceil() as SignalType ); //int8(math.Ceil(o - 0.5)));
// 			result[1].push(u);
// 			result[2].push(d);
// 		}
//
// 		result
// 	}
//
// 	fn name(&self) -> &str { "Aroon" }
//
// 	fn set(&mut self, name: &str, value: String) {
// 		match name {
// 			"n"				=> self.n = value.parse().unwrap(),
// 			"period1"		=> self.n = value.parse().unwrap(),
// 			"signal_zone"	=> self.signal_zone = String::from(value).parse().unwrap(),
//
// 			_			=> {
// 				dbg!(format!("Unknown attribute `{:}` with value `{:}` for `{:}`", name, value, std::any::type_name::<Self>(),));
// 			},
// 		}
// 	}
// }
