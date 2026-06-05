//! `rusty-idd-spec` — the Rust-native port of the OpenSpec intent-driven
//! lifecycle engine (spec parse, delta-merge, validate, archive).
//!
//! Architecture (hexagonal, per `docs/rusty-idd/spec-engine-design.md`):
//! - [`model`] is the PURE core: requirement / scenario / spec / delta structs
//!   and the transactional [`model::apply_delta`] merge. No comrak / serde / io.
//! - [`parse`] is the comrak edge (AST <-> model, plus the emitter).
//! - [`validate`] is the serde edge (structural rules -> a report matching the
//!   oracle JSON shape).
//! - [`archive`] orchestrates merge + emit transactionally (the filesystem move
//!   is left to the CLI).
//! - [`schema`] is the serde_norway edge: loads `schema.yaml` into the artifact
//!   DAG and answers `next_ready` / `is_archivable` over a `done` set.
//!
//! Still deferred (later slices): `scaffold/` (minijinja templates) and `adr/`.
//! The `cli/` edge lives in `crates/cli`.

pub mod archive;
pub mod model;
pub mod parse;
pub mod schema;
pub mod validate;

// Convenience re-exports of the most-used items.
pub use model::{apply_delta, Delta, DeltaOp, MergeError, Requirement, Scenario, SpecDoc};
pub use parse::{emit_spec, parse_delta, parse_spec};
pub use schema::{load_schema, Artifact, Schema, SchemaError};
pub use validate::{validate_spec, Report};
