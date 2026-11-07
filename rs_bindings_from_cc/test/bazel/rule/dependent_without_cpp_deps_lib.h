// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_RULE_DEPENDENT_WITHOUT_CPP_DEPS_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_RULE_DEPENDENT_WITHOUT_CPP_DEPS_LIB_H_

#include <cstddef>

#include "rs_bindings_from_cc/test/bazel/rule/non_crubit_str_ref_bridge.h"

inline int DependentBoundFn() { return 7; }

// `str_ref` is only in `embedded_cpp_with_support_deps`'s local `cpp_deps` (and
// behind a non-Crubit C++ dep here), so its header mapping must not leak to
// this target: `rs::StrRef` belongs to an unknown target and this function is
// skipped rather than referencing an unlinked `str_ref` crate.
inline size_t DependentUnboundStrRefFn(rs::StrRef s) { return s.size(); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_RULE_DEPENDENT_WITHOUT_CPP_DEPS_LIB_H_
