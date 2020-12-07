// #![warn(missing_docs)]
#![warn(missing_debug_implementations)]
//! Some useful features and definitions

mod action;
mod candles;
mod errors;
mod indicator;
mod method;
mod ohlcv;
mod sequence;
mod window;

pub use action::Action;
pub use candles::*;
pub use errors::Error;
pub use indicator::*;
pub use method::Method;
pub use ohlcv::OHLCV;
pub use sequence::*;
pub use window::Window;

/// Main value type for calculations
///
/// Default is `f64`
///
/// If you want use `f32` which is (may be) faster, you can use `cargo build --features value_type_f32`
///
/// Or in your `cargo.toml`:
///
/// ```toml
/// [dependencies]
/// yata = { value_type_f32 = true }
/// ```
///
/// Read more at [Features section](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section)
///
/// # See also
///
/// [`PeriodType`]
#[cfg(not(feature = "value_type_f32"))]
pub type ValueType = f64;
#[cfg(feature = "value_type_f32")]
#[allow(missing_docs)]
pub type ValueType = f32;

/// `PeriodType` is a type for using on methods and indicators params.
///
/// For default it is u8 (from 0 to 255). That means you can use up to `SMA::new(254)`, `WMA::new(254)`, etc...
/// That's right, there are not 255, but 254 (`u8::MAX` - 1)
///
/// If you want use larger periods, you can switch it by using crate features: `period_type_u16`, `period_type_u32`, `period_type_u64`.
///
/// F.e. `cargo build --features period_type_u16`
///
/// or in your `cargo.toml`:
///
/// ```toml
/// [dependencies]
/// yata = { period_type_u16 = true }
/// ```
///
/// Read more at [Features section](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section)
///
/// # See also
///
/// [`ValueType`]
#[cfg(not(any(
	feature = "period_type_u16",
	feature = "period_type_u32",
	feature = "period_type_u64"
)))]
pub type PeriodType = u8;
#[cfg(all(
	feature = "period_type_u16",
	not(any(feature = "period_type_u32", feature = "period_type_u64"))
))]
#[allow(missing_docs)]
pub type PeriodType = u16;
#[cfg(all(feature = "period_type_u32", not(feature = "period_type_u64")))]
#[allow(missing_docs)]
pub type PeriodType = u32;
#[cfg(feature = "period_type_u64")]
#[allow(missing_docs)]
pub type PeriodType = u64;
