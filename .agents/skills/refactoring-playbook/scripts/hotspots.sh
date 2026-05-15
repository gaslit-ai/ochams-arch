#!/bin/bash
# hotspots.sh — Rank top-decile refactoring targets via Tornhill churn × LOC.
#
# Path (Vercel Labs convention):
#   /mnt/skills/user/refactoring-playbook/scripts/hotspots.sh
#
# Backend:
#   `hotspot` (https://github.com/huangsam/hotspot) — actively maintained
#   Go CLI implementing Adam Tornhill's behavioral-code-analysis methodology.
#   This script is a thin wrapper that:
#     1. Picks ROI mode (refactoring-prioritization) by default
#     2. Defaults to a 90-day lookback matching empirical refactoring studies
#     3. Emits machine-readable JSON to stdout, status to stderr
#     4. Cleans up temp files via trap
#
# Empirical grounding (see rule `triggers-hot-files`):
#   - Nagappan & Ball (ICSE 2005) — relative-churn predicts defects at 89.0%
#     classification accuracy on Windows Server 2003.
#     https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/icse05churn.pdf
#   - Pantiuchina et al. (TOSEM 2020) — prior-change frequency is a strong
#     correlate of where developer-initiated refactoring concentrates across
#     287,813 operations in 150 systems.
#     https://arxiv.org/pdf/2101.01430
#   - Tornhill (2024) — Pareto regime: ~1–2% of files account for ~70% of
#     edit activity; defects cluster in the same subset.
#     https://pragprog.com/titles/atcrime2/
#
# Usage:
#   hotspots.sh [PATH] [options]
#
# Options:
#   --since DURATION   Lookback window (default: 90d). Accepts hotspot
#                      duration syntax: 30d, 6mo, 1y.
#   --limit N          Top-N hotspots to emit (default: 20).
#   --mode MODE        Scoring mode: hot | risk | complexity | roi
#                      (default: roi, recommended for refactoring).
#   --filter PREFIX    Restrict to a path prefix (hotspot's --filter).
#   --format FORMAT    Output format: json | text | csv | markdown
#                      (default: json).
#   --help, -h         Print this help to stderr and exit 0.
#
# Output discipline (Vercel Labs convention):
#   stdout: machine-readable output in --format (JSON by default)
#   stderr: human-readable status, errors, and hotspot's metadata header
#
# Examples:
#   hotspots.sh
#   hotspots.sh . --since 180d --limit 10
#   hotspots.sh . --mode roi --format json | jq '.results[0:5] | .[].path'
#   hotspots.sh . --filter src/ --format markdown

set -e

print_help() {
  # Print the leading block-comment doc to stderr, stripping the shebang and "# " prefix.
  sed -n '2,/^$/p' "$0" | sed -E 's/^# ?//' >&2
}

# --- Defaults ---
TARGET_PATH="."
LOOKBACK="90d"
LIMIT="20"
MODE="roi"
FILTER=""
FORMAT="json"

# --- Temp dir + cleanup trap (Vercel convention) ---
TMPDIR_HS="$(mktemp -d -t hotspots.XXXXXX)"
trap 'rm -rf "$TMPDIR_HS"' EXIT INT TERM

# --- Arg parsing ---
while [ $# -gt 0 ]; do
  case "$1" in
    --since)    LOOKBACK="$2"; shift 2 ;;
    --limit)    LIMIT="$2"; shift 2 ;;
    --mode)     MODE="$2"; shift 2 ;;
    --filter)   FILTER="$2"; shift 2 ;;
    --format)   FORMAT="$2"; shift 2 ;;
    --help|-h)  print_help; exit 0 ;;
    --)         shift; break ;;
    -*)
      echo "error: unknown flag: $1" >&2
      echo "run 'hotspots.sh --help' for usage" >&2
      exit 2
      ;;
    *)
      TARGET_PATH="$1"; shift
      ;;
  esac
done

# --- Preconditions ---
if ! command -v hotspot >/dev/null 2>&1; then
  cat >&2 <<'EOF'
error: 'hotspot' is not installed or not on PATH.

This script wraps github.com/huangsam/hotspot for behavioral code analysis.

Install with Go (>= 1.21):
  go install github.com/huangsam/hotspot@latest

Then ensure $(go env GOPATH)/bin is on your PATH and re-run this script.
EOF
  exit 1
fi

if ! git -C "$TARGET_PATH" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "error: '$TARGET_PATH' is not a git repository" >&2
  exit 1
fi

# --- Validate enum args ---
case "$MODE" in
  hot|risk|complexity|roi) ;;
  *)
    echo "error: invalid --mode '$MODE' (expected: hot | risk | complexity | roi)" >&2
    exit 2
    ;;
esac

case "$FORMAT" in
  json|text|csv|markdown) ;;
  *)
    echo "error: invalid --format '$FORMAT' (expected: json | text | csv | markdown)" >&2
    exit 2
    ;;
esac

# --- Status to stderr ---
echo "hotspots.sh: ranking top $LIMIT $MODE-mode hotspots in '$TARGET_PATH' (lookback=$LOOKBACK)" >&2

# --- Invoke hotspot ---
ARGS=( files "$TARGET_PATH"
       --mode "$MODE"
       --limit "$LIMIT"
       --lookback "$LOOKBACK"
       --output "$FORMAT"
       --detail
       --color no )

if [ -n "$FILTER" ]; then
  ARGS+=( --filter "$FILTER" )
fi

# hotspot writes its own status to stderr and machine-readable output to stdout,
# so we can let both streams pass through directly.
hotspot "${ARGS[@]}"
