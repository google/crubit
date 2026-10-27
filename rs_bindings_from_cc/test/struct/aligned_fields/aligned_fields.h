// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_ALIGNED_FIELDS_ALIGNED_FIELDS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_ALIGNED_FIELDS_ALIGNED_FIELDS_H_

#include <cstddef>
#include <cstdint>

// Overaligned typedef, field-level `alignas(8)`, and `__aligned__(16)` array
// field on an unpacked struct.
//
// `#[repr(C, align(16))]` on the struct alone only aligns the struct as a whole
// and rounds up its total size; it does not increase the alignment of
// individual fields:
// - `typedef_val` (natural align 4) is at offset 8 instead of 4.
// - `x` (natural align 4, following `typedef_val` at 8..12) is at offset 16
//   instead of 12, placing `mid` at offset 20.
// - `reserved` (natural align 1, following `mid` at 20..21) is at offset 32
//   instead of 21, placing `tail` at offset 48 and total size at 64.
using OveralignedInt __attribute__((aligned(8))) = int;

struct OveralignedFields final {
  uint8_t prefix;
  OveralignedInt typedef_val;
  alignas(8) int x;
  uint8_t mid;
  uint8_t reserved[16] __attribute__((__aligned__(16)));
  uint8_t tail;
};

static_assert(sizeof(OveralignedFields) == 64);
static_assert(alignof(OveralignedFields) == 16);
static_assert(offsetof(OveralignedFields, prefix) == 0);
static_assert(offsetof(OveralignedFields, typedef_val) == 8);
static_assert(offsetof(OveralignedFields, x) == 16);
static_assert(offsetof(OveralignedFields, mid) == 20);
static_assert(offsetof(OveralignedFields, reserved) == 32);
static_assert(offsetof(OveralignedFields, tail) == 48);

inline OveralignedFields MakeOveralignedFields() {
  OveralignedFields s{};
  s.prefix = 10;
  s.typedef_val = 2000;
  s.x = 3000;
  s.mid = 40;
  for (size_t i = 0; i < 16; ++i) {
    s.reserved[i] = static_cast<uint8_t>(100 + i);
  }
  s.tail = 50;
  return s;
}

inline int GetTypedefVal(OveralignedFields s) { return s.typedef_val; }
inline int GetX(OveralignedFields s) { return s.x; }
inline uint8_t GetReservedByte(OveralignedFields s, size_t i) {
  return s.reserved[i];
}
inline uint8_t GetMid(OveralignedFields s) { return s.mid; }
inline uint8_t GetTail(OveralignedFields s) { return s.tail; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_ALIGNED_FIELDS_ALIGNED_FIELDS_H_
