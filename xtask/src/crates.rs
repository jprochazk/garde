use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use semver::{BuildMetadata, Prerelease};
use toml_edit::Document;

use crate::Result;

pub struct Crate {
    path: PathBuf,
    doc: Document,
}

impl Crate {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let source = fs::read_to_string(path)?;
        Self::from_str(path, &source)
    }

    pub fn from_str(path: impl AsRef<Path>, source: &str) -> Result<Self> {
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            doc: source.parse::<Document>()?,
        })
    }

    pub fn save(&self) -> Result {
        fs::write(&self.path, self.doc.to_string())?;
        Ok(())
    }

    pub fn bump_version(&mut self, bump: Bump) -> Result {
        let version = self.doc["package"]["version"].as_str().unwrap().to_string();
        let mut parsed = semver::Version::parse(&version)?;
        match bump {
            Bump::Major => {
                parsed.major += 1;
                parsed.minor = 0;
                parsed.patch = 0;
                parsed.build = BuildMetadata::EMPTY;
                parsed.pre = Prerelease::EMPTY;
                self.doc["package"]["version"] = toml_edit::value(parsed.to_string());
            }
            Bump::Minor => {
                parsed.minor += 1;
                parsed.patch = 0;
                parsed.build = BuildMetadata::EMPTY;
                parsed.pre = Prerelease::EMPTY;
                self.doc["package"]["version"] = toml_edit::value(parsed.to_string());
            }
            Bump::Patch => {
                parsed.patch += 1;
                parsed.build = BuildMetadata::EMPTY;
                parsed.pre = Prerelease::EMPTY;
                self.doc["package"]["version"] = toml_edit::value(parsed.to_string());
            }
            Bump::Prerelease => {
                let n: Option<u64> = parsed
                    .pre
                    .as_str()
                    .split_once('.')
                    .and_then(|(_, n)| n.parse().ok());
                let next = match n {
                    Some(n) => n + 1,
                    None => 1,
                };
                parsed.pre = Prerelease::new(&format!("alpha.{next}"))?;
                self.doc["package"]["version"] = toml_edit::value(parsed.to_string());
            }
            Bump::Finalize => {
                parsed.build = BuildMetadata::EMPTY;
                parsed.pre = Prerelease::EMPTY;
                self.doc["package"]["version"] = toml_edit::value(parsed.to_string());
            }
        }
        Ok(())
    }

    pub fn dependencies(&self) -> impl Iterator<Item = (&str, Dependency)> {
        let direct = self
            .dependencies_direct()
            .map(|key| (key, Dependency::Direct));
        let dev = self.dependencies_dev().map(|key| (key, Dependency::Dev));
        let build = self
            .dependencies_build()
            .map(|key| (key, Dependency::Build));

        direct.chain(dev).chain(build)
    }

    pub fn dependencies_direct(&self) -> impl Iterator<Item = &str> {
        self.doc["dependencies"]
            .as_table_like()
            .unwrap()
            .iter()
            .map(|(key, _)| key)
    }

    pub fn dependencies_dev(&self) -> impl Iterator<Item = &str> {
        self.doc["dev-dependencies"]
            .as_table_like()
            .unwrap()
            .iter()
            .map(|(key, _)| key)
    }

    pub fn dependencies_build(&self) -> impl Iterator<Item = &str> {
        self.doc["build-dependencies"]
            .as_table_like()
            .unwrap()
            .iter()
            .map(|(key, _)| key)
    }
}

pub enum Bump {
    Major,
    Minor,
    Patch,
    Prerelease,
    Finalize,
}

pub enum Dependency {
    Direct,
    Dev,
    Build,
}

pub struct Graph {
    root: Crate,
    crates: BTreeMap<String, Crate>,
}

impl Graph {
    pub fn from_manifest(manifest: Crate) -> Result<Self> {
        let patterns = manifest.doc["workspace"]["members"]
            .as_array()
            .unwrap()
            .iter();

        let mut crates = BTreeMap::new();
        for pattern in patterns {
            let pattern = pattern.as_str().unwrap();

            collect_crates(pattern, &mut crates)?;
        }

        Ok(Graph {
            root: manifest,
            crates,
        })
    }

    pub fn sorted_for_publish(&self) -> Vec<(String, &Crate)> {
        fn helper<'a>(
            crates: &'a BTreeMap<String, Crate>,
            name: &'a str,
            out: &mut Vec<(String, &'a Crate)>,
            visited: &mut HashSet<&'a str>,
        ) {
            let krate = &crates[name];
            for (dependency, _) in krate.dependencies() {
                if !crates.contains_key(dependency) {
                    continue;
                }
                helper(crates, dependency, out, visited);
            }

            if visited.contains(name) {
                return;
            }
            visited.insert(name);

            let publish = krate.doc["package"]
                .get("publish")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            if publish {
                out.push((name.into(), krate));
            }
        }

        let mut out = Vec::with_capacity(self.crates.len());
        let mut visited = HashSet::with_capacity(self.crates.len());
        for name in self.crates.keys() {
            helper(&self.crates, name, &mut out, &mut visited);
        }

        out
    }
}

fn collect_crates(pattern: &str, crates: &mut BTreeMap<String, Crate>) -> Result<()> {
    for dir in glob::glob(pattern)? {
        let dir = dir?;
        if !dir.is_dir() {
            continue;
        }

        let krate = Crate::from_file(dir.join("Cargo.toml"))?;
        let name = krate.doc["package"]["name"].as_str().unwrap();
        crates.insert(name.into(), krate);
    }
    Ok(())
}
