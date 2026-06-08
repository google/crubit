// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/async_fn/async_fn.h"

#include "gtest/gtest.h"

namespace {

TEST(AsyncFnTest, Linkage) {
  // The header is currently empty of functions due to lack of support,
  // but we ensure it compiles and includes successfully.
}

}  // namespace
