#![allow(missing_docs)]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::core::{ValueType, OHLCV};

pub mod example;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct HLC {
	high: ValueType,
	low: ValueType,
	close: ValueType,
}

impl HLC {
	fn from<T: OHLCV>(src: &T) -> Self {
		Self {
			high: src.high(),
			low: src.low(),
			close: src.close(),
		}
	}
}

impl OHLCV for HLC {
	fn open(&self) -> ValueType {
		ValueType::NAN
	}

	#[inline]
	fn high(&self) -> ValueType {
		self.high
	}

	#[inline]
	fn low(&self) -> ValueType {
		self.low
	}

	#[inline]
	fn close(&self) -> ValueType {
		self.close
	}

	fn volume(&self) -> ValueType {
		ValueType::NAN
	}
}

mod aroon;
pub use aroon::{Aroon, AroonInstance};

mod average_directional_index;
pub use average_directional_index::{AverageDirectionalIndex, AverageDirectionalIndexInstance};

mod awesome_oscillator;
pub use awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorInstance};

mod bollinger_bands;
pub use bollinger_bands::{BollingerBands, BollingerBandsInstance};

mod chaikin_money_flow;
pub use chaikin_money_flow::{ChaikinMoneyFlow, ChaikinMoneyFlowInstance};

mod chaikin_oscillator;
pub use chaikin_oscillator::{ChaikinOscillator, ChaikinOscillatorInstance};

mod chande_kroll_stop;
pub use chande_kroll_stop::{ChandeKrollStop, ChandeKrollStopInstance};

mod chande_momentum_oscillator;
pub use chande_momentum_oscillator::{ChandeMomentumOscillator, ChandeMomentumOscillatorInstance};

mod commodity_channel_index;
pub use commodity_channel_index::{CommodityChannelIndex, CommodityChannelIndexInstance};

mod coppock_curve;
pub use coppock_curve::{CoppockCurve, CoppockCurveInstance};

mod detrended_price_oscillator;
pub use detrended_price_oscillator::{DetrendedPriceOscillator, DetrendedPriceOscillatorInstance};

mod donchian_channel;
pub use donchian_channel::{DonchianChannel, DonchianChannelInstance};

mod ease_of_movement;
pub use ease_of_movement::{EaseOfMovement, EaseOfMovementInstance};

mod elders_force_index;
pub use elders_force_index::{EldersForceIndex, EldersForceIndexInstance};

mod envelopes;
pub use envelopes::{Envelopes, EnvelopesInstance};

mod fisher_transform;
pub use fisher_transform::{FisherTransform, FisherTransformInstance};

mod hull_moving_average;
pub use hull_moving_average::{HullMovingAverage, HullMovingAverageInstance};

mod ichimoku_cloud;
pub use ichimoku_cloud::{IchimokuCloud, IchimokuCloudInstance};

mod kaufman;
pub use kaufman::{Kaufman, KaufmanInstance, KAMA};

mod keltner_channel;
pub use keltner_channel::{KeltnerChannel, KeltnerChannelInstance};

mod klinger_volume_oscillator;
pub use klinger_volume_oscillator::{KlingerVolumeOscillator, KlingerVolumeOscillatorInstance};

mod know_sure_thing;
pub use know_sure_thing::{KnowSureThing, KnowSureThingInstance};

mod macd;
pub use macd::{MACDInstance, MovingAverageConvergenceDivergence, MACD};

mod momentum_index;
pub use momentum_index::{MomentumIndex, MomentumIndexInstance};

mod money_flow_index;
pub use money_flow_index::{MoneyFlowIndex, MoneyFlowIndexInstance};

mod parabolic_sar;
pub use parabolic_sar::{ParabolicSAR, ParabolicSARInstance, ParabolicStopAndReverse};

mod pivot_reversal_strategy;
pub use pivot_reversal_strategy::{PivotReversalStrategy, PivotReversalStrategyInstance};

mod price_channel_strategy;
pub use price_channel_strategy::{PriceChannelStrategy, PriceChannelStrategyInstance};

mod relative_strength_index;
pub use relative_strength_index::{RelativeStrengthIndex, RelativeStrengthIndexInstance, RSI};

mod relative_vigor_index;
pub use relative_vigor_index::{RelativeVigorIndex, RelativeVigorIndexInstance};

mod smi_ergodic_indicator;
pub use smi_ergodic_indicator::{SMIErgodicIndicator, SMIErgodicIndicatorInstance};

mod stochastic_oscillator;
pub use stochastic_oscillator::{StochasticOscillator, StochasticOscillatorInstance};

mod trix;
pub use trix::{TRIXInstance, Trix};

mod trend_strength_index;
pub use trend_strength_index::{TrendStrengthIndex, TrendStrengthIndexInstance};

mod true_strength_index;
pub use true_strength_index::{TrueStrengthIndex, TrueStrengthIndexInstance};

mod woodies_cci;
pub use woodies_cci::{WoodiesCCI, WoodiesCCIInstance};
