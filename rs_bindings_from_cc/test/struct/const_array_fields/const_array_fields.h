// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONST_ARRAY_FIELDS_CONST_ARRAY_FIELDS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONST_ARRAY_FIELDS_CONST_ARRAY_FIELDS_H_

#include <cstdint>

struct PrimitiveArrays {
  bool flags[3];
  char chars[4];
  int ints[4];
  double doubles[2];
  uint8_t bytes[4];
  const int const_ints[2];
};

inline PrimitiveArrays MakePrimitiveArrays() {
  return PrimitiveArrays{
      .flags = {true, false, true},
      .chars = {'a', 'b', 'c', '\0'},
      .ints = {10, 20, 30, 40},
      .doubles = {1.5, -2.25},
      .bytes = {0xAA, 0xBB, 0xCC, 0xDD},
      .const_ints = {7, 9},
  };
}

inline int SumPrimitiveInts(const PrimitiveArrays& s) {
  return s.ints[0] + s.ints[1] + s.ints[2] + s.ints[3] + s.const_ints[0] +
         s.const_ints[1];
}

using IntRow = int[4];

struct MatrixStruct {
  int matrix[2][4];
  IntRow alias_matrix[2];
};

inline MatrixStruct MakeMatrixStruct() {
  MatrixStruct m{};
  for (int r = 0; r < 2; ++r) {
    for (int c = 0; c < 4; ++c) {
      m.matrix[r][c] = r * 100 + c;
      m.alias_matrix[r][c] = r * 1000 + c;
    }
  }
  return m;
}

inline int ReadMatrixCell(const MatrixStruct& m, int r, int c) {
  return m.matrix[r][c];
}

union UnionOfArrays {
  uint8_t bytes[8];
  uint32_t words[2];
};

inline UnionOfArrays MakeUnionOfArraysFromWords(uint32_t w0, uint32_t w1) {
  UnionOfArrays u{};
  u.words[0] = w0;
  u.words[1] = w1;
  return u;
}

inline uint32_t ReadUnionOfArraysWord(const UnionOfArrays& u, int idx) {
  return u.words[idx];
}

struct alignas(16) AlignedLargeArrays {
  char data[64];
  uint8_t block[256];
};

inline uint8_t ChecksumLargeBlock(const AlignedLargeArrays& a) {
  return static_cast<uint8_t>(a.data[0] + a.data[63] + a.block[0] +
                              a.block[255]);
}

struct [[clang::trivial_abi]] NonTrivialHasArrayFields {
  int bar[4] = {11, 22, 33, 44};
  // We want to make the destructor non-trivial, so don't use `= default`.
  // NOLINTNEXTLINE(modernize-use-equals-default)
  ~NonTrivialHasArrayFields() {}
};

struct ArrayPtrFields {
  int id = 42;
  int (*row_ptr)[4] = nullptr;
  int (*matrix_ptr)[2][4] = nullptr;
};

inline void ScaleInPlaceDecayed(int x[4], int factor) {
  for (int i = 0; i < 4; ++i) {
    x[i] *= factor;
  }
}

inline int SumDecayedAlias(const IntRow x) { return x[0] + x[1] + x[2] + x[3]; }

enum UnscopedColor {
  kRed = 1,
  kGreen = 2,
  kBlue = 3,
};

enum class ScopedChannel {
  kAlpha = 10,
  kBeta = 20,
  kGamma = 30,
};

enum class ByteFlag : uint8_t {
  kNone = 0,
  kRead = 1,
  kWrite = 2,
  kExec = 4,
};

using ScopedRow = ScopedChannel[3];

template <typename T, int N>
using FixedArray = T[N];

struct EnumArrays {
  UnscopedColor unscoped[3];
  ScopedChannel scoped[3];
  ByteFlag flags[4];
  ScopedChannel matrix[2][3];
  ScopedRow alias_grid[2];
  FixedArray<ScopedChannel, 2> tmpl_enums;
  UnscopedColor large_enums[40];
};

inline EnumArrays MakeEnumArrays() {
  EnumArrays e{};
  e.unscoped[0] = kRed;
  e.unscoped[1] = kGreen;
  e.unscoped[2] = kBlue;
  e.scoped[0] = ScopedChannel::kAlpha;
  e.scoped[1] = ScopedChannel::kBeta;
  e.scoped[2] = ScopedChannel::kGamma;
  e.flags[0] = ByteFlag::kNone;
  e.flags[1] = ByteFlag::kRead;
  e.flags[2] = ByteFlag::kWrite;
  e.flags[3] = ByteFlag::kExec;
  e.matrix[0][0] = ScopedChannel::kAlpha;
  e.matrix[0][1] = ScopedChannel::kBeta;
  e.matrix[0][2] = ScopedChannel::kGamma;
  e.matrix[1][0] = ScopedChannel::kGamma;
  e.matrix[1][1] = ScopedChannel::kAlpha;
  e.matrix[1][2] = ScopedChannel::kBeta;
  e.alias_grid[0][0] = ScopedChannel::kBeta;
  e.alias_grid[1][2] = ScopedChannel::kGamma;
  e.tmpl_enums[0] = ScopedChannel::kAlpha;
  e.tmpl_enums[1] = ScopedChannel::kGamma;
  e.large_enums[0] = kRed;
  e.large_enums[39] = kBlue;
  return e;
}

inline ScopedChannel ReadEnumMatrixCell(const EnumArrays& e, int r, int c) {
  return e.matrix[r][c];
}

inline uint8_t SumEnumFlags(const EnumArrays& e) {
  return static_cast<uint8_t>(
      static_cast<uint8_t>(e.flags[0]) + static_cast<uint8_t>(e.flags[1]) +
      static_cast<uint8_t>(e.flags[2]) + static_cast<uint8_t>(e.flags[3]));
}

inline int CountMatchingScopedDecayed(const ScopedChannel arr[3],
                                      ScopedChannel target) {
  int count = 0;
  for (int i = 0; i < 3; ++i) {
    if (arr[i] == target) {
      ++count;
    }
  }
  return count;
}

union UnionOfEnumArrays {
  UnscopedColor unscoped[2];
  ScopedChannel channels[2];
  int32_t raw_words[2];
};

inline UnionOfEnumArrays MakeUnionOfEnumArrays(ScopedChannel c0,
                                               ScopedChannel c1) {
  UnionOfEnumArrays u{};
  u.channels[0] = c0;
  u.channels[1] = c1;
  return u;
}

inline int32_t ReadUnionOfEnumArraysRawWord(const UnionOfEnumArrays& u,
                                            int idx) {
  return u.raw_words[idx];
}

struct __attribute__((packed)) PackedArrayStruct {
  uint8_t tag;
  int ints[2];
  ScopedChannel channels[2];
};

inline int SumPackedArrayStruct(const PackedArrayStruct& p) {
  return static_cast<int>(p.tag) + p.ints[0] + p.ints[1] +
         static_cast<int>(p.channels[0]) + static_cast<int>(p.channels[1]);
}

struct AlignedQueueInfo {
  uint64_t addr;
};

struct TrailingZeroLengthArrayHeader {
  uint16_t num_queues;
  uint8_t pad[6];
  AlignedQueueInfo queues[0];
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONST_ARRAY_FIELDS_CONST_ARRAY_FIELDS_H_
