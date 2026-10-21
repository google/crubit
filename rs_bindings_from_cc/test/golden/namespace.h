// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_


namespace test_namespace_bindings {
struct S final {
  int i;
};

// Free comment inside namespace

int f(S s);

inline void inline_function() {}

namespace inner {
void i();
}  // namespace inner
}  // namespace test_namespace_bindings

test_namespace_bindings::S identity(test_namespace_bindings::S s);

namespace test_namespace_bindings_reopened {
void x();
namespace inner {
struct S final {};
}  // namespace inner
}  // namespace test_namespace_bindings_reopened

namespace test_namespace_bindings_reopened {
void y();
namespace inner {
void z(S s);
}  // namespace inner
}  // namespace test_namespace_bindings_reopened

namespace test_namespace_bindings_inline {
inline namespace inner {
struct StructInInlineNamespace final {};
}  // namespace inner
}  // namespace test_namespace_bindings_inline

void useStructInInlineNamespaceWithFullQualifier(
    test_namespace_bindings_inline::inner::StructInInlineNamespace s);
void useStructInInlineNamespaceSkipInlineQualifier(
    test_namespace_bindings_inline::StructInInlineNamespace s);

namespace impl {  // `impl` is a reserved keyword in Rust
inline void foo() {}
}  // namespace impl

// libc++ marks `namespace std` with visibility attributes like these, unless
// `_LIBCPP_DISABLE_VISIBILITY_ANNOTATIONS` is defined.
namespace __attribute__((type_visibility("default"))) ns_with_type_visibility {
  struct S final {
    int x;
  };
}  // namespace ns_with_type_visibility

namespace __attribute__((visibility("default"))) ns_with_visibility {
  struct S final {
    int x;
  };
}  // namespace ns_with_visibility

namespace __attribute__((visibility("hidden"))) ns_with_hidden_visibility {
  struct S final {
    int x;
  };
}  // namespace ns_with_hidden_visibility

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NAMESPACE_H_
