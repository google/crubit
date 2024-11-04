// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/supported_feature_set/has_bindings.h"

namespace crubit::has_bindings {

void crubit_non_inline_function() {}
void crubit_extern_c_directly_function() {}

}  // namespace crubit::has_bindings
