#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "error: memory leak checks require Linux (Valgrind is unsupported on this platform)" >&2
  exit 2
fi

if ! command -v valgrind >/dev/null 2>&1; then
  echo "error: valgrind is required; install it with your system package manager" >&2
  exit 2
fi

workspace_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$workspace_root"
export PG_RAW_PARSE_USE_VALGRIND=1

test_binary="$({
  cargo test \
    --test postgres_regress \
    --no-run \
    --color never \
    --message-format=json
} | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -n 1)"

if [[ -z "$test_binary" || ! -x "$test_binary" ]]; then
  echo "error: failed to locate the compiled postgres_regress test binary" >&2
  exit 2
fi

exec valgrind \
  --tool=memcheck \
  --leak-check=full \
  --show-leak-kinds=all \
  --errors-for-leak-kinds=definite,indirect,possible \
  --error-exitcode=1 \
  --num-callers=40 \
  --suppressions="$workspace_root/libpg_query/test/valgrind.supp" \
  "$test_binary" \
  --exact postgres_regression_sql_parses_and_round_trips \
  --nocapture
