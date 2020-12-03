# YATA

## Yet Another Technical Analysis library

[![Crates.io](https://img.shields.io/crates/v/yata?style=for-the-badge)](https://crates.io/crates/yata)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/amv-dev/yata/Rust?style=for-the-badge)
[![Read docs](https://img.shields.io/badge/docs.rs-read-brightgreen?style=for-the-badge)](https://docs.rs/yata/latest/yata/)
[![License Apache 2.0](https://img.shields.io/github/license/amv-dev/yata.svg?style=for-the-badge)](https://github.com/amv-dev/yata/blob/master/LICENSE)
[![GitHub issues](https://img.shields.io/github/issues/amv-dev/yata?style=for-the-badge)](https://github.com/amv-dev/yata/issues)
[![Made with Rust](https://forthebadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/)

YaTa implements most common technical analysis [methods](https://docs.rs/yata/latest/yata/methods/index.html#structs)
and [indicators](https://docs.rs/yata/latest/yata/indicators/index.html#structs).

It also provides you an interface to create your own indicators.

```yaml
[dependencies]
yata = "0.3"
```

## Available moving averages:

- [Simple moving average (SMA)](https://docs.rs/yata/latest/yata/methods/struct.SMA.html)
- [Weighted moving average (WMA)](https://docs.rs/yata/latest/yata/methods/struct.WMA.html)
- Exponential moving average family: [EMA](https://docs.rs/yata/latest/yata/methods/struct.EMA.html), [DMA](https://docs.rs/yata/latest/yata/methods/struct.DMA.html), [TMA](https://docs.rs/yata/latest/yata/methods/struct.TMA.html), [DEMA](https://docs.rs/yata/latest/yata/methods/struct.DEMA.html), [TEMA](https://docs.rs/yata/latest/yata/methods/struct.TEMA.html)
- [Simple moving median (SMM)](https://docs.rs/yata/latest/yata/methods/struct.SMM.html)
- [Linear regression moving average](https://docs.rs/yata/latest/yata/methods/struct.LinReg.html)
- [Volume weighted moving average (VWMA)](https://docs.rs/yata/latest/yata/methods/struct.VWMA.html)
- [Symmetrically weighted moving average (SWMA)](https://docs.rs/yata/latest/yata/methods/struct.SWMA.html)
- [Hull moving average (HMA)](https://docs.rs/yata/latest/yata/methods/struct.HMA.html)
- [Running Moving Average (RMA)](https://docs.rs/yata/latest/yata/methods/struct.RMA.html)
- [Triangular Moving Average (TRIMA)](https://docs.rs/yata/latest/yata/methods/struct.TRIMA.html)
- [Wilder’s Smoothing Average (WSMA)](https://docs.rs/yata/latest/yata/methods/struct.WSMA.html)
- [Kaufman Adaptive Moving Average (KAMA)](https://docs.rs/yata/latest/yata/indicators/struct.Kaufman.html)
- [Convolution Moving Average](https://docs.rs/yata/latest/yata/methods/struct.Conv.html)

[See all](https://docs.rs/yata/latest/yata/methods/index.html#structs)

## Some commonly used **methods**:

- [Accumulation-distribution index](https://docs.rs/yata/latest/yata/methods/struct.ADI.html);
- [Commodity channel index](https://docs.rs/yata/latest/yata/methods/struct.CCI.html);
- [Cross](https://docs.rs/yata/latest/yata/methods/struct.Cross.html) / [CrossAbove](https://docs.rs/yata/latest/yata/methods/struct.CrossAbove.html) / [CrossUnder](https://docs.rs/yata/latest/yata/methods/struct.CrossUnder.html);
- [Derivative](https://docs.rs/yata/latest/yata/methods/struct.Derivative.html) (differential)
- [Highest](https://docs.rs/yata/latest/yata/methods/struct.Highest.html) / [Lowest](https://docs.rs/yata/latest/yata/methods/struct.Lowest.html) / [Highest-Lowest Delta](https://docs.rs/yata/latest/yata/methods/struct.HighestLowestDelta.html)
- [Highest Index](https://docs.rs/yata/latest/yata/methods/struct.HighestIndex.html) / [Lowest Index](https://docs.rs/yata/latest/yata/methods/struct.LowestIndex.html)
- [Integral](https://docs.rs/yata/latest/yata/methods/struct.Integral.html) (sum)
- [Mean absolute deviation](https://docs.rs/yata/latest/yata/methods/struct.MeanAbsDev.html)
- [Median absolute deviation](https://docs.rs/yata/latest/yata/methods/struct.MedianAbsDev.html)
- [Momentum](https://docs.rs/yata/latest/yata/methods/struct.Momentum.html)
- [Past](https://docs.rs/yata/latest/yata/methods/struct.Past.html)
- [Rate Of Change](https://docs.rs/yata/latest/yata/methods/struct.RateOfChange.html) (ROC)
- [Reverse points](https://docs.rs/yata/latest/yata/methods/struct.ReverseSignal.html)
- [Standart Deviation](https://docs.rs/yata/latest/yata/methods/struct.StDev.html)
- [Volatility](https://docs.rs/yata/latest/yata/methods/struct.LinearVolatility.html)

[See all](https://docs.rs/yata/latest/yata/methods/index.html#structs)

## Some commonly used **indicators**:

- Average Directional Index;
- Awesome Oscillator;
- Bollinger Bands;
- Commodity Channel Index;
- Detrended Price Oscillator;
- Ease Of Movement;
- Elders Force Index;
- Envelopes;
- Fisher Transform;
- Ichimoku Cloud;
- Keltner Channels;
- Moving Average Convergence Divergence (MACD);
- Money Flow Index;
- Price Channel Strategy;
- Relative Strength Index (RSI);
- Stochastic Oscillator;
- Trix;
- Woodies CCI;

And many others. [See all](https://docs.rs/yata/latest/yata/indicators/index.html#structs)

## Method usage example

```
use yata::prelude::*;
use yata::methods::EMA;

// EMA of length=3
let mut ema = EMA::new(3, 3.0).unwrap();

ema.next(3.0);
ema.next(6.0);

assert_eq!(ema.next(9.0), 6.75);
assert_eq!(ema.next(12.0), 9.375);
```

## Indicator usage example

```
use yata::helpers::{RandomCandles, RegularMethods};
use yata::indicators::MACD;
use yata::prelude::*;
use std::convert::TryInto;

let mut candles = RandomCandles::new();
let mut macd = MACD::default();
macd.period3 = 4; // setting signal period MA to 4

macd.method1 = "sma".try_into().unwrap(); // one way of defining methods inside indicators

macd.method3 = RegularMethods::TEMA; // another way of defining methods inside indicators

let mut macd = macd.init(candles.first()).unwrap();

for candle in candles.take(10) {
	let result = macd.next(candle);

	println!("{:?}", result);
}
```

# Benchmarks

## Methods

- **\_w10** - method with window `length`=10
- **\_w100** - method with window `length`=100

```
test bench_adi_w10                   ... bench:          10 ns/iter (+/- 0)
test bench_adi_w100                  ... bench:          10 ns/iter (+/- 0)
test bench_cci_w10                   ... bench:          21 ns/iter (+/- 0)
test bench_cci_w100                  ... bench:          80 ns/iter (+/- 0)
test bench_conv_w10                  ... bench:          12 ns/iter (+/- 1)
test bench_conv_w100                 ... bench:         114 ns/iter (+/- 2)
test bench_cross                     ... bench:           6 ns/iter (+/- 3)
test bench_cross_above               ... bench:           4 ns/iter (+/- 0)
test bench_cross_under               ... bench:           4 ns/iter (+/- 0)
test bench_dema_w10                  ... bench:           9 ns/iter (+/- 0)
test bench_dema_w100                 ... bench:           9 ns/iter (+/- 0)
test bench_derivative_w10            ... bench:           2 ns/iter (+/- 0)
test bench_derivative_w100           ... bench:           2 ns/iter (+/- 0)
test bench_dma_w10                   ... bench:           3 ns/iter (+/- 0)
test bench_dma_w100                  ... bench:           3 ns/iter (+/- 0)
test bench_ema_w10                   ... bench:           7 ns/iter (+/- 1)
test bench_ema_w100                  ... bench:           7 ns/iter (+/- 0)
test bench_highest_index_w10         ... bench:           8 ns/iter (+/- 0)
test bench_highest_index_w100        ... bench:           9 ns/iter (+/- 0)
test bench_highest_lowest_delta_w10  ... bench:          12 ns/iter (+/- 0)
test bench_highest_lowest_delta_w100 ... bench:           5 ns/iter (+/- 0)
test bench_highest_w10               ... bench:           3 ns/iter (+/- 0)
test bench_highest_w100              ... bench:           4 ns/iter (+/- 0)
test bench_hma_w10                   ... bench:           8 ns/iter (+/- 0)
test bench_hma_w100                  ... bench:           8 ns/iter (+/- 0)
test bench_integral_w10              ... bench:           4 ns/iter (+/- 0)
test bench_integral_w100             ... bench:          10 ns/iter (+/- 0)
test bench_lin_reg_w10               ... bench:          11 ns/iter (+/- 0)
test bench_lin_reg_w100              ... bench:          11 ns/iter (+/- 0)
test bench_linear_volatility_w10     ... bench:           5 ns/iter (+/- 0)
test bench_linear_volatility_w100    ... bench:           5 ns/iter (+/- 0)
test bench_lowest_index_w10          ... bench:           7 ns/iter (+/- 4)
test bench_lowest_index_w100         ... bench:           4 ns/iter (+/- 0)
test bench_lowest_w10                ... bench:           3 ns/iter (+/- 0)
test bench_lowest_w100               ... bench:           4 ns/iter (+/- 0)
test bench_mean_abs_dev_w10          ... bench:           6 ns/iter (+/- 0)
test bench_mean_abs_dev_w100         ... bench:          77 ns/iter (+/- 0)
test bench_median_abs_dev_w10        ... bench:          43 ns/iter (+/- 1)
test bench_median_abs_dev_w100       ... bench:         288 ns/iter (+/- 1)
test bench_momentum_w10              ... bench:           4 ns/iter (+/- 0)
test bench_momentum_w100             ... bench:           4 ns/iter (+/- 0)
test bench_past_w10                  ... bench:           2 ns/iter (+/- 0)
test bench_past_w100                 ... bench:           2 ns/iter (+/- 0)
test bench_rate_of_change_w10        ... bench:           2 ns/iter (+/- 0)
test bench_rate_of_change_w100       ... bench:           2 ns/iter (+/- 0)
test bench_reverse_high_w10          ... bench:           5 ns/iter (+/- 0)
test bench_reverse_high_w100         ... bench:          10 ns/iter (+/- 0)
test bench_reverse_low_w10           ... bench:          12 ns/iter (+/- 0)
test bench_reverse_low_w100          ... bench:          10 ns/iter (+/- 0)
test bench_reverse_signal_w10        ... bench:          23 ns/iter (+/- 0)
test bench_reverse_signal_w100       ... bench:          20 ns/iter (+/- 0)
test bench_rma_w10                   ... bench:           7 ns/iter (+/- 0)
test bench_rma_w100                  ... bench:           3 ns/iter (+/- 0)
test bench_sma_w10                   ... bench:           2 ns/iter (+/- 0)
test bench_sma_w100                  ... bench:           2 ns/iter (+/- 0)
test bench_smm_w10                   ... bench:          14 ns/iter (+/- 0)
test bench_smm_w100                  ... bench:          44 ns/iter (+/- 4)
test bench_st_dev_w10                ... bench:          11 ns/iter (+/- 0)
test bench_st_dev_w100               ... bench:          10 ns/iter (+/- 0)
test bench_swma_w10                  ... bench:          12 ns/iter (+/- 1)
test bench_swma_w100                 ... bench:          12 ns/iter (+/- 0)
test bench_tema_w10                  ... bench:          11 ns/iter (+/- 0)
test bench_tema_w100                 ... bench:           5 ns/iter (+/- 0)
test bench_tma_w10                   ... bench:           4 ns/iter (+/- 0)
test bench_tma_w100                  ... bench:           4 ns/iter (+/- 0)
test bench_trima_w10                 ... bench:           3 ns/iter (+/- 0)
test bench_trima_w100                ... bench:           3 ns/iter (+/- 0)
test bench_vwma_w10                  ... bench:           3 ns/iter (+/- 0)
test bench_vwma_w100                 ... bench:           6 ns/iter (+/- 0)
test bench_wma_w10                   ... bench:           8 ns/iter (+/- 0)
test bench_wma_w100                  ... bench:           8 ns/iter (+/- 0)
```

## Indicators

```
test bench_awsome_oscillator                   ... bench:          27 ns/iter (+/- 4)
test bench_detrended_price_oscillator          ... bench:           9 ns/iter (+/- 11)
test bench_indicator_aroon                     ... bench:          65 ns/iter (+/- 1)
test bench_indicator_average_directional_index ... bench:          69 ns/iter (+/- 0)
```

# Current unsafe status

By default, there is no `unsafe` code in the crate. But you can optionally enable `unsafe_performance` feature throw you `Cargo.toml` or by `--feature` flag in your CLI.

`usafe_performance` enables some unsafe code blocks, most of them are unsafe access to a vector's elements. For some methods it may increase performance by ~5-10%.

# Features

- `serde` - enables [`serde`](https://crates.io/crates/serde) crate support;
- `period_type_u16` - sets `PeriodType` to `u16`;
- `period_type_u32` - sets `PeriodType` to `u32`;
- `period_type_u64` - sets `PeriodType` to `u64`;
- `value_type_f32` - sets `ValueType` to `f32`;
- `unsafe_performance` - enables optional unsafe code blocks, which may increase performance;

# Rust version

YaTa library supports **Rust stable** except that you can't run benchmarks with it.

# Suggestions

You are welcome to [give any suggestions](https://github.com/amv-dev/yata/issues) about implementing new indicators and methods.

# Say thanks

If you like this library, and you want to say thanks, you can do it also by donating to bitcoin address `1P3gTnaTK9LKSYx2nETrKe2zjP4HMkdhvK`
