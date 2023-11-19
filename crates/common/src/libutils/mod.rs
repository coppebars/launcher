use {
	regex::Regex,
	std::{
		iter,
		path::PathBuf,
	},
};

pub fn libname_to_path(name: &str) -> Option<PathBuf> {
	let re = Regex::new(r"^([^:]+):([^:]+):(.+)").unwrap();

	let ca = match re.captures(name) {
		Some(it) => it,
		None => return None,
	};

	let package = &ca[0];
	let artifact = &ca[1];
	let version = &ca[2];

	let sub = format!("{}-{}.jar", artifact, version);

	Some(PathBuf::from_iter(
		package
			.split('.')
			.chain(iter::once(artifact))
			.chain(iter::once(version))
			.chain(iter::once(sub.as_str())),
	))
}
