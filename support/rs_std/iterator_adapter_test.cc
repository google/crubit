// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Declare the namespace we use for C++ bindings to the Rust `std` crate to
// catch any uses of `std` from inside the `rs` namespace that were meant to
// refer to top-level `std`.
namespace rs::std {}

#include "support/rs_std/iterator_adapter.h"

#include "gtest/gtest.h"

TEST(IteratorAdapterTest, Compiles) {}
