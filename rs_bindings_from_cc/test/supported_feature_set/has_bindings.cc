// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/supported_feature_set/has_bindings.h"

#include "absl/base/attributes.h"

namespace crubit::has_bindings {

void crubit_non_inline_function() {}
void crubit_extern_c_directly_function() {}

ABSL_ATTRIBUTE_NO_SANITIZE_CFI void crubit_invoke_callback(void (&f)(int* x),
                                                           int* x) {
  f(x);
}

ABSL_ATTRIBUTE_NO_SANITIZE_CFI void crubit_invoke_nullable_callback(
    void (*f)(int* x), int* x) {
  f(x);
}

}  // namespace crubit::has_bindings
