use argp::FromArgs;

use crate::util::{cargo, CommandExt as _};
use crate::Result;

#[derive(FromArgs)]
#[argp(subcommand, name = "check", description = "Run all checks")]
pub struct Check {}

impl Check {
    pub fn run(self) -> Result {
        cargo("fmt").with_args(["--all", "--", "--check"]).run()?;
        cargo("clippy")
            .with_args(["--all-features", "--", "-D", "warnings"])
            .run()?;
        cargo("deny").with_args(["--all-features", "check"]).run()?;
        cargo("udeps").with_arg("--all-features").run()?;
        cargo("pants").with_arg("--dev").run()?;

        // TODO:
        // rm -rf ~/.cargo/advisory-db
        // cargo audit

        Ok(())
    }
}
