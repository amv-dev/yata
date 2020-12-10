use crate::core::{Error, Method, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// [Variable Index Dynamic Average](https://www.metatrader5.com/en/terminal/help/indicators/trend_indicators/vida) of specified `length` for timeseries of type [`ValueType`]
///
/// # Parameters
///
/// Has a single parameter `length`: [`PeriodType`]
///
/// `length` must be > `0`
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`ValueType`]
///
/// # Examples
///
/// ```
/// use yata::prelude::*;
/// use yata::methods::Vidya;
///
/// // Vidya with period length=3
/// let mut vidya = Vidya::new(3, 1.0).unwrap();
///
/// vidya.next(3.0);
/// vidya.next(6.0);
///
/// println!("{}", vidya.next(9.0));
/// println!("{}", vidya.next(12.0));
/// ```
///
/// # Performance
///
/// O\(1\)
///
/// [`ValueType`]: crate::core::ValueType
/// [`PeriodType`]: crate::core::PeriodType
#[derive(Debug, Clone)]
#[doc(alias = "VariableIndexDynamicAverage")]
#[doc(alias = "Variable")]
#[doc(alias = "Index")]
#[doc(alias = "Dynamic")]
#[doc(alias = "Average")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vidya {
	f: ValueType,
	up_sum: ValueType,
	dn_sum: ValueType,
	last_input: ValueType,
	last_output: ValueType,
	window: Window<ValueType>,
}

impl Vidya {
	/// Returns last calculated value
	#[must_use]
	pub const fn get_last_value(&self) -> <Self as Method>::Output {
		self.last_output
	}
}

impl Method<'_> for Vidya {
	type Params = PeriodType;
	type Input = ValueType;
	type Output = Self::Input;

	fn new(length: Self::Params, input: Self::Input) -> Result<Self, Error> {
		match length {
			0 | PeriodType::MAX => Err(Error::WrongMethodParameters),
			length => Ok(Self {
				f: 2. / (1 + length) as ValueType,
				up_sum: 0.,
				dn_sum: 0.,
				last_input: input,
				last_output: input,
				window: Window::new(length, 0.),
			}),
		}
	}

	#[inline]
	fn next(&mut self, input: Self::Input) -> Self::Output {
		let change = input - self.last_input;
		self.last_input = input;

		let left_change = self.window.push(change);

		self.up_sum -= left_change * (left_change > 0.) as u8 as ValueType;
		self.dn_sum += left_change * (left_change < 0.) as u8 as ValueType;

		self.up_sum += change * (change > 0.) as u8 as ValueType;
		self.dn_sum -= change * (change < 0.) as u8 as ValueType;

		self.last_output = if self.up_sum != 0. || self.dn_sum != 0. {
			let cmo = ((self.up_sum - self.dn_sum) / (self.up_sum + self.dn_sum)).abs();
			let f_cmo = self.f * cmo;
			input.mul_add(f_cmo, (1.0 - f_cmo) * self.last_output)
		} else {
			input
		};

		self.last_output
	}
}

#[cfg(test)]
mod tests {
	use super::Vidya as TestingMethod;
	use super::{Method, ValueType};
	use crate::helpers::{assert_eq_float, RandomCandles};
	use crate::methods::tests::test_const;

	#[test]
	fn test_vidya_const() {
		for i in 1..255 {
			let input = (i as ValueType + 56.0) / 16.3251;
			let mut method = TestingMethod::new(i, input).unwrap();

			let output = method.next(input);
			test_const(&mut method, input, output);
		}
	}

	#[test]
	fn test_vidya1() {
		let mut candles = RandomCandles::default();

		let mut ma = TestingMethod::new(1, candles.first().close).unwrap();

		candles.take(100).for_each(|x| {
			assert_eq_float(x.close, ma.next(x.close));
		});
	}

	#[test]
	#[allow(clippy::suboptimal_flops)]
	fn test_vidya() {
		let candles = RandomCandles::default();

		let src: Vec<ValueType> = candles.take(300).map(|x| x.close).collect();

		let change: Vec<_> = (0..1)
			.map(|_| 0.0)
			.chain(src.windows(2).map(|x| x[1] - x[0]))
			.collect();
		let pos_change: Vec<_> = change
			.iter()
			.map(|&x| if x > 0.0 { x } else { 0.0 })
			.collect();
		let neg_change: Vec<_> = change
			.iter()
			.map(|&x| if x < 0.0 { x.abs() } else { 0.0 })
			.collect();

		(1..255).for_each(|ma_length| {
			let mut ma = TestingMethod::new(ma_length, src[0]).unwrap();
			let ma_length = ma_length as usize;

			let mut value = src[0];
			src.iter().enumerate().for_each(|(i, &x)| {
				let from_slice = i.saturating_sub(ma_length - 1);
				let pos: ValueType = pos_change[from_slice..=i].iter().sum();
				let neg: ValueType = neg_change[from_slice..=i].iter().sum();

				value = if (pos + neg) == 0.0 {
					x
				} else {
					let cmo = (pos - neg) / (pos + neg);
					let f = 2.0 / (ma_length + 1) as ValueType;

					x * f * cmo.abs() + value * (1.0 - f * cmo.abs())
				};

				assert_eq_float(value, ma.next(x));
			});
		});
	}
}
