use {
  super::Launcher,
  common::{
    manifest::{
      Arch,
      Args,
      ArgsContainer,
      Argument,
      ConditionalArgument,
      Library,
      Manifest,
      Os,
      Rule,
      RuleAction,
    },
  },
};

fn check_rule(rule: Rule) -> bool {
  let condition = rule.condition;
  let mut allow = true;

  if let Some(os_condition) = condition.os {
    if let Some(os_name) = os_condition.name {
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

    if let Some(os_arch) = os_condition.arch {
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

  match rule.action {
    RuleAction::Allow => allow,
    RuleAction::Disallow => !allow,
  }
}

fn process_arg(argument: Argument) -> Option<Vec<String>> {
  match argument {
    Argument::Constant(it) => Some(vec![it]),
    Argument::Conditional { rules, value } => {
      if rules.into_iter().all(check_rule) {
        match value {
          ConditionalArgument::Single(it) => Some(vec![it]),
          ConditionalArgument::List(it) => Some(it),
        }
      } else {
        None
      }
    }
  }
}

impl From<Manifest> for Launcher {
  fn from(manifest: Manifest) -> Self {
    let libs = manifest
      .libraries
      .into_iter()
      .filter_map(|it| match it {
        Library::Seminative {
          rules, downloads, ..
        } => {
          if rules.into_iter().all(check_rule) {
            Some(downloads.artifact.path)
          } else {
            None
          }
        }
        Library::Default { downloads, .. } => Some(downloads.artifact.path),
        _ => None,
      })
      .collect::<Vec<_>>();

    let args = match manifest.arguments {
      ArgsContainer::Modern { arguments } => arguments,
      ArgsContainer::Legacy { arguments } => Args {
        game: arguments
          .split_whitespace()
          .map(|it| Argument::Constant(it.to_owned()))
          .collect::<Vec<_>>(),
        jvm: Vec::new(),
      },
    };

    let jvm_ext = args
      .jvm
      .into_iter()
      .filter_map(process_arg)
      .flatten()
      .collect::<Vec<_>>();
    let game = args
      .game
      .into_iter()
      .filter_map(process_arg)
      .flatten()
      .collect::<Vec<_>>();

		let mut jvm: Vec<_> =  ["-Xms", "${init_alloc}", "-Xmx", "${max_alloc}"].map(ToString::to_string).into();

		jvm.extend(jvm_ext);

    Self {
      id: manifest.id,
      main: String::from("client.jar"),
      libs,
      jvm_args: jvm,
      game_args: game,
    }
  }
}
