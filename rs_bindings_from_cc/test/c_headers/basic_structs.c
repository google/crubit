// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "basic_structs.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kPointSize = sizeof(struct Point);
const size_t kPointAlign = alignof(struct Point);
const size_t kPoint_xOffset = offsetof(struct Point, x);
const size_t kPoint_yOffset = offsetof(struct Point, y);
const size_t kDataSize = sizeof(union Data);
const size_t kDataAlign = alignof(union Data);
const size_t kData_iOffset = offsetof(union Data, i);
const size_t kData_fOffset = offsetof(union Data, f);
