#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# The purpose of this little script is to build rs_bindings_from_cc with custom 
# Bazel flags, most typically --config=asan, and then use this binary as a tool
# in a Bazel build that generates bindings for some target.

if [[ "${PWD}" != *'/google3'* ]]; then
  echo "Usage: Please run this script from within a client."
  exit 1
fi

if [[ "${#}" -le 1 ]]; then
  echo "Usage: ${0} <label> {bazel flags}"
  exit 1
fi

readonly LABEL="${1}"
shift

echo "##################### Building crubit with flags: ${@}"
bazel build "//rs_bindings_from_cc" "${@}"
echo

echo "##################### Copying the tool to rs_bindings_from_cc/bazel_support/prebuilt_rs_bindings_from_cc"
cp \
  bazel-bin/rs_bindings_from_cc/rs_bindings_from_cc \
  rs_bindings_from_cc/bazel_support/prebuilt_rs_bindings_from_cc
echo

echo "##################### Running the build of ${LABEL} with the tool"

# copybara:strip_end_and_replace_begin
bazel run "${LABEL}" --//rs_bindings_from_cc/bazel_support:use_prebuilt_rs_bindings_from_cc_for_debugging=True
# copybara:replace_end

trap 'rm -rf -- rs_bindings_from_cc/bazel_support/prebuilt_rs_bindings_from_cc' EXIT
