use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::anyhow;

use crate::Result;

pub fn project_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn rustup(cmd: &str) -> Command {
    Command::new("rustup").with_arg(cmd)
}

pub fn cargo(cmd: &str) -> Command {
    Command::new(env!("CARGO")).with_arg(cmd)
}

pub fn has_cargo_subcmd(cmd: &str) -> Result<bool> {
    Ok(cargo("--list")
        .run_with_output()?
        .split('\n')
        .filter(|v| v.starts_with(|c: char| c.is_ascii_whitespace()))
        .map(str::trim)
        .map(|v| v.split_ascii_whitespace().next().unwrap())
        .any(|v| v == cmd))
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

    fn run_async(self) -> Result;

    fn run(self) -> Result;

    fn run_with_output(self) -> Result<String>;
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

    fn run_async(mut self) -> Result {
        self.spawn()?.wait_async()
    }

    fn run(mut self) -> Result {
        println!("> {self:?}");
        self.spawn()?.wait()?.check()
    }

    fn run_with_output(mut self) -> Result<String> {
        let output = self
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?;
        output.check()?;
        Ok(String::from_utf8(output.stdout)?)
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

impl CheckStatus for std::process::Output {
    fn check(&self) -> Result {
        self.status.check()
    }
}

pub trait WaitAsync {
    /// Wait with inherited IO
    fn wait_async(self) -> Result;
}

impl WaitAsync for std::process::Child {
    fn wait_async(mut self) -> Result {
        loop {
            if let Some(status) = self.try_wait()? {
                status.check()?;
                return Ok(());
            }
        }
    }
}
