#![feature(test)]
use yata::core::Candle;
use yata::helpers::RandomCandles;
use yata::indicators::*;
use yata::prelude::*;

extern crate test;

fn bench_indicator<T: IndicatorConfig + IndicatorInitializer<Candle> + Default>(
	b: &mut test::Bencher,
) {
	let candles: Vec<_> = RandomCandles::new().take(1000).collect();
	let mut iter = candles.iter().copied().cycle();
	let mut indicator = T::default().init(iter.next().unwrap());

	for _ in 0..50 {
		indicator.next(iter.next().unwrap());
	}

	b.iter(|| indicator.next(iter.next().unwrap()))
}

#[bench]
fn bench_indicator_aroon(b: &mut test::Bencher) {
	bench_indicator::<Aroon>(b);
}

#[bench]
fn bench_indicator_average_directional_index(b: &mut test::Bencher) {
	bench_indicator::<AverageDirectionalIndex>(b);
}
