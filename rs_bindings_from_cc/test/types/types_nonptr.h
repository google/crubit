// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NONPTR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NONPTR_H_

#include <stddef.h>
#include <stdint.h>

#include <cstddef>
#include <cstdint>

#include "crubit/support/annotations_internal.h"

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

namespace ns {
struct ExampleStruct final {};
}  // namespace ns
TEST(Struct, ns::ExampleStruct);

using Alias = ns::ExampleStruct;
TEST(TypeAlias, Alias);
// NOLINTNEXTLINE(google-global-names-in-headers)
using ns::ExampleStruct;
TEST(Using, ExampleStruct);

namespace ns {
enum ExampleEnum {};
}  // namespace ns
TEST(Enum, ns::ExampleEnum);

using AliasEnum = ns::ExampleEnum;
TEST(TypeAliasEnum, AliasEnum);
// NOLINTNEXTLINE(google-global-names-in-headers)
using ns::ExampleEnum;
TEST(UsingEnum, ExampleEnum);

struct CRUBIT_INTERNAL_RUST_TYPE("i8") MyI8Struct final {
  signed char x;
};

struct CRUBIT_INTERNAL_RUST_TYPE("i8") MyI8Class final {
  signed char x;
};

// The enum should become i8, even though it has underlying type of u8.
enum CRUBIT_INTERNAL_RUST_TYPE("i8") MyI8Enum : unsigned char { kX };

// The alias should become i8, even though it's an alias for u8.
using MyI8Alias CRUBIT_INTERNAL_RUST_TYPE("i8") = unsigned char;

TEST(ExistingRustTypeStruct, MyI8Struct);
TEST(ExistingRustTypeClass, MyI8Class);
TEST(ExistingRustTypeEnum, MyI8Enum);
TEST(ExistingRustTypeAlias, MyI8Alias);

template <typename T>
struct CRUBIT_INTERNAL_RUST_TYPE("&[]") SliceRef final {
  size_t size;
  T* data;
};

// Test all numerical types...
TEST(ExistingRustTypeSliceRefConstUint8, SliceRef<const uint8_t>);
TEST(ExistingRustTypeSliceRefUint8, SliceRef<uint8_t>);
TEST(ExistingRustTypeSliceRefConstUint16, SliceRef<const uint16_t>);
TEST(ExistingRustTypeSliceRefUint16, SliceRef<uint16_t>);
TEST(ExistingRustTypeSliceRefConstUint32, SliceRef<const uint32_t>);
TEST(ExistingRustTypeSliceRefUint32, SliceRef<uint32_t>);
TEST(ExistingRustTypeSliceRefConstUint64, SliceRef<const uint64_t>);
TEST(ExistingRustTypeSliceRefUint64, SliceRef<uint64_t>);

TEST(ExistingRustTypeSliceRefConstInt8, SliceRef<const int8_t>);
TEST(ExistingRustTypeSliceRefInt8, SliceRef<int8_t>);
TEST(ExistingRustTypeSliceRefConstInt16, SliceRef<const int16_t>);
TEST(ExistingRustTypeSliceRefInt16, SliceRef<int16_t>);
TEST(ExistingRustTypeSliceRefConstInt32, SliceRef<const int32_t>);
TEST(ExistingRustTypeSliceRefInt32, SliceRef<int32_t>);
TEST(ExistingRustTypeSliceRefConstInt64, SliceRef<const int64_t>);
TEST(ExistingRustTypeSliceRefInt64, SliceRef<int64_t>);

TEST(ExistingRustTypeSliceRefConstFloat, SliceRef<const float>);
TEST(ExistingRustTypeSliceRefFloat, SliceRef<float>);
TEST(ExistingRustTypeSliceRefConstDouble, SliceRef<const double>);
TEST(ExistingRustTypeSliceRefDouble, SliceRef<double>);

// ... and arbitrary structs/enums.
TEST(ExistingRustTypeSliceRefArbitraryStruct, SliceRef<ns::ExampleStruct>);
TEST(ExistingRustTypeSliceRefArbitraryEnum, SliceRef<const ns::ExampleEnum>);
TEST(ExistingRustTypeSliceRefArbitraryAliasEnum, SliceRef<AliasEnum>);

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_TYPES_NONPTR_H_
