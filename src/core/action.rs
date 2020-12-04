use crate::core::ValueType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Neg, Sub};

type SignalType = u8;
const BOUND: SignalType = SignalType::MAX;
const BOUND_FLOAT: f64 = BOUND as f64;

/// Action is basic type of Indicator's signals
///
/// It may be positive \(means *Buy* some amount\). It may be negative \(means *Sell* some amount\). Or there may be no signal at all.
///
/// You can convert `Action` to *analog* `i8` value using [`analog()`](Action::analog) method, where:
/// * `1` means *buy*;
/// * `-1` means *sell*;
/// * `0` means no signal.
///
/// You can convert `Action` to *digital* `Option<f64>` value using [`ratio()`](Action::ratio) method with internal value in range \[`-1.0`; `1.0`\], where:
/// * negative value means *sell* some portion;
/// * positive value means *buy* some potion;
/// * zero value means there is no distinct decision;
/// * [`None`](core::option::Option::None) means no signal.
#[derive(Clone, Copy, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Action {
	/// Buy signal
	Buy(SignalType),
	/// No signal
	None,
	/// Sell signal
	Sell(SignalType),
}

impl Action {
	/// Shortcut for *Buy All* signal
	pub const BUY_ALL: Self = Self::Buy(BOUND);

	/// Shortcut for *Sell All* signal
	pub const SELL_ALL: Self = Self::Sell(BOUND);

	/// Create instance from *analog* signal (which can be only `-1`, `0` or `1`)
	///
	/// Any positive number converts to `BUY_ALL`
	///
	/// Any negative number converts to `SELL_ALL`
	///
	/// Zero converts to None
	#[must_use]
	pub fn from_analog(value: i8) -> Self {
		Self::from(value)
	}

	/// Converts value with the interval \[`-1.0`; `1.0`\]
	#[must_use]
	pub fn ratio(self) -> Option<ValueType> {
		self.into()
	}

	/// Returns a sign (`1` or `-1`) of internal value if value exists and not zero.
	///
	/// Otherwise returns `0`.
	#[must_use]
	pub fn analog(self) -> i8 {
		self.into()
	}

	/// Returns a sign of internal value if value exists
	///
	/// Otherwise returns None
	#[must_use]
	pub fn sign(self) -> Option<i8> {
		self.into()
	}

	/// Return an internal representation of the value if signal exists or None if it doesn't.
	#[must_use]
	pub const fn value(self) -> Option<SignalType> {
		match self {
			Self::None => None,
			Self::Buy(v) | Self::Sell(v) => Some(v),
		}
	}

	/// Checks if there is no signal
	#[must_use]
	pub const fn is_none(self) -> bool {
		matches!(self, Self::None)
	}

	/// Checks if there is signal
	#[must_use]
	pub const fn is_some(self) -> bool {
		!self.is_none()
	}
}

impl PartialEq for Action {
	fn eq(&self, other: &Self) -> bool {
		match (*self, *other) {
			(Self::None, Self::None)
			| (Self::Buy(0), Self::Sell(0))
			| (Self::Sell(0), Self::Buy(0)) => true,
			(Self::Buy(a), Self::Buy(b)) | (Self::Sell(a), Self::Sell(b)) => a == b,
			_ => false,
		}
	}
}

impl Default for Action {
	fn default() -> Self {
		Self::None
	}
}

impl From<bool> for Action {
	fn from(value: bool) -> Self {
		if value {
			Self::BUY_ALL
		} else {
			Self::None
		}
	}
}

impl From<i8> for Action {
	fn from(value: i8) -> Self {
		match value {
			0 => Self::None,
			v => {
				if v > 0 {
					Self::BUY_ALL
				} else {
					Self::SELL_ALL
				}
			}
		}
	}
}

impl From<Action> for i8 {
	fn from(value: Action) -> Self {
		match value {
			Action::Buy(value) => (value > 0) as Self,
			Action::None => 0,
			Action::Sell(value) => -((value > 0) as Self),
		}
	}
}

impl From<Option<i8>> for Action {
	fn from(value: Option<i8>) -> Self {
		match value {
			None => Self::None,
			Some(v) => v.into(),
		}
	}
}

impl From<Action> for Option<i8> {
	fn from(value: Action) -> Self {
		match value {
			Action::None => None,
			_ => Some(value.into()),
		}
	}
}

#[inline]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn from_normalized_f64_to_bounded(value: f64) -> SignalType {
	debug_assert!((0.0..=1.0).contains(&value));

	(value * BOUND_FLOAT).round() as SignalType
}

impl From<f64> for Action {
	fn from(v: f64) -> Self {
		if v.is_nan() {
			return Self::None;
		}

		let normalized = v.max(-1.0).min(1.0);

		let value = from_normalized_f64_to_bounded(normalized.abs());

		if normalized.is_sign_negative() {
			if value == BOUND {
				Self::SELL_ALL
			} else {
				Self::Sell(value)
			}
		} else if value == BOUND {
			Self::BUY_ALL
		} else {
			Self::Buy(value)
		}
	}
}

impl From<Option<f64>> for Action {
	fn from(value: Option<f64>) -> Self {
		match value {
			None => Self::None,
			Some(value) => value.into(),
		}
	}
}

impl From<f32> for Action {
	#[allow(clippy::cast_possible_truncation)]
	fn from(v: f32) -> Self {
		Self::from(v as f64)
	}
}

impl From<Option<f32>> for Action {
	fn from(value: Option<f32>) -> Self {
		match value {
			None => Self::None,
			Some(value) => value.into(),
		}
	}
}

impl From<Action> for Option<ValueType> {
	fn from(value: Action) -> Self {
		match value {
			Action::None => None,
			Action::Buy(value) => Some((value as ValueType) / (BOUND as ValueType)),
			Action::Sell(value) => Some(-(value as ValueType) / (BOUND as ValueType)),
		}
	}
}

impl<T: Into<Action> + Copy> From<&T> for Action {
	fn from(value: &T) -> Self {
		(*value).into()
	}
}

// impl<T: Borrow<Action>> From<T> for i8 {
// 	fn from(value: T) -> Self {
// 		//value.
// 	}
// }

impl Neg for Action {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			Self::None => Self::None,
			Self::Buy(value) => Self::Sell(value),
			Self::Sell(value) => Self::Buy(value),
		}
	}
}

impl Sub for Action {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Self::None, Self::None) => Self::None,
			(s, Self::None) => s,
			(Self::None, s) => -s,
			(Self::Buy(v1), Self::Buy(v2)) => {
				if v1 >= v2 {
					Self::Buy(v1 - v2)
				} else {
					Self::Sell(v2 - v1)
				}
			}
			(Self::Sell(v1), Self::Sell(v2)) => {
				if v1 >= v2 {
					Self::Sell(v1 - v2)
				} else {
					Self::Buy(v2 - v1)
				}
			}
			(s1, s2) => s1 - (-s2),
		}
	}
}

impl fmt::Debug for Action {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::None => write!(f, "N"),
			Self::Buy(value) => write!(f, "+{}", value),
			Self::Sell(value) => write!(f, "-{}", value),
		}
	}
}

impl fmt::Display for Action {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::None => write!(f, "N"),
			Self::Buy(_) => write!(f, "+{:.2}", self.ratio().unwrap()),
			Self::Sell(_) => write!(f, "-{:.2}", self.ratio().unwrap().abs()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Action, BOUND};
	use crate::core::ValueType;
	use std::cmp::Ordering;

	#[test]
	fn test_action_ratio() {
		assert_eq!(Some(1.0), Action::Buy(BOUND).ratio());
		assert_eq!(Some(-1.0), Action::Sell(BOUND).ratio());
		assert_eq!(Some(0.0), Action::Sell(0).ratio());
		assert_eq!(Some(0.0), Action::Buy(0).ratio());
		assert_eq!(Action::Sell(0), Action::Buy(0));
	}
	#[test]
	fn test_action_from_float() {
		let half_bound = if BOUND % 2 == 1 {
			BOUND / 2 + 1
		} else {
			BOUND / 2
		};
		// f64
		assert_eq!(Action::from(0.0_f64), Action::Buy(0));
		assert_eq!(Action::from(-0.5_f64), Action::Sell(half_bound));
		assert_eq!(Action::from(1.0_f64), Action::BUY_ALL);
		assert_eq!(Action::from(-1.0_f64), Action::SELL_ALL);
		assert_eq!(Action::from(2.0_f64), Action::BUY_ALL);
		assert_eq!(Action::from(-2.0_f64), Action::SELL_ALL);

		// f32
		assert_eq!(Action::from(0.0_f32), Action::Buy(0));
		assert_eq!(Action::from(-0.5_f32), Action::Sell(half_bound));
		assert_eq!(Action::from(1.0_f32), Action::BUY_ALL);
		assert_eq!(Action::from(-1.0_f32), Action::SELL_ALL);
		assert_eq!(Action::from(2.0_f32), Action::BUY_ALL);
		assert_eq!(Action::from(-2.0_f32), Action::SELL_ALL);

		// other
		assert_eq!(Action::from(1. / BOUND as ValueType), Action::Buy(1));
		assert_eq!(Action::from(-1. / BOUND as ValueType), Action::Sell(1));
		assert_eq!(Action::from(-2. / BOUND as ValueType), Action::Sell(2));
	}

	#[test]
	fn test_action_from_into() {
		(1..=BOUND).for_each(|x| {
			let action = if x < BOUND {
				Action::Buy(x)
			} else {
				Action::BUY_ALL
			};
			let ratio = action.ratio().unwrap();
			let action2: Action = ratio.into();

			assert!(ratio > 0.);
			assert_eq!(
				action,
				ratio.into(),
				"at index {} with action {:?} ratio {}, action#2 {:?}",
				x,
				action,
				ratio,
				action2,
			);

			let action = if x < BOUND {
				Action::Sell(x)
			} else {
				Action::SELL_ALL
			};
			let ratio = action.ratio().unwrap();
			let action2: Action = ratio.into();

			assert!(ratio < 0.);
			assert_eq!(
				action,
				ratio.into(),
				"at index {} with action {:?} ratio {}, action#2 {:?}",
				x,
				action,
				ratio,
				action2,
			);
		});
	}

	#[test]
	fn test_action_from_float_histogram() {
		let half_value = Action::Buy(1).ratio().unwrap() / 2.0;
		let delta = if cfg!(feature = "value_type_f32") {
			1e-7
		} else {
			1e-15
		};

		println!("{}", delta);
		(0..=BOUND).for_each(|x| {
			let xx = x as ValueType;
			assert_eq!(Action::Buy(x), (half_value * 2. * xx).into());
			assert_eq!(Action::Sell(x), (-half_value * 2. * xx).into());

			if x > 0 {
				let y = x - 1;
				assert_eq!(
					Action::Buy(y),
					(half_value * 2. * xx - half_value - delta).into()
				);
				assert_eq!(
					Action::Sell(y),
					(-(half_value * 2. * xx - half_value - delta)).into()
				);
			}
		});

		assert_eq!(Action::Buy(1), (half_value * 3. - delta).into());
		assert_eq!(Action::Buy(2), (half_value * 3.).into());
	}

	#[test]
	fn test_action_from_i8() {
		(i8::MIN..=i8::MAX).for_each(|s| {
			let action = Action::from(s);

			match s.cmp(&0) {
				Ordering::Greater => assert_eq!(action, Action::BUY_ALL),
				Ordering::Less => assert_eq!(action, Action::SELL_ALL),
				Ordering::Equal => assert_eq!(action, Action::None),
			}
		});
	}

	#[test]
	fn test_action_from_i8_optional() {
		(i8::MIN..=i8::MAX).for_each(|s| {
			let action = Action::from(Some(s));

			match s.cmp(&0) {
				Ordering::Greater => assert_eq!(action, Action::BUY_ALL),
				Ordering::Less => assert_eq!(action, Action::SELL_ALL),
				Ordering::Equal => assert_eq!(action, Action::None),
			}
		});
	}

	#[test]
	fn test_action_neg() {
		(0..=BOUND).for_each(|x| {
			let s = Action::Buy(x);
			let b = Action::Sell(x);

			assert_eq!(s, -b);
			assert_eq!(-s, b);
		});
	}

	#[test]
	#[allow(clippy::eq_op)]
	fn test_action_eq() {
		assert_eq!(Action::None, Action::None);
		assert_ne!(Action::Buy(0), Action::None);
		assert_ne!(Action::Sell(0), Action::None);
		assert_eq!(Action::Buy(0), Action::Buy(0));
		assert_eq!(Action::Sell(0), Action::Sell(0));
		assert_eq!(Action::Buy(0), Action::Sell(0));
		assert_eq!(Action::Sell(0), Action::Buy(0));
		assert_ne!(Action::Sell(2), Action::Buy(5));
		assert_ne!(Action::Buy(2), Action::Sell(5));
		assert_ne!(Action::Buy(2), Action::Buy(5));
		assert_eq!(Action::Buy(5), Action::Buy(5));
		assert_ne!(Action::Sell(2), Action::Sell(5));
		assert_eq!(Action::Sell(5), Action::Sell(5));
	}
}
