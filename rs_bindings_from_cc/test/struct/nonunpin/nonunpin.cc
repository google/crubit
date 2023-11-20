// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/nonunpin/nonunpin.h"

#include "absl/log/check.h"

void Nonunpin::CheckInvariant() const {
  CHECK_EQ(reinterpret_cast<const void*>(addr_), static_cast<const void*>(this))
      << "Object was trivially relocated, but that is not supported.";
}
