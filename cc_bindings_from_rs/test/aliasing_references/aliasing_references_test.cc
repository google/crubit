// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/aliasing_references/aliasing_references.h"

#include <array>
#include <cstdint>

#include "gtest/gtest.h"
#include "absl/types/span.h"

namespace {

using ::aliasing_references::mut_ref_and_mut_slice;
using ::aliasing_references::mut_ref_and_shared_refs;
using ::aliasing_references::mut_ref_and_shared_slice;
using ::aliasing_references::mut_ref_and_str;
using ::aliasing_references::mut_refs;
using ::aliasing_references::NonFreezeType;
using ::aliasing_references::SomeStruct;

TEST(AliasingReferencesTest, NonOverlappingMutableReferencesAreAllowed) {
  int32_t x = 0;
  int32_t y = 0;
  mut_refs(x, y);

  SomeStruct s;
  s.mut_self_and_mut_ref(x);
  s.mut_self_and_shared_ref(x);
  s.shared_self_and_mut_ref(x);

  std::array<int32_t, 3> array = {1, 2, 3};
  mut_ref_and_mut_slice(x, absl::MakeSpan(array));
  mut_ref_and_shared_slice(x, array);
  mut_ref_and_str(x, "foo");
}

TEST(AliasingReferencesTest, OverlappingSharedReferencesAreAllowed) {
  int32_t x = 0;
  int32_t y = 0;
  mut_ref_and_shared_refs(x, y, y);

  SomeStruct s;
  s.shared_self_and_shared_ref_allows_alias(s.field);
}

TEST(AliasingReferencesTest, NonFreezeTypeMayOverlap) {
  NonFreezeType aliasable;
  aliasable.shared_self_mut_ref_allows_alias(aliasable.as_mut_unchecked());
}

}  // namespace
