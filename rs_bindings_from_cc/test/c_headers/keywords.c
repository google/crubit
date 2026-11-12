// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "keywords.h"

#include <stddef.h>

#include <stdalign.h>
const size_t kKeywordsSize = sizeof(struct Keywords);
const size_t kKeywordsAlign = alignof(struct Keywords);
