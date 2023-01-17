// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/offsetof.h"

#include <stdint.h>

namespace crubit {

struct BasicStruct {
  int64_t offset0;
  int64_t offset8;
};

static_assert(CRUBIT_OFFSET_OF(offset0, BasicStruct) == 0, "");
static_assert(CRUBIT_OFFSET_OF(offset8, BasicStruct) == 8, "");

template <typename T1, typename T2>
struct TemplateWithTwoArgs {
  T1 t1;
  T2 t2;
};

static_assert(CRUBIT_OFFSET_OF(t1, TemplateWithTwoArgs<int64_t, int32_t>) == 0,
              "");
static_assert(CRUBIT_OFFSET_OF(t2, TemplateWithTwoArgs<int64_t, int32_t>) == 8,
              "");

}  // namespace crubit
