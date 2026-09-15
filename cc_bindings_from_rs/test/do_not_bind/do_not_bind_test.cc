// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/do_not_bind/do_not_bind.h"

#include "gtest/gtest.h"

namespace do_not_bind {
namespace {

using TraitImpl = rs_std::impl<Struct, Trait>;

// `do_not_bind` used to parse and then be silently dropped on associated items,
// so assert absence directly rather than relying only on the golden. These must
// be dependent on `T` for the requires-expression to soft-fail.
template <typename T>
concept HasSuppressedInherentMethod =
    requires(const T& s) { s.suppressed_inherent_method(); };

template <typename T>
concept HasSuppressedTraitMethod = requires(const T& s) {
  rs_std::impl<T, Trait>::suppressed_trait_method(s);
};

static_assert(!HasSuppressedInherentMethod<Struct>);
static_assert(!HasSuppressedTraitMethod<Struct>);

TEST(DoNotBindTest, UnannotatedItemsStillReceiveBindings) {
  Struct s = {.value = 0};
  EXPECT_EQ(bound_free_fn(), 2);
  EXPECT_EQ(s.bound_inherent_method(), 4);
  EXPECT_EQ(TraitImpl::bound_trait_method(s), 6);
}

TEST(DoNotBindTest, SuppressingOneMethodKeepsTheTraitImplemented) {
  static_assert(TraitImpl::kIsImplemented);
}

}  // namespace
}  // namespace do_not_bind
