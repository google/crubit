// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_

#include <cstdint>

namespace absl {

// A stub implementation of absl::flat_hash_map to test code generation without
// the absl dependency.
template <typename K, typename V, int ThirdParameterMustBeAccepted>
class flat_hash_map final {
 public:
  void FunctionRemovedByOverride() const;

  template <typename F>
  void HarmlessTemplateFunction(F f) const;
};

}  // namespace absl

namespace crubit::test {

using MyMap = absl::flat_hash_map<int32_t, uint64_t, 42>;

}  //  namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_
