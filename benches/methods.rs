#![feature(test)]
use yata::core::ValueType;
use yata::helpers::RandomCandles;
use yata::methods::*;
use yata::prelude::Method;

extern crate test;

// ADI -----------------------------------------------------------------------------------
#[bench]
fn bench_adi_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = ADI::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_adi_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = ADI::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// Conv -----------------------------------------------------------------------------------
#[bench]
fn bench_conv_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Conv::new((0..10).map(|x| x as ValueType).collect(), candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_conv_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Conv::new((0..100).map(|x| x as ValueType).collect(), candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// Cross -----------------------------------------------------------------------------------
#[bench]
fn bench_cross(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new()
		.take(1000)
		.map(|c| c.close)
		.zip(RandomCandles::new().skip(15).take(1000).map(|c| c.close))
		.collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Cross::new((), candles[0]);
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_cross_above(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new()
		.take(1000)
		.map(|c| c.close)
		.zip(RandomCandles::new().skip(15).take(1000).map(|c| c.close))
		.collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = CrossAbove::new((), candles[0]);
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_cross_under(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new()
		.take(1000)
		.map(|c| c.close)
		.zip(RandomCandles::new().skip(15).take(1000).map(|c| c.close))
		.collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = CrossUnder::new((), candles[0]);
	b.iter(|| method.next(iter.next().unwrap()))
}

// SMA -----------------------------------------------------------------------------------
#[bench]
fn bench_sma_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = SMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_sma_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = SMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// WMA -----------------------------------------------------------------------------------
#[bench]
fn bench_wma_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = WMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_wma_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = WMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// EMA -----------------------------------------------------------------------------------
#[bench]
fn bench_ema_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = EMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_ema_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = EMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// DMA -----------------------------------------------------------------------------------
#[bench]
fn bench_dma_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = DMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_dma_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = DMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// TMA -----------------------------------------------------------------------------------
#[bench]
fn bench_tma_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = TMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_tma_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = TMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// DEMA -----------------------------------------------------------------------------------
#[bench]
fn bench_dema_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = DEMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_dema_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = DEMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// TEMA -----------------------------------------------------------------------------------
#[bench]
fn bench_tema_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = TEMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_tema_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = TEMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// SMM -----------------------------------------------------------------------------------
#[bench]
fn bench_smm_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = SMM::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_smm_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = SMM::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// HMA -----------------------------------------------------------------------------------
#[bench]
fn bench_hma_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = HMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_hma_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = HMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// LinReg -----------------------------------------------------------------------------------
#[bench]
fn bench_lin_reg_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = LinReg::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_lin_reg_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = LinReg::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// Derivative -----------------------------------------------------------------------------------
#[bench]
fn bench_derivative_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Derivative::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_derivative_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Derivative::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// Integral -----------------------------------------------------------------------------------
#[bench]
fn bench_integral_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Integral::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_integral_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Integral::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// Momentum -----------------------------------------------------------------------------------
#[bench]
fn bench_momentum_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Momentum::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_momentum_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Momentum::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// Past -----------------------------------------------------------------------------------
#[bench]
fn bench_past_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Past::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_past_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Past::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// RateOfChange -----------------------------------------------------------------------------------
#[bench]
fn bench_rate_of_change_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = RateOfChange::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_rate_of_change_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = RateOfChange::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// Reverse -----------------------------------------------------------------------------------
#[bench]
fn bench_reverse_signal_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = ReverseSignal::new(5, 5, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_reverse_signal_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = ReverseSignal::new(50, 50, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// ReverseLow -----------------------------------------------------------------------------------
#[bench]
fn bench_reverse_low_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = ReverseLowSignal::new(5, 5, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_reverse_low_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = ReverseLowSignal::new(50, 50, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// ReverseHigh -----------------------------------------------------------------------------------
#[bench]
fn bench_reverse_high_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = ReverseLowSignal::new(5, 5, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_reverse_high_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = ReverseHighSignal::new(50, 50, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// RMA -----------------------------------------------------------------------------------
#[bench]
fn bench_rma_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = RMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_rma_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = RMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// StDev -----------------------------------------------------------------------------------
#[bench]
fn bench_st_dev_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = StDev::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_st_dev_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = StDev::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// SWMA -----------------------------------------------------------------------------------
#[bench]
fn bench_swma_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = SWMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_swma_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = SWMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// TRIMA -----------------------------------------------------------------------------------
#[bench]
fn bench_trima_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = TRIMA::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_trima_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = TRIMA::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// LinearVolatility -----------------------------------------------------------------------------------
#[bench]
fn bench_linear_volatility_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = LinearVolatility::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_linear_volatility_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = LinearVolatility::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// VWMA -----------------------------------------------------------------------------------
#[bench]
fn bench_vwma_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = VWMA::new(10, (candles[0].close, candles[0].volume));
	for _ in 0..10 {
		let candle = iter.next().unwrap();
		method.next((candle.close, candle.volume));
	}
	b.iter(|| {
		let candle = iter.next().unwrap();
		method.next((candle.close, candle.volume))
	})
}

#[bench]
fn bench_vwma_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = VWMA::new(100, (candles[0].close, candles[0].volume));
	for _ in 0..100 {
		let candle = iter.next().unwrap();
		method.next((candle.close, candle.volume));
	}
	b.iter(|| {
		let candle = iter.next().unwrap();
		method.next((candle.close, candle.volume))
	})
}

// Highest -----------------------------------------------------------------------------------
#[bench]
fn bench_highest_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Highest::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_highest_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Highest::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// Lowest -----------------------------------------------------------------------------------
#[bench]
fn bench_lowest_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Lowest::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_lowest_w100(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = Lowest::new(100, candles[0]);
	for _ in 0..100 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

// HighestLowestDelta -----------------------------------------------------------------------------------
#[bench]
fn bench_highest_lowest_delta_w10(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).map(|c| c.close).collect();
	let mut iter = candles.iter().cycle().copied();
	let mut method = HighestLowestDelta::new(10, candles[0]);
	for _ in 0..10 {
		method.next(iter.next().unwrap());
	}
	b.iter(|| method.next(iter.next().unwrap()))
}

#[bench]
fn bench_highest_lowest_delta_w100(b: &mut test::Bencher) {
	let mut candles = RandomCandles::new();
	let mut method = HighestLowestDelta::new(100, candles.first().close);
	for _ in 0..100 {
		method.next(candles.next().unwrap().close);
	}
	b.iter(|| method.next(candles.next().unwrap().close))
}
