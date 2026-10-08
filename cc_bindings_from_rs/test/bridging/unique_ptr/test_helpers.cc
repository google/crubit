// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/unique_ptr/test_helpers.h"

#include <cstdint>
#include <memory>
#include <utility>

namespace unique_ptr_test {

Target::Target(std::shared_ptr<int32_t> token) : token(std::move(token)) {}

std::unique_ptr<Target> create_target(std::shared_ptr<int32_t> token) {
  return std::make_unique<Target>(std::move(token));
}

Derived::Derived(std::shared_ptr<int32_t> token) : token(std::move(token)) {}

std::unique_ptr<Base> create_virtual_base(std::shared_ptr<int32_t> token) {
  return std::make_unique<Derived>(std::move(token));
}

}  // namespace unique_ptr_test
