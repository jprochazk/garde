use std::path::Path;

use argp::FromArgs;
use toml_edit::DocumentMut;

use crate::util::{cargo, CommandExt as _};
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

    fn apply(&self, mut version: semver::Version) -> semver::Version {
        match self {
            Bump::Major => {
                version.major += 1;
                version.minor = 0;
                version.patch = 0;
            }
            Bump::Minor => {
                version.minor += 1;
                version.patch = 0;
            }
            Bump::Patch => {
                version.patch += 1;
            }
        }
        version
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
        let cargo_toml_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("Cargo.toml");
        let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path)?.parse::<DocumentMut>()?;

        let workspace = &mut cargo_toml["workspace"];

        let version = workspace["package"]["version"]
            .as_str()
            .unwrap()
            .parse::<semver::Version>()?;
        println!("current version: {version}");

        cargo("semver-checks")
            .with_arg("--all-features")
            .with_args(["-p", "garde"])
            .with_args(["--release-type", self.bump.as_str()])
            .run()?;

        let version = self.bump.apply(version);
        workspace["package"]["version"] = version.to_string().into();
        println!("new version: {version}");

        let members = workspace["members"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();

        println!("bumping member dependency versions: {}", members.join(", "));
        let dependencies = workspace["dependencies"].as_table_mut().unwrap();
        for member in members {
            if let Some((_, dependency)) = dependencies.iter_mut().find(|(_, v)| {
                v.get("path")
                    .filter(|v| v.as_str() == Some(&member))
                    .is_some()
            }) {
                dependency["version"] = format!("={version}").into();
            }
        }

        std::fs::write(&cargo_toml_path, cargo_toml.to_string())?;

        Ok(())
    }
}
