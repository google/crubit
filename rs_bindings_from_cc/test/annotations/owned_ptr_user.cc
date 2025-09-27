// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exceptiono

#include "rs_bindings_from_cc/test/annotations/owned_ptr_user.h"

#include "rs_bindings_from_cc/test/annotations/owned_ptr.h"
#include "support/annotations.h"

Thing* MakeThing(int value) { return new Thing(value); }
Thing* CRUBIT_OWNED_PTR MakeOwnedThing(int value) { return MakeThing(value); }

int GetThingValue(Thing* thing) { return thing->value; }
int ThingToValue(Thing* CRUBIT_OWNED_PTR thing) { return thing->value; }
