// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_ANY_INVOCABLE_ALIAS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_ANY_INVOCABLE_ALIAS_H_

#include "rs_bindings_from_cc/test/consume_absl/any_invocable_alias.h"

// This target only depends on `:any_invocable_alias`, but its generated
// bindings use the AnyInvocable bridge support header. That header must be
// reachable (under `layering_check`) from the generated `_rust_api_impl.cc`.
inline int CallAliasedIntMapper(AliasedIntMapper f, int i) { return f(i); }

inline AliasedIntMapper ReturnAliasedIntMapper() {
  return [](int x) { return x * 2; };
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_USES_ANY_INVOCABLE_ALIAS_H_
