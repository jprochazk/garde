use argp::FromArgs;

use crate::util::{cargo, CommandExt as _};
use crate::Result;

#[derive(FromArgs)]
#[argp(subcommand, name = "publish", description = "Publish to crates.io")]
pub struct Publish {}

impl Publish {
    pub fn run(self) -> Result {
        cargo("publish").with_args(["-p", "garde_derive"]).run()?;
        cargo("publish").with_args(["-p", "garde"]).run()?;
        cargo("publish").with_args(["-p", "axum_garde"]).run()?;

        Ok(())
    }
}
