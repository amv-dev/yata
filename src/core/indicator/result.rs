use crate::core::{Action, ValueType};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Every `Indicator` proceed an input of [`OHLCV`](crate::core::OHLCV) and returns an `IndicatorResult` which consist of some returned raw values and some calculated signals.
///
/// `Indicator` may return up to 4 signals and 4 raw values at each step
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndicatorResult {
	signals: [Action; IndicatorResult::SIZE],
	values: [ValueType; IndicatorResult::SIZE],
	length: (u8, u8),
}

impl IndicatorResult {
	/// Size of pre-allocated result array
	/// For the most of cases it should not be used anywhere outside this crate
	pub const SIZE: usize = 4;

	/// Returns a slice of signals of current indicator result
	#[must_use]
	pub fn signals(&self) -> &[Action] {
		let len = self.length.1 as usize;
		&self.signals[..len]
	}

	/// Returns a slice of raw indicator values of current indicator result
	#[must_use]
	pub fn values(&self) -> &[ValueType] {
		let len = self.length.0 as usize;
		&self.values[..len]
	}

	/// Returns count of signals
	#[must_use]
	pub const fn signals_length(&self) -> u8 {
		self.length.1
	}

	/// Returns count of raw values
	#[must_use]
	pub const fn values_length(&self) -> u8 {
		self.length.0
	}

	/// Returns a tuple of count of raw values and count of signals
	#[must_use]
	pub const fn size(&self) -> (u8, u8) {
		self.length
	}

	/// Returns a raw value at given index
	#[inline]
	#[must_use]
	pub fn value(&self, index: usize) -> ValueType {
		debug_assert!(index < self.length.0 as usize);
		self.values[index]
	}

	/// Returns a signal at given index
	#[inline]
	#[must_use]
	pub fn signal(&self, index: usize) -> Action {
		debug_assert!(index < self.length.1 as usize);
		self.signals[index]
	}

	/// Creates a new instance of `IndicatorResult` with provided *values* and *signals*
	#[inline]
	#[must_use]
	pub fn new(values_slice: &[ValueType], signals_slice: &[Action]) -> Self {
		let mut values = [0 as ValueType; Self::SIZE];
		let mut signals = [Action::default(); Self::SIZE];

		let values_length = Self::SIZE.min(values_slice.len());
		values[..values_length].copy_from_slice(&values_slice[..values_length]);

		let signals_length = Self::SIZE.min(signals_slice.len());
		signals[..signals_length].copy_from_slice(&signals_slice[..signals_length]);

		#[allow(clippy::cast_possible_truncation)]
		let length = (values_length as u8, signals_length as u8);

		Self {
			values,
			signals,
			length,
		}
	}
}

impl fmt::Debug for IndicatorResult {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let values: Vec<String> = self
			.values
			.iter()
			.take(self.length.0 as usize)
			.map(|&x| format!("{:>7.4}", x))
			.collect();
		let signals: Vec<String> = self
			.signals
			.iter()
			.take(self.length.1 as usize)
			.map(std::string::ToString::to_string)
			.collect();
		write!(
			f,
			"S: [{:}], V: [{:}]",
			signals.join(", "),
			values.join(", ")
		)
	}
}
