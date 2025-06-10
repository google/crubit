// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_

#include "rs_bindings_from_cc/test/wrapper/pub_crate_types/other_pub_crate_types.h"

// Forward-declared types are a simple instance of a `pub(crate)` type.

struct ForwardDeclared;

using ForwardDeclaredAlias = ForwardDeclared;

struct CompoundDataType {
  ForwardDeclared* forward_declared;
};

ForwardDeclared* CreateForwardDeclared();

extern ForwardDeclared* ForwardDeclaredConstant;

int ConsumeCompoundDataType(CompoundDataType container);

// Using pub(crate) types from other libraries doesn't work well.
inline void OtherPubCrateTypes(ForwardDeclared2*) {}

// Don't uncomment this: a `pair` include starts polluting the golden test with
// a lot of implementation details.
// But this function would produce a different error from the first,
// because it sees the types earlier.
// inline void MixedPubCrateTypes(std::pair<ForwardDeclared*,
// ForwardDeclared2*>) {}

// Other types are essentially the same, and just get an abbreviated test:

template <typename T>
struct Template {
  T value;

  void IndirectCannotBeInstantiated(T t) {
    // trigger b/248542210,
    CannotBeInstantiated();
  }
  void CannotBeInstantiated() { static_assert(false); }
  ~Template() { value = T(); }
};

inline Template<int> GetTemplateInt() { return Template<int>{42}; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_
