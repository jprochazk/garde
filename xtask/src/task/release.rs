use argp::FromArgs;

use crate::Result;

#[derive(FromArgs)]
#[argp(subcommand, name = "release", description = "Release to crates.io")]
pub struct Release {}

impl Release {
    pub fn run(self) -> Result {
        Ok(())
    }
}
