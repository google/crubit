// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include <iostream>

#include "rust_lib.h"
#include "std.h"

int main() {
  // Test we can successfully use a type from a Rust dependency.
  base64::engine::GeneralPurpose engine = rust_lib::make_engine();
  std::cout << "Successfully linked crubit generated C++ api: "
            << rust_lib::add(1, 2) << std::endl;

  // Test we can use bindings generated from the Rust standard library.
  rs::std::path::PathBuf temp_dir = rs::std::env::temp_dir();
  rust_lib::Gymnastics g = rust_lib::Gymnastics::new_(temp_dir);
  absl::string_view temp_dir_str = g.as_str();
  std::cout << "Successfully used Rust std::env::temp_dir(): " << temp_dir_str
            << std::endl;
  return 0;
}
