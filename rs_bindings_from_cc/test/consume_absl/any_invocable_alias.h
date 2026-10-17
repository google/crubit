// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ANY_INVOCABLE_ALIAS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ANY_INVOCABLE_ALIAS_H_

#include "absl/functional/any_invocable.h"

// A type alias to a bridged type. Targets that depend on this one (and not on
// `absl/functional:any_invocable` directly) can use the bridged type through
// the alias.
using AliasedIntMapper = absl::AnyInvocable<int(int) const>;

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ANY_INVOCABLE_ALIAS_H_
