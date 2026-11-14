// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/matchers/matchers.h"

#include <string>
#include <string_view>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/strings/string_view.h"
#include "support/rs_std/rs_std.h"

namespace {

using ::testing::Contains;
using ::testing::ElementsAre;
using ::testing::ElementsAreArray;
using ::testing::Eq;
using ::testing::UnorderedElementsAre;

// `rs::StrRef` is implicitly convertible from `const char*` only in constant
// expressions (its constructor is consteval), so gMock can't convert raw
// string-literal element matchers to `Matcher<const StrRef&>`. Use `Eq` to
// compare via `operator==(StrRef, const char*)` instead.
TEST(MatchersTest, SliceRefStrRef) {
  rs::SliceRef<const rs::StrRef> s = matchers::return_slice_str_ref();
  EXPECT_THAT(s, ElementsAre(Eq("foo"), Eq("bar")));
  EXPECT_THAT(s, Contains(Eq("foo")));
  EXPECT_THAT(s, UnorderedElementsAre(Eq("bar"), Eq("foo")));
}

TEST(MatchersTest, VecStrRef) {
  rs::Vec<rs::StrRef> v = matchers::return_vec_str_ref();
  EXPECT_THAT(v, ElementsAre(Eq("foo"), Eq("bar")));
  EXPECT_THAT(v, Contains(Eq("foo")));
  EXPECT_THAT(v, UnorderedElementsAre(Eq("bar"), Eq("foo")));
}

TEST(MatchersTest, VecString) {
  rs::Vec<rs::String> v = matchers::return_vec_string();
  EXPECT_THAT(v, ElementsAre("foo", "bar"));
  EXPECT_THAT(v, ElementsAreArray({"foo", "bar"}));
  EXPECT_THAT(v, Contains("foo"));
  EXPECT_THAT(v, Contains(absl::string_view("foo")));
  EXPECT_THAT(v, Contains(std::string_view("bar")));
  EXPECT_THAT(v, UnorderedElementsAre("bar", "foo"));
  EXPECT_THAT(v, ElementsAre(std::string("foo"), absl::string_view("bar")));
  EXPECT_THAT(v, ElementsAre(rs::StrRef("foo"), rs::StrRef("bar")));

  const char* foo_ptr = "foo";
  EXPECT_EQ(v[0], foo_ptr);
  EXPECT_EQ(foo_ptr, v[0]);
  EXPECT_NE(v[1], foo_ptr);
  EXPECT_EQ(v[0], absl::string_view("foo"));
  EXPECT_EQ(absl::string_view("foo"), v[0]);
  EXPECT_NE(absl::string_view("bar"), v[0]);
  EXPECT_EQ(v[0], std::string_view("foo"));
  EXPECT_EQ(std::string_view("foo"), v[0]);
  EXPECT_EQ(v[0], rs::StrRef("foo"));
  EXPECT_EQ(rs::StrRef("foo"), v[0]);
}

TEST(MatchersTest, PrintsReadably) {
  rs::Vec<rs::String> v = matchers::return_vec_string();
  EXPECT_EQ(::testing::PrintToString(v), "{ foo, bar }");

  rs::Vec<rs::StrRef> v2 = matchers::return_vec_str_ref();
  EXPECT_EQ(::testing::PrintToString(v2), "{ foo, bar }");

  rs::SliceRef<const rs::StrRef> s = matchers::return_slice_str_ref();
  EXPECT_EQ(::testing::PrintToString(s), "{ foo, bar }");
}

}  // namespace
