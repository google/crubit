// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "self_referential.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kNodeSize = sizeof(struct Node);
const size_t kNodeAlign = alignof(struct Node);
const size_t kNode_valueOffset = offsetof(struct Node, value);
const size_t kNode_nextOffset = offsetof(struct Node, next);
