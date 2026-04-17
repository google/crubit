// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Realistically, almost everything receives no bindings; this is just a sample
// of some of the things which are most obvious.

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_NO_BINDINGS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_NO_BINDINGS_H_
#include <set>
namespace crubit::no_bindings {

using UnknownAttrAlias [[gnu::unused]] = int;

// This struct would receive bindings, if it weren't for the unrecognized
// attribute on the struct.
struct [[gnu::visibility("default")]] UnknownAttrStruct final {
  int* x;
  float y;
  UnknownAttrStruct* z;
};

// This enum would receive bindings, if it weren't for the unrecognized
// attribute on the enum.
enum [[gnu::visibility("default")]] UnknownAttrEnum {
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

[[gnu::visibility("default")]] inline void crubit_unknown_function_attribute() {
}

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
