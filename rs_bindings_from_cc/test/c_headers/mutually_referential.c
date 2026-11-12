// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "mutually_referential.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kASize = sizeof(struct A);
const size_t kAAlign = alignof(struct A);
const size_t kA_bOffset = offsetof(struct A, b);
const size_t kBSize = sizeof(struct B);
const size_t kBAlign = alignof(struct B);
const size_t kB_aOffset = offsetof(struct B, a);
