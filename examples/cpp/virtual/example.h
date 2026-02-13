// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_VIRTUAL_EXAMPLE_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_VIRTUAL_EXAMPLE_H_

#include <utility>

#include "examples/cpp/virtual/base.h"
#include "examples/cpp/virtual/definition.h"

class RustDerived : public ExampleBase {
 public:
  explicit RustDerived(definition::RustDerived rust) : rust_(std::move(rust)) {}
  int Method1() const override { return rust_.rust_method1(*this); }

  ExampleBase* Upcast() { return this; }

 private:
  definition::RustDerived rust_;
};

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_VIRTUAL_EXAMPLE_H_
