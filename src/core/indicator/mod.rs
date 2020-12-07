//! Every indicator has it's own **Configuration** and **State**.
//!
//! Every indicator **Configuration** must implement [`IndicatorConfig`].
//!
//! Every indicator **State** must implement [`IndicatorInstance`].

mod config;
mod dd;
mod instance;
mod result;

pub use config::*;
pub use dd::*;
pub use instance::*;
pub use result::*;
