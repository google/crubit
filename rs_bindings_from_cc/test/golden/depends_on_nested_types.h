// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DEPENDS_ON_NESTED_TYPES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DEPENDS_ON_NESTED_TYPES_H_

#include "rs_bindings_from_cc/test/golden/nested_types.h"

// This should have bindings because Bar is a nested item of Foo, and the module
// "foo" can be generated because it wouldn't conflict with anything else.
using FooBar = Foo::Bar;

// This should not have bindings because Bar is a nested item of Foo, and the
// module "conflicting_snake_case_names" cannot be generated because it
// conflicts with the child module of ConflictingSnakeCaseNames_.
using ConflictingSnakeCaseNamesInner = ConflictingSnakeCaseNames::Inner;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DEPENDS_ON_NESTED_TYPES_H_
