use super::{IndicatorConfig, IndicatorInstance, IndicatorResult};
use crate::core::{Error, OHLCV};

/// Dynamically dispatchable [`IndicatorConfig`](crate::core::IndicatorConfig)
pub trait IndicatorConfigDyn<T: OHLCV> {
	/// Dynamically initializes the **State** based on the current **Configuration**
	fn init(&self, initial_value: &T) -> Result<Box<dyn IndicatorInstanceDyn<T>>, Error>;

	/// Evaluates dynamically dispatched [`IndicatorConfig`](crate::core::IndicatorConfig)  over series of OHLC and returns series of `IndicatorResult`s
	/// ```
	/// use yata::prelude::dd::*;
	/// use yata::helpers::{RandomCandles};
	/// use yata::indicators::Trix;
	///
	/// let candles: Vec<_> = RandomCandles::new().take(10).collect();
	/// let static_config = Trix::default();
	/// let dyn_config: Box<dyn IndicatorConfigDyn<_>> = Box::new(static_config); // here we are loosing information about `IndicatorConfig`s type.
	/// let results = dyn_config.over(&candles).unwrap();
	/// println!("{:?}", results);
	/// ```
	fn over(&self, inputs: &dyn AsRef<[T]>) -> Result<Vec<IndicatorResult>, Error>;

	/// Returns a name of the indicator
	fn name(&self) -> &'static str;

	/// Validates if **Configuration** is OK
	fn validate(&self) -> bool;

	/// Dynamically sets **Configuration** parameters
	fn set(&mut self, name: &str, value: String) -> Result<(), Error>;

	/// Returns an [`IndicatorResult`](crate::core::IndicatorResult) size processing by the indicator `(count of raw values, count of signals)`
	fn size(&self) -> (u8, u8);
}

impl<T, I, C> IndicatorConfigDyn<T> for C
where
	T: OHLCV,
	I: IndicatorInstanceDyn<T> + IndicatorInstance<Config = Self> + 'static,
	C: IndicatorConfig<Instance = I> + Clone + 'static,
{
	fn init(&self, initial_value: &T) -> Result<Box<dyn IndicatorInstanceDyn<T>>, Error> {
		let instance = IndicatorConfig::init(self.clone(), initial_value)?;
		Ok(Box::new(instance))
	}

	fn over(&self, inputs: &dyn AsRef<[T]>) -> Result<Vec<IndicatorResult>, Error> {
		IndicatorConfig::over(self.clone(), inputs)
	}

	fn name(&self) -> &'static str {
		<Self as IndicatorConfig>::NAME
	}

	fn validate(&self) -> bool {
		IndicatorConfig::validate(self)
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		IndicatorConfig::set(self, name, value)
	}

	fn size(&self) -> (u8, u8) {
		IndicatorConfig::size(self)
	}
}

/// Dynamically dispatchable [`IndicatorInstance`](crate::core::IndicatorInstance)
pub trait IndicatorInstanceDyn<T: OHLCV> {
	/// Evaluates given candle and returns [`IndicatorResult`](crate::core::IndicatorResult)
	fn next(&mut self, candle: &T) -> IndicatorResult;

	/// Evaluates the **State** over the given sequence of candles and returns sequence of `IndicatorResult`s.
	/// ```
	/// use yata::prelude::dd::*;
	/// use yata::helpers::{RandomCandles};
	/// use yata::indicators::Trix;
	///
	/// let candles: Vec<_> = RandomCandles::new().take(10).collect();
	/// let static_config = Trix::default();
	/// let dyn_config: Box<dyn IndicatorConfigDyn<_>> = Box::new(static_config); // here we are loosing information about `IndicatorConfig`s type.
	/// let mut state = dyn_config.init(&candles[0]).unwrap();
	///
	/// let results = state.over(&candles);
	/// println!("{:?}", results);
	/// ```
	fn over(&mut self, inputs: &dyn AsRef<[T]>) -> Vec<IndicatorResult>;

	/// Returns a reference to dynamically dispatched **Configuration**, associated with the current **State**
	fn config(&self) -> &dyn IndicatorConfigDyn<T>;

	/// Returns count of indicator's raw values and count of indicator's signals.
	///
	/// See more at [`IndicatorConfig`](crate::core::IndicatorConfig::size)
	fn size(&self) -> (u8, u8);

	/// Returns a name of the indicator
	fn name(&self) -> &'static str;
}

impl<T, I> IndicatorInstanceDyn<T> for I
where
	T: OHLCV,
	I: IndicatorInstance + 'static,
{
	fn next(&mut self, candle: &T) -> IndicatorResult {
		IndicatorInstance::next(self, candle)
	}

	fn over(&mut self, inputs: &dyn AsRef<[T]>) -> Vec<IndicatorResult> {
		IndicatorInstance::over(self, inputs)
	}

	fn config(&self) -> &dyn IndicatorConfigDyn<T> {
		self.config()
	}

	fn size(&self) -> (u8, u8) {
		IndicatorInstance::size(self)
	}

	fn name(&self) -> &'static str {
		IndicatorInstance::name(self)
	}
}
