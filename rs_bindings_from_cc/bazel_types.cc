// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/bazel_types.h"

#include <cstddef>
#include <limits>
#include <string>

#include "absl/log/check.h"
#include "absl/strings/ascii.h"
#include "absl/strings/str_cat.h"

namespace crubit {

std::string ConvertToCcIdentifier(const BazelLabel& target) {
  std::string result;
  {
    size_t predicted_length = target.value().size();
    if (predicted_length < (std::numeric_limits<size_t>::max() / 2))
      predicted_length *= 2;
    result.reserve(predicted_length);
  }

  // This is yet another escaping scheme... :-/  Compare this with
  // https://github.com/bazelbuild/rules_rust/blob/1f2e6231de29d8fad8d21486f0d16403632700bf/rust/private/utils.bzl#L459-L586
  for (char c : target.value()) {
    if (absl::ascii_isalnum(c)) {
      result += c;
    } else {
      absl::StrAppend(&result, "_", absl::Hex(c, absl::kZeroPad2));
    }
  }
  result.shrink_to_fit();

  CHECK(!result.empty());
  CHECK(absl::ascii_isalpha(result[0]) || result[0] == '_');
  return result;
}

}  // namespace crubit
