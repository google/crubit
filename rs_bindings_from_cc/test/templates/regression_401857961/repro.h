// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_REGRESSION_401857961_REPRO_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_REGRESSION_401857961_REPRO_H_

namespace repro {

template <typename T>
struct optional {
  T operator*() const { return T{}; }
};

struct Interval final {
  char nanos[1] = {};
};

template <typename T>
struct Nullable {
  static void Assign(T* out, optional<T>& input) { *out = *input; }
};

inline void crash(Nullable<Interval>) {}

}  // namespace repro

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_REGRESSION_401857961_REPRO_H_
