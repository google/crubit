#!/usr/bin/env bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Script for running Crubit tools in Bazel with verbose logging
#
# Accepts zero or more Bazel flags (each starting with `--`) followed by one
# target label.

set -euo pipefail

# Used throughout for comma-separated array expansion.
IFS=,

declare -a bazel_flags=()
declare -a aspects=(
  //cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl%cc_bindings_from_rust_aspect
  //rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl%rust_bindings_from_cc_aspect
)
declare -a output_paths=()

function print_usage_and_exit() {
  cat <<END >&2
Script for running Crubit tools in Bazel with verbose logging

${0} [BAZEL_FLAGS...] <TARGET>

BAZEL_FLAGS   Additional flags to pass to Bazel, each starting with \`--\`
TARGET        Bazel target label to build
END
  exit 1
}

function process_cmdline() {
  while (($# > 1)); do
    case "${1}" in
      --*)
        bazel_flags+=("${1}")
        shift
        ;;
      *)
        echo "Bad element of BAZEL_FLAGS: ${1}" >&2
        echo >&2
        print_usage_and_exit
        ;;
    esac
  done
  if (($# == 1)); then
    target="${1}"
  else
    echo "TARGET is required" >&2
    echo >&2
    print_usage_and_exit
  fi
}

function query_for_outputs() {
  echo "Querying for output files" >&2

  declare -a cmd=(
    "bazel"
    "cquery"
    "--output=files"
    "--aspects=${aspects[*]}"
    "--output_groups=out"
    "${bazel_flags[@]}"
    "${target}"
  )
  echo "> ${cmd[@]}" >&2
  while read path; do
    output_paths+=("${path}")
  done < <("${cmd[@]}")
  if ((${#output_paths[@]} == 0)); then
    exit 1
  fi
}

function delete_outputs() {
  echo >&2
  echo "Deleting any existing output files" >&2

  declare -a cmd=(
    "rm"
    "-f"
    "${output_paths[@]}"
  )
  echo "> ${cmd[@]}" >&2
  "${cmd[@]}"
}

function perform_build() {
  echo >&2
  echo "Performing build" >&2

  declare -a cmd=(
    "bazel"
    "build"
    "--aspects=${aspects[*]}"
    "--output_groups=out"
    "--//common/bazel_support:verbose_log_targets=${target}"
    "${bazel_flags[@]}"
    "${target}"
  )
  echo "> ${cmd[@]}" >&2
  "${cmd[@]}"
}

process_cmdline "$@"
query_for_outputs
delete_outputs
perform_build
