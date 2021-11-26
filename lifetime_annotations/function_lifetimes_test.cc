// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/function_lifetimes.h"

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"

namespace devtools_rust {
namespace {

TEST(FunctionLifetimesTest, ContainsLifetimes) {
  EXPECT_FALSE(FunctionLifetimes().ContainsLifetimes());

  {
    FunctionLifetimes lifetimes;
    lifetimes.param_lifetimes.push_back({});
    EXPECT_FALSE(lifetimes.ContainsLifetimes());
  }

  {
    FunctionLifetimes lifetimes;
    lifetimes.param_lifetimes.push_back({Lifetime::Static()});
    EXPECT_TRUE(lifetimes.ContainsLifetimes());
  }

  {
    FunctionLifetimes lifetimes;
    lifetimes.this_lifetimes = {Lifetime::Static()};
    EXPECT_TRUE(lifetimes.ContainsLifetimes());
  }

  {
    FunctionLifetimes lifetimes;
    lifetimes.return_lifetimes = {Lifetime::Static()};
    EXPECT_TRUE(lifetimes.ContainsLifetimes());
  }
}

}  // namespace
}  // namespace devtools_rust
