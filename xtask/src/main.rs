#![allow(dead_code)] // temporary

mod crates;
mod task;
mod util;

use std::io::{stderr, Write};
use std::process::ExitCode;

use clap::Parser;

use self::task::Task;

pub type Result<T = ()> = anyhow::Result<T>;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub task: Task,
}

fn try_main() -> Result {
    Cli::try_parse()?.task.run()
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
