// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/constructors/constructors.h"

StructWithUserProvidedConstructor::StructWithUserProvidedConstructor()
    : int_field(42) {}

StructWithPrivateConstructor::StructWithPrivateConstructor() : int_field(42) {}
