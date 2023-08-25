use std::collections::BTreeSet;
use std::process::Command;
use std::str::FromStr;

use argh::FromArgs;

use crate::util::{cargo, CommandExt};
use crate::Result;

#[derive(FromArgs)]
#[argh(subcommand, name = "test", description = "Run tests")]
pub struct Test {
    #[argh(positional)]
    targets: Vec<Target>,
    #[argh(option, description = "run insta with --review")]
    review: Option<bool>,
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

impl FromStr for Target {
    type Err = InvalidTarget;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        match s.as_str() {
            "unit" => Ok(Self::Unit),
            "doc" => Ok(Self::Doc),
            "ui" => Ok(Self::Ui),
            "rules" => Ok(Self::Rules),
            "axum" => Ok(Self::Axum),
            _ => Err(InvalidTarget),
        }
    }
}

impl Test {
    pub fn run(mut self) -> Result {
        let review = self.review.unwrap_or(false);
        let commands = if self.targets.is_empty() {
            vec![unit(), ui(), rules(review), axum()]
        } else {
            self.targets.sort();
            BTreeSet::from_iter(self.targets)
                .into_iter()
                .map(|target| match target {
                    Target::Unit => unit(),
                    Target::Doc => doc(),
                    Target::Ui => ui(),
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

fn ui() -> Command {
    cargo("test")
        .with_args(["--package=garde", "--all-features", "--test=ui"])
        .with_env("TRYBUILD", "overwrite")
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
