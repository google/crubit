// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "alignment.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kPackedStructSize = sizeof(struct PackedStruct);
const size_t kPackedStructAlign = alignof(struct PackedStruct);
const size_t kPackedStruct_cOffset = offsetof(struct PackedStruct, c);
const size_t kPackedStruct_iOffset = offsetof(struct PackedStruct, i);
const size_t kPragmaPackedSize = sizeof(struct PragmaPacked);
const size_t kPragmaPackedAlign = alignof(struct PragmaPacked);
const size_t kPragmaPacked_cOffset = offsetof(struct PragmaPacked, c);
const size_t kPragmaPacked_iOffset = offsetof(struct PragmaPacked, i);
