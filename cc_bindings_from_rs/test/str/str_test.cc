// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/str/str.h"

#include "gtest/gtest.h"

namespace {

using ::rs_std::StrRef;

TEST(StrTest, StrAsArgument) {
  EXPECT_EQ(str::get_str_len(StrRef("hello")), 5);

  constexpr static StrRef kHello = StrRef("hello");
  EXPECT_EQ(reinterpret_cast<const char*>(str::get_str_data(kHello)),
            kHello.data());
}

TEST(StrTest, StrAsReturn) { EXPECT_EQ(str::foo_as_str(), "foo"); }

TEST(StrTest, StrAsField) {
  constexpr static StrRef kFieldValue = StrRef("field_value");
  str::TypeWithStr t = str::TypeWithStr::create(kFieldValue);
  EXPECT_EQ(t.get_str_len(), kFieldValue.size());
  EXPECT_EQ(reinterpret_cast<const char*>(t.get_str_data()),
            kFieldValue.data());
}

TEST(StrTest, StrAsPotentiallyAliasingArgument) {
  uint8_t x = 5;
  str::str_checked_as_potentially_aliasing(StrRef("hello"), x);
}

constexpr static StrRef kConst = str::CONST_STR_FOO;
static_assert(kConst == "foo");
static_assert(kConst != "bar");

}  // namespace
