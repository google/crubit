// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "bitfields.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kBitfieldsSize = sizeof(struct Bitfields);
const size_t kBitfieldsAlign = alignof(struct Bitfields);
