// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_

#pragma clang lifetime_elision

struct alignas(64) HasCustomAlignment {};

struct HasFieldWithCustomAlignment {
  HasCustomAlignment field;
};

struct InheritsFromBaseWithCustomAlignment : public HasCustomAlignment {};

struct HasCustomAlignmentWithGnuAttr {
} __attribute__((aligned(64)));

// This namespace is a regression test for b/244350186.  In the past the
// generated `..._rs_api_impl.cc` would fail to compile.
namespace template_with_preferred_name {

// Based on `llvm/include/c++/v1/__fwd/string_view.h` - mimics
// forward declaration of `basic_string_view` class template.
template <typename T>
struct SomeTemplate;

// Based on `llvm/include/c++/v1/__fwd/string_view.h` - mimics
// definition of the `string_view` type alias.
using SpecializedTypeAlias = SomeTemplate<int>;

// Based on `llvm/include/c++/v1/string_view` - mimics definition of
// `basic_string_view` class template (focusing on the attributes related to the
// preferred name).
template <typename T>
struct __attribute__((__preferred_name__(SpecializedTypeAlias))) SomeTemplate {
  int foo() { return 42; }
};

}  // namespace template_with_preferred_name

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_CLANG_ATTRS_H_
