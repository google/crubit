// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "types.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kTypesSize = sizeof(struct Types);
const size_t kTypesAlign = alignof(struct Types);
