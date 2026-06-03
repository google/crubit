// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/known_traits/from/from.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(FromTest, FromImplsBecomeConversionOperators) {
  from::Opaque opaque(123);
  EXPECT_EQ(static_cast<int32_t>(opaque), 123);
  EXPECT_EQ(static_cast<int64_t>(opaque), 123);
  EXPECT_EQ(static_cast<from::OpaqueRef>(std::move(opaque)).get_arg(),
            "Opaque");

  from::OpaqueRef opaque_ref = from::OpaqueRef::create(rs_std::StrRef("hello"));
  EXPECT_EQ(static_cast<rs_std::StrRef>(opaque_ref), "hello");

  from::NotFfiSafe not_ffi_safe = from::NotFfiSafe::create();
  EXPECT_EQ(static_cast<int32_t>(not_ffi_safe), 42);
}

TEST(FromTest, CloneCopyConstructorAndConversion) {
  from::CloneCopySource src(42);
  from::CloneCopyType dest(src);
  EXPECT_EQ(dest.__field0, 42);

  EXPECT_EQ(static_cast<from::CloneCopyType>(src).__field0, 42);
}

TEST(FromTest, CloneAllocConstructorAndConversion) {
  from::CloneAllocSource src =
      from::CloneAllocSource::create(rs_std::StrRef("hello alloc"));
  from::CloneAllocType dest(std::move(src));
  EXPECT_EQ(dest.get_value().to_string_view(), "hello alloc");

  from::CloneAllocSource src2 =
      from::CloneAllocSource::create(rs_std::StrRef("hello alloc 2"));
  EXPECT_EQ(static_cast<from::CloneAllocType>(std::move(src2))
                .get_value()
                .to_string_view(),
            "hello alloc 2");
}

TEST(FromTest, NoCloneDefaultConstructorAndConversion) {
  from::NoCloneDefaultSource src(100);

  from::NoCloneDefaultType dest(std::move(src));
  EXPECT_EQ(dest.__field0, 100);

  from::NoCloneDefaultSource src2(200);
  EXPECT_EQ(static_cast<from::NoCloneDefaultType>(std::move(src2)).__field0,
            200);
}

TEST(FromTest, NoCloneCopyDropConstructorAndConversion) {
  from::NoCloneCopyDropSource src(500);
  from::NoCloneCopyDropType dest(std::move(src));
  EXPECT_EQ(dest.__field0, 500);

  from::NoCloneCopyDropSource src2(600);
  EXPECT_EQ(static_cast<from::NoCloneCopyDropType>(std::move(src2)).__field0,
            600);
}

TEST(FromTest, FromLoop) {
  from::LoopA a(1);
  from::LoopB b(std::move(a));
  EXPECT_EQ(b.__field0, 1);

  from::LoopB b2(2);
  from::LoopA a2(std::move(b2));
  EXPECT_EQ(a2.__field0, 2);

  from::LoopA a3(3);
  EXPECT_EQ(static_cast<from::LoopB>(std::move(a3)).__field0, 3);

  from::LoopB b3(4);
  EXPECT_EQ(static_cast<from::LoopA>(std::move(b3)).__field0, 4);
}

}  // namespace
}  // namespace crubit
