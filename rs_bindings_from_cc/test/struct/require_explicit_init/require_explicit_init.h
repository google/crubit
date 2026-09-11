// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_REQUIRE_EXPLICIT_INIT_REQUIRE_EXPLICIT_INIT_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_REQUIRE_EXPLICIT_INIT_REQUIRE_EXPLICIT_INIT_H_

#include "absl/base/attributes.h"

// An aggregate with a field that must be explicitly initialized.
struct HasExplicitInitField {
  int a = 0;
  int b ABSL_REQUIRE_EXPLICIT_INIT;
};

// Transitively requires explicit initialization via a member.
struct ContainsExplicitInitField {
  HasExplicitInitField inner;
};

// Transitively requires explicit initialization via a base.
struct DerivesExplicitInitField : HasExplicitInitField {};

struct NonTriviallyDestructible {
  ~NonTriviallyDestructible() {}
};

// Requires explicit initialization, and is not `Unpin` on the Rust side, so it
// exercises the `CtorNew` code path rather than the `Default` code path.
struct NonUnpinWithExplicitInitField {
  NonTriviallyDestructible nontrivial;
  int b ABSL_REQUIRE_EXPLICIT_INIT;
};

// Control: no explicit-init fields.
struct NoExplicitInitField {
  int a = 0;
};

// Control: a user-provided constructor makes this a non-aggregate, so Clang
// ignores the attribute entirely (and clears
// `hasUninitializedExplicitInitFields`). Crubit must still bind `Default`.
struct NonAggregateWithIgnoredAttr {
  NonAggregateWithIgnoredAttr() : b(0) {}
  int a = 0;
  int b ABSL_REQUIRE_EXPLICIT_INIT;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_REQUIRE_EXPLICIT_INIT_REQUIRE_EXPLICIT_INIT_H_
