use std::collections::HashSet;
use std::iter;
use std::path::PathBuf;
use regex::Regex;
use crate::specs::manifest::{Argument, ConditionalArgument, Library, Os, Rule};

pub fn join_classpath(classpath: &[String]) -> String {
	let separator = if cfg!(target_os = "windows") {
		";"
	} else {
		":"
	};

	classpath.join(separator)
}

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

pub fn libraries_to_classpath(libs: Vec<Library>) -> Vec<PathBuf> {
	let mut classpath = Vec::with_capacity(libs.len() + 16);

	for lib in libs {
		match lib {
			Library::Custom { name, .. } => classpath.push(
				// TODO: Drop unwrap
				libname_to_path(&name).unwrap(),
			),
			Library::Seminative {
				rules, downloads, ..
			} => {
				if rules.iter().all(Rule::unwrap) {
					classpath.push(downloads.artifact.path);
				}
			}
			Library::Default { downloads, .. } => classpath.push(downloads.artifact.path),
			Library::Native { mut downloads, rules, natives, .. } => {
				classpath.push(downloads.artifact.path);

				if rules.iter().all(Rule::unwrap) {
					let native_id = if cfg!(target_os = "linux") {
						Os::Linux
					} else if cfg!(target_os = "windows") {
						Os::Windows
					} else if cfg!(target_os = "macos") {
						Os::Osx
					} else {
						panic!("Unknown target os!")
					};

					let classifier_id = natives.get(&native_id).unwrap();
					let artifact = downloads.classifiers.remove(classifier_id).unwrap();

					classpath.push(artifact.path);
				}
			},
		}
	}

	classpath
}

pub fn process_args(args: Vec<Argument>, to: &mut Vec<String>, with_features: &HashSet<&str>) {
	for arg in args {
		match arg {
			Argument::Constant(it) => to.push(it),
			Argument::Conditional { rules, value } => {
				if !rules.iter().all(|it| it.unwrap_featured(with_features)) {
					continue;
				}

				match value {
					ConditionalArgument::Single(it) => to.push(it),
					ConditionalArgument::List(it) => to.extend(it),
				}
			}
		}
	}
}