// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_VEC_VEC_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_VEC_VEC_H_

#include <cstdint>

#include "support/rs/vec.h"
#include "support/rs_std/rs_alloc.h"

namespace test {

void PushBytes(rs::Vec<uint8_t>& dst);
uint8_t SumBytes(const rs::Vec<uint8_t>& src);
rs::Vec<uint8_t> MakeBytes();
uint8_t ConsumeBytes(rs::Vec<uint8_t> v);

}  // namespace test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_VEC_VEC_H_
