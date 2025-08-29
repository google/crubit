// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/non_local_crate_bindings/core_proxy.h"

namespace {

TEST(CoreBindingsTest, AtomicBoolExists) {
  core::sync::atomic::AtomicBool unused;
}

}  // namespace
