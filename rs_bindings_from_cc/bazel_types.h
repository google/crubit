// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_

#include <string>

#include "rs_bindings_from_cc/util/string_type.h"

namespace crubit {

// Representation of a Bazel label (for example //foo/bar:baz).
CRUBIT_DEFINE_STRING_TYPE(BazelLabel);

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_BAZEL_TYPES_H_
