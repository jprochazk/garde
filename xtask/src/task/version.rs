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
        [
            ("patch", Bump::Patch),
            ("minor", Bump::Minor),
            ("major", Bump::Major),
        ]
        .into_iter()
        .find(|(name, _)| value.eq_ignore_ascii_case(name))
        .map(|(_, bump)| bump)
        .ok_or_else(|| "invalid bump kind, expected one of: major, minor, patch".into())
    }
}

impl Version {
    pub fn run(self) -> Result {
        cargo("semver-checks")
            .with_arg("--all-features")
            .with_args(["-p", "garde"])
            .with_args(["--release-type", self.bump.as_str()])
            .run_async()?;

        // TODO: manually parse workspace and bump versions
        cargo("workspaces")
            .with_arg("version")
            .with_arg(self.bump.as_str())
            .with_args(["--force", "*"])
            .run_async()?;

        Ok(())
    }
}
