use clap::Args;

use crate::Result;

#[derive(Args)]
pub struct Release {}

impl Release {
    pub fn run(self) -> Result {
        Ok(())
    }
}
