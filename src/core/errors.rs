/// Crate errors enum
#[derive(Debug, Clone)]
pub enum Error {
	/// Error parsing string to [`Source`](crate::core::Source)
	SourceParse(String),

	/// Error parsing indicator parameter
	ParameterParse(String, String),

	/// Invalid parameters for method creation
	WrongMethodParameters,

	/// Invalid indicator config error
	WrongConfig,

	/// Invalid candles error
	InvalidCandles,

	/// Any other error
	Other(String),
}
