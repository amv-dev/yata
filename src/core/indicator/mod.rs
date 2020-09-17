//! Every indicator has it's own **Configuration** and **State**.
//!
//! Every indicator **Configuration** should implement [IndicatorConfig] and [IndicatorInitializer].
//!
//! Every indicator **State** should implement [IndicatorInstance].

mod config;
mod instance;
mod result;

pub use config::*;
pub use instance::*;
pub use result::*;
