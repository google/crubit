// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/vec/vec.h"

#include <cstdint>

#include "support/rs/vec.h"
#include "support/rs_std/rs_alloc.h"

namespace test {

void PushBytes(rs::Vec<uint8_t>& dst) {
  dst.push_back(4);
  dst.push_back(5);
}

uint8_t SumBytes(const rs::Vec<uint8_t>& src) {
  uint8_t sum = 0;
  for (uint8_t b : src) {
    sum += b;
  }
  return sum;
}

rs::Vec<uint8_t> MakeBytes() { return {10, 20, 30}; }

uint8_t ConsumeBytes(rs::Vec<uint8_t> v) { return SumBytes(v); }

}  // namespace test
