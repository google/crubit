#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

set -euo pipefail

if [ $# -ne 0 ]; then
  TESTS_TO_RUN=$@
else
  TESTS_TO_RUN=(//...)
fi

bazel test \
  --test_tag_filters=crubit_golden_test,-manual \
  --build_tag_filters=crubit_golden_test,-manual \
  --config=llvm-unstable \
  --test_strategy=local \
  --test_env=WRITE_GOLDENS=1 \
  --cache_test_results=no \
  -k \
  $TESTS_TO_RUN

# Helper function to remove files in src_dir that are byte-for-byte identical to files in base_dir.
prune_matching_files() {
  local src_dir="$1"
  local base_dir="$2"
  [ -d "$src_dir" ] && [ -d "$base_dir" ] || return 0

  for f in "$src_dir"/*; do
    [ -f "$f" ] || continue
    local fname
    fname="$(basename "$f")"
    if [ -f "$base_dir/$fname" ] && cmp -s "$f" "$base_dir/$fname"; then
      rm -f "$f"
    fi
  done
  rmdir "$src_dir" 2>/dev/null || true
}

# Automatically consolidate multiplatform golden files into tiered shared directories
# (e.g. goldens/android_32/) and prune redundant architecture-specific overrides.
consolidate_goldens() {
  local script_dir
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  local crubit_dir
  crubit_dir="$(cd "${script_dir}/.." && pwd)"

  find "$crubit_dir" -type d -name "goldens" | while read -r goldens_dir; do
    [ -d "$goldens_dir" ] || continue
    local pkg_dir
    pkg_dir="$(dirname "$goldens_dir")"

    local x86_dir="$goldens_dir/android_x86"
    local tier32_dir="$goldens_dir/android_32"
    local tier64_dir="$goldens_dir/android_64"

    local ios_sim_arm64_dir="$goldens_dir/ios_sim_arm64"
    local ios_arm64_dir="$goldens_dir/ios_arm64"
    local ios_tier_dir="$goldens_dir/ios"

    # Step 1: Promote android_x86 -> android_32 (32-bit baseline)
    if [ -d "$x86_dir" ]; then
      mkdir -p "$tier32_dir"
      for f in "$x86_dir"/*; do
        [ -f "$f" ] || continue
        mv -f "$f" "$tier32_dir/"
      done
      rmdir "$x86_dir" 2>/dev/null || rm -rf "$x86_dir"
    fi

    # Step 1b: Promote ios_sim_arm64 / ios_arm64 -> ios (shared baseline)
    for arm64_dir in "$ios_sim_arm64_dir" "$ios_arm64_dir"; do
      if [ -d "$arm64_dir" ]; then
        mkdir -p "$ios_tier_dir"
        for f in "$arm64_dir"/*; do
          [ -f "$f" ] || continue
          mv -f "$f" "$ios_tier_dir/"
        done
        rmdir "$arm64_dir" 2>/dev/null || rm -rf "$arm64_dir"
      fi
    done

    # Step 2: Prune redundant files in android_armeabi-v7a (against android_32)
    prune_matching_files "$goldens_dir/android_armeabi-v7a" "$tier32_dir"

    # Step 3: Prune redundant files in 64-bit Android overrides (against android_64 and host)
    for arch in android_arm64-v8a android_x86_64; do
      prune_matching_files "$goldens_dir/$arch" "$tier64_dir"
      prune_matching_files "$goldens_dir/$arch" "$pkg_dir"
    done

    # Step 3b: Prune redundant files in iOS overrides (against ios tier and host)
    for arch in ios_sim_arm64 ios_arm64; do
      prune_matching_files "$goldens_dir/$arch" "$ios_tier_dir"
      prune_matching_files "$goldens_dir/$arch" "$pkg_dir"
    done

    # Step 4: Prune redundant files in android_64 (against host)
    prune_matching_files "$tier64_dir" "$pkg_dir"

    # Step 4b: Prune redundant files in ios tier (against host)
    prune_matching_files "$ios_tier_dir" "$pkg_dir"

    # Step 5: Prune redundant files in android_32 (against host)
    prune_matching_files "$tier32_dir" "$pkg_dir"

    # Step 6: Remove empty goldens directory if everything fell back to host
    rmdir "$goldens_dir" 2>/dev/null || true
  done
}

consolidate_goldens
