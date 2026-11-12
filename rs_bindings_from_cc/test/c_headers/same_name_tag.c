// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "same_name_tag.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kTagSameNameSize = sizeof(TagSameName);
const size_t kTagSameNameAlign = alignof(TagSameName);
const size_t kTagSameName_xOffset = offsetof(TagSameName, x);
