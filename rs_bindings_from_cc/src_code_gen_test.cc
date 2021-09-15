// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen.h"

#include <string>

#include "rs_bindings_from_cc/ir.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"

namespace rs_bindings_from_cc {

namespace {

using ::testing::StrEq;

TEST(SrcGenTest, FFIIntegration) {
  IR ir({HeaderName(std::string("foo/bar.h"))},
        {Func{.identifier = Identifier(std::string("hello_world")),
              .mangled_name = std::string("$$mangled_name$$"),
              .return_type = Type{std::string("i32"), std::string("int")},
              .params = {FuncParam{Type{std::string("i32"), std::string("int")},
                                   Identifier(std::string("arg"))}},
              .is_inline = true}});
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
          "pub (crate) fn __rust_thunk__hello_world (arg : i32) -> i32 ; "
          "} "
          "}"));

  EXPECT_THAT(
      // TODO(hlopko): Run generated C++ sources through clang-format.
      bindings.rs_api_impl,
      StrEq(
          // TODO(hlopko): Run generated C++ sources through clang-format.
          "# include \"foo/bar.h\" \n "
          "extern \"C\" int __rust_thunk__hello_world (int arg) { "
          "return hello_world (arg) ; "
          "}"));
}

}  // namespace
}  // namespace rs_bindings_from_cc
