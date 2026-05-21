use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const RUN_ENV: &str = "GARDE_RUN_FEATURE_FLAG_TESTS";

struct TempProject {
    path: PathBuf,
}

impl TempProject {
    fn new(name: &str, garde_features: &[&str]) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "garde-{name}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        fs::create_dir_all(path.join("src")).unwrap();

        let features = garde_features
            .iter()
            .map(|feature| format!("\"{feature}\""))
            .collect::<Vec<_>>()
            .join(", ");

        fs::write(
            path.join("Cargo.toml"),
            format!(
                r#"[package]
name = "{name}"
version = "0.0.0"
edition = "2021"

[dependencies]
garde = {{ path = "{}", default-features = false, features = [{features}] }}
"#,
                Path::new(env!("CARGO_MANIFEST_DIR")).display()
            ),
        )
        .unwrap();

        Self { path }
    }

    fn write_lib(&self, contents: &str) {
        fs::write(self.path.join("src/lib.rs"), contents).unwrap();
    }

    fn cargo_check(&self) -> std::process::Output {
        Command::new(env!("CARGO"))
            .arg("check")
            .arg("--quiet")
            .arg("--manifest-path")
            .arg(self.path.join("Cargo.toml"))
            .output()
            .unwrap()
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn feature_gated_rules_report_missing_feature_flags() {
    if std::env::var_os(RUN_ENV).is_none() {
        eprintln!("skipping; set {RUN_ENV}=1 or run `cargo xtask test feature-flags`");
        return;
    }

    for (rule, feature) in [
        ("email", "email"),
        ("url", "url"),
        ("credit_card", "credit-card"),
        ("phone_number", "phone-number"),
    ] {
        let project = TempProject::new(
            &format!("missing-{}", feature.replace('-', "_")),
            &["derive"],
        );
        project.write_lib(&format!(
            r#"#[derive(garde::Validate)]
struct Test {{
    #[garde({rule})]
    value: String,
}}
"#
        ));

        let output = project.cargo_check();
        assert!(
            !output.status.success(),
            "`#[garde({rule})]` unexpectedly compiled without the `{feature}` feature"
        );

        let stderr = String::from_utf8_lossy(&output.stderr);
        let expected = format!("validation rule `{rule}` requires the `{feature}` feature flag");
        assert!(
            stderr.contains(&expected),
            "missing expected error `{expected}` in stderr:\n{stderr}"
        );
    }
}

#[test]
fn feature_gated_rules_compile_when_their_features_are_enabled() {
    if std::env::var_os(RUN_ENV).is_none() {
        eprintln!("skipping; set {RUN_ENV}=1 or run `cargo xtask test feature-flags`");
        return;
    }

    for (rule, feature) in [
        ("email", "email"),
        ("url", "url"),
        ("credit_card", "credit-card"),
        ("phone_number", "phone-number"),
    ] {
        let project = TempProject::new(
            &format!("enabled-{}", feature.replace('-', "_")),
            &["derive", feature],
        );
        project.write_lib(&format!(
            r#"#[derive(garde::Validate)]
struct Test {{
    #[garde({rule})]
    value: String,
}}
"#
        ));

        let output = project.cargo_check();
        assert!(
            output.status.success(),
            "`#[garde({rule})]` did not compile with the `{feature}` feature:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
