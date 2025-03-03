// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_H_

void Overload();
void Overload(int);

// Both Overload2() overloads should be generated, because one should be
// renamed.
void Overload2();
[[clang::annotate("crubit_rust_name", "RenamedOverload2")]]
void Overlaod2(int);

// An overload where at least one of the functions is uncallable.
// This can happen in real code, one example is the `void*` overload of
// absl::flags_internal::FlagImpl::Read().
inline void UncallableOverload(void* x) {}

// TODO(b/251045039): delete this overload
inline void UncallableOverload(int* x) {}

template <typename T>
struct Sizeof {
  static constexpr int size = sizeof(T);
};

// This template is attempting sfinae, but is ill-formed. :(
// Attempts to call UncallableOverload with void* will lead to checking
// sizeof(void) and a hard error, not SFINAE.
// For example, see https://godbolt.org/z/xccjez61s
template <typename T, int = Sizeof<T>::size>
void UncallableOverload(T* x) {}

inline void AlsoTemplateOverload() {}
template <typename T>
void AlsoTemplateOverload(T x) {}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_H_
