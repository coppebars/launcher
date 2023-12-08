pub macro trace($($tokens:tt)*) {
	{
		#[cfg(feature = "tracing")]
		tracing::trace!($($tokens)+);
		#[cfg(not(feature = "tracing"))]
		();
	}
}

pub macro debug($($tokens:tt)*) {
	{
		#[cfg(feature = "tracing")]
		tracing::debug!($($tokens)+);
		#[cfg(not(feature = "tracing"))]
		();
	}
}

pub macro info($($tokens:tt)*) {
	{
		#[cfg(feature = "tracing")]
		tracing::info!($($tokens)+);
		#[cfg(not(feature = "tracing"))]
		();
	}
}

pub macro warn($($tokens:tt)*) {
	{
		#[cfg(feature = "tracing")]
		tracing::warn!($($tokens)+);
		#[cfg(not(feature = "tracing"))]
		();
	}
}

pub macro error($($tokens:tt)*) {
	{
		#[cfg(feature = "tracing")]
		tracing::error!($($tokens)+);
		#[cfg(not(feature = "tracing"))]
		();
	}
}
