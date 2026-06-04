#!/usr/bin/env bash
# Rust-native drift gate for idd-merge-idd.
# Deterministic checks that the QA agent runs after every module.
# Exit 0 = no drift; exit 1 = drift detected (details on stdout).
# Usage: drift-check.sh [repo-root]   (defaults to current directory)

set -uo pipefail
ROOT="${1:-.}"
cd "$ROOT" || { echo "drift-check: cannot cd to $ROOT"; exit 2; }

fail=0

echo "== Rust-native source purity =="
# Both crate src/ trees must contain only .rs files. Foreign-language source = drift.
foreign=$(find intent-driven-development/src openspec-tui-main/src -type f 2>/dev/null | grep -v '\.rs$')
if [ -n "$foreign" ]; then
  echo "DRIFT: non-Rust source files found in crate src/ trees:"
  echo "$foreign" | sed 's/^/  - /'
  fail=1
else
  echo "OK: both src/ trees are .rs only"
fi

echo "== idd zero-dependency invariant =="
# intent-driven-development is std-only by design: Cargo.lock must hold exactly 1 package (itself).
if [ -f intent-driven-development/Cargo.lock ]; then
  pkgs=$(grep -c '^\[\[package\]\]' intent-driven-development/Cargo.lock)
  if [ "$pkgs" -ne 1 ]; then
    echo "DRIFT: idd Cargo.lock has $pkgs packages (expected 1 — itself). A dependency was added."
    grep '^name = ' intent-driven-development/Cargo.lock | sed 's/^/  /'
    fail=1
  else
    echo "OK: idd Cargo.lock has exactly 1 package (std-only preserved)"
  fi
else
  echo "WARN: intent-driven-development/Cargo.lock not found"
fi

echo "== Stray auto-generated foreign packages at repo root =="
# Agent tooling sometimes auto-pushes a package in another language/format (e.g. .omc, ecc-style,
# stray package.json / pyproject.toml / go.mod). These are drift to catch, not adopt.
stray=$(find . -maxdepth 3 \
  \( -path ./.git -o -path '*/target/*' -o -path '*/node_modules/*' \) -prune -o \
  -type f \( -name '*.omc' -o -name 'package.json' -o -name 'pyproject.toml' \
             -o -name 'go.mod' -o -name 'requirements.txt' -o -name '*.ecc' \) -print 2>/dev/null)
if [ -n "$stray" ]; then
  echo "REVIEW: foreign package/manifest files present — confirm each is intended, not auto-generated drift:"
  echo "$stray" | sed 's/^/  - /'
  # Non-fatal by itself (the template tree is legitimately non-Rust); QA must classify intentional vs drift.
else
  echo "OK: no stray foreign package manifests near the root"
fi

echo
if [ "$fail" -ne 0 ]; then
  echo "RESULT: DRIFT DETECTED — port to Rust-native and re-run before proceeding."
  exit 1
fi
echo "RESULT: no Rust-native drift detected."
exit 0
