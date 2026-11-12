// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "zero_length_arrays.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kZeroLengthSize = sizeof(struct ZeroLength);
const size_t kZeroLengthAlign = alignof(struct ZeroLength);
const size_t kZeroLength_lengthOffset = offsetof(struct ZeroLength, length);
const size_t kZeroLength_dataOffset = offsetof(struct ZeroLength, data);
const size_t kMultiDimSize = sizeof(struct MultiDim);
const size_t kMultiDimAlign = alignof(struct MultiDim);
const size_t kMultiDim_dataOffset = offsetof(struct MultiDim, data);
