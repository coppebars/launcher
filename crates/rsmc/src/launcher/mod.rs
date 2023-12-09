use std::process::Stdio;

use {
	crate::{
		spec::*,
		tracing::debug,
		tracing::trace,
		Error,
	},
	std::{
		collections::HashMap,
		path::{
			Path,
			PathBuf,
		},
		sync::Arc,
	},
	tokio::{
		fs::File,
		io::{
			AsyncBufReadExt,
			AsyncReadExt,
			BufReader,
		},
		process::Command,
		sync::{
			mpsc,
			mpsc::Receiver,
		},
		task::JoinHandle,
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
pub struct Launcher {
	pub root: PathBuf,
	pub manifest: Box<NativeManifest>,
	pub extra_libs: Vec<PathBuf>,
	pub extra_jvm_args: Vec<String>,
	pub extra_game_args: Vec<String>,
	pub vars: HashMap<String, String>,
}

type LauncherProcessHandle = (JoinHandle<()>, JoinHandle<()>, Receiver<String>);

fn process_args(args: Vec<Argument>, to: &mut Vec<String>) {
	for arg in args {
		match arg {
			Argument::Constant(it) => {
				trace!("+arg: {}", it);
				to.push(it)
			},
			Argument::Conditional { rules, value } => {
				if rules.iter().all(unpack_rule) {
					match value {
						ConditionalArgument::Single(it) => {
							trace!("+arg: {}", it);
							to.push(it)
						},
						ConditionalArgument::List(it) => {
							trace!("+args: {:?}", it);
							to.extend(it)
						},
					}
				}
			}
		}
	}
}

fn unpack_rule(rule: &Rule) -> bool {
	let mut allow = true;

	match &rule.condition {
		Some(RuleCondition::Os { name, arch, .. }) => {
			if let Some(os_name) = &name {
				allow = match os_name {
					#[cfg(target_os = "linux")]
					Os::Linux => true,
					#[cfg(target_os = "windows")]
					Os::Windows => true,
					#[cfg(target_os = "macos")]
					Os::Osx => true,
					#[allow(unreachable_patterns)]
					_ => false,
				};
			}

			if let Some(os_arch) = &arch {
				allow = match os_arch {
					#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
					Arch::X64 => true,
					#[cfg(target_arch = "x86")]
					Arch::X86 => true,
					#[allow(unreachable_patterns)]
					_ => false,
				};
			}
		}
		Some(RuleCondition::Features(_)) => allow = false,
		_ => {}
	};

	match rule.action {
		RuleAction::Allow => allow,
		RuleAction::Disallow => !allow,
	}
}

fn set_vars(vars: &HashMap<String, String>, target: &str) -> String {
	let mut target = target.to_owned();

	for (key, value) in vars {
		target = target.replace(&format!("${{{key}}}"), value);
	}

	target
}

impl Launcher {
	pub async fn try_from_root(root: &Path, id: &String) -> Result<Self, Error> {
		let manifest_path = root.join("versions").join(id).join(format!("{}.json", &id));

		debug!("Reading manifest from {:?}", manifest_path);

		let mut file = File::open(manifest_path).await?;
		let mut content = String::new();
		file.read_to_string(&mut content).await?;

		let manifest: Manifest = serde_json::from_str(&content)?;

		let manifest = match manifest {
			Manifest::Native(it) => it,
			Manifest::Inherited(_) => return Err(Error::Inherited),
		};

		debug!(?id, "Manifest loaded");

		Ok(Self {
			root: root.to_owned(),
			manifest,
			extra_libs: Vec::new(),
			extra_game_args: Vec::new(),
			extra_jvm_args: Vec::new(),
			vars: HashMap::new(),
		})
	}

	pub async fn launch(self) -> Result<LauncherProcessHandle, Error> {
		let mut classpath: Vec<PathBuf> = Vec::new();

		let assets_dir = self.root.join("assets");
		let libraries_dir = self.root.join("libraries");
		let version_dir = self.root.join("versions").join(&self.manifest.id);
		let jre_dir = self
			.root
			.join("jre")
			.join(&self.manifest.java_version.component);

		trace!(?version_dir, ?jre_dir, ?assets_dir, ?libraries_dir);

		for lib in self.manifest.libraries {
			use Library::*;

			match lib {
				Custom(it) => {
					trace!("Including (custom) : {}", it.name);
					classpath.push(it.name.to_path());
				},
				Common(it) => {
					trace!("Including         : {}", it.name);
					classpath.push(it.downloads.artifact.path);
				}
				Seminative(it) => {
					if it.rules.iter().all(unpack_rule) {
						trace!("Including (native): {}", it.name);
						classpath.push(it.downloads.artifact.path);
					}
				}
				Native(mut it) => {
					trace!(?it);
					if it.rules.iter().all(unpack_rule) {
						let platform = if cfg!(target_os = "windows") {
							&Os::Windows
						} else if cfg!(target_os = "linux") {
							&Os::Linux
						} else if cfg!(target_os = "macos") {
							&Os::Osx
						} else {
							return Err(Error::UnsupportedPlatform);
						};

						let classifier = it
							.natives
							.get(platform)
							.ok_or(Error::InvalidManifest("Missing native classifier".into()))?
							.as_str();

						let artifact = it
							.downloads
							.classifiers
							.remove(classifier)
							.ok_or(Error::InvalidManifest("Missing native classifier".into()))?;

						trace!("Including (base)  : {}", it.name);
						classpath.push(it.downloads.artifact.path);

						trace!("Including (native): {}", it.name);
						classpath.push(artifact.path);
					}
				}
			};
		}

		let args = match self.manifest.arguments {
			ArgsContainer::Modern(it) => it,
			ArgsContainer::Legacy(it) => it.into(),
		};

		let mut jvm_args = Vec::new();
		let mut game_args = Vec::new();

		process_args(args.arguments.jvm, &mut jvm_args);
		process_args(args.arguments.game, &mut game_args);

		jvm_args.extend(self.extra_jvm_args);
		game_args.extend(self.extra_game_args);

		// This is intentional! Do not remove without RFC
		jvm_args.retain(|it| !it.starts_with("-Djava.library.path"));

		macro vars($($name:ident:$value:expr,)*) {
			HashMap::from([
				$((stringify!($name).into(), { $value }.into()),)*
			])
		}

		let mut classpath: Vec<_> = classpath
			.into_iter()
			.map(|it| libraries_dir.join(it))
			.collect();

		classpath.push(version_dir.join(format!("{}.jar", &self.manifest.id)));

		classpath.extend(self.extra_libs);

		let cp_items: Vec<_> = classpath
			.into_iter()
			.map(|it| {
				it.to_str()
					.ok_or(Error::InvalidUtf8Path)
					.map(|it| it.to_owned())
			})
			.collect::<Result<_, _>>()?;

		trace!(?cp_items, "Classpath");

		let cp_string = cp_items.join(if cfg!(target_os = "windows") {
			";"
		} else {
			":"
		});

		let mut vars: HashMap<String, String> = vars! {
			version_name: self.manifest.id,
			version_type: self.manifest.version_type,
			assets_index_name: self.manifest.assets,
			assets_root: assets_dir.to_str().ok_or(Error::InvalidUtf8Path)?,
			natives_directory: version_dir.join("natives").to_str().ok_or(Error::InvalidUtf8Path)?,
			classpath: cp_string,
		};

		vars.extend(self.vars);

		let executable = jre_dir.join("bin").join(EXECUTABLE_NAME);
		trace!(?executable);

		#[cfg(target_family = "unix")]
		{
			use std::{
				fs::Permissions,
				os::unix::fs::PermissionsExt,
			};

			trace!("Set permissions");

			tokio::fs::set_permissions(&executable, Permissions::from_mode(0o744)).await?
		}

		let mut command = Command::new(executable);

		command.stdout(Stdio::piped());
		command.stderr(Stdio::piped());

		command.args(jvm_args.into_iter().map(|it| set_vars(&vars, &it)));
		command.arg(self.manifest.main_class);
		command.args(game_args.into_iter().map(|it| set_vars(&vars, &it)));

		trace!(?command);

		let process = command.spawn()?;
		trace!("Spawned");

		let (tx, rx) = mpsc::channel::<String>(64);

		let tx = Arc::new(tx);

		let stdout_task = tokio::spawn({
			let pipe = process.stdout.unwrap();
			let tx = tx.clone();
			let mut buf = BufReader::new(pipe);

			async move {
				loop {
					let mut out = String::new();

					if let Ok(bytes) = buf.read_line(&mut out).await {
						let is_empty = out.is_empty();

						if bytes == 0 && is_empty {
							break;
						}

						let _ = tx.send(out).await;
					}
				}
			}
		});

		let stderr_task = tokio::spawn({
			let pipe = process.stderr.unwrap();
			let tx = tx.clone();
			let mut buf = BufReader::new(pipe);

			async move {
				loop {
					let mut out = String::new();

					if let Ok(bytes) = buf.read_line(&mut out).await {
						let is_empty = out.is_empty();

						if bytes == 0 && is_empty {
							break;
						}

						let _ = tx.send(out).await;
					}
				}
			}
		});

		Ok((stdout_task, stderr_task, rx))
	}
}
