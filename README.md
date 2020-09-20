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

## Some commonly used **methods**:

- [Accumulation-distribution index](https://docs.rs/yata/latest/yata/methods/struct.ADI.html);
- [Cross](https://docs.rs/yata/latest/yata/methods/struct.Cross.html) / [CrossAbove](https://docs.rs/yata/latest/yata/methods/struct.CrossAbove.html) / [CrossUnder](https://docs.rs/yata/latest/yata/methods/struct.CrossUnder.html);
- [Derivative](https://docs.rs/yata/latest/yata/methods/struct.Derivative.html) (differential)
- [Highest](https://docs.rs/yata/latest/yata/methods/struct.Highest.html) / [Lowest](https://docs.rs/yata/latest/yata/methods/struct.Lowest.html) / [Highest-Lowest Delta](https://docs.rs/yata/latest/yata/methods/struct.HighestLowestDelta.html)
- [Hull moving average](https://docs.rs/yata/latest/yata/methods/struct.HMA.html)
- [Integral](https://docs.rs/yata/latest/yata/methods/struct.Integral.html) (sum)
- [Linear regression moving average](https://docs.rs/yata/latest/yata/methods/struct.LinReg.html)
- [Momentum](https://docs.rs/yata/latest/yata/methods/struct.Momentum.html)
- [Pivot points](https://docs.rs/yata/latest/yata/methods/struct.PivotSignal.html)
- [Simple moving average](https://docs.rs/yata/latest/yata/methods/struct.SMA.html)
- [Weighted moving average](https://docs.rs/yata/latest/yata/methods/struct.WMA.html)
- [Volume weighted moving average](https://docs.rs/yata/latest/yata/methods/struct.VWMA.html)
- Exponential moving average family: [EMA](https://docs.rs/yata/latest/yata/methods/struct.EMA.html), [DMA](https://docs.rs/yata/latest/yata/methods/struct.DMA.html), [TMA](https://docs.rs/yata/latest/yata/methods/struct.TMA.html), [DEMA](https://docs.rs/yata/latest/yata/methods/struct.DEMA.html), [TEMA](https://docs.rs/yata/latest/yata/methods/struct.TEMA.html)
- [Symmetrically weighted moving average](https://docs.rs/yata/latest/yata/methods/struct.SWMA.html)

And many others. [See all](https://docs.rs/yata/latest/yata/methods/index.html#structs)

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
let mut ema = EMA::new(3, 3.0);

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

let mut candles = RandomCandles::new();
let mut macd = MACD::default();
macd.period3 = 4; // setting signal period MA to 4

macd.method1 = "sma".into(); // one way of defining methods inside indicators

macd.method3 = RegularMethods::TEMA; // another way of defining methods inside indicators

let mut macd = macd.init(candles.first());

for candle in candles.take(10) {
	let result = macd.next(candle);

	println!("{:?}", result);
}
```

## Current usafe status

Currently there is no `unsafe` code in the crate.

## Suggestions

You are welcome to [give any suggestions](https://github.com/amv-dev/yata/issues) about implementing new indicators and methods.

# Say thanks

If you like this library and you want to say thanks, you can do it also by donating to bitcoin address _1P3gTnaTK9LKSYx2nETrKe2zjP4HMkdhvK_
