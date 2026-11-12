// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "unions.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kUnionOfStructsSize = sizeof(union UnionOfStructs);
const size_t kUnionOfStructsAlign = alignof(union UnionOfStructs);
const size_t kUnionOfStructs_s1Offset = offsetof(union UnionOfStructs, s1);
const size_t kUnionOfStructs_s2Offset = offsetof(union UnionOfStructs, s2);
const size_t kUnionWithArraysSize = sizeof(union UnionWithArrays);
const size_t kUnionWithArraysAlign = alignof(union UnionWithArrays);
const size_t kUnionWithArrays_arr1Offset = offsetof(union UnionWithArrays, arr1);
const size_t kUnionWithArrays_arr2Offset = offsetof(union UnionWithArrays, arr2);
