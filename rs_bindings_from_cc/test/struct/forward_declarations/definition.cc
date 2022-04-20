// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "rs_bindings_from_cc/test/struct/forward_declarations/definition.h"

int ReadUnpinStruct(const UnpinStruct& s) { return s.field; }
void WriteUnpinStruct(UnpinStruct& s, int value) { s.field = value; }
int ReadNonunpinStruct(const NonunpinStruct& s) { return s.field; }
void WriteNonunpinStruct(NonunpinStruct& s, int value) { s.field = value; }
