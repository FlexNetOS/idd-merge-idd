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
    /// Show a change's artifact-DAG status (which artifacts are done/ready) and
    /// whether it is archivable.
    Status {
        /// The change directory (e.g. `openspec/changes/<change>`).
        change_dir: PathBuf,
    },
    /// Print the next ready artifact for a change (scriptable), per the schema
    /// DAG.
    Next {
        /// The change directory (e.g. `openspec/changes/<change>`).
        change_dir: PathBuf,
    },
    /// Inspect Architecture Decision Records (in-force set / next number).
    Adr {
        #[command(subcommand)]
        command: crate::commands::spec_adr::AdrCommand,
    },
    /// Render an artifact stub (proposal/spec/design/adr/tasks) to stdout.
    Scaffold {
        /// Artifact to render: proposal | spec | design | adr | tasks.
        artifact: String,
        /// Change name (for proposal/design/tasks titles).
        #[arg(long)]
        change: Option<String>,
        /// ADR number (for the adr template).
        #[arg(long)]
        number: Option<String>,
        /// ADR title (for the adr template).
        #[arg(long)]
        title: Option<String>,
        /// ADR date (for the adr template).
        #[arg(long)]
        date: Option<String>,
    },
    /// Create a new change directory seeded with a proposal stub.
    New {
        /// The change name (kebab-case).
        change: String,
        /// Base directory containing `openspec/` (defaults to the current dir).
        #[arg(long, default_value = ".")]
        base: PathBuf,
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
        SpecCommand::Status { change_dir } => crate::commands::spec_status::run_status(&change_dir),
        SpecCommand::Next { change_dir } => crate::commands::spec_status::run_next(&change_dir),
        SpecCommand::Adr { command } => crate::commands::spec_adr::run(command),
        SpecCommand::Scaffold {
            artifact,
            change,
            number,
            title,
            date,
        } => crate::commands::spec_scaffold::run_scaffold(&artifact, change, number, title, date),
        SpecCommand::New { change, base } => {
            crate::commands::spec_scaffold::run_new(&change, &base)
        }
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
        // The JSON payload mirrors the oracle shape exactly and is strict-blind
        // (the oracle's `valid`/`totals` count ERRORs only). `--strict` affects
        // only the human summary and the process exit code, never this payload.
        match serde_json::to_string_pretty(&report) {
            Ok(s) => println!("{s}"),
            Err(e) => {
                eprintln!("rusty-idd: failed to serialize report: {e}");
                return 1;
            }
        }
    } else {
        print_human_report(&report, strict);
    }

    if report_failed(&report, strict) {
        1
    } else {
        0
    }
}

/// Whether a single item fails: any ERROR, or (under `--strict`) any WARNING.
/// The strict-aware predicate the human summary and the exit code share, so they
/// can never disagree (the bug this slice fixes).
fn item_failed(item: &rusty_idd_spec::validate::Item, strict: bool) -> bool {
    item.issues.iter().any(|issue| {
        issue.level == IssueLevel::Error || (strict && issue.level == IssueLevel::Warning)
    })
}

/// A report fails when any item fails.
fn report_failed(report: &Report, strict: bool) -> bool {
    report.items.iter().any(|item| item_failed(item, strict))
}

fn print_human_report(report: &Report, strict: bool) {
    let mut passed = 0u64;
    let mut failed = 0u64;
    for item in &report.items {
        // Strict-aware status: under --strict a WARNING-only item reads INVALID,
        // consistent with the exit code (without --strict it stays VALID, as the
        // JSON payload reports). This is what reconciles summary with exit code.
        let failed_item = item_failed(item, strict);
        if failed_item {
            failed += 1;
        } else {
            passed += 1;
        }
        let status = if failed_item { "INVALID" } else { "VALID" };
        println!("{} [{}] {}", status, item.item_type, item.id);
        for issue in &item.issues {
            let level = match issue.level {
                IssueLevel::Error => "ERROR",
                IssueLevel::Warning => "WARNING",
            };
            println!("  {level} {}: {}", issue.path, issue.message);
        }
    }
    println!(
        "Summary: {} item(s), {} passed, {} failed.{}",
        report.items.len(),
        passed,
        failed,
        if strict {
            " (strict: WARNINGs count as failures)"
        } else {
            ""
        }
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
