//! Integration tests for the `rusty-idd spec` subcommands, run against the
//! compiled binary and the oracle fixtures.

use std::path::{Path, PathBuf};
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_rusty-idd")
}

/// Repo root = three levels up from `crates/cli/tests` → the workspace root,
/// where `docs/rusty-idd/oracle-fixtures/` lives.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn fixture(name: &str) -> PathBuf {
    repo_root()
        .join("docs/rusty-idd/oracle-fixtures")
        .join(name)
}

#[test]
fn spec_validate_base_fixture_is_valid_json() {
    let out = Command::new(bin())
        .args(["spec", "validate"])
        .arg(fixture("01-base-spec.md"))
        .arg("--json")
        .output()
        .expect("run rusty-idd");

    assert!(out.status.success(), "base spec should validate (exit 0)");

    let json: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("stdout must be valid JSON");
    let item = &json["items"][0];
    assert_eq!(item["type"], "spec");
    assert_eq!(item["valid"], true);
    // The only issue is the Purpose-too-brief WARNING (matching oracle 04).
    assert_eq!(item["issues"][0]["level"], "WARNING");
    assert_eq!(item["issues"][0]["path"], "overview");
    assert_eq!(json["summary"]["totals"]["passed"], 1);
    assert_eq!(json["summary"]["totals"]["failed"], 0);
    assert_eq!(json["version"], "1.0");
}

#[test]
fn spec_validate_no_scenario_is_invalid_with_error() {
    let dir = tempfile::tempdir().unwrap();
    let spec = dir.path().join("spec.md");
    std::fs::write(
        &spec,
        "# no-scenario Specification\n\n## Purpose\n\
         A sufficiently long purpose so the brevity warning does not fire here.\n\n\
         ## Requirements\n\n### Requirement: Some rule\nThe system SHALL do a thing.\n",
    )
    .unwrap();

    let out = Command::new(bin())
        .args(["spec", "validate"])
        .arg(&spec)
        .arg("--json")
        .output()
        .expect("run rusty-idd");

    assert_eq!(
        out.status.code(),
        Some(1),
        "a no-scenario spec must exit nonzero"
    );

    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let item = &json["items"][0];
    assert_eq!(item["valid"], false);
    let err = &item["issues"][0];
    assert_eq!(err["level"], "ERROR");
    assert_eq!(err["path"], "requirements.0.scenarios");
    assert!(err["message"]
        .as_str()
        .unwrap()
        .contains("at least one scenario"));
    assert_eq!(json["summary"]["totals"]["failed"], 1);
}

#[test]
fn spec_validate_strict_fails_on_warning() {
    // The base fixture is valid but has a WARNING; --strict must fail it.
    let out = Command::new(bin())
        .args(["spec", "validate"])
        .arg(fixture("01-base-spec.md"))
        .arg("--strict")
        .output()
        .expect("run rusty-idd");
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn spec_show_lists_requirements() {
    let out = Command::new(bin())
        .args(["spec", "show"])
        .arg(fixture("01-base-spec.md"))
        .output()
        .expect("run rusty-idd");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("Requirements: 4"));
    assert!(text.contains("CSV export"));
    assert!(text.contains("Export rate limit"));
}
