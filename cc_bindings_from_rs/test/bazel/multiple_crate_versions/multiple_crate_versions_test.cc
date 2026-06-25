// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bazel/multiple_crate_versions/consumer_v1.h"
#include "cc_bindings_from_rs/test/bazel/multiple_crate_versions/consumer_v2.h"

namespace crubit {
namespace {

TEST(MultipleCrateVersionsTest, BasicTest) {
  EXPECT_EQ("v1", consumer_v1::GetV1String());
  EXPECT_EQ("v1", consumer_v2::GetV2String());
  EXPECT_EQ("v1 free", consumer_v1::GetV1FreeFunction());
  EXPECT_EQ("v1 method", consumer_v1::GetV1Method());
  EXPECT_EQ("v1 assoc", consumer_v1::GetV1AssocFunction());
  EXPECT_EQ("v1", consumer_v1::GetV1Clone());

  // Due to ODR and because we don't currently handle multiple versions of
  // crates, consumer_v2 ends up using consumer_v1's symbols after
  // linking.
  EXPECT_EQ("v1", consumer_v2::GetV2String());
  EXPECT_EQ("v2 free", consumer_v2::GetV2FreeFunction());
  EXPECT_EQ("v2 method", consumer_v2::GetV2Method());
  EXPECT_EQ("v2 assoc", consumer_v2::GetV2AssocFunction());
  EXPECT_EQ("v1", consumer_v2::GetV2Clone());
}

}  // namespace
}  // namespace crubit
