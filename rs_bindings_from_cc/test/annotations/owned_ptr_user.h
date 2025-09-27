// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_OWNED_PTR_USER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_OWNED_PTR_USER_H_

#include "rs_bindings_from_cc/test/annotations/owned_ptr.h"
#include "support/annotations.h"

// An example of a C++ file that defines functions that create a
// CRUBIT_OWNED_PTR type as well as consume it.

Thing* CRUBIT_OWNED_POINTER MakeOwnedThing(int value);

Thing* MakeThing(int value);

int ThingToValue(Thing* CRUBIT_OWNED_POINTER thingptr);

int GetThingValue(Thing* thingptr);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_OWNED_PTR_USER_H_
