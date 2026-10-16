// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_PACKED_PACKED_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_PACKED_PACKED_H_

#include <cstddef>
#include <cstdint>

// Struct with __attribute__((packed))
struct __attribute__((packed)) PackedStruct {
  uint8_t a;
  uint32_t b;
  uint16_t c;
};

inline constexpr size_t kPackedStructSize = 7;
inline constexpr size_t kPackedStructAlignment = 1;
inline constexpr size_t kPackedStructOffsetA = 0;
inline constexpr size_t kPackedStructOffsetB = 1;
inline constexpr size_t kPackedStructOffsetC = 5;
inline constexpr size_t kPackedStructArray2Size = 14;
inline constexpr size_t kPackedStructArray3Size = 21;

static_assert(sizeof(PackedStruct) == kPackedStructSize);
static_assert(alignof(PackedStruct) == kPackedStructAlignment);
static_assert(offsetof(PackedStruct, a) == kPackedStructOffsetA);
static_assert(offsetof(PackedStruct, b) == kPackedStructOffsetB);
static_assert(offsetof(PackedStruct, c) == kPackedStructOffsetC);
static_assert(sizeof(PackedStruct[2]) == kPackedStructArray2Size);
static_assert(sizeof(PackedStruct[3]) == kPackedStructArray3Size);

// Struct with #pragma pack(push, 1)
#pragma pack(push, 1)
struct PragmaPushPacked1 {
  uint8_t a;
  uint32_t b;
  uint16_t c;
};
#pragma pack(pop)

inline constexpr size_t kPragmaPushPacked1Size = 7;
inline constexpr size_t kPragmaPushPacked1Alignment = 1;
inline constexpr size_t kPragmaPushPacked1OffsetA = 0;
inline constexpr size_t kPragmaPushPacked1OffsetB = 1;
inline constexpr size_t kPragmaPushPacked1OffsetC = 5;
inline constexpr size_t kPragmaPushPacked1Array2Size = 14;
inline constexpr size_t kPragmaPushPacked1Array3Size = 21;

static_assert(sizeof(PragmaPushPacked1) == kPragmaPushPacked1Size);
static_assert(alignof(PragmaPushPacked1) == kPragmaPushPacked1Alignment);
static_assert(offsetof(PragmaPushPacked1, a) == kPragmaPushPacked1OffsetA);
static_assert(offsetof(PragmaPushPacked1, b) == kPragmaPushPacked1OffsetB);
static_assert(offsetof(PragmaPushPacked1, c) == kPragmaPushPacked1OffsetC);
static_assert(sizeof(PragmaPushPacked1[2]) == kPragmaPushPacked1Array2Size);
static_assert(sizeof(PragmaPushPacked1[3]) == kPragmaPushPacked1Array3Size);

// Structs with bare #pragma pack(N) without push/pop
#pragma pack(1)
struct BarePacked1 {
  uint8_t a;
  uint32_t b;
};

#pragma pack(2)
struct BarePacked2 {
  uint8_t a;
  uint32_t b;
};

#pragma pack(4)
struct BarePacked4 {
  uint8_t a;
  uint64_t b;
};

#pragma pack()

inline constexpr size_t kBarePacked1Size = 5;
inline constexpr size_t kBarePacked1Alignment = 1;
inline constexpr size_t kBarePacked1OffsetA = 0;
inline constexpr size_t kBarePacked1OffsetB = 1;
inline constexpr size_t kBarePacked1Array2Size = 10;
inline constexpr size_t kBarePacked1Array3Size = 15;

static_assert(sizeof(BarePacked1) == kBarePacked1Size);
static_assert(alignof(BarePacked1) == kBarePacked1Alignment);
static_assert(offsetof(BarePacked1, a) == kBarePacked1OffsetA);
static_assert(offsetof(BarePacked1, b) == kBarePacked1OffsetB);
static_assert(sizeof(BarePacked1[2]) == kBarePacked1Array2Size);
static_assert(sizeof(BarePacked1[3]) == kBarePacked1Array3Size);

inline constexpr size_t kBarePacked2Size = 6;
inline constexpr size_t kBarePacked2Alignment = 2;
inline constexpr size_t kBarePacked2OffsetA = 0;
inline constexpr size_t kBarePacked2OffsetB = 2;
inline constexpr size_t kBarePacked2Array2Size = 12;
inline constexpr size_t kBarePacked2Array3Size = 18;

static_assert(sizeof(BarePacked2) == kBarePacked2Size);
static_assert(alignof(BarePacked2) == kBarePacked2Alignment);
static_assert(offsetof(BarePacked2, a) == kBarePacked2OffsetA);
static_assert(offsetof(BarePacked2, b) == kBarePacked2OffsetB);
static_assert(sizeof(BarePacked2[2]) == kBarePacked2Array2Size);
static_assert(sizeof(BarePacked2[3]) == kBarePacked2Array3Size);

inline constexpr size_t kBarePacked4Size = 12;
inline constexpr size_t kBarePacked4Alignment = 4;
inline constexpr size_t kBarePacked4OffsetA = 0;
inline constexpr size_t kBarePacked4OffsetB = 4;
inline constexpr size_t kBarePacked4Array2Size = 24;
inline constexpr size_t kBarePacked4Array3Size = 36;

static_assert(sizeof(BarePacked4) == kBarePacked4Size);
static_assert(alignof(BarePacked4) == kBarePacked4Alignment);
static_assert(offsetof(BarePacked4, a) == kBarePacked4OffsetA);
static_assert(offsetof(BarePacked4, b) == kBarePacked4OffsetB);
static_assert(sizeof(BarePacked4[2]) == kBarePacked4Array2Size);
static_assert(sizeof(BarePacked4[3]) == kBarePacked4Array3Size);

// Struct with #pragma pack(2)
#pragma pack(push, 2)
struct PragmaPacked2 {
  uint8_t a;
  uint32_t b;
};
#pragma pack(pop)

inline constexpr size_t kPragmaPacked2Size = 6;
inline constexpr size_t kPragmaPacked2Alignment = 2;
inline constexpr size_t kPragmaPacked2OffsetA = 0;
inline constexpr size_t kPragmaPacked2OffsetB = 2;
inline constexpr size_t kPragmaPacked2Array2Size = 12;
inline constexpr size_t kPragmaPacked2Array3Size = 18;

static_assert(sizeof(PragmaPacked2) == kPragmaPacked2Size);
static_assert(alignof(PragmaPacked2) == kPragmaPacked2Alignment);
static_assert(offsetof(PragmaPacked2, a) == kPragmaPacked2OffsetA);
static_assert(offsetof(PragmaPacked2, b) == kPragmaPacked2OffsetB);
static_assert(sizeof(PragmaPacked2[2]) == kPragmaPacked2Array2Size);
static_assert(sizeof(PragmaPacked2[3]) == kPragmaPacked2Array3Size);

// Struct with #pragma pack(4)
#pragma pack(push, 4)
struct PragmaPacked4 {
  uint8_t a;
  uint64_t b;
};
#pragma pack(pop)

inline constexpr size_t kPragmaPacked4Size = 12;
inline constexpr size_t kPragmaPacked4Alignment = 4;
inline constexpr size_t kPragmaPacked4OffsetA = 0;
inline constexpr size_t kPragmaPacked4OffsetB = 4;
inline constexpr size_t kPragmaPacked4Array2Size = 24;
inline constexpr size_t kPragmaPacked4Array3Size = 36;

static_assert(sizeof(PragmaPacked4) == kPragmaPacked4Size);
static_assert(alignof(PragmaPacked4) == kPragmaPacked4Alignment);
static_assert(offsetof(PragmaPacked4, a) == kPragmaPacked4OffsetA);
static_assert(offsetof(PragmaPacked4, b) == kPragmaPacked4OffsetB);
static_assert(sizeof(PragmaPacked4[2]) == kPragmaPacked4Array2Size);
static_assert(sizeof(PragmaPacked4[3]) == kPragmaPacked4Array3Size);

// Functions accepting and returning packed structs
inline PackedStruct MakePackedStruct(uint8_t a, uint32_t b, uint16_t c) {
  PackedStruct s;
  s.a = a;
  s.b = b;
  s.c = c;
  return s;
}

inline uint32_t SumPackedStruct(PackedStruct s) {
  return static_cast<uint32_t>(s.a) + s.b + static_cast<uint32_t>(s.c);
}

// Function taking a pointer to an array of packed structs and length
inline uint32_t SumPackedArray(const PackedStruct* arr, size_t len) {
  uint32_t sum = 0;
  for (size_t i = 0; i < len; ++i) {
    sum += static_cast<uint32_t>(arr[i].a) + arr[i].b +
           static_cast<uint32_t>(arr[i].c);
  }
  return sum;
}

// Function modifying packed array elements in-place to verify zero-copy
// mutability
inline void IncrementPackedArray(PackedStruct* arr, size_t len) {
  for (size_t i = 0; i < len; ++i) {
    arr[i].a += 1;
    arr[i].b += 1;
    arr[i].c += 1;
  }
}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_PACKED_PACKED_H_
