// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_BLAZE_TYPES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_BLAZE_TYPES_H_

#include "third_party/absl/strings/string_view.h"
#include "util/gtl/labs/string_type.h"

namespace rs_bindings_from_cc {

// Representation of a Blaze label (for example //foo/bar:baz).
DEFINE_STRING_TYPE(BlazeLabel);

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_BLAZE_TYPES_H_
