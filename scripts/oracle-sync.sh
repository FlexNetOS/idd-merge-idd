#!/usr/bin/env bash
#
# oracle-sync.sh — Cross-engine differential test for OpenSpec lifecycle.
#
# Runs both the Rust-native 'rusty-idd' engine and the Node 'openspec' oracle (1.4.1)
# on the same input and asserts they produce the same result.
#
# Usage:
#   scripts/oracle-sync.sh validate <name> --type spec|change
#   scripts/oracle-sync.sh archive <change_dir>
#

set -euo pipefail

ORACLE_VERSION="1.4.1"
RUSTY_BIN="$(pwd)/target/debug/rusty-idd"
NODE_ORACLE="bunx --quiet @fission-ai/openspec@${ORACLE_VERSION}"

# Work in a temp dir to avoid polluting the workspace
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

case "$1" in
    validate)
        NAME="$2"
        shift 2
        echo "== Validating $NAME (Oracle vs Rust) =="
        
        # 1. Oracle
        echo "Oracle..."
        $NODE_ORACLE validate "$NAME" "$@" --json > "$TMP_DIR/oracle.json"
        
        # 2. Rust
        echo "Rust..."
        # rusty-idd needs a path for a single item, or name + --specs/--changes
        if [[ "$*" == *"--type spec"* ]]; then
            PATH_TO_VALIDATE="openspec/specs/$NAME/spec.md"
        else
            PATH_TO_VALIDATE="openspec/changes/$NAME/spec.md"
        fi
        
        $RUSTY_BIN spec validate "$PATH_TO_VALIDATE" --json > "$TMP_DIR/rust.json"
        
        # Compare (ignoring durationMs)
        jq -S 'walk(if type == "object" and has("durationMs") then .durationMs = null else . end)' "$TMP_DIR/oracle.json" > "$TMP_DIR/oracle_norm.json"
        jq -S 'walk(if type == "object" and has("durationMs") then .durationMs = null else . end)' "$TMP_DIR/rust.json" > "$TMP_DIR/rust_norm.json"
        
        if diff -u "$TMP_DIR/oracle_norm.json" "$TMP_DIR/rust_norm.json"; then
            echo "SUCCESS: Validation parity confirmed."
        else
            echo "FAILURE: Parity mismatch in validation report."
            exit 1
        fi
        ;;
    
    archive)
        CHANGE_DIR="$2"
        CHANGE_NAME=$(basename "$CHANGE_DIR")
        echo "== Archiving $CHANGE_NAME (Oracle vs Rust) =="
        
        setup_env() {
            local target="$1"
            mkdir -p "$target/openspec/specs"
            mkdir -p "$target/openspec/changes"
            cp -r openspec/specs/* "$target/openspec/specs/" 2>/dev/null || true
            cp -r "openspec/changes/$CHANGE_NAME" "$target/openspec/changes/"
        }
        
        # 1. Oracle run
        ORACLE_ENV="$TMP_DIR/oracle_env"
        setup_env "$ORACLE_ENV"
        echo "Oracle..."
        (cd "$ORACLE_ENV" && $NODE_ORACLE archive "$CHANGE_NAME" -y > /dev/null)
        
        # 2. Rust run
        RUST_ENV="$TMP_DIR/rust_env"
        setup_env "$RUST_ENV"
        echo "Rust..."
        (cd "$RUST_ENV" && $RUSTY_BIN spec archive "openspec/changes/$CHANGE_NAME" -y > /dev/null)
        
        # Compare results
        echo "Comparing specs..."
        if diff -r "$ORACLE_ENV/openspec/specs" "$RUST_ENV/openspec/specs"; then
            echo "SUCCESS: Spec merge parity confirmed."
        else
            echo "FAILURE: Merged specs differ."
            exit 1
        fi
        ;;

    *)
        echo "Usage: $0 {validate|archive} [args]"
        exit 1
        ;;
esac
