#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

DATA_DIRS=(
    "$ROOT_DIR/backend/data/node_data"
    "$ROOT_DIR/backend/data/node2"
)

for DIR in "${DATA_DIRS[@]}"; do
    echo "Cleaning $DIR..."
    rm -f \
        "$DIR/operations.sqlite" \
        "$DIR/operations.sqlite-wal" \
        "$DIR/operations.sqlite-shm" \
        "$DIR/projections.sqlite" \
        "$DIR/projections.sqlite-wal" \
        "$DIR/projections.sqlite-shm"
    sed -i '/^region_ids = \[/,/^\]/d' "$DIR/config.toml"
done

echo "Done."
