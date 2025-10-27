// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/uses/uses.h"

#include <cstdint>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(UsesTest, UsesExportsAsUsing) { EXPECT_EQ(uses::f(), 42); }

TEST(UsesTest, ReexportPrivateStruct) {
  constexpr int kField = 42;
  uses::ExportedStruct x = uses::ExportedStruct::create(kField);
  EXPECT_EQ(x.field, kField);

  EXPECT_TRUE(
      (std::is_same_v<uses::ExportedStruct, uses::AliasOfExportedStruct>));
}

TEST(UsesTest, ReexportPrivateFunction) { EXPECT_EQ(uses::private_fn(), 42); }

TEST(UsesTest, RexportExternCrateStruct) {
  uses::X x = uses::return_x();
  EXPECT_EQ(x.field, 42);

  extern_crate::Y y = uses::return_y();
  EXPECT_EQ(y.field, 42);

  EXPECT_TRUE((std::is_same_v<uses::Alias, uses::Original>));
}

TEST(UsesTest, DocHidden) {
  EXPECT_EQ(uses::doc_hidden_test::hidden::private_fn(), 14568);
  EXPECT_EQ(uses::doc_hidden_test::visible::private_fn(), 14568);
}

TEST(UsesTest, PrivateModuleInMiddleOfPath) {
  EXPECT_EQ(uses::a::c::private_middle_path(), 742);
}

}  // namespace
}  // namespace crubit