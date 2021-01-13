#![feature(test)]
// use yata::core::Candle;
use yata::helpers::RandomCandles;
use yata::indicators::*;
use yata::prelude::*;

extern crate test;

fn bench_indicator<T: IndicatorConfig + Default>(b: &mut test::Bencher) {
	let candles: Vec<_> = RandomCandles::new().take(1000).collect();
	let mut iter = candles.iter().cycle();
	let mut indicator = T::default().init(iter.next().unwrap()).unwrap();

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

#[bench]
fn bench_awesome_oscillator(b: &mut test::Bencher) {
	bench_indicator::<AwesomeOscillator>(b);
}

#[bench]
fn bench_bollinger_bands(b: &mut test::Bencher) {
	bench_indicator::<BollingerBands>(b);
}

#[bench]
fn bench_chaikin_money_flow(b: &mut test::Bencher) {
	bench_indicator::<ChaikinMoneyFlow>(b);
}

#[bench]
fn bench_chaikin_oscillator(b: &mut test::Bencher) {
	bench_indicator::<ChaikinOscillator>(b);
}

#[bench]
fn bench_chande_kroll_stop(b: &mut test::Bencher) {
	bench_indicator::<ChandeKrollStop>(b);
}

#[bench]
fn bench_chande_momentum_oscillator(b: &mut test::Bencher) {
	bench_indicator::<ChandeMomentumOscillator>(b);
}

#[bench]
fn bench_commodity_channel_index(b: &mut test::Bencher) {
	bench_indicator::<CommodityChannelIndex>(b);
}

#[bench]
fn bench_coppock_curve(b: &mut test::Bencher) {
	bench_indicator::<CoppockCurve>(b);
}

#[bench]
fn bench_detrended_price_oscillator(b: &mut test::Bencher) {
	bench_indicator::<DetrendedPriceOscillator>(b);
}

#[bench]
fn bench_donchian_channel(b: &mut test::Bencher) {
	bench_indicator::<DonchianChannel>(b);
}

#[bench]
fn bench_ease_of_movement(b: &mut test::Bencher) {
	bench_indicator::<EaseOfMovement>(b);
}

#[bench]
fn bench_elders_force_index(b: &mut test::Bencher) {
	bench_indicator::<EldersForceIndex>(b);
}

#[bench]
fn bench_envelopes(b: &mut test::Bencher) {
	bench_indicator::<Envelopes>(b);
}

#[bench]
fn bench_fisher_transform(b: &mut test::Bencher) {
	bench_indicator::<FisherTransform>(b);
}

#[bench]
fn bench_hull_moving_average(b: &mut test::Bencher) {
	bench_indicator::<HullMovingAverage>(b);
}

#[bench]
fn bench_ichimoku_cloud(b: &mut test::Bencher) {
	bench_indicator::<IchimokuCloud>(b);
}

#[bench]
fn bench_kaufman(b: &mut test::Bencher) {
	bench_indicator::<Kaufman>(b);
}

#[bench]
fn bench_keltner_channel(b: &mut test::Bencher) {
	bench_indicator::<KeltnerChannel>(b);
}

#[bench]
fn bench_klinger_volume_oscillator(b: &mut test::Bencher) {
	bench_indicator::<KlingerVolumeOscillator>(b);
}

#[bench]
fn bench_know_sure_thing(b: &mut test::Bencher) {
	bench_indicator::<KnowSureThing>(b);
}

#[bench]
fn bench_macd(b: &mut test::Bencher) {
	bench_indicator::<MACD>(b);
}

#[bench]
fn bench_momentum_index(b: &mut test::Bencher) {
	bench_indicator::<MomentumIndex>(b);
}

#[bench]
fn bench_money_flow_index(b: &mut test::Bencher) {
	bench_indicator::<MoneyFlowIndex>(b);
}

#[bench]
fn bench_parabolic_sar(b: &mut test::Bencher) {
	bench_indicator::<ParabolicSAR>(b);
}

#[bench]
fn bench_pivot_reversal_strategy(b: &mut test::Bencher) {
	bench_indicator::<PivotReversalStrategy>(b);
}

#[bench]
fn bench_relative_strength_index(b: &mut test::Bencher) {
	bench_indicator::<RelativeStrengthIndex>(b);
}

#[bench]
fn bench_relative_vigor_index(b: &mut test::Bencher) {
	bench_indicator::<RelativeVigorIndex>(b);
}

#[bench]
fn bench_smi_ergodic_indicator(b: &mut test::Bencher) {
	bench_indicator::<SMIErgodicIndicator>(b);
}

#[bench]
fn bench_stochastic_oscillator(b: &mut test::Bencher) {
	bench_indicator::<StochasticOscillator>(b);
}

#[bench]
fn bench_trend_strength_index(b: &mut test::Bencher) {
	bench_indicator::<TrendStrengthIndex>(b);
}

#[bench]
fn bench_trix(b: &mut test::Bencher) {
	bench_indicator::<Trix>(b);
}

#[bench]
fn bench_true_strength_index(b: &mut test::Bencher) {
	bench_indicator::<TrueStrengthIndex>(b);
}

#[bench]
fn bench_woodies_cci(b: &mut test::Bencher) {
	bench_indicator::<WoodiesCCI>(b);
}
