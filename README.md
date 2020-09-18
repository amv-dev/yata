# YATA

Yet Another Technical Analysis library

YaTa implements most common technical analysis **methods** and **indicators**.

It also provides you an interface to create your own indicators.

## Some commonly used **methods**:

- Accumulation-distribution index;
- Cross / CrossAbove / CrossUnder;
- Derivative (differential)
- Highest / Lowest / Highest-Lowest Delta
- Hull moving average
- Integral (sum)
- Linear regression moving average
- Momentum
- Pivot points
- Simple moving average
- Weighted moving average
- Volume weighted moving average
- Exponential moving average family: EMA, DMA, TMA DEMA, TEMA
- Symmetrically weighted moving average

And many others.

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

And many others

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
