// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/escaping_target_names/uses-target-with-dashes.h"

#include "rs_bindings_from_cc/test/escaping_target_names/has-dashes-in-name.h"

int SomeFunc(SomeStruct s) { return s.value; }
