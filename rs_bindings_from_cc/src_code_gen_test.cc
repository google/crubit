// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen.h"

#include "rs_bindings_from_cc/ir.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"

namespace rs_bindings_from_cc {

namespace {

using ::testing::StrEq;

TEST(SrcGenTest, FFIIntegration) {
  IR ir = {.used_headers = {HeaderName("foo/bar.h")},
           .items = {Func{.name = Identifier("hello_world"),
                          .mangled_name = "mangled_name",
                          .return_type = MappedType::Simple("i32", "int"),
                          .params = {FuncParam{MappedType::Simple("i32", "int"),
                                               Identifier("arg")}},
                          .is_inline = true}}};
  Bindings bindings = GenerateBindings(ir);
  EXPECT_THAT(
      bindings.rs_api,
      StrEq(
          "#![rustfmt::skip]\n"
          "#![feature(custom_inner_attributes)]\n"
          "#![allow(non_camel_case_types)]\n"
          "#![allow(non_snake_case)]\n"
          "\n"
          "#[inline(always)]\n"
          "pub fn hello_world(arg: i32) -> i32 {\n"
          "    unsafe { crate::detail::__rust_thunk__mangled_name(arg) }\n"
          "}\n"
          "\n"
          "mod detail {\n"
          "    #[allow(unused_imports)]\n"
          "    use super::*;\n"
          "    extern \"C\" {\n"
          "        pub(crate) fn __rust_thunk__mangled_name(arg: i32) -> i32;\n"
          "    }\n"
          "}\n"
          "\n"
          "const _: () = assert!(std::mem::size_of::<Option<&i32>>() == "
          "std::mem::size_of::<&i32>());\n"));

  EXPECT_THAT(bindings.rs_api_impl,
              StrEq("#include <memory>\n"
                    "\n"
                    "#include "
                    "\"rs_bindings_from_cc/support/"
                    "cxx20_backports.h\"\n"
                    "#include \"foo/bar.h\"\n"
                    "\n"
                    "extern \"C\" int __rust_thunk__mangled_name(int arg) { "
                    "return hello_world(arg); "
                    "}\n"));
}

}  // namespace
}  // namespace rs_bindings_from_cc
