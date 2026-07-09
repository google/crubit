// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_

#include <cstdint>

#include "crubit/support/annotations.h"

namespace absl {

// An empty implementation of absl::flat_hash_map to test code generation
// without the absl dependency.
template <typename K, typename V, int ThirdParameterMustBeAccepted = 42>
class flat_hash_map final {
 public:
  void FunctionRemovedByCustomization() const;

  template <typename F>
  void HarmlessTemplateFunction(F f) const;
};

}  // namespace absl

namespace crubit::test {

using MapWithTwoParams = absl::flat_hash_map<int32_t, uint64_t, 42>;
using MapWithThreeParams = absl::flat_hash_map<int32_t, uint64_t, 42>;

// Fake and non-functional bridged type.
struct CRUBIT_BRIDGE("Bridged", "BridgedAbi", "BridgedAbi") Bridged final {
  int i;
};

using MapWithBridgedKey = absl::flat_hash_map<Bridged, int>;
using MapWithBridgedValue = absl::flat_hash_map<int, Bridged>;

class Incomplete;

using MapWithIncompleteKey = absl::flat_hash_map<Incomplete, int>;
using MapWithIncompleteValue = absl::flat_hash_map<int, Incomplete>;

class NoDestructor final {
 private:
  ~NoDestructor();
};

using MapWithNoDestructorKey = absl::flat_hash_map<NoDestructor, int>;
using MapWithNoDestructorValue = absl::flat_hash_map<int, NoDestructor>;

class NoDelete final {
 private:
  void operator delete(void* ptr);
};

using MapWithNoDeleteKey = absl::flat_hash_map<NoDelete, int>;
using MapWithNoDeleteValue = absl::flat_hash_map<int, NoDelete>;

}  //  namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ABSL_FLAT_HASH_MAP_ISOLATED_H_
