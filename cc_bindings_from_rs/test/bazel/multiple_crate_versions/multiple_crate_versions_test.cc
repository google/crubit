// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bazel/multiple_crate_versions/consumer_v1.h"
#include "cc_bindings_from_rs/test/bazel/multiple_crate_versions/consumer_v2.h"

namespace crubit {
namespace {

TEST(MultipleCrateVersionsTest, NoDuplicateLinkerSymbols) {
  // Due to an interaction between ODR and multiple versions of the
  // same crate, it's non-deterministic which version of the functions will be
  // linked in the final binary. We should fix that, but in the mean time this
  // test just verifies multiple versions don't cause a linker error.
  //
  // To make this test consistent despite the non-determinism, we return the
  // same value out of both versions of our functions.
  EXPECT_EQ("SomeStruct", consumer_v1::GetV1String());
  EXPECT_EQ("SomeStruct free", consumer_v1::GetV1FreeFunction());
  EXPECT_EQ("SomeStruct method", consumer_v1::GetV1Method());
  EXPECT_EQ("SomeStruct assoc", consumer_v1::GetV1AssocFunction());
  EXPECT_EQ("SomeStruct", consumer_v1::GetV1Clone());

  EXPECT_EQ("SomeStruct", consumer_v2::GetV2String());
  EXPECT_EQ("SomeStruct free", consumer_v2::GetV2FreeFunction());
  EXPECT_EQ("SomeStruct method", consumer_v2::GetV2Method());
  EXPECT_EQ("SomeStruct assoc", consumer_v2::GetV2AssocFunction());
  EXPECT_EQ("SomeStruct", consumer_v2::GetV2Clone());
}

}  // namespace
}  // namespace crubit
