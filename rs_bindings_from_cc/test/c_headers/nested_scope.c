// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nested_scope.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kOuterScopeSize = sizeof(struct OuterScope);
const size_t kOuterScopeAlign = alignof(struct OuterScope);
const size_t kOuterScope_innerOffset = offsetof(struct OuterScope, inner);
