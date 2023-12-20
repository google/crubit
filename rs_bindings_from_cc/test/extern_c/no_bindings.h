// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Realistically, almost everything receives no bindings; this is just a sample
// of some of the things which are most obvious.

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_NO_BINDINGS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_NO_BINDINGS_H_

namespace crubit::no_bindings {

inline void crubit_non_extern_c() {}

struct Nontrivial {
  ~Nontrivial() {}  // NOLINT(modernize-use-equals-default)
};

extern "C" {
inline void crubit_accepts_nontrivial_ptr(Nontrivial*) {}
inline void crubit_accepts_nontrivial_value(Nontrivial) {}
inline Nontrivial* crubit_returns_nontrivial_ptr() { return nullptr; }
inline Nontrivial crubit_returns_nontrivial_value() { return {}; }

[[clang::vectorcall]] inline void crubit_vectorcall() {}

[[noreturn]] inline void crubit_noreturn() {
  for (volatile unsigned int x = 0;; ++x) {
  }
}
}

}  // namespace crubit::no_bindings

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_NO_BINDINGS_H_
