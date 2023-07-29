use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use anyhow::anyhow;

use crate::Result;

pub fn project_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn cargo(cmd: &str) -> Command {
    Command::new(env!("CARGO")).with_arg(cmd)
}

pub trait CommandExt {
    fn with_arg<S>(self, arg: S) -> Self
    where
        S: AsRef<OsStr>;

    fn with_args<I, S>(self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;

    fn with_env<K, V>(self, key: K, val: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>;

    fn run(self) -> Result;
}

impl CommandExt for Command {
    fn with_arg<S>(mut self, arg: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        self.arg(arg);
        self
    }

    fn with_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args(args);
        self
    }

    fn with_env<K, V>(mut self, key: K, val: V) -> Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.env(key, val);
        self
    }

    fn run(mut self) -> Result {
        self.spawn()?.wait()?.check()
    }
}

pub trait CheckStatus {
    fn check(&self) -> Result;
}

impl CheckStatus for std::process::ExitStatus {
    fn check(&self) -> Result {
        match self.success() {
            true => Ok(()),
            false => Err(anyhow!(
                "Process exited with error code {}",
                self.code().unwrap_or(-1)
            )),
        }
    }
}
