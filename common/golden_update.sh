#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

set -euo pipefail

if [ $# -ne 0 ]; then
  TESTS_TO_RUN=("$@")
else
  TESTS_TO_RUN=(//...)
fi

TEST_FLAGS=(
  --test_tag_filters=crubit_golden_test,-manual
  --build_tag_filters=crubit_golden_test,-manual
  --config=llvm-unstable
  --test_strategy=local
  --test_env=WRITE_GOLDENS=1
  --cache_test_results=no
  --test_summary=none
  --test_output=errors
  -k
)

if [ -t 1 ]; then
  TEST_FLAGS+=(--color=yes)
fi

# Filter out cascading "failed to build" output and redundant test summaries
# when targets fail to compile, leaving only the root compiler diagnostics.
bazel test "${TEST_FLAGS[@]}" "${TESTS_TO_RUN[@]}" 2>&1 | awk '
  BEGIN {
    esc = sprintf("%c", 27)
    esc_pat = "(" esc "\\[[0-9;]*m)*"
  }
  /Target .* failed to build/ { next }
  /^[[:space:]]*due to action in / { next }
  /FAILED TO BUILD/ { next }
  /\(Skipping other failed to build tests\)/ { next }
  $0 ~ "^[[:space:]]*" esc_pat "FAILED:" esc_pat "[[:space:]]*$" { next }
  /^[[:space:]]*$/ {
    if (blank) next
    blank = 1
    print
    fflush()
    next
  }
  {
    blank = 0
    print
    fflush()
  }
'

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

# Removes the per-arch goldens in src_dir that the test doesn't need.
#
# golden_test.bzl reads a golden from these directories: src_dir, then
# tier_dir, then host_dir. So a file in src_dir is redundant if it is identical
# to the files in the other directories.
prune_against_fallback() {
  local src_dir="$1"
  local tier_dir="$2"
  local host_dir="$3"
  [ -d "$src_dir" ] || return 0

  for f in "$src_dir"/*; do
    [ -f "$f" ] || continue
    local fname fallback
    fname="$(basename "$f")"
    if [ -f "$tier_dir/$fname" ]; then
      fallback="$tier_dir/$fname"
    else
      fallback="$host_dir/$fname"
    fi
    if [ -f "$fallback" ] && cmp -s "$f" "$fallback"; then
      rm -f "$f"
    fi
  done
  rmdir "$src_dir" 2>/dev/null || true
}

# Moves every golden in src_dir into tier_dir (overwriting) and removes src_dir.
promote_to_tier() {
  local src_dir="$1"
  local tier_dir="$2"
  [ -d "$src_dir" ] || return 0

  mkdir -p "$tier_dir"
  for f in "$src_dir"/*; do
    [ -f "$f" ] || continue
    mv -f "$f" "$tier_dir/"
  done
  rm -rf "$src_dir"
}

# Automatically consolidate multiplatform golden files into tiered shared directories
# (e.g. goldens/android_32/, goldens/android_64/) and prune redundant overrides.
consolidate_goldens() {
  local script_dir
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  local crubit_dir
  crubit_dir="$(cd "${script_dir}/.." && pwd)"

  find "$crubit_dir" -type d -name "goldens" | while read -r goldens_dir; do
    [ -d "$goldens_dir" ] || continue
    local pkg_dir
    pkg_dir="$(dirname "$goldens_dir")"

    local x86_32_dir="$goldens_dir/android_x86"
    local arm_32_dir="$goldens_dir/android_armeabi-v7a"
    local tier32_dir="$goldens_dir/android_32"

    local x86_64_dir="$goldens_dir/android_x86_64"
    local arm_64_dir="$goldens_dir/android_arm64-v8a"
    local tier64_dir="$goldens_dir/android_64"

    local ios_sim_arm64_dir="$goldens_dir/ios_sim_arm64"
    local ios_arm64_dir="$goldens_dir/ios_arm64"
    local ios_tier_dir="$goldens_dir/ios"

    # 64-bit Windows is LLP64 on both x86_64 and arm64.
    # 32-bit x86 Windows (ILP32) has its own windows_32 tier.
    # The directory names match the Windows arch names and
    # ABI tiers used by golden_test.bzl.
    local windows_x86_dir="$goldens_dir/windows_x86"
    local windows_tier32_dir="$goldens_dir/windows_32"

    local windows_x86_64_dir="$goldens_dir/windows_x86_64"
    local windows_arm64_dir="$goldens_dir/windows_arm64"
    local windows_tier64_dir="$goldens_dir/windows_64"
    # macOS golden tests are written directly to the shared tier by both archs given that arm64 and
    # x86_64 must produce identical bindings. So there are no per-architecture directories.
    local macos_tier_dir="$goldens_dir/macos"


    # Step 1: Promote x86 baselines to shared ABI tiers
    promote_to_tier "$x86_32_dir" "$tier32_dir"
    promote_to_tier "$x86_64_dir" "$tier64_dir"

    # Step 1b: Promote ios_sim_arm64 / ios_arm64 -> ios (shared baseline)
    promote_to_tier "$ios_sim_arm64_dir" "$ios_tier_dir"
    promote_to_tier "$ios_arm64_dir" "$ios_tier_dir"

    # Step 1c: Promote windows_x86 -> windows_32 and windows_x86_64 -> windows_64
    promote_to_tier "$windows_x86_dir" "$windows_tier32_dir"
    promote_to_tier "$windows_x86_64_dir" "$windows_tier64_dir"

    # Step 2: Prune redundant files in 32-bit ARM overrides (against android_32 and host)
    prune_matching_files "$arm_32_dir" "$tier32_dir"
    prune_matching_files "$arm_32_dir" "$pkg_dir"

    # Step 3: Prune redundant files in 64-bit ARM overrides (against android_64 and host)
    prune_matching_files "$arm_64_dir" "$tier64_dir"
    prune_matching_files "$arm_64_dir" "$pkg_dir"

    # Step 3b: Prune redundant files in iOS overrides (against ios tier and host)
    for arch in ios_sim_arm64 ios_arm64; do
      prune_matching_files "$goldens_dir/$arch" "$ios_tier_dir"
      prune_matching_files "$goldens_dir/$arch" "$pkg_dir"
    done

    # Step 3c: Prune redundant files in the Windows arm64 override (against the
    # golden it would read otherwise: windows_64 if present, else host)
    prune_against_fallback "$windows_arm64_dir" "$windows_tier64_dir" "$pkg_dir"

    # Step 4: Prune redundant files in shared ABI tiers against host
    prune_matching_files "$tier32_dir" "$pkg_dir"
    prune_matching_files "$tier64_dir" "$pkg_dir"

    # Step 4b: Prune redundant files in ios and macos tiers (against host)
    prune_matching_files "$ios_tier_dir" "$pkg_dir"
    prune_matching_files "$macos_tier_dir" "$pkg_dir"

    # Step 4c: Prune redundant files in Windows tiers (against host)
    prune_matching_files "$windows_tier32_dir" "$pkg_dir"
    prune_matching_files "$windows_tier64_dir" "$pkg_dir"

    # Step 5: Remove empty goldens directory if everything fell back to host
    rmdir "$goldens_dir" 2>/dev/null || true
  done
}

consolidate_goldens
