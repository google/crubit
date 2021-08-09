// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/rs_src_code_gen.h"

#include <string>

#include "rs_bindings_from_cc/ir.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/strings/cord.h"

namespace rs_bindings_from_cc {

namespace {

using ::testing::StrEq;

TEST(RsSrcGenTest, FFIIntegration) {
  IR ir({Func(
      Identifier(absl::Cord("hello_world")), absl::Cord("$$mangled_name$$"),
      Type(absl::Cord("i32")),
      {FuncParam(Type(absl::Cord("i32")), Identifier(absl::Cord("arg")))})});
  std::string rs_api = GenerateRustApi(ir);
  EXPECT_THAT(
      rs_api,
      StrEq(
          // TODO(hlopko): Run generated sources through rustfmt.
          "# [inline (always)] "
          "pub fn hello_world (arg : i32) -> i32 { "
          "unsafe { crate :: detail :: __rust_thunk__hello_world (arg) } "
          "} "
          "mod detail { "
          "extern \"C\" { "
          "# [link_name = \"$$mangled_name$$\"] "
          "pub (crate) fn __rust_thunk__hello_world (arg : i32) -> i32 ; "
          "} "
          "}"));
}

}  // namespace
}  // namespace rs_bindings_from_cc
