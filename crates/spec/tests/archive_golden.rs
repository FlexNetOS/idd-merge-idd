//! Archive semantic golden test (the GATE item 1).
//!
//! Parse `01-base-spec.md` + `02-delta-spec.md`, merge, emit; then parse BOTH
//! the emitted output and `03-archived-result.md` into `SpecDoc` and assert the
//! MODELS are equal. This proves correct merge semantics independent of
//! whitespace (byte-exact parity with the oracle's quirky formatter is an
//! explicit NON-goal).

use rusty_idd_spec::{apply_delta, emit_spec, parse_delta, parse_spec};

const BASE: &str = include_str!("../../../docs/rusty-idd/oracle-fixtures/01-base-spec.md");
const DELTA: &str = include_str!("../../../docs/rusty-idd/oracle-fixtures/02-delta-spec.md");
const EXPECTED: &str =
    include_str!("../../../docs/rusty-idd/oracle-fixtures/03-archived-result.md");

#[test]
fn archive_produces_semantically_equal_model() {
    let base = parse_spec(BASE);
    let delta = parse_delta(DELTA);
    let merged = apply_delta(&base, &delta).expect("merge must succeed");

    let emitted = emit_spec(&merged);
    let from_emitted = parse_spec(&emitted);
    let from_oracle = parse_spec(EXPECTED);

    assert_eq!(
        from_emitted, from_oracle,
        "merged model must equal the oracle's archived model"
    );
}

#[test]
fn archive_requirement_order_and_ops() {
    let base = parse_spec(BASE);
    let delta = parse_delta(DELTA);
    let merged = apply_delta(&base, &delta).unwrap();

    let names: Vec<&str> = merged
        .requirements
        .iter()
        .map(|r| r.name.as_str())
        .collect();
    assert_eq!(
        names,
        vec![
            "CSV export",           // untouched, position 0
            "Export rate limit",    // MODIFIED in place
            "Exported file naming", // RENAMED in place (was "Export filename")
            "JSON export",          // ADDED last
        ],
        "Legacy XML export REMOVED; RENAMED in place; ADDED appended last"
    );

    // MODIFIED = whole-block replace: rate limit body now says 20, scenarios replaced.
    let rate = &merged.requirements[1];
    assert!(rate.body[0].text.contains("20 per hour"));
    assert_eq!(rate.scenarios.len(), 2);
    assert!(rate.scenarios[0].steps[0].text.contains("19 times"));
}

#[test]
fn emit_is_well_formed_and_idempotent() {
    let base = parse_spec(BASE);
    let delta = parse_delta(DELTA);
    let merged = apply_delta(&base, &delta).unwrap();

    let once = emit_spec(&merged);
    // parse(emit(x)) == x  (model round-trips through emit)
    assert_eq!(parse_spec(&once), merged, "parse(emit(x)) must equal x");

    // emit(parse(emit(x))) == emit(parse(emit(x)))  -> idempotent
    let twice = emit_spec(&parse_spec(&once));
    assert_eq!(once, twice, "emit must be idempotent");

    // Well-formed: ends in exactly one trailing newline.
    assert!(once.ends_with('\n'));
    assert!(!once.ends_with("\n\n"));
}

#[test]
fn round_trip_base_spec_stability() {
    // parse(emit(parse(x))) == parse(x) for the base spec.
    let doc = parse_spec(BASE);
    let re = parse_spec(&emit_spec(&doc));
    assert_eq!(doc, re);
}
