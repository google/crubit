// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_THREAD_SAFETY_THREAD_UNSAFE_TYPES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_THREAD_SAFETY_THREAD_UNSAFE_TYPES_H_

#include "support/annotations.h"

// Records are `Send` and `Sync` when rustc can infer it from their fields, so
// thread-unsafe types must opt out explicitly.
struct CRUBIT_TRAIT_DERIVE("!Send", "!Sync") Struct {};
union CRUBIT_TRAIT_DERIVE("!Send", "!Sync") Union {};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_THREAD_SAFETY_THREAD_UNSAFE_TYPES_H_
