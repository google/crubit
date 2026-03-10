// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <iostream>

#include "support/rs_std/str_ref.h"
#include "support/rs_std/traits.h"
// The generated bindings are in a header at the same path as the
// `example_crate` rust_library, with a `.h` suffix.
#include "examples/rust/trait/example_crate.h"

template <typename T>
  requires(rs_std::where_v<T, example_crate::MyTrait>)
uint32_t add_with_2(T const& self) {
  return example_crate::MyTrait::impl<T>::add_with(self, 2);
}

int main(int argc, char* argv[]) {
  // The generated bindings are in a namespace with the same name as the
  // target crate:
  example_crate::MyStruct s = example_crate::MyStruct::new_(2);
  uint32_t result =
      example_crate::MyTrait::impl<example_crate::MyStruct>::add_with(s, 3);
  rs_std::StrRef description =
      example_crate::MyTrait::impl<example_crate::MyStruct>::describe(s);
  std::cout << "Result: " << result
            << "\ndescription: " << description.to_string_view() << std::endl;
  return 0;
}
