use argh::FromArgs;

use crate::Result;

#[derive(FromArgs)]
#[argh(subcommand, name = "release", description = "Release to crates.io")]
pub struct Release {}

impl Release {
    pub fn run(self) -> Result {
        Ok(())
    }
}
