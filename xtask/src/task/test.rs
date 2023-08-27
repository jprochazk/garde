use std::collections::BTreeSet;
use std::process::Command;

use argp::FromArgs;

use crate::util::{cargo, CommandExt};
use crate::Result;

#[derive(FromArgs)]
#[argp(subcommand, name = "test", description = "Run tests")]
pub struct Test {
    #[argp(positional)]
    targets: Vec<Target>,
    #[argp(switch, description = "Run insta with --review")]
    review: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Target {
    Unit,
    Doc,
    Ui,
    Rules,
    Axum,
}

struct InvalidTarget;

impl std::fmt::Display for InvalidTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid target, expected one of: unit, doc, ui, rules, axum"
        )
    }
}

impl argp::FromArgValue for Target {
    fn from_arg_value(value: &std::ffi::OsStr) -> std::result::Result<Self, String> {
        let options = [
            ("unit", Self::Unit),
            ("doc", Self::Doc),
            ("ui", Self::Ui),
            ("rules", Self::Rules),
            ("axum", Self::Axum),
        ];

        options
            .iter()
            .find(|(name, _)| value.eq_ignore_ascii_case(std::ffi::OsStr::new(*name)))
            .map(|(_, target)| *target)
            .ok_or_else(|| "invalid target, expected one of: unit, doc, ui, rules, axum".into())
    }
}

impl Test {
    pub fn run(mut self) -> Result {
        let review = self.review;
        let commands = if self.targets.is_empty() {
            vec![unit(), ui(review), rules(review), axum()]
        } else {
            self.targets.sort();
            BTreeSet::from_iter(self.targets)
                .into_iter()
                .map(|target| match target {
                    Target::Unit => unit(),
                    Target::Doc => doc(),
                    Target::Ui => ui(review),
                    Target::Rules => rules(review),
                    Target::Axum => axum(),
                })
                .collect()
        };

        for command in commands {
            command.run()?;
        }

        Ok(())
    }
}

fn unit() -> Command {
    cargo("test").with_args(["--package=garde", "--all-features", "--lib"])
}

fn doc() -> Command {
    cargo("test").with_args(["--package=garde", "--all-features", "--doc"])
}

fn ui(review: bool) -> Command {
    let cmd = cargo("test").with_args(["--package=garde", "--all-features", "--test=ui"]);
    if review {
        cmd.with_env("TRYBUILD", "overwrite")
    } else {
        cmd
    }
}

fn rules(review: bool) -> Command {
    if review {
        cargo("insta").with_args(["test", "--review", "--package=garde", "--test=rules"])
    } else {
        cargo("test").with_args(["--package=garde", "--all-features", "--test=rules"])
    }
}

fn axum() -> Command {
    cargo("test").with_args(["--package=axum_garde", "--all-features"])
}
