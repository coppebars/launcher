use {
  crate::launcher::Launcher,
  common::{
    libutils::libname_to_path,
    manifest::{
      Argument,
      ConditionalArgument,
      Library::{
        self,
      },
      ModernArgs,
      RootManifest,
      Rule,
      VersionType,
    },
  },
  once_cell::sync::Lazy,
  std::collections::HashSet,
  thiserror::Error,
};

static DEFAULT_FEATURES: Lazy<HashSet<&str>> =
  Lazy::new(|| HashSet::from(["has_custom_resolution"]));

fn process_args(args: Vec<Argument>, to: &mut Vec<String>) {
  for arg in args {
    match arg {
      Argument::Constant(it) => to.push(it),
      Argument::Conditional { rules, value } => {
        if !rules.iter().all(|it| it.unwrap_featured(&DEFAULT_FEATURES)) {
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

#[derive(Debug, Error)]
pub enum FromError {
  #[error("Failed to convert manifest to launcher instance: {0}")]
  InvalidManifest(String),
}

impl TryFrom<RootManifest> for Launcher {
  type Error = FromError;

  fn try_from(value: RootManifest) -> Result<Self, Self::Error> {
    let mut launcher = Launcher {
      id: value.id,
      asset_index_name: value.assets,
      main_class: value.main_class,
      version_type: match value.version_type {
        VersionType::Release => "release",
        VersionType::Snapshot => "snapshot",
        VersionType::OldBeta => "old_beta",
        VersionType::OldAlpha => "old_alpha",
      }
      .into(),
			..Default::default()
    };

    for lib in value.libraries {
      match lib {
        Library::Custom { name, .. } => launcher.classpath.push(
          libname_to_path(&name).ok_or(FromError::InvalidManifest("Invalid lib name".into()))?,
        ),
        Library::Seminative {
          rules, downloads, ..
        } => {
          if rules.iter().all(Rule::unwrap) {
            launcher.classpath.push(downloads.artifact.path);
          }
        }
        Library::Default { downloads, .. } => launcher.classpath.push(downloads.artifact.path),
        Library::Native { downloads, .. } => launcher.classpath.push(downloads.artifact.path),
      }
    }

    launcher
      .classpath
      .push(format!("../versions/{}/client.jar", &launcher.id).into());

    let ModernArgs { arguments }: ModernArgs = value.arguments.into();

    process_args(arguments.jvm, &mut launcher.jvm_args);
    process_args(arguments.game, &mut launcher.game_args);

    Ok(launcher)
  }
}
