// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_

#include "rs_bindings_from_cc/test/wrapper/pub_crate_types/other_pub_crate_types.h"

template <typename T>
struct Template {
  T value;

  void IndirectCannotBeInstantiated(T t) {
    // trigger b/248542210,
    CannotBeInstantiated();
  }
  void CannotBeInstantiated() { static_assert(false); }
};

using TemplateIntAlias = Template<int>;

struct CompoundDataType {
  Template<int> template_int;
};

inline Template<int> GetTemplateInt() { return Template<int>{42}; }

extern Template<int> TemplateConstant;

inline int ConsumeCompoundDataType(CompoundDataType container) {
  return container.template_int.value;
}

// Forward-declared types are not pub(crate), but could be in an alternate
// implementation.

struct ForwardDeclared;

// Forward declared types are not pub(crate) so that they can work across
// module boundaries like this.
inline void OtherPubCrateTypes(ForwardDeclared2*) {}

// Templates, otoh, are pub(crate), but work because templates are already
// instantiated once per crate.
inline Template2<int> GetOtherPubCrateTemplate2Int() {
  return Template2<int>{42};
}

// Don't uncomment this: a `pair` include starts polluting the golden test with
// a lot of implementation details.
// But this function would produce a different error from the first,
// because it sees the types earlier.
// inline void MixedPubCrateTypes(std::pair<Template<int>*,
// Template2<int>*>) {}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_
