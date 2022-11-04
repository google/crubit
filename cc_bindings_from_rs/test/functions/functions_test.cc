// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/functions/functions_cc_api.h"

namespace crubit {
namespace {

// Import all the APIs from `functions_cc_api.h`
using namespace functions;

using testing::DoubleEq;

TEST(FunctionsTest, Get42AsFloat64ViaNoMangleExternC) {
  EXPECT_THAT(get_42_as_f64_via_no_mangle_extern_c(), DoubleEq(42.0));
}

TEST(FunctionsTest, AddFloat64ViaNoMangleExternC) {
  EXPECT_THAT(add_f64_via_no_mangle_extern_c(12.0, 34.0),
              DoubleEq(12.0 + 34.0));
}

TEST(FunctionsTest, AddInt32ViaNoMangleExternC) {
  EXPECT_EQ(12 + 34, add_i32_via_no_mangle_extern_c(12, 34));
}

TEST(FunctionsTest, AddInt32ViaExternCWithExportName) {
  EXPECT_EQ(12 + 34, add_i32_via_extern_c_with_export_name(12, 34));
}

TEST(FunctionsTest, AddInt32ViaExternCWithMangling) {
  // TODO(b/254097223): Uncomment the test assertion below after ensuring that
  // the `genrule` in `test/functions/BUILD` invokes `cc_bindings_from_rs` with
  // the same rustc cmdline flags as when `rustc` is used to build
  // `functions.rs` for `rust_library`.  Otherwise, the mangled name will be
  // slightly different - e.g.:
  // _ZN9functions34add_i32_via_extern_c_with_mangling17h9cf06f3d70bfe03aE vs
  // _ZN9functions34add_i32_via_extern_c_with_mangling17hc48a5cd0f6e44291E
  // EXPECT_EQ(12 + 34, add_i32_via_extern_c_with_mangling(12, 34));
}

TEST(FunctionsTest, VoidReturningFunctionWithExportName) {
  set_global_i32_via_extern_c_with_export_name(123);
  EXPECT_EQ(123, get_global_i32_via_extern_c_with_export_name());

  set_global_i32_via_extern_c_with_export_name(456);
  EXPECT_EQ(456, get_global_i32_via_extern_c_with_export_name());
}

}  // namespace
}  // namespace crubit
