// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_RUST_LAYOUT_EQUIVALENT_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_RUST_LAYOUT_EQUIVALENT_H_

#include <cstdint>

#include "support/annotations.h"

namespace crubit::test {

// SAFETY: SameCrateCppType and crate::SameCrateRustType are both
// standard-layout / #[repr(C)] structs containing a single int32_t / i32.
struct CRUBIT_RUST_LAYOUT_EQUIVALENT("crate::SameCrateRustType")
    SameCrateCppType final {
  int32_t x;
};

inline SameCrateCppType AddOneToSameCrate(SameCrateCppType v) {
  return SameCrateCppType{v.x + 1};
}

// SAFETY: TaggedValue<T, Tag> and crate::TaggedValue<T, Tag> both store a
// single value of type T with identical alignment and layout.
template <typename T, int64_t Tag>
struct CRUBIT_RUST_LAYOUT_EQUIVALENT("crate::TaggedValue<{}, {}>", T,
                                     crubit::rust_type::Const<Tag>)
    TaggedValue final {
  T value;
};

inline int64_t AddTag(TaggedValue<int32_t, 10> v) { return v.value + 10; }

// SAFETY: ExternalCppType and
// ::rust_layout_equivalent_other_crate::ExternalRustType are both
// standard-layout / #[repr(C)] structs containing a single int32_t / i32.
struct CRUBIT_RUST_LAYOUT_EQUIVALENT(
    "::rust_layout_equivalent_other_crate::ExternalRustType")
    CRUBIT_RUST_LAYOUT_EQUIVALENT_LABEL_HINT(
        "//rs_bindings_from_cc/test/annotations:"
        "rust_layout_equivalent_other_crate") ExternalCppType final {
  int32_t value;
};

inline ExternalCppType DoubleExternal(ExternalCppType v) {
  return ExternalCppType{v.value * 2};
}

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_RUST_LAYOUT_EQUIVALENT_H_
