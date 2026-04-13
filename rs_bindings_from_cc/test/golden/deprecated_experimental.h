// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DEPRECATED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DEPRECATED_H_

[[deprecated]] inline void deprecated_function() {}
[[deprecated("old")]] inline void deprecated_function_with_message() {}

struct [[deprecated]] DeprecatedStruct {};
struct [[deprecated("old")]] DeprecatedStructWithMessage {};

enum [[deprecated]] DeprecatedEnum {};
enum [[deprecated("old")]] DeprecatedEnumWithMessage {};

namespace [[deprecated]] DeprecatedNamespace {
inline void f() {}
}  // namespace DeprecatedNamespace

namespace [[deprecated("old")]] DeprecatedNamespaceWithMessage {
inline void f() {}
}  // namespace DeprecatedNamespaceWithMessage

enum DeprecatedEnumerators {
  kDeprecatedEnumerator [[deprecated]] = 0,
  kDeprecatedEnumeratorWithMessage [[deprecated("old")]] = 1,
};

using DeprecatedUsing [[deprecated]] = int;
using DeprecatedUsingWithMessage [[deprecated("old")]] = int;

struct DeprecatedFields {
  int no_message [[deprecated]];
  int message [[deprecated("old")]];
};

extern int global_var [[deprecated]];
extern int global_var_with_message [[deprecated("old")]];

template <typename T>
struct SomeTotalSpecialization {};
template <>
struct [[deprecated]] SomeTotalSpecialization<int> {};
template <>
struct [[deprecated("old")]] SomeTotalSpecialization<float> {};

template <typename T>
struct [[deprecated]] SomeTemplate {};
template <typename T>
struct [[deprecated("old")]] SomeTemplateWithMessage {};

template <typename T, typename S>
struct SomePartialSpecialization {};
template <typename T>
struct [[deprecated]] SomePartialSpecialization<T, int> {};
template <typename T>
struct [[deprecated("old")]] SomePartialSpecialization<T, float> {};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DEPRECATED_H_
