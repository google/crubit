// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NONPTR_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NONPTR_H_

#include <stddef.h>
#include <stdint.h>

#include <cstddef>
#include <cstdint>

#include "support/internal/attribute_macros.h"

// Not a template, so that it isn't visible to the bindings generator.
// We're just here to save typing.
#define TEST(Name, T)                            \
  struct Name {                                  \
    T field;                                     \
    static T Function(T param) { return param; } \
  }

TEST(Bool, bool);
TEST(Char, char);

TEST(UnsignedChar, unsigned char);
TEST(SignedChar, signed char);
TEST(Char16, char16_t);
TEST(Char32, char32_t);
TEST(WChar, wchar_t);

TEST(Short, short);
TEST(Int, int);
TEST(Long, long);
TEST(LongLong, long long);

TEST(UnsignedShort, unsigned short);
TEST(UnsignedInt, unsigned int);
TEST(UnsignedLong, unsigned long);
TEST(UnsignedLongLong, unsigned long long);

TEST(SignedShort, signed short);
TEST(SignedInt, signed int);
TEST(SignedLong, signed long);
TEST(SignedLongLong, signed long long);

TEST(Int8, int8_t);
TEST(Int16, int16_t);
TEST(Int32, int32_t);
TEST(Int64, int64_t);
TEST(Uint8, uint8_t);
TEST(Uint16, uint16_t);
TEST(Uint32, uint32_t);
TEST(Uint64, uint64_t);

TEST(PtrDiff, ptrdiff_t);
TEST(Size, size_t);
TEST(IntPtr, intptr_t);
TEST(UintPtr, uintptr_t);

TEST(StdInt8, std::int8_t);
TEST(StdInt16, std::int16_t);
TEST(StdInt32, std::int32_t);
TEST(StdInt64, std::int64_t);
TEST(StdUint8, std::uint8_t);
TEST(StdUint16, std::uint16_t);
TEST(StdUint32, std::uint32_t);
TEST(StdUint64, std::uint64_t);

TEST(StdPtrDiff, std::ptrdiff_t);
TEST(StdSize, std::size_t);
TEST(StdIntPtr, std::intptr_t);
TEST(StdUintPtr, std::uintptr_t);

TEST(Float, float);
TEST(Double, double);

struct ExampleStruct final {};
TEST(Struct, ExampleStruct);

struct CRUBIT_INTERNAL_RUST_TYPE("i8") MyI8Struct final {
  signed char x;
};

struct CRUBIT_INTERNAL_RUST_TYPE("i8") MyI8Class final {
  signed char x;
};

enum CRUBIT_INTERNAL_RUST_TYPE("i8") MyI8Enum : unsigned char { kX };

TEST(TypeMapOverrideStruct, MyI8Struct);
TEST(TypeMapOverrideClass, MyI8Class);
TEST(TypeMapOverrideEnum, MyI8Enum);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NONPTR_H_
