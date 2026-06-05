//! `rusty-idd spec archive` — the filesystem-owning half of the archive flow
//! that the spec lib deferred to the CLI (design §5).
//!
//! Flow:
//! 1. Discover delta `spec.md` files under `<change>/specs/<cap>/spec.md`.
//! 2. Resolve each capability's base spec under `<openspec>/specs/<cap>/spec.md`.
//! 3. Drive the transactional [`rusty_idd_spec::archive::archive_specs`] merge:
//!    if ANY merge errors, write NOTHING and abort.
//! 4. On success: write the merged base specs, then move the change dir to
//!    `<changes>/archive/<change>/`.

use std::path::{Path, PathBuf};

use rusty_idd_runner::data::discover_specs;
use rusty_idd_spec::archive::{archive_specs, SpecMerge};

/// Run `spec archive`. Returns a process exit code.
pub fn run(change_dir: &Path, skip_specs: bool, no_validate: bool, _yes: bool) -> i32 {
    let _ = no_validate; // validation of the merged result is handled by the lib's
                         // structural rules at `spec validate`; archive here mirrors
                         // the oracle's merge transactionality, not re-validation.

    if !change_dir.is_dir() {
        eprintln!(
            "rusty-idd: change directory not found: {}",
            change_dir.display()
        );
        return 1;
    }

    if skip_specs {
        return move_change_dir(change_dir);
    }

    // 1. Discover delta specs in the change.
    let specs = discover_specs(change_dir);
    if specs.is_empty() {
        // No delta specs to merge — just move the change dir.
        println!("No delta specs found under {}.", change_dir.display());
        return move_change_dir(change_dir);
    }

    // 2. Resolve base specs and read sources. We must read EVERYTHING up front
    //    so the transactional driver sees all pairs before any write.
    let specs_root = match base_specs_root(change_dir) {
        Some(p) => p,
        None => {
            eprintln!(
                "rusty-idd: could not locate the base specs root for {}",
                change_dir.display()
            );
            return 1;
        }
    };

    struct Loaded {
        capability: String,
        base_path: PathBuf,
        base_src: String,
        delta_src: String,
    }

    let mut loaded: Vec<Loaded> = Vec::with_capacity(specs.len());
    for item in &specs {
        let base_path = specs_root.join(&item.name).join("spec.md");
        let base_src = match std::fs::read_to_string(&base_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "rusty-idd: failed to read base spec {}: {e}",
                    base_path.display()
                );
                return 1;
            }
        };
        let delta_src = match std::fs::read_to_string(&item.path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "rusty-idd: failed to read delta spec {}: {e}",
                    item.path.display()
                );
                return 1;
            }
        };
        loaded.push(Loaded {
            capability: item.name.clone(),
            base_path,
            base_src,
            delta_src,
        });
    }

    // 3. Transactional merge — abort-all-on-any-failure, write nothing on error.
    let merges: Vec<SpecMerge<'_>> = loaded
        .iter()
        .map(|l| SpecMerge {
            capability: &l.capability,
            base_src: &l.base_src,
            delta_src: &l.delta_src,
        })
        .collect();

    let merged = match archive_specs(&merges) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("rusty-idd: {err}");
            eprintln!("Aborted. No files were changed.");
            return 1;
        }
    };

    // 4a. Write merged base specs (all merges already succeeded in memory).
    for result in &merged {
        // Find the matching base path by capability.
        let base_path = loaded
            .iter()
            .find(|l| l.capability == result.capability)
            .map(|l| l.base_path.clone());
        let Some(base_path) = base_path else {
            // Unreachable: every merged result comes from a loaded entry.
            eprintln!(
                "rusty-idd: internal error: no base path for {}",
                result.capability
            );
            return 1;
        };
        if let Err(e) = std::fs::write(&base_path, &result.markdown) {
            eprintln!(
                "rusty-idd: failed to write merged spec {}: {e}",
                base_path.display()
            );
            return 1;
        }
        let c = result.counts;
        println!(
            "  {} (+{} ~{} -{} →{})",
            result.capability, c.added, c.modified, c.removed, c.renamed
        );
    }

    // 4b. Move the change dir into archive/.
    move_change_dir(change_dir)
}

/// The base specs root for a change at `<openspec>/changes/<change>/` is
/// `<openspec>/specs/`. We compute it as `change_dir/../../specs` and verify it
/// exists.
fn base_specs_root(change_dir: &Path) -> Option<PathBuf> {
    let changes_dir = change_dir.parent()?; // .../changes
    let openspec_root = changes_dir.parent()?; // .../openspec (or repo root)
    let candidate = openspec_root.join("specs");
    if candidate.is_dir() {
        Some(candidate)
    } else {
        None
    }
}

/// Move `<changes>/<change>/` to `<changes>/archive/<change>/`.
fn move_change_dir(change_dir: &Path) -> i32 {
    let Some(name) = change_dir.file_name() else {
        eprintln!("rusty-idd: change directory has no name component");
        return 1;
    };
    let Some(changes_dir) = change_dir.parent() else {
        eprintln!("rusty-idd: change directory has no parent");
        return 1;
    };
    let archive_dir = changes_dir.join("archive");
    if let Err(e) = std::fs::create_dir_all(&archive_dir) {
        eprintln!(
            "rusty-idd: failed to create archive dir {}: {e}",
            archive_dir.display()
        );
        return 1;
    }
    let dest = archive_dir.join(name);
    if dest.exists() {
        eprintln!(
            "rusty-idd: archive destination already exists: {}",
            dest.display()
        );
        return 1;
    }
    if let Err(e) = std::fs::rename(change_dir, &dest) {
        eprintln!(
            "rusty-idd: failed to move {} -> {}: {e}",
            change_dir.display(),
            dest.display()
        );
        return 1;
    }
    println!("Archived change to {}.", dest.display());
    0
}
