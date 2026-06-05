//! The delta-merge algorithm (the crux). Pure: `SpecDoc` + `Delta` in, a new
//! `SpecDoc` (or a `MergeError`) out — no comrak, no I/O.
//!
//! Faithful to the `oracle-verified` archive semantics (contract §3, design §5):
//! - **ADDED**  — name must be absent; append to the end.
//! - **MODIFIED**— name must exist; replace the WHOLE requirement in place
//!   (keep index, discard old scenarios). NOT a scenario merge.
//! - **REMOVED** — name must exist; delete the block.
//! - **RENAMED** — `from` must exist, `to` must be absent; change the name in
//!   place (keep index, body, scenarios).
//!
//! **Transactional.** Every op's precondition is validated against the *base*
//! first; on any failure nothing is mutated and the original `SpecDoc` is left
//! untouched (matching the CLI's `"Aborted. No files were changed."`).

use super::{DeltaOp, Requirement, SpecDoc};

/// A precondition failure during merge. The messages mirror the Node CLI's
/// abort messages so the CLI slice can surface them faithfully.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MergeError {
    /// ADDED of a name that already exists.
    #[error("ADDED failed for header \"### Requirement: {name}\" - already exists")]
    AlreadyExists { name: String },
    /// MODIFIED / REMOVED of a name that is not present.
    #[error("{op} failed for header \"### Requirement: {name}\" - not found")]
    NotFound { op: &'static str, name: String },
    /// RENAMED whose `to` name already exists (would collide).
    #[error("RENAMED failed for header \"### Requirement: {name}\" - already exists")]
    RenameTargetExists { name: String },
}

/// Apply a delta to a base spec, transactionally. All ops are validated against
/// `base` before any mutation; on any error the function returns `Err` and
/// `base` is observably unchanged (it is taken by reference; only a working
/// clone is mutated).
pub fn apply_delta(base: &SpecDoc, delta: &super::Delta) -> Result<SpecDoc, MergeError> {
    // 1. Validate EVERY precondition against the base state first. This is the
    //    transactional guarantee: a mid-merge failure mutates nothing.
    validate_preconditions(base, delta)?;

    // 2. Apply against a working clone. Preconditions held against base; the
    //    ops below cannot fail.
    let mut out = base.clone();
    for op in &delta.ops {
        apply_one(&mut out, op);
    }
    Ok(out)
}

/// Validate all op preconditions against the (immutable) base state.
fn validate_preconditions(base: &SpecDoc, delta: &super::Delta) -> Result<(), MergeError> {
    for op in &delta.ops {
        match op {
            DeltaOp::Added(req) => {
                if base.contains(&req.name) {
                    return Err(MergeError::AlreadyExists {
                        name: req.name.clone(),
                    });
                }
            }
            DeltaOp::Modified(req) => {
                if !base.contains(&req.name) {
                    return Err(MergeError::NotFound {
                        op: "MODIFIED",
                        name: req.name.clone(),
                    });
                }
            }
            DeltaOp::Removed { name, .. } => {
                if !base.contains(name) {
                    return Err(MergeError::NotFound {
                        op: "REMOVED",
                        name: name.clone(),
                    });
                }
            }
            DeltaOp::Renamed { from, to } => {
                if !base.contains(from) {
                    return Err(MergeError::NotFound {
                        op: "RENAMED",
                        name: from.clone(),
                    });
                }
                if base.contains(to) {
                    return Err(MergeError::RenameTargetExists { name: to.clone() });
                }
            }
        }
    }
    Ok(())
}

/// Apply a single op to the working doc. Preconditions are assumed already
/// validated against base, so the lookups here cannot legitimately miss; if a
/// later op references a name a previous op created/removed the `position_of`
/// fallback keeps the apply infallible (faithful "validate-against-base" model,
/// design §5).
fn apply_one(out: &mut SpecDoc, op: &DeltaOp) {
    match op {
        DeltaOp::Added(req) => out.requirements.push(req.clone()),
        DeltaOp::Modified(req) => {
            if let Some(i) = out.position_of(&req.name) {
                out.requirements[i] = req.clone();
            }
        }
        DeltaOp::Removed { name, .. } => {
            if let Some(i) = out.position_of(name) {
                out.requirements.remove(i);
            }
        }
        DeltaOp::Renamed { from, to } => {
            if let Some(i) = out.position_of(from) {
                rename_in_place(&mut out.requirements[i], to);
            }
        }
    }
}

/// Change only the heading text; body and scenarios are preserved verbatim.
/// Op matching uses `normalize_name`, but storage keeps the authored `to` text.
fn rename_in_place(req: &mut Requirement, to: &str) {
    req.name = to.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Block, Delta, Requirement, Scenario};

    fn req(name: &str, body: &str, scenarios: &[&str]) -> Requirement {
        let mut r = Requirement::new(name);
        r.body.push(Block::new(body));
        for s in scenarios {
            r.scenarios.push(Scenario::new(*s));
        }
        r
    }

    fn base() -> SpecDoc {
        SpecDoc {
            title: Some("widget-export Specification".into()),
            purpose: None,
            requirements: vec![
                req("CSV export", "The system SHALL export CSV.", &["ok"]),
                req(
                    "Export rate limit",
                    "The system SHALL limit to 10.",
                    &["within", "over"],
                ),
                req(
                    "Legacy XML export",
                    "The system SHALL export XML.",
                    &["xml"],
                ),
                req(
                    "Export filename",
                    "The system SHALL name files.",
                    &["fname"],
                ),
            ],
        }
    }

    // ---- happy paths ----

    #[test]
    fn added_appends_to_end() {
        let d = Delta::new(vec![DeltaOp::Added(req(
            "JSON export",
            "The system SHALL export JSON.",
            &["json"],
        ))]);
        let out = apply_delta(&base(), &d).unwrap();
        assert_eq!(out.requirements.len(), 5);
        assert_eq!(out.requirements[4].name, "JSON export");
    }

    #[test]
    fn modified_replaces_whole_block_in_place() {
        let newblock = req(
            "Export rate limit",
            "The system SHALL limit to 20.",
            &["within20"], // fewer scenarios than base (2) — proves whole replace, not merge
        );
        let d = Delta::new(vec![DeltaOp::Modified(newblock.clone())]);
        let out = apply_delta(&base(), &d).unwrap();
        // same position
        assert_eq!(out.requirements[1].name, "Export rate limit");
        // whole block replaced: body + scenarios are the delta's, old discarded
        assert_eq!(out.requirements[1], newblock);
        assert_eq!(out.requirements[1].scenarios.len(), 1);
    }

    #[test]
    fn removed_deletes_block() {
        let d = Delta::new(vec![DeltaOp::Removed {
            name: "Legacy XML export".into(),
            reason: Some("deprecated".into()),
            migration: Some("use JSON".into()),
        }]);
        let out = apply_delta(&base(), &d).unwrap();
        assert_eq!(out.requirements.len(), 3);
        assert!(!out.contains("Legacy XML export"));
    }

    #[test]
    fn renamed_changes_name_in_place_keeps_body() {
        let d = Delta::new(vec![DeltaOp::Renamed {
            from: "Export filename".into(),
            to: "Exported file naming".into(),
        }]);
        let out = apply_delta(&base(), &d).unwrap();
        assert_eq!(out.requirements[3].name, "Exported file naming");
        // body + scenarios preserved
        assert_eq!(out.requirements[3].scenarios.len(), 1);
        assert_eq!(
            out.requirements[3].body[0].text,
            "The system SHALL name files."
        );
    }

    #[test]
    fn whitespace_insensitive_match() {
        let d = Delta::new(vec![DeltaOp::Modified(req(
            "Export   rate  limit", // extra internal whitespace
            "The system SHALL limit to 20.",
            &["x"],
        ))]);
        let out = apply_delta(&base(), &d).unwrap();
        assert_eq!(out.requirements[1].name, "Export   rate  limit");
    }

    // ---- the four error modes + transactionality ----

    #[test]
    fn added_exists_errors_and_does_not_mutate() {
        let b = base();
        let d = Delta::new(vec![DeltaOp::Added(req(
            "CSV export",
            "The system SHALL export CSV again.",
            &["dup"],
        ))]);
        let err = apply_delta(&b, &d).unwrap_err();
        assert_eq!(
            err,
            MergeError::AlreadyExists {
                name: "CSV export".into()
            }
        );
        // transactionality: base is unchanged
        assert_eq!(b, base());
    }

    #[test]
    fn modified_not_found_errors_and_does_not_mutate() {
        let b = base();
        let d = Delta::new(vec![DeltaOp::Modified(req(
            "Nonexistent",
            "The system MUST do x.",
            &["s"],
        ))]);
        let err = apply_delta(&b, &d).unwrap_err();
        assert_eq!(
            err,
            MergeError::NotFound {
                op: "MODIFIED",
                name: "Nonexistent".into()
            }
        );
        assert_eq!(b, base());
    }

    #[test]
    fn removed_not_found_errors_and_does_not_mutate() {
        let b = base();
        let d = Delta::new(vec![DeltaOp::Removed {
            name: "Nonexistent".into(),
            reason: None,
            migration: None,
        }]);
        let err = apply_delta(&b, &d).unwrap_err();
        assert_eq!(
            err,
            MergeError::NotFound {
                op: "REMOVED",
                name: "Nonexistent".into()
            }
        );
        assert_eq!(b, base());
    }

    #[test]
    fn renamed_not_found_errors_and_does_not_mutate() {
        let b = base();
        let d = Delta::new(vec![DeltaOp::Renamed {
            from: "Nonexistent".into(),
            to: "Whatever".into(),
        }]);
        let err = apply_delta(&b, &d).unwrap_err();
        assert_eq!(
            err,
            MergeError::NotFound {
                op: "RENAMED",
                name: "Nonexistent".into()
            }
        );
        assert_eq!(b, base());
    }

    #[test]
    fn partial_failure_in_multi_op_delta_aborts_all() {
        // A valid ADDED followed by an invalid MODIFIED: nothing should apply.
        let b = base();
        let d = Delta::new(vec![
            DeltaOp::Added(req("JSON export", "SHALL export JSON.", &["j"])),
            DeltaOp::Modified(req("Nonexistent", "MUST x.", &["s"])),
        ]);
        assert!(apply_delta(&b, &d).is_err());
        assert_eq!(b, base()); // the ADDED did not leak through
    }
}
