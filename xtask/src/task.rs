pub mod check;
pub mod release;
pub mod test;
pub mod version;

use clap::Subcommand;

use self::check::Check;
use self::test::Test;
use crate::Result;

#[derive(Subcommand)]
pub enum Task {
    Test(Test),
    Check(Check),
    // Version(Version),
    // Release(Release),
}

impl Task {
    pub fn run(self) -> Result {
        match self {
            Task::Test(cmd) => cmd.run(),
            Task::Check(cmd) => cmd.run(),
        }
    }
}
