use {
	sha1::{
		Digest,
		Sha1,
	},
	thiserror::Error,
	tokio::{
		fs::File,
		io::AsyncReadExt,
	},
};

#[derive(Debug, Error)]
pub enum IntegrityCheckError {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	ParseInt(#[from] std::num::ParseIntError),
}

pub async fn check(file: &mut File, sha: &str) -> Result<bool, IntegrityCheckError> {
	let mut hasher = Sha1::new();

	let mut buffer: Vec<u8> = vec![0; 2097152];

	loop {
		let bytes_read = file.read(&mut buffer).await?;

		if bytes_read == 0 {
			break;
		}

		hasher.update(&buffer[..bytes_read]);
	}

	let hex = &hasher.finalize()[..];

	let bytes = (0..sha.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&sha[i..i + 2], 16))
		.collect::<Result<Vec<_>, _>>()?;

	Ok(hex.iter().zip(&bytes).all(|(a, b)| a == b))
}
