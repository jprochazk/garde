use clap::Args;

use crate::Result;

#[derive(Args)]
pub struct Version {}

impl Version {
    pub fn run(self) -> Result {
        Ok(())
    }
}
