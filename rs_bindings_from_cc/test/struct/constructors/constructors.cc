// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/constructors/constructors.h"

StructWithUserProvidedConstructors::StructWithUserProvidedConstructors()
    : int_field(42) {}

StructWithUserProvidedConstructors::StructWithUserProvidedConstructors(int i)
    : int_field(i) {}

StructWithPrivateConstructor::StructWithPrivateConstructor() : int_field(42) {}

NonTrivialStructWithConstructors::NonTrivialStructWithConstructors()
    : int_field(43) {}

NonTrivialStructWithConstructors::~NonTrivialStructWithConstructors() {}
