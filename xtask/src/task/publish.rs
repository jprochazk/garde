use argp::FromArgs;

use crate::util::get_workspace_manifest;
use crate::util::{cargo, git, CommandExt as _};
use crate::Result;

#[derive(FromArgs)]
#[argp(subcommand, name = "publish", description = "Publish to crates.io")]
pub struct Publish {}

impl Publish {
    pub fn run(self) -> Result {
        cargo("publish").with_args(["-p", "garde_derive"]).run()?;
        cargo("publish").with_args(["-p", "garde"]).run()?;

        let version: semver::Version = get_workspace_manifest()?["workspace"]["package"]["version"]
            .as_str()
            .unwrap()
            .parse()?;

        // push tag
        git("tag").with_arg(format!("v{}", version)).run()?;
        git("push")
            .with_arg("origin")
            .with_arg(format!("v{}", version))
            .run()?;

        Ok(())
    }
}
