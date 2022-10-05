// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/collect_instantiations.h"

#include <climits>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/status_test_matchers.h"
#include "common/test_utils.h"

namespace crubit {
namespace {

using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::StrEq;

// A minimal test showing that C++ and Rust link and talk to each other.
TEST(CollectInstantiationsTest, EmptyRustFileReturnsEmptyCollectionTest) {
  EXPECT_THAT(CollectInstantiations({}), IsOkAndHolds(IsEmpty()));
}

// A minimal test showing that C++ and Rust link and talk to each other.
TEST(CollectInstantiationsTest, ReturnInstantiationsFromRustTest) {
  std::string path =
      WriteFileForCurrentTest("a.rs", "cc_template!(std::vector<bool>);");
  EXPECT_THAT(CollectInstantiations({std::move(path)}),
              IsOkAndHolds(ElementsAre(StrEq("std :: vector < bool >"))));
}

}  // namespace
}  // namespace crubit
