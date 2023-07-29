use std::collections::BTreeSet;
use std::process::Command;

use clap::{Args, ValueEnum};

use crate::util::{cargo, CommandExt};
use crate::Result;

#[derive(Args)]
pub struct Test {
    targets: Vec<Target>,
    #[arg(long)]
    review: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Target {
    Unit,
    Doc,
    Ui,
    Rules,
    Axum,
}

impl Test {
    pub fn run(mut self) -> Result {
        let commands = if self.targets.is_empty() {
            vec![unit(), ui(), rules(self.review), axum()]
        } else {
            self.targets.sort();
            BTreeSet::from_iter(self.targets)
                .into_iter()
                .map(|target| match target {
                    Target::Unit => unit(),
                    Target::Doc => doc(),
                    Target::Ui => ui(),
                    Target::Rules => rules(self.review),
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
