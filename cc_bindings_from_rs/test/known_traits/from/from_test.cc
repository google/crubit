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
  EXPECT_EQ(static_cast<from::OpaqueRef>(opaque).get_arg(), "Opaque");

  from::OpaqueRef opaque_ref = from::OpaqueRef::create(rs_std::StrRef("hello"));
  EXPECT_EQ(static_cast<rs_std::StrRef>(opaque_ref), "hello");

  from::NotFfiSafe not_ffi_safe = from::NotFfiSafe::create();
  EXPECT_EQ(static_cast<int32_t>(not_ffi_safe), 42);
}

}  // namespace
}  // namespace crubit
