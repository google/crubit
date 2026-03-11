// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/uses/trasitive_reexports/transitive_reexports.h"

#include "gtest/gtest.h"

namespace {

TEST(TransitiveReexportsTest, DirectToTransitive) {
  static_assert(std::is_same_v<direct::Transitive, transitive::Transitive>);
  direct::Transitive transitive = direct::Transitive::new_(1);
  direct::Direct direct = direct::Direct::new_(direct::Transitive::new_(1));
  transitive::Transitive loopback_transitive =
      transitive_reexports::direct_to_transitive(direct);
  EXPECT_EQ(transitive.value, loopback_transitive.value);
}

}  // namespace
