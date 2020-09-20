#![feature(test)]
use yata::helpers::RandomCandles;
use yata::methods::SMM;
use yata::prelude::*;
extern crate test;

#[bench]
fn bench_smm_w10(b: &mut test::Bencher) {
	let mut candles = RandomCandles::new();
	let mut smm = SMM::new(10, candles.first().close);
	b.iter(|| smm.next(candles.next().unwrap().close))
}

#[bench]
fn bench_smm_w100(b: &mut test::Bencher) {
	let mut candles = RandomCandles::new();
	let mut smm = SMM::new(100, candles.first().close);
	b.iter(|| smm.next(candles.next().unwrap().close))
}
