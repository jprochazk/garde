#![allow(dead_code)] // temporary

mod crates;
mod task;
mod util;

use std::io::{stderr, Write};
use std::process::ExitCode;

use argh::FromArgs;

use self::task::Task;

pub type Result<T = ()> = anyhow::Result<T>;

#[derive(FromArgs)]
#[argh(description = "Common tasks")]
pub struct Cli {
    #[argh(subcommand)]
    pub task: Task,
}

fn try_main() -> Result {
    argh::from_env::<Cli>().task.run()
}

fn main() -> ExitCode {
    match try_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let _ = write!(stderr(), "{e}");
            ExitCode::FAILURE
        }
    }
}
