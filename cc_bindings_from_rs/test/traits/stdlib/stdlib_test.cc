// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/traits/stdlib/stdlib.h"

#include <cstdint>
#include <type_traits>

#include "gtest/gtest.h"
#include "support/rs_std/rs_core.h"

namespace crubit {
namespace {

TEST(StdlibTraitTest, Iterator) {
  using impl = rs::core::iter::Iterator::impl<stdlib::MyStruct>;
  static_assert(std::is_same_v<impl::Item, std::int32_t>);

  // TODO(b/483382648): Also test bindings of `next()` method once it works.
}

}  // namespace
}  // namespace crubit
