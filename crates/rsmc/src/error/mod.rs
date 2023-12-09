use {
	crate::private::tracing::error,
	thiserror::Error,
};

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Json(#[from] serde_json::Error),

	#[cfg(feature = "reqwest")]
	#[error(transparent)]
	Network(#[from] reqwest::Error),

	#[cfg(feature = "url")]
	#[error(transparent)]
	UrlParse(#[from] url::ParseError),

	#[error("Invalid utf-8 in path")]
	InvalidUtf8Path,

	#[error("Inherited manifest contains not enough data to launch or install from it. Consider merging it with parent")]
	Inherited,

	#[error("You're using unsupported platform")]
	UnsupportedPlatform,

	#[error("Invalid manifest: {0}")]
	InvalidManifest(String),

	#[error("{0}")]
	Inconsistent(String)
}
