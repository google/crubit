// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/tag_decl_annotations/tag_decl_annotations.h"

#include <type_traits>

#include "gtest/gtest.h"

namespace {

TEST(TagDeclAnnotationsTest, EnumVariants) {
  using namespace tag_decl_annotations;

  static_assert(std::is_enum_v<SomeEnum>);
  EXPECT_EQ(static_cast<int>(SomeEnum::VARIANT_0), 0);
  EXPECT_EQ(static_cast<int>(SomeEnum::VARIANT_1), 1);
  EXPECT_EQ(static_cast<int>(SomeEnum::VARIANT_2), 2);
}

}  // namespace
