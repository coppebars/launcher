use {
	crate::specs::manifest::{
		Argument,
		ConditionalArgument,
	},
	std::{
		collections::HashSet,
		fs::{
			self,
			Permissions,
		},
		path::Path,
	},
};

pub const CLASSPATH_SEPARATOR: &str = {
	if cfg!(target_os = "windows") {
		";"
	} else {
		":"
	}
};

pub const BINARY_NAME: &str = {
	if cfg!(target_os = "windows") {
		"javaw.exe"
	} else {
		"java"
	}
};

pub fn join_classpath(classpath: &[String]) -> String {
	classpath.join(CLASSPATH_SEPARATOR)
}



pub fn setup_permissions(path: &Path) {
	#[cfg(target_family = "unix")]
	{
		use std::os::unix::fs::PermissionsExt;

		fs::set_permissions(path, Permissions::from_mode(0o744))
			.expect("Could not set execute permissions");
	}
}
