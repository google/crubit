#!/bin/bash
# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


set -euo pipefail

if [[ "${PWD}" != *'/google3'* ]]; then
  echo "Usage: Please run this script from within a client."
  exit 1
fi

G3="${PWD%%/google3*}/google3"
PKG="rs_bindings_from_cc/test/golden"

cd "${G3}/${PKG}"

TARGETS=()
HEADERS_TO_SKIP=(
  # The namespaces_json.h header is used to inspect the generated .json file.
  # Updates to namespaces_json.json, if needed, shall be performed manually.
  "namespaces_json.h"
)

for header in *.h; do
  if [[ "${HEADERS_TO_SKIP[@]}" =~ "${header}" ]]; then
    continue;
  fi
  TARGETS+=(":${header%.h}_cc_file")
  TARGETS+=(":${header%.h}_rs_file")
done

BUILD_EVENT_PROTOCOL_JSON="${G3}/${PKG}/build_event_protocol.json"
bazel build "${TARGETS[@]}" --build_event_json_file="${BUILD_EVENT_PROTOCOL_JSON}"

# As the targets undergo bazel transition (to be built under Crubit development flavor), the output
# will be stored in the directory of k8-fastbuild-ST-<hash>. To get the output directory, we parse
# the output of Build Event Protocol (https://bazel.build/remote/bep), which contains the precise
# output path prefix.
OUTPUT_FILES=($(jq '.completed.importantOutput | select(.) | .[] | .name' -r "${BUILD_EVENT_PROTOCOL_JSON}"))
OUTPUT_PATH_PREFIXES=($(jq '.completed.importantOutput | select(.) | .[] | (.pathPrefix | join ("/"))' -r "${BUILD_EVENT_PROTOCOL_JSON}"))
rm "${BUILD_EVENT_PROTOCOL_JSON}"

for i in "${!OUTPUT_FILES[@]}"; do
  # Goldens are stored in the same directory as the build targets, so their names need to be
  # different from the file outputted by the build rule.
  output_file=$(basename "${OUTPUT_FILES[$i]}" | sed "s/_cc_rust/_rs/g")
  # Prepend license headers to output files, since they are checked in.
  cat LICENSE_HEADER "$(bazel info execution_root)/${OUTPUT_PATH_PREFIXES[$i]}/${OUTPUT_FILES[$i]}" > "${output_file}"
done
