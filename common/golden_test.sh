#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

function prepend_license() {
  cat common/LICENSE_HEADER "$1"
}

# Optional platform directory (e.g. android_arm64-v8a) for multi-platform tests.
PLATFORM_DIR=""
if [[ "$#" -ge 2 && "$1" == "--platform" ]]; then
  PLATFORM_DIR="$2"
  shift 2
fi

STATUS=0
while (( $# > 0 )); do
  if [[ -n "$PLATFORM_DIR" ]]; then
    # Multi-platform tests pass (read_target, gen_file, golden_filename) tuples.
    if (( $# < 3 )); then
      echo >&2 "INTERNAL ERROR: multiplatform golden_test requires (read_target, gen_file, golden_filename) tuples."
      exit 1
    fi
    READ_TARGET="$1"
    GEN_FILE="$2"
    GOLDEN_FILENAME="$3"
    shift 3
  else
    # Default host tests pass (read_target, gen_file) tuples for backwards compatibility.
    if (( $# < 2 )); then
      echo >&2 "INTERNAL ERROR: test.sh requires an even number of arguments."
      exit 1
    fi
    READ_TARGET="$1"
    GEN_FILE="$2"
    GOLDEN_FILENAME=""
    shift 2
  fi

  if ! diff -u "$READ_TARGET" <(prepend_license "$GEN_FILE"); then
    if [[ -n "$WRITE_GOLDENS" ]]; then
      if [[ -z "$PLATFORM_DIR" ]]; then
        REAL_WRITE_TARGET="$READ_TARGET"
      else
        # Resolve symlink to source workspace file and construct platform-specific golden path.
        TARGET_RESOLVED="$(readlink -e "$READ_TARGET" 2>/dev/null || echo "$READ_TARGET")"
        TARGET_DIR="$(dirname "$TARGET_RESOLVED")"
        if [[ "$TARGET_DIR" == */goldens/$PLATFORM_DIR ]]; then
          REAL_WRITE_TARGET="${TARGET_DIR}/${GOLDEN_FILENAME}"
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