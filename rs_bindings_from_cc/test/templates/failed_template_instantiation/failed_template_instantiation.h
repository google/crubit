// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_FAILED_TEMPLATE_INSTANTIATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_FAILED_TEMPLATE_INSTANTIATION_H_

#pragma clang lifetime_elision

template <typename T>
struct UninstantiableTemplate final {
  static_assert(false);
};

template <typename T>
struct InstantiableTemplate final {};

template <typename T>
struct C final {
  using Fail = UninstantiableTemplate<T>;
  using Ok = InstantiableTemplate<T>;
};

// This type alias is created just so that we can easily refer to these types in
// test.rs instead of using their mangled names.
using Ok = C<int>::Ok;
using CSpecializedForInt = C<int>;

// This is not an error as long as `C<T>::Fail` is not instantiated.
inline void Func1(C<int>::Ok){};

inline void Func2(C<int>){};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_FAILED_TEMPLATE_INSTANTIATION_H_
