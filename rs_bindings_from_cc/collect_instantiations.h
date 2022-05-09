// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_INSTANTIATIONS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_INSTANTIATIONS_H_

#include <string>

#include "absl/status/statusor.h"
#include "absl/types/span.h"

namespace crubit {

// Parses Rust source files given their filenames and returns a vector with all
// C++ class template instantiations requested by calls to the `cc_template!`
// macro.
absl::StatusOr<std::vector<std::string>> CollectInstantiations(
    absl::Span<const std::string> rust_sources);

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_INSTANTIATIONS_H_
