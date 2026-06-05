//! Integration tests for `rusty-idd spec archive`: the transactional merge +
//! filesystem move, exercised through the compiled binary against the oracle
//! fixtures.

use std::path::{Path, PathBuf};
use std::process::Command;

use rusty_idd_spec::parse_spec;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_rusty-idd")
}

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

/// Build a temp OpenSpec layout with a base spec and a delta, returning the
/// (root, change_dir, base_spec_path).
fn make_change(delta_fixture: &str) -> (tempfile::TempDir, PathBuf, PathBuf) {
    let root = tempfile::tempdir().unwrap();
    let base = root.path().join("openspec/specs/widget-export");
    let delta = root
        .path()
        .join("openspec/changes/add-json/specs/widget-export");
    std::fs::create_dir_all(&base).unwrap();
    std::fs::create_dir_all(&delta).unwrap();
    std::fs::copy(fixture("01-base-spec.md"), base.join("spec.md")).unwrap();
    std::fs::copy(fixture(delta_fixture), delta.join("spec.md")).unwrap();
    let change_dir = root.path().join("openspec/changes/add-json");
    let base_spec = base.join("spec.md");
    (root, change_dir, base_spec)
}

#[test]
fn archive_merges_and_moves() {
    let (root, change_dir, base_spec) = make_change("02-delta-spec.md");

    let out = Command::new(bin())
        .args(["spec", "archive"])
        .arg(&change_dir)
        .output()
        .expect("run rusty-idd");
    assert!(
        out.status.success(),
        "archive should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Merged base spec is semantically correct.
    let merged_src = std::fs::read_to_string(&base_spec).unwrap();
    let doc = parse_spec(&merged_src);
    let names: Vec<&str> = doc.requirements.iter().map(|r| r.name.as_str()).collect();

    // ADDED appends last; REMOVED dropped "Legacy XML"; RENAMED kept position.
    assert_eq!(
        names,
        vec![
            "CSV export",
            "Export rate limit",
            "Exported file naming", // RENAMED from "Export filename", in place
            "JSON export",          // ADDED, appended last
        ],
        "requirement order/content must reflect the delta"
    );

    // MODIFIED = whole-block replacement: rate limit 10 -> 20.
    let rate = doc
        .requirements
        .iter()
        .find(|r| r.name == "Export rate limit")
        .unwrap();
    let body: String = rate.body.iter().map(|b| b.text.clone()).collect();
    assert!(
        body.contains("20 per hour"),
        "MODIFIED should replace to 20"
    );
    assert!(!body.contains("10 per hour"));

    // The change dir was moved under archive/.
    assert!(
        !change_dir.exists(),
        "original change dir must be gone after archive"
    );
    let archived = root.path().join("openspec/changes/archive/add-json");
    assert!(archived.is_dir(), "change must be moved to archive/");
}

#[test]
fn archive_aborts_transactionally_on_bad_delta() {
    let root = tempfile::tempdir().unwrap();
    let base = root.path().join("openspec/specs/widget-export");
    let delta = root.path().join("openspec/changes/bad/specs/widget-export");
    std::fs::create_dir_all(&base).unwrap();
    std::fs::create_dir_all(&delta).unwrap();
    std::fs::copy(fixture("01-base-spec.md"), base.join("spec.md")).unwrap();
    // MODIFIED of a requirement that does not exist -> NotFound -> abort.
    std::fs::write(
        delta.join("spec.md"),
        "## MODIFIED Requirements\n\n### Requirement: Nonexistent thing\n\
         The system SHALL do a thing.\n\n#### Scenario: x\n- **WHEN** y\n- **THEN** z\n",
    )
    .unwrap();

    let base_spec = base.join("spec.md");
    let before = std::fs::read_to_string(&base_spec).unwrap();
    let change_dir = root.path().join("openspec/changes/bad");

    let out = Command::new(bin())
        .args(["spec", "archive"])
        .arg(&change_dir)
        .output()
        .expect("run rusty-idd");

    assert_eq!(out.status.code(), Some(1), "bad delta must exit nonzero");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("Aborted. No files were changed."),
        "stderr must announce the abort: {stderr}"
    );

    // Nothing was written: base spec unchanged, change dir not moved.
    let after = std::fs::read_to_string(&base_spec).unwrap();
    assert_eq!(before, after, "base spec must be untouched on abort");
    assert!(change_dir.is_dir(), "change dir must NOT be moved on abort");
    assert!(
        !root.path().join("openspec/changes/archive/bad").exists(),
        "no archived copy on abort"
    );
}
