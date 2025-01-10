#![allow(dead_code)] // temporary

mod crates;
mod task;
mod util;

use std::io::{stderr, Write};
use std::process::ExitCode;

use argp::FromArgs;

use self::task::Task;

pub type Result<T = ()> = anyhow::Result<T>;

#[derive(FromArgs)]
#[argp(description = "Common tasks")]
pub struct Cli {
    #[argp(subcommand)]
    pub task: Task,
}

fn try_main() -> Result {
    let args: Cli = argp::parse_args_or_exit(argp::DEFAULT);
    args.task.run()
}

fn main() -> ExitCode {
    match try_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let _ = writeln!(stderr(), "{e}");
            ExitCode::FAILURE
        }
    }
}
