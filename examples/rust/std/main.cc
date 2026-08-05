// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <iostream>

// The generated bindings are in a header at the same path as the
// `example_crate` rust_library, with a `.h` suffix.
#include "support/rs_std/str_ref.h"
#include "examples/rust/std/example_crate.h"
#include "support/rs_std/rs_alloc.h"

int main(int argc, char* argv[]) {
  rs_std::Result<rs_std::Option<rs_std::StrRef>, ::rs::alloc::string::String>
      result = example_crate::returns_result(true);
  if (result.has_value()) {
    if (result->has_value()) {
      std::cout << "ok: " << **result << std::endl;
    } else {
      std::cout << "ok: None" << std::endl;
    }
  } else {
    std::cout << "err:" << result.error() << std::endl;
  }
}
