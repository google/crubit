// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/library_config_aspect_hint/namespace_crate2.h"

namespace crubit {
namespace {

TEST(NamespaceTest, RuleWithNamespaceConfigurationChangesDefaultNamespace) {
  crate2_namespace::crate2_subnamespace::Y y =
      crate2_namespace::crate2_subnamespace::Y::create(42);
  crate1_namespace::crate1_subnamespace::X x =
      crate2_namespace::crate2_subnamespace::f(y);
  EXPECT_EQ(x.field, 42);
}

TEST(NamespaceTest,
     RuleWithOutNamespaceConfigurationUseCrateNameAsDefaultNamespace) {
  namespace_crate3::Z z = namespace_crate3::Z::create(42);
  EXPECT_EQ(z.field, 42);
}

}  // namespace
}  // namespace crubit
