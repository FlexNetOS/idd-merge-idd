#!/usr/bin/env bash
# Rust-native drift gate for idd-merge-idd / rusty-idd.
#
# Layout-agnostic by design: works on the CURRENT two-crate repo
# (intent-driven-development/, openspec-tui-main/) AND on the TARGET rusty-idd
# Cargo workspace (crates/*). The unification restructures the tree, so this
# gate must not hardcode the old paths — earlier versions did, and went blind
# (purity check passed vacuously, lockfile check false-positived) the moment
# crates moved. Run from the repo root.
#
# Exit 0 = no drift; exit 1 = drift detected OR unverifiable; exit 2 = cannot run.

set -uo pipefail
ROOT="${1:-.}"
cd "$ROOT" || { echo "drift-check: cannot cd to $ROOT"; exit 2; }

fail=0

# Asset trees that are LEGITIMATELY non-Rust (OpenSpec schema, agent skills,
# opencode commands, CI, docs). Never flagged as drift.
WHITELIST_RE='(^|/)(intent-driven-template|\.gemini|\.github|docs|openspec|\.opencode|\.agents)(/|$)'

echo "== Rust-native source purity =="
# Discover every Rust crate by its Cargo.toml, then assert its src/ tree is
# .rs-only. Works whether crates sit at the repo root or under crates/*.
mapfile -t crate_dirs < <(find . \
  \( -path ./.git -o -path '*/target/*' -o -path '*/node_modules/*' \) -prune -o \
  -name Cargo.toml -print 2>/dev/null | sed 's#/Cargo.toml$##' | sort -u)

src_trees=()
for d in "${crate_dirs[@]}"; do
  [ -d "$d/src" ] && src_trees+=("$d/src")   # skips a virtual workspace root (no src/)
done

if [ "${#src_trees[@]}" -eq 0 ]; then
  echo "ERROR: no crate src/ trees found — cannot verify purity (layout moved or run from wrong dir)."
  fail=1
else
  foreign=$(find "${src_trees[@]}" -type f 2>/dev/null | grep -v '\.rs$')
  if [ -n "$foreign" ]; then
    echo "DRIFT: non-Rust source files inside crate src/ trees:"
    echo "$foreign" | sed 's/^/  - /'
    fail=1
  else
    echo "OK: ${#src_trees[@]} crate src/ tree(s) are .rs only:"
    printf '  - %s\n' "${src_trees[@]}"
  fi
fi

echo "== Zero-dependency core invariant =="
# The std-only core (today: intent-driven-development; target: crates/core or
# crates/idd) must keep its OWN [dependencies] table empty. Parse the crate's
# Cargo.toml — NOT a lockfile package count, which legitimately grows once the
# core lives in a workspace alongside crates that DO have dependencies.
core_manifest=""
for cand in crates/core/Cargo.toml crates/idd/Cargo.toml intent-driven-development/Cargo.toml; do
  [ -f "$cand" ] && { core_manifest="$cand"; break; }
done
if [ -z "$core_manifest" ]; then
  echo "WARN: zero-dep core crate not found at known paths — skipping (update drift-check when the core crate is created/moved)."
else
  # Body of [dependencies] up to the next [section], minus comments/blank lines.
  deps=$(awk '/^\[dependencies\]/{f=1;next} /^\[/{f=0} f{print}' "$core_manifest" \
         | grep -v '^[[:space:]]*#' | grep '[^[:space:]]')
  if [ -n "$deps" ]; then
    echo "DRIFT: core crate $core_manifest declares dependencies (must stay std-only):"
    echo "$deps" | sed 's/^/  - /'
    fail=1
  else
    echo "OK: core crate $core_manifest has an empty [dependencies] table (std-only preserved)"
  fi
fi

echo "== Stray auto-generated foreign packages =="
# Catch agent tooling that auto-pushes a package in another language/format
# (.omc, ecc-style, Node/Python/Go manifests) OUTSIDE the whitelisted asset trees.
stray=$(find . -maxdepth 4 \
  \( -path ./.git -o -path '*/target/*' -o -path '*/node_modules/*' \) -prune -o \
  -type f \( -name '*.omc' -o -name '*.ecc' -o -name 'package.json' -o -name 'pyproject.toml' \
             -o -name 'go.mod' -o -name 'requirements.txt' \) -print 2>/dev/null \
  | grep -Ev "$WHITELIST_RE")
if [ -n "$stray" ]; then
  echo "REVIEW: foreign package/manifest files outside whitelisted asset trees — confirm intended, not drift:"
  echo "$stray" | sed 's/^/  - /'
else
  echo "OK: no stray foreign package manifests outside whitelisted trees"
fi

echo
if [ "$fail" -ne 0 ]; then
  echo "RESULT: DRIFT DETECTED (or unverifiable) — port to Rust-native / fix layout, then re-run."
  exit 1
fi
echo "RESULT: no Rust-native drift detected."
exit 0
