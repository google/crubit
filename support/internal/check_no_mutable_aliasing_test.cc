// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/internal/check_no_mutable_aliasing.h"

#include <cstdint>

#include "gtest/gtest.h"

namespace crubit::internal {
namespace {

TEST(AsPtrDatas, ReturnsAddressOfPointerAndReference) {
  const int32_t x = 42;
  const auto ptrs = AsPtrDatas<const int32_t&, const int32_t*>(x, &x);
  ASSERT_EQ(ptrs.size(), 2);
  const uintptr_t x_addr = reinterpret_cast<uintptr_t>(&x);
  const uintptr_t x_size = sizeof(int32_t);
  EXPECT_EQ(ptrs[0].start, x_addr);
  EXPECT_EQ(ptrs[0].end, x_addr + x_size);
  EXPECT_EQ(ptrs[1].start, x_addr);
  EXPECT_EQ(ptrs[1].end, x_addr + x_size);
}

TEST(AsMutPtrDatas, ReturnsAddressOfPointerAndReference) {
  int32_t x = 42;
  const auto ptrs = AsMutPtrDatas<int32_t&, int32_t*>(x, &x);
  ASSERT_EQ(ptrs.size(), 2);
  const uintptr_t x_addr = reinterpret_cast<uintptr_t>(&x);
  const uintptr_t x_size = sizeof(int32_t);
  EXPECT_EQ(ptrs[0].start, x_addr);
  EXPECT_EQ(ptrs[0].end, x_addr + x_size);
  EXPECT_EQ(ptrs[1].start, x_addr);
  EXPECT_EQ(ptrs[1].end, x_addr + x_size);
}

struct TestStruct {
  int32_t x;
  int32_t y;
};

TEST(HasMutableAliasing, NoMutableReferencesReturnsTrue) {
  const int32_t x = 42;
  EXPECT_FALSE(HasMutableAliasing({}, {}));
  EXPECT_FALSE(HasMutableAliasing({}, AsPtrDatas<const int32_t&>(x)));
  EXPECT_FALSE(
      HasMutableAliasing({}, AsPtrDatas<const int32_t&, const int32_t&>(x, x)));
}

TEST(HasMutableAliasing, OverlappingMutableReferences) {
  TestStruct ts = {42, 43};
  EXPECT_EQ(AsPtrData<const TestStruct&>(ts).start,
            AsPtrData<const int32_t&>(ts.x).start);
  EXPECT_EQ(AsPtrData<const TestStruct&>(ts).end,
            AsPtrData<const int32_t&>(ts.y).end);

  EXPECT_TRUE(
      HasMutableAliasing(AsMutPtrDatas<TestStruct&, TestStruct&>(ts, ts), {}));
  EXPECT_TRUE(
      HasMutableAliasing(AsMutPtrDatas<TestStruct&, int32_t&>(ts, ts.x), {}));
  EXPECT_TRUE(
      HasMutableAliasing(AsMutPtrDatas<TestStruct&, int32_t&>(ts, ts.y), {}));
  EXPECT_FALSE(
      HasMutableAliasing(AsMutPtrDatas<int32_t&, int32_t&>(ts.x, ts.y), {}));
}

TEST(HasMutableAliasing, OverlappingMutableAndConstReference) {
  TestStruct ts = {42, 43};
  EXPECT_TRUE(HasMutableAliasing(AsMutPtrDatas<TestStruct&>(ts),
                                 AsPtrDatas<const TestStruct&>(ts)));
  EXPECT_TRUE(HasMutableAliasing(AsMutPtrDatas<TestStruct&>(ts),
                                 AsPtrDatas<const TestStruct&>(ts)));
  EXPECT_TRUE(HasMutableAliasing(AsMutPtrDatas<TestStruct&>(ts),
                                 AsPtrDatas<const int32_t&>(ts.x)));
  EXPECT_TRUE(HasMutableAliasing(AsMutPtrDatas<int32_t&>(ts.x),
                                 AsPtrDatas<const TestStruct&>(ts)));
  EXPECT_TRUE(HasMutableAliasing(AsMutPtrDatas<TestStruct&>(ts),
                                 AsPtrDatas<const int32_t&>(ts.y)));
  EXPECT_TRUE(HasMutableAliasing(AsMutPtrDatas<int32_t&>(ts.y),
                                 AsPtrDatas<const TestStruct&>(ts)));
}

TEST(HasMutableAliasing, MutableReferenceOverlapsWithOneConstReference) {
  int mut_1, mut_2, mut_3 = 0;
  const int const_1 = 0;
  const int const_2 = 0;
  const int const_3 = 0;

  EXPECT_FALSE(
      HasMutableAliasing(AsMutPtrDatas<int&, int&, int&>(mut_1, mut_2, mut_3),
                         AsPtrDatas<const int&, const int&, const int&>(
                             const_1, const_2, const_3)));
  EXPECT_TRUE(HasMutableAliasing(
      AsMutPtrDatas<int&, int&, int&>(mut_1, mut_2, mut_3),
      AsPtrDatas<const int&, const int&, const int&, const int&>(
          const_1, const_2, mut_2, const_3)));
}

}  // namespace
}  // namespace crubit::internal