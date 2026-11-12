// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nested_anonymous.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kNestedSize = sizeof(struct Nested);
const size_t kNestedAlign = alignof(struct Nested);
