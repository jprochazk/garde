use argp::FromArgs;

use crate::util::{cargo, CommandExt};
use crate::Result;

#[derive(FromArgs)]
#[argp(subcommand, name = "version", description = "Bump crate versions")]
pub struct Version {
    #[argp(positional, description = "one of: major, minor, patch")]
    bump: Bump,
}

enum Bump {
    Patch,
    Minor,
    Major,
}

impl Bump {
    fn as_str(&self) -> &'static str {
        match self {
            Bump::Patch => "patch",
            Bump::Minor => "minor",
            Bump::Major => "major",
        }
    }
}

impl argp::FromArgValue for Bump {
    fn from_arg_value(value: &std::ffi::OsStr) -> std::result::Result<Self, String> {
        let options = [
            ("patch", Bump::Patch),
            ("minor", Bump::Minor),
            ("major", Bump::Major),
        ];

        options
            .into_iter()
            .find(|(name, _)| value.eq_ignore_ascii_case(name))
            .map(|(_, bump)| bump)
            .ok_or_else(|| "invalid bump kind, expected one of: major, minor, patch".into())
    }
}

impl Version {
    pub fn run(self) -> Result {
        // TODO: manually parse workspace and bump versions
        cargo("workspaces")
            .with_arg("version")
            .with_arg(self.bump.as_str())
            .with_args(["--force", "*"])
            .run_async()?;

        cargo("semver-checks")
            .with_arg("--all-features")
            .run_async()?;

        Ok(())
    }
}
