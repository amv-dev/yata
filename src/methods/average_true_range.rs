use crate::core::{Method, OHLCV};
use crate::core::{Error, PeriodType, ValueType, Window};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::methods::SMA;

/// Returns the Average True Range for timeseries of type [`OHLC`]
///
/// # Parameters
///
/// Has a single parameter `period`: [`PeriodType`]
///
/// `length` should be > `0`
///
/// # Input type
///
/// Input type is [`ValueType`]
///
/// # Output type
///
/// Output type is [`PeriodType`]
///
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AverageTrueRange {
    period: PeriodType,
    window: Window<ValueType>,
    sma: SMA,
    prev_close: ValueType,
}

impl<'a> Method<'a> for AverageTrueRange {
    type Params = PeriodType;
    type Input = &'a dyn OHLCV;
    type Output = ValueType;

    fn new(period: Self::Params, value: Self::Input) -> Result<Self, Error> {
        match period {
            0 => Err(Error::WrongMethodParameters),
            length => Ok(Self {
                period,
                window: Window::new(length, 0.0),
                sma: SMA::new(length, 0.0)?,
                prev_close: value.close(),
            }),
        }
    }

    #[inline]
    fn next(&mut self, value: Self::Input) -> Self::Output {
        let temp_candle = &(0.0, 0.0, 0.0, self.prev_close, 0.0);
        self.sma.next(value.tr(temp_candle))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Method, Candle};
    use crate::core::ValueType;
    use crate::helpers::RandomCandles;
    use crate::methods::tests::test_const;


    #[test]
    fn test_average_true_range_const() {
        use super::AverageTrueRange as TestingMethod;
        for i in 1..255 {
            let input = RandomCandles::new();
            let mut method = TestingMethod::new(i, &input).unwrap();
            let output = method.next(&input);
            test_const(&mut method, input, output);
        }
    }
}
