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

```toml
[dependencies]
yata = "0.6"
```

## Available **moving averages**:

- [Simple moving average (SMA)](https://docs.rs/yata/latest/yata/methods/struct.SMA.html);
- [Weighted moving average (WMA)](https://docs.rs/yata/latest/yata/methods/struct.WMA.html);
- Exponential moving average family: [EMA](https://docs.rs/yata/latest/yata/methods/struct.EMA.html),
  [DMA](https://docs.rs/yata/latest/yata/methods/struct.DMA.html), [TMA](https://docs.rs/yata/latest/yata/methods/struct.TMA.html),
  [DEMA](https://docs.rs/yata/latest/yata/methods/struct.DEMA.html), [TEMA](https://docs.rs/yata/latest/yata/methods/struct.TEMA.html);
- [Simple moving median (SMM)](https://docs.rs/yata/latest/yata/methods/struct.SMM.html);
- [Linear regression moving average (LSMA)](https://docs.rs/yata/latest/yata/methods/struct.LinReg.html);
- [Volume weighted moving average (VWMA)](https://docs.rs/yata/latest/yata/methods/struct.VWMA.html);
- [Symmetrically weighted moving average (SWMA)](https://docs.rs/yata/latest/yata/methods/struct.SWMA.html);
- [Hull moving average (HMA)](https://docs.rs/yata/latest/yata/methods/struct.HMA.html);
- [Running Moving Average (RMA)](https://docs.rs/yata/latest/yata/methods/struct.RMA.html);
- [Triangular Moving Average (TRIMA)](https://docs.rs/yata/latest/yata/methods/struct.TRIMA.html);
- [Wilder’s Smoothing Average (WSMA)](https://docs.rs/yata/latest/yata/methods/struct.WSMA.html);
- [Kaufman Adaptive Moving Average (KAMA)](https://docs.rs/yata/latest/yata/indicators/struct.Kaufman.html);
- [Convolution Moving Average](https://docs.rs/yata/latest/yata/methods/struct.Conv.html);
- [Variable Index Dynamic Average (Vidya)](https://docs.rs/yata/latest/yata/methods/struct.Vidya.html);

[See all](https://docs.rs/yata/latest/yata/methods/index.html#structs)

## Timeseries conversion

- [Timeframe Collapsing](https://docs.rs/yata/latest/yata/methods/struct.CollapseTimeframe.html);
- [Heikin Ashi](https://docs.rs/yata/latest/yata/methods/struct.HeikinAshi.html);
- [Renko](https://docs.rs/yata/latest/yata/methods/struct.Renko.html);

## Some commonly used **methods**:

- [Accumulation-distribution index](https://docs.rs/yata/latest/yata/methods/struct.ADI.html);
- [Commodity channel index](https://docs.rs/yata/latest/yata/methods/struct.CCI.html);
- [Cross](https://docs.rs/yata/latest/yata/methods/struct.Cross.html) / [CrossAbove](https://docs.rs/yata/latest/yata/methods/struct.CrossAbove.html) / [CrossUnder](https://docs.rs/yata/latest/yata/methods/struct.CrossUnder.html);
- [Derivative](https://docs.rs/yata/latest/yata/methods/struct.Derivative.html) (differential);
- [Highest](https://docs.rs/yata/latest/yata/methods/struct.Highest.html) / [Lowest](https://docs.rs/yata/latest/yata/methods/struct.Lowest.html) / [Highest-Lowest Delta](https://docs.rs/yata/latest/yata/methods/struct.HighestLowestDelta.html);
- [Highest Index](https://docs.rs/yata/latest/yata/methods/struct.HighestIndex.html) / [Lowest Index](https://docs.rs/yata/latest/yata/methods/struct.LowestIndex.html);
- [Integral](https://docs.rs/yata/latest/yata/methods/struct.Integral.html) (sum);
- [Mean absolute deviation](https://docs.rs/yata/latest/yata/methods/struct.MeanAbsDev.html);
- [Median absolute deviation](https://docs.rs/yata/latest/yata/methods/struct.MedianAbsDev.html);
- [Momentum](https://docs.rs/yata/latest/yata/methods/struct.Momentum.html);
- [Past](https://docs.rs/yata/latest/yata/methods/struct.Past.html);
- [Rate Of Change](https://docs.rs/yata/latest/yata/methods/struct.RateOfChange.html) (ROC);
- [Reversal points](https://docs.rs/yata/latest/yata/methods/struct.ReversalSignal.html);
- [Standard Deviation](https://docs.rs/yata/latest/yata/methods/struct.StDev.html);
- [True Range](https://docs.rs/yata/latest/yata/methods/struct.TR.html);
- [True Strength Index](https://docs.rs/yata/latest/yata/methods/struct.TSI.html);
- [Volatility](https://docs.rs/yata/latest/yata/methods/struct.LinearVolatility.html);

[See all](https://docs.rs/yata/latest/yata/methods/index.html#structs)

## Some commonly used **indicators**:

- [Average Directional Index](https://docs.rs/yata/latest/yata/indicators/struct.AverageDirectionalIndex.html);
- [Awesome Oscillator](https://docs.rs/yata/latest/yata/indicators/struct.AwesomeOscillator.html);
- [Bollinger Bands](https://docs.rs/yata/latest/yata/indicators/struct.BollingerBands.html);
- [Commodity Channel Index](https://docs.rs/yata/latest/yata/indicators/struct.CommodityChannelIndex.html);
- [Detrended Price Oscillator](https://docs.rs/yata/latest/yata/indicators/struct.DetrendedPriceOscillator.html);
- [Ease Of Movement](https://docs.rs/yata/latest/yata/indicators/struct.EaseOfMovement.html);
- [Elders Force Index](https://docs.rs/yata/latest/yata/indicators/struct.EldersForceIndex.html);
- [Envelopes](https://docs.rs/yata/latest/yata/indicators/struct.Envelopes.html);
- [Fisher Transform](https://docs.rs/yata/latest/yata/indicators/struct.FisherTransform.html);
- [Ichimoku Cloud](https://docs.rs/yata/latest/yata/indicators/struct.IchimokuCloud.html);
- [Keltner Channels](https://docs.rs/yata/latest/yata/indicators/struct.KeltnerChannel.html);
- [Moving Average Convergence Divergence (MACD)](https://docs.rs/yata/latest/yata/indicators/struct.MACD.html);
- [Money Flow Index](https://docs.rs/yata/latest/yata/indicators/struct.MoneyFlowIndex.html);
- [Price Channel Strategy](https://docs.rs/yata/latest/yata/indicators/struct.PriceChannelStrategy.html);
- [Relative Strength Index (RSI)](https://docs.rs/yata/latest/yata/indicators/struct.RelativeStrengthIndex.html);
- [Stochastic Oscillator](https://docs.rs/yata/latest/yata/indicators/struct.StochasticOscillator.html);
- [Trix](https://docs.rs/yata/latest/yata/indicators/struct.Trix.html);
- [Woodies CCI](https://docs.rs/yata/latest/yata/indicators/struct.WoodiesCCI.html);

And many others. [See all](https://docs.rs/yata/latest/yata/indicators/index.html#structs)

## Method usage example

```rust
use yata::prelude::*;
use yata::methods::EMA;

// EMA of length=3
let mut ema = EMA::new(3, &3.0).unwrap();

ema.next(&3.0);
ema.next(&6.0);

assert_eq!(ema.next(&9.0), 6.75);
assert_eq!(ema.next(&12.0), 9.375);
```

## Indicator usage example

```rust
use yata::helpers::{RandomCandles, MA};
use yata::indicators::MACD;
use yata::prelude::*;

let mut candles = RandomCandles::new();
let mut macd = MACD::default();

macd.method1 = "sma-4".parse().unwrap(); // one way of defining methods inside indicators

macd.signal = MA::TEMA(5); // another way of defining methods inside indicators

let mut macd = macd.init(&candles.first()).unwrap();

for candle in candles.take(10) {
	let result = macd.next(&candle);

	println!("{:?}", result);
}
```

# Benchmarks

## Methods

- **\_w10** - method with window `length`=10
- **\_w100** - method with window `length`=100

```
test bench_adi_w10                   ... bench:           7 ns/iter (+/- 0)
test bench_adi_w100                  ... bench:           7 ns/iter (+/- 0)
test bench_cci_w10                   ... bench:          16 ns/iter (+/- 0)
test bench_cci_w100                  ... bench:         133 ns/iter (+/- 8)
test bench_conv_w10                  ... bench:          23 ns/iter (+/- 2)
test bench_conv_w100                 ... bench:         197 ns/iter (+/- 0)
test bench_cross                     ... bench:           5 ns/iter (+/- 0)
test bench_cross_above               ... bench:           3 ns/iter (+/- 0)
test bench_cross_under               ... bench:           3 ns/iter (+/- 0)
test bench_dema_w10                  ... bench:           7 ns/iter (+/- 0)
test bench_dema_w100                 ... bench:           6 ns/iter (+/- 0)
test bench_derivative_w10            ... bench:           4 ns/iter (+/- 0)
test bench_derivative_w100           ... bench:           3 ns/iter (+/- 0)
test bench_dma_w10                   ... bench:           5 ns/iter (+/- 0)
test bench_dma_w100                  ... bench:           5 ns/iter (+/- 0)
test bench_ema_w10                   ... bench:           5 ns/iter (+/- 0)
test bench_ema_w100                  ... bench:           5 ns/iter (+/- 0)
test bench_heikin_ashi               ... bench:           4 ns/iter (+/- 0)
test bench_highest_index_w10         ... bench:           6 ns/iter (+/- 0)
test bench_highest_index_w100        ... bench:           6 ns/iter (+/- 0)
test bench_highest_lowest_delta_w10  ... bench:          10 ns/iter (+/- 0)
test bench_highest_lowest_delta_w100 ... bench:          10 ns/iter (+/- 0)
test bench_highest_w10               ... bench:           6 ns/iter (+/- 0)
test bench_highest_w100              ... bench:           7 ns/iter (+/- 0)
test bench_hma_w10                   ... bench:          14 ns/iter (+/- 0)
test bench_hma_w100                  ... bench:          15 ns/iter (+/- 0)
test bench_integral_w10              ... bench:           7 ns/iter (+/- 0)
test bench_integral_w100             ... bench:           7 ns/iter (+/- 0)
test bench_lin_reg_w10               ... bench:           8 ns/iter (+/- 1)
test bench_lin_reg_w100              ... bench:           8 ns/iter (+/- 0)
test bench_linear_volatility_w10     ... bench:           4 ns/iter (+/- 0)
test bench_linear_volatility_w100    ... bench:           4 ns/iter (+/- 0)
test bench_lowest_index_w10          ... bench:           6 ns/iter (+/- 0)
test bench_lowest_index_w100         ... bench:           7 ns/iter (+/- 0)
test bench_lowest_w10                ... bench:           6 ns/iter (+/- 0)
test bench_lowest_w100               ... bench:           6 ns/iter (+/- 0)
test bench_mean_abs_dev_w10          ... bench:          11 ns/iter (+/- 0)
test bench_mean_abs_dev_w100         ... bench:         123 ns/iter (+/- 4)
test bench_median_abs_dev_w10        ... bench:          31 ns/iter (+/- 7)
test bench_median_abs_dev_w100       ... bench:         190 ns/iter (+/- 8)
test bench_momentum_w10              ... bench:           3 ns/iter (+/- 0)
test bench_momentum_w100             ... bench:           3 ns/iter (+/- 0)
test bench_past_w10                  ... bench:           3 ns/iter (+/- 0)
test bench_past_w100                 ... bench:           3 ns/iter (+/- 0)
test bench_rate_of_change_w10        ... bench:           3 ns/iter (+/- 0)
test bench_rate_of_change_w100       ... bench:           3 ns/iter (+/- 0)
test bench_reverse_high_w10          ... bench:           5 ns/iter (+/- 0)
test bench_reverse_high_w100         ... bench:           5 ns/iter (+/- 0)
test bench_reverse_low_w10           ... bench:           5 ns/iter (+/- 0)
test bench_reverse_low_w100          ... bench:           5 ns/iter (+/- 0)
test bench_reverse_signal_w10        ... bench:           9 ns/iter (+/- 0)
test bench_reverse_signal_w100       ... bench:           9 ns/iter (+/- 0)
test bench_rma_w10                   ... bench:           4 ns/iter (+/- 0)
test bench_rma_w100                  ... bench:           4 ns/iter (+/- 0)
test bench_sma_w10                   ... bench:           3 ns/iter (+/- 0)
test bench_sma_w100                  ... bench:           3 ns/iter (+/- 0)
test bench_smm_w10                   ... bench:          17 ns/iter (+/- 1)
test bench_smm_w100                  ... bench:          35 ns/iter (+/- 2)
test bench_st_dev_w10                ... bench:           7 ns/iter (+/- 0)
test bench_st_dev_w100               ... bench:           7 ns/iter (+/- 0)
test bench_swma_w10                  ... bench:           8 ns/iter (+/- 0)
test bench_swma_w100                 ... bench:           8 ns/iter (+/- 0)
test bench_tema_w10                  ... bench:           8 ns/iter (+/- 0)
test bench_tema_w100                 ... bench:           7 ns/iter (+/- 0)
test bench_tma_w10                   ... bench:           5 ns/iter (+/- 0)
test bench_tma_w100                  ... bench:           5 ns/iter (+/- 1)
test bench_trima_w10                 ... bench:           5 ns/iter (+/- 0)
test bench_trima_w100                ... bench:           5 ns/iter (+/- 1)
test bench_tsi_w10                   ... bench:           9 ns/iter (+/- 0)
test bench_tsi_w100                  ... bench:          10 ns/iter (+/- 0)
test bench_vidya_w10                 ... bench:           8 ns/iter (+/- 1)
test bench_vidya_w100                ... bench:           8 ns/iter (+/- 0)
test bench_vwma_w10                  ... bench:           5 ns/iter (+/- 0)
test bench_vwma_w100                 ... bench:           5 ns/iter (+/- 0)
test bench_wma_w10                   ... bench:           6 ns/iter (+/- 1)
test bench_wma_w100                  ... bench:           6 ns/iter (+/- 0)
```

## Indicators

```
test bench_awesome_oscillator                  ... bench:          36 ns/iter (+/- 0)
test bench_bollinger_bands                     ... bench:          53 ns/iter (+/- 2)
test bench_chaikin_money_flow                  ... bench:          22 ns/iter (+/- 0)
test bench_chaikin_oscillator                  ... bench:          23 ns/iter (+/- 0)
test bench_chande_kroll_stop                   ... bench:          58 ns/iter (+/- 0)
test bench_chande_momentum_oscillator          ... bench:          25 ns/iter (+/- 0)
test bench_commodity_channel_index             ... bench:          38 ns/iter (+/- 0)
test bench_coppock_curve                       ... bench:          38 ns/iter (+/- 2)
test bench_detrended_price_oscillator          ... bench:          10 ns/iter (+/- 0)
test bench_ease_of_movement                    ... bench:          20 ns/iter (+/- 0)
test bench_elders_force_index                  ... bench:          17 ns/iter (+/- 1)
test bench_envelopes                           ... bench:          14 ns/iter (+/- 0)
test bench_fisher_transform                    ... bench:         125 ns/iter (+/- 13)
test bench_hull_moving_average                 ... bench:          28 ns/iter (+/- 0)
test bench_ichimoku_cloud                      ... bench:          65 ns/iter (+/- 7)
test bench_indicator_aroon                     ... bench:          46 ns/iter (+/- 12)
test bench_indicator_average_directional_index ... bench:          49 ns/iter (+/- 1)
test bench_kaufman                             ... bench:          34 ns/iter (+/- 3)
test bench_keltner_channel                     ... bench:          26 ns/iter (+/- 2)
test bench_klinger_volume_oscillator           ... bench:          40 ns/iter (+/- 1)
test bench_know_sure_thing                     ... bench:          41 ns/iter (+/- 0)
test bench_macd                                ... bench:          23 ns/iter (+/- 0)
test bench_momentum_index                      ... bench:          13 ns/iter (+/- 0)
test bench_money_flow_index                    ... bench:          59 ns/iter (+/- 12)
test bench_parabolic_sar                       ... bench:          17 ns/iter (+/- 2)
test bench_pivot_reversal_strategy             ... bench:          30 ns/iter (+/- 4)
test bench_relative_strength_index             ... bench:          35 ns/iter (+/- 1)
test bench_relative_vigor_index                ... bench:          59 ns/iter (+/- 0)
test bench_smi_ergodic_indicator               ... bench:          30 ns/iter (+/- 2)
test bench_stochastic_oscillator               ... bench:          48 ns/iter (+/- 4)
test bench_trend_strength_index                ... bench:          46 ns/iter (+/- 2)
test bench_trix                                ... bench:          32 ns/iter (+/- 10)
test bench_true_strength_index                 ... bench:          38 ns/iter (+/- 16)
test bench_woodies_cci                         ... bench:          68 ns/iter (+/- 6)
```

# Current unsafe status

By default, there is no `unsafe` code in the crate. But you can optionally enable `unsafe_performance` feature throw you `Cargo.toml` or by `--feature` flag in your CLI.

`unsafe_performance` enables some unsafe code blocks, most of them are unsafe access to a vector's elements. For some methods it may increase performance by ~5-10%.

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
