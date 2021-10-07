// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include "rs_bindings_from_cc/test/golden/nontrivial_type.h"
static_assert(sizeof(Nontrivial) == 4);
static_assert(alignof(Nontrivial) == 4);
static_assert(offsetof(Nontrivial, field) * 8 == 0);
