#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

prepend_license() {
  cat common/LICENSE_HEADER "$1"
}

# Optional platform directory (e.g. android_arm64-v8a) and ABI tier (e.g. android_32) for multi-platform tests.
PLATFORM_DIR=""
ABI_TIER=""
while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --platform)
      [[ "$#" -ge 2 ]] && PLATFORM_DIR="$2" && shift 2 || shift 1
      ;;
    --tier)
      [[ "$#" -ge 2 ]] && ABI_TIER="$2" && shift 2 || shift 1
      ;;
    *)
      break
      ;;
  esac
done

STATUS=0
while (( $# > 0 )); do
  if (( $# < 2 )); then
    echo >&2 "INTERNAL ERROR: golden_test.sh requires pairs of (read_target, gen_file) arguments."
    exit 1
  fi
  READ_TARGET="$1"
  GEN_FILE="$2"
  shift 2

  if ! diff -u "$READ_TARGET" <(prepend_license "$GEN_FILE"); then
    if [[ -n "$WRITE_GOLDENS" ]]; then
      # Resolve symlink to the real workspace file.
      TARGET_RESOLVED="$(readlink -e "$READ_TARGET" 2>/dev/null || echo "$READ_TARGET")"

      if [[ -z "$PLATFORM_DIR" ]]; then
        # Host golden update in place.
        REAL_WRITE_TARGET="$TARGET_RESOLVED"
      else
        TARGET_DIR="$(dirname "$TARGET_RESOLVED")"
        GOLDEN_FILENAME="$(basename "$TARGET_RESOLVED")"

        if [[ "$TARGET_DIR" == */goldens/* ]]; then
          BASE_PKG_DIR="${TARGET_DIR%/goldens/*}"
          REAL_WRITE_TARGET="${BASE_PKG_DIR}/goldens/${PLATFORM_DIR}/${GOLDEN_FILENAME}"
        else
          REAL_WRITE_TARGET="${TARGET_DIR}/goldens/${PLATFORM_DIR}/${GOLDEN_FILENAME}"
        fi
      fi

      mkdir -p "$(dirname "$REAL_WRITE_TARGET")"
      prepend_license "$GEN_FILE" > "$REAL_WRITE_TARGET"
      echo >&2 "Updated golden file: $REAL_WRITE_TARGET"
    else
      STATUS=1
    fi
  fi
done

if [[ $STATUS -ne 0 ]]; then
  echo >&2 "To regenerate the goldens, run common/golden_update.sh"
  exit 1
fi