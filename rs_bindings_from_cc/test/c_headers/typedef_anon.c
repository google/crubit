// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "typedef_anon.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kPointTypedefSize = sizeof(PointTypedef);
const size_t kPointTypedefAlign = alignof(PointTypedef);
const size_t kPointTypedef_xOffset = offsetof(PointTypedef, x);
const size_t kPointTypedef_yOffset = offsetof(PointTypedef, y);
