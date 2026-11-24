// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>

#include "support/rs_std/str_ref.h"

inline size_t GetHelloSizeFromHandwrittenCpp(rs::StrRef name) {
  return name.size();
}
