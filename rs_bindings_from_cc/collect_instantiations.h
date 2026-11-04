// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_INSTANTIATIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_INSTANTIATIONS_H_

#include <string>
#include <vector>

#include "common/status.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/Support/Error.h"

namespace crubit {

// Parses Rust source files given their filenames and returns a vector with all
// C++ class template instantiations requested by calls to the `cc_template!`
// macro.
llvm::Expected<std::vector<std::string>> CollectInstantiations(
    llvm::ArrayRef<std::string> rust_sources);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_COLLECT_INSTANTIATIONS_H_
