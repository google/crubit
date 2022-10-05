// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_H_

#include <type_traits>

void Overload();
void Overload(int);

// An overload where at least one of the functions is uncallable.
// This can happen in real code, one example is the `void*` overload of
// absl::flags_internal::FlagImpl::Read().
inline void UncallableOverload(void* x) {}

// TODO(b/251045039): delete this overload
inline void UncallableOverload(int* x) {}

// This template is attempting sfinae, but is ill-formed. :(
template <typename T, std::enable_if_t<sizeof(T) == 1> = 0>
void UncallableOverload(T* x) {}

inline void AlsoTemplateOverload() {}
template <typename T>
void AlsoTemplateOverload(T x) {}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_H_
