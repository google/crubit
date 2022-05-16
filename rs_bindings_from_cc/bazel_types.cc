// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/bazel_types.h"

#include "absl/strings/ascii.h"

namespace crubit {

std::string ConvertToCcIdentifier(const BazelLabel& target) {
  std::string result = target.value();

  // TODO(b/222001243): The escaping below can arrive at the same result for 2
  // distinct targets like //foo/bar:baz and //foo_bar:baz.  In the long-term
  // this should be fixed, or alternatively ConvertToCcIdentifier should be
  // removed (the latter is the current plan of record - see also "Handling
  // thunks" section in <internal link>).
  for (char& c : result) {
    if (!absl::ascii_isalnum(c)) {
      c = '_';
    }
  }
  if (!result.empty() && !absl::ascii_isalpha(result[0])) {
    result[0] = '_';
  }

  return result;
}

}  // namespace crubit
