use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Json(#[from] serde_json::Error),

	#[error(transparent)]
	Network(#[from] reqwest::Error),

	#[error(transparent)]
	UrlParse(#[from] url::ParseError),

	#[error("Invalid utf-8 in path")]
	InvalidUtf8Path,

	#[error("Target manifest is inheriting. It does not contain enough data to to launch from it")]
	Inherited,

	#[error("Manifest is invalid: {0}")]
	Validation(String),

	#[error("Manifest with provided version options not found in registry")]
	ManifestNotFound,

	#[error("This machine cannot run minecraft")]
	UnsupportedTarget,
}
