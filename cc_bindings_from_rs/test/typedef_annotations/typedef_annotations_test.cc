// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/typedef_annotations/typedef_annotations.h"

#include <type_traits>

#include "gtest/gtest.h"

namespace {

TEST(TypedefAnnotationsTest, AliasType) {
  static_assert(std::is_same_v<typedef_annotations::Alias, std::int32_t>);
}

}  // namespace
