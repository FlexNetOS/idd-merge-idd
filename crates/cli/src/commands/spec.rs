//! `rusty-idd spec` — the new CLI over `rusty-idd-spec` (validate / archive /
//! show). This completes the `cli/` edge the spec lib deliberately deferred.

use std::path::{Path, PathBuf};

use clap::Subcommand;
use rusty_idd_spec::validate::IssueLevel;
use rusty_idd_spec::{parse_spec, validate_spec, Report};

#[derive(Subcommand)]
pub enum SpecCommand {
    /// Structurally validate a spec file; exit nonzero if invalid.
    Validate {
        /// Path to the spec markdown file.
        file: PathBuf,
        /// Emit the JSON report (oracle shape) instead of a human summary.
        #[arg(long)]
        json: bool,
        /// Treat WARNINGs as failures too.
        #[arg(long)]
        strict: bool,
    },
    /// Archive a completed change: merge its delta specs into the base specs
    /// transactionally, then move the change dir under `archive/`.
    Archive {
        /// The change directory (containing `specs/<cap>/spec.md` deltas).
        change_dir: PathBuf,
        /// Skip the spec merge entirely; only move the change dir.
        #[arg(long)]
        skip_specs: bool,
        /// Skip pre-merge validation of the merged result.
        #[arg(long)]
        no_validate: bool,
        /// Assume yes; do not prompt for confirmation.
        #[arg(short = 'y', long)]
        yes: bool,
    },
    /// Parse a spec and print a concise requirement/scenario summary.
    Show {
        /// Path to the spec markdown file.
        file: PathBuf,
    },
}

/// Dispatch a `spec` subcommand, returning a process exit code.
pub fn run(cmd: SpecCommand) -> i32 {
    match cmd {
        SpecCommand::Validate { file, json, strict } => cmd_validate(&file, json, strict),
        SpecCommand::Archive {
            change_dir,
            skip_specs,
            no_validate,
            yes,
        } => crate::commands::spec_archive::run(&change_dir, skip_specs, no_validate, yes),
        SpecCommand::Show { file } => cmd_show(&file),
    }
}

/// Derive the capability id used in the report from a spec path: the parent dir
/// name when the file is `spec.md` (the OpenSpec `specs/<cap>/spec.md` layout),
/// else the file stem.
pub fn capability_id(path: &Path) -> String {
    let is_spec_md = path.file_name().map(|n| n == "spec.md").unwrap_or(false);
    if is_spec_md {
        if let Some(parent) = path.parent().and_then(|p| p.file_name()) {
            return parent.to_string_lossy().into_owned();
        }
    }
    path.file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned())
}

fn cmd_validate(file: &Path, json: bool, strict: bool) -> i32 {
    let src = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("rusty-idd: failed to read {}: {e}", file.display());
            return 1;
        }
    };
    let id = capability_id(file);
    let report = validate_spec(&id, &src);

    if json {
        match serde_json::to_string_pretty(&report) {
            Ok(s) => println!("{s}"),
            Err(e) => {
                eprintln!("rusty-idd: failed to serialize report: {e}");
                return 1;
            }
        }
    } else {
        print_human_report(&report);
    }

    if report_failed(&report, strict) {
        1
    } else {
        0
    }
}

/// A report fails when any item has an ERROR, or (under `--strict`) a WARNING.
fn report_failed(report: &Report, strict: bool) -> bool {
    report.items.iter().any(|item| {
        item.issues.iter().any(|issue| {
            issue.level == IssueLevel::Error || (strict && issue.level == IssueLevel::Warning)
        })
    })
}

fn print_human_report(report: &Report) {
    for item in &report.items {
        let status = if item.valid { "VALID" } else { "INVALID" };
        println!("{} [{}] {}", status, item.item_type, item.id);
        for issue in &item.issues {
            let level = match issue.level {
                IssueLevel::Error => "ERROR",
                IssueLevel::Warning => "WARNING",
            };
            println!("  {level} {}: {}", issue.path, issue.message);
        }
    }
    let t = &report.summary.totals;
    println!(
        "Summary: {} item(s), {} passed, {} failed.",
        t.items, t.passed, t.failed
    );
}

fn cmd_show(file: &Path) -> i32 {
    let src = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("rusty-idd: failed to read {}: {e}", file.display());
            return 1;
        }
    };
    let doc = parse_spec(&src);
    let title = doc.title.as_deref().unwrap_or("(untitled)");
    println!("Spec: {title}");
    println!("Requirements: {}", doc.requirements.len());
    for req in &doc.requirements {
        println!("  - {} ({} scenario(s))", req.name, req.scenarios.len());
        for sc in &req.scenarios {
            println!("      • {}", sc.name);
        }
    }
    0
}
