// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Realistically, almost everything receives no bindings; this is just a sample
// of some of the things which are most obvious.

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_NO_BINDINGS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_NO_BINDINGS_H_
#include <set>
namespace crubit::no_bindings {

using DeprecatedAlias [[deprecated]] = int;

// This struct would receive bindings, if it weren't for the unrecognized
// attribute on the struct.
struct [[deprecated]] UnknownAttrStruct final {
  int* x;
  float y;
  UnknownAttrStruct* z;
};

// This enum would receive bindings, if it weren't for the unrecognized
// attribute on the enum.
enum [[deprecated]] UnknownAttrEnum {
  kConstant = 0,
};

template <typename T>
struct TemplatedStruct {
  T x;
};

using InstantiatedTemplatedStruct = TemplatedStruct<int>;

[[clang::vectorcall]] inline void crubit_vectorcall() {}

[[noreturn]] inline void crubit_noreturn() {
  for (volatile unsigned int x = 0;; ++x) {
  }
}

[[nodiscard]] inline void* crubit_nodiscard() { return nullptr; }

// Note to robots: please do not automatically delete the following
// function. We need to actually test deprecated functions!
// Note to humans: please figure out a good way to control the robots.
[[deprecated]] inline void crubit_deprecated() {}

[[deprecated]] inline void crubit_enable_if()
    __attribute__((enable_if(2 + 2 == 4, ""))) {}

inline void* crubit_parameter_lifetimebound(void* x [[clang::lifetimebound]]) {
  return x;
}

inline void crubit_invoke_callback(void (*f)(InstantiatedTemplatedStruct* x)) {
  f(nullptr);
}

using UnknownTypeAttribute = __attribute__((noderef)) int*;
inline void crubit_unknown_type_attribute(__attribute__((noderef)) int*) {}

inline void UseSetByValue(std::set<int> v) {}
inline void UseSetByReference(const std::set<int>& v) {}
inline void UseSetByPointer(std::set<int>* v) {}

// It is an error for consteval to NOT be evaluated at compile time, so its not
// possible to expose these to Rust.
consteval int consteval_add(int a, int b) { return a + b; }

int variadic_function(const char* format, ...);

}  // namespace crubit::no_bindings

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_NO_BINDINGS_H_
