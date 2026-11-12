// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "flexible_arrays.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kFlexSize = sizeof(struct Flex);
const size_t kFlexAlign = alignof(struct Flex);
const size_t kFlex_lengthOffset = offsetof(struct Flex, length);
const size_t kFlex_dataOffset = offsetof(struct Flex, data);
