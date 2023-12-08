use {
	crate::tracing::trace,
	std::{
		path::PathBuf,
		process::Command,
	},
};

pub const EXECUTABLE_NAME: &str = {
	if cfg!(target_os = "windows") {
		"javaw.exe"
	} else {
		"java"
	}
};

#[derive(Debug, Clone)]
pub struct Program {
	pub jre_home: PathBuf,
	pub jvm_args: Vec<String>,
	pub main_class: String,
	pub program_args: Vec<String>,
}

impl Program {
	pub fn executable(&self) -> PathBuf {
		self.jre_home.join("bin").join(EXECUTABLE_NAME)
	}

	pub async fn set_permissions(&self) -> Result<(), std::io::Error> {
		if cfg!(target_family = "unix") {
			use std::{
				fs::Permissions,
				os::unix::fs::PermissionsExt,
			};

			tokio::fs::set_permissions(self.executable(), Permissions::from_mode(0o744)).await?
		} else {
			trace!("set_permissions has no effect on target systems other than unix")
		}

		Ok(())
	}

	pub async fn to_command(&self) -> Command {
		let executable = self.executable();

		let mut cmd = Command::new(executable);

		cmd.args(&self.jvm_args);
		cmd.arg(&self.main_class);
		cmd.args(&self.program_args);

		trace!(?cmd);

		cmd
	}
}

pub mod ext {
	use std::fmt::{
		Display,
		Formatter,
	};

	#[derive(Debug, Clone)]
	pub enum AllocUnit {
		Kb,
		Mb,
		Gb,
	}

	impl AllocUnit {
		#[allow(clippy::should_implement_trait)]
		pub fn from_str(value: &str) -> Option<AllocUnit> {
			match value {
				"K" | "k" => Some(Self::Kb),
				"M" | "m" => Some(Self::Mb),
				"G" | "g" => Some(Self::Gb),
				_ => None,
			}
		}
	}

	impl Display for AllocUnit {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			use AllocUnit::*;

			match self {
				Kb => write!(f, "K"),
				Mb => write!(f, "M"),
				Gb => write!(f, "G"),
			}
		}
	}

	#[derive(Debug, Clone)]
	pub struct AllocOptions<T>
	where
		T: Display + Clone,
	{
		pub unit: AllocUnit,
		pub min: T,
		pub max: T,
	}

	impl<T> AllocOptions<T>
	where
		T: Display + Clone,
	{
		pub fn range(unit: AllocUnit, min: T, max: T) -> Self {
			Self { unit, min, max }
		}

		pub fn single(unit: AllocUnit, value: T) -> Self {
			Self {
				unit,
				min: value.clone(),
				max: value,
			}
		}

		pub fn to_args(&self) -> [String; 2] {
			let unit = &self.unit;
			let min = &self.min;
			let max = &self.max;

			[format!("-Xms{min}{unit}"), format!("-Xmx{max}{unit}")]
		}
	}

	impl Default for AllocOptions<u64> {
		fn default() -> Self {
			Self {
				unit: AllocUnit::Mb,
				min: 512,
				max: 2048
			}
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn it_works() {
			let alloc = AllocOptions::range(AllocUnit::Mb, 150, 1024);

			assert_eq!(alloc.to_args(), ["-Xms150M", "-Xmx1024M"])
		}
	}
}
