// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen.h"

#include <string>

#include "rs_bindings_from_cc/ir.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/strings/cord.h"

namespace rs_bindings_from_cc {

namespace {

using ::testing::StrEq;

TEST(SrcGenTest, FFIIntegration) {
  IR ir({HeaderName(absl::Cord("foo/bar.h"))},
        {Func(Identifier(absl::Cord("hello_world")),
              absl::Cord("$$mangled_name$$"), Type(absl::Cord("i32")),
              {FuncParam(Type(absl::Cord("i32")),
                         Identifier(absl::Cord("arg")))})});
  Bindings bindings = GenerateBindings(ir);
  EXPECT_THAT(
      bindings.rs_api,
      StrEq(
          // TODO(hlopko): Run generated Rust sources through rustfmt.
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

  EXPECT_THAT(
      // TODO(hlopko): Run generated C++ sources through clang-format.
      bindings.rs_api_impl,
      StrEq("// No bindings implementation code was needed."));
}

}  // namespace
}  // namespace rs_bindings_from_cc
