// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
// We explicitly do not include mapped_cpp_type_def.h here because we expect
// that Crubit will have included it in mapped_cpp_type.h.
#include "cc_bindings_from_rs/test/bridging/mapped_cpp_type.h"

namespace mapped_cpp_type {
namespace {

TEST(MappedCppTypeTest, EqTraitGenerated) {
  // Verify that rs_std::impl<MappedCppType, Eq> is successfully generated and
  // compiled!
  EXPECT_TRUE((rs_std::impl<MappedCppType, rs::core::cmp::Eq>::kIsImplemented));
}

}  // namespace
}  // namespace mapped_cpp_type
