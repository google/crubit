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
  IR ir = {
      .used_headers = {HeaderName("foo/bar.h")},
      .functions = {Func{.identifier = Identifier("hello_world"),
                         .mangled_name = "$$mangled_name$$",
                         .return_type = MappedType::Simple("i32", "int"),
                         .params = {FuncParam{MappedType::Simple("i32", "int"),
                                              Identifier("arg")}},
                         .is_inline = true}},
      .records = {}};
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

  EXPECT_THAT(bindings.rs_api_impl,
              StrEq("#include \"foo/bar.h\"\n"
                    "extern \"C\" int __rust_thunk__hello_world(int arg) { "
                    "return hello_world(arg); "
                    "}"));
}

}  // namespace
}  // namespace rs_bindings_from_cc
