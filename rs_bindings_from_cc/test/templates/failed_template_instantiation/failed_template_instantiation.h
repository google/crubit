// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_FAILED_TEMPLATE_INSTANTIATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_FAILED_TEMPLATE_INSTANTIATION_H_

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

// This is not an error as long as `C<T>::Fail` is not instantiated.
// However, currently Crubit attempts to instantiate all members of `C<int>` and
// will error out when the instantiation attempt fails.
inline void Func1(C<int>::Ok){};

inline void Func2(C<int>){};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_FAILED_TEMPLATE_INSTANTIATION_H_
