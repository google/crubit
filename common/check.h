// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_CHECK_H_
#define CRUBIT_COMMON_CHECK_H_

#include <cstdlib>
#include <iostream>
#include <utility>

namespace crubit {
namespace internal {

class CheckFail {
 public:
  CheckFail(const char* file, int line, const char* condition) {
    std::cerr << file << ":" << line << ": Check failed: " << condition << " ";
  }
  [[noreturn]] ~CheckFail() {
    std::cerr << "\n";
    std::abort();
  }
  template <typename T>
  CheckFail& operator<<(const T& value) {
    std::cerr << value;
    return *this;
  }
};

template <typename T>
T DieIfNull(const char* file, int line, const char* exprtext, T&& t) {
  if (t == nullptr) {
    CheckFail(file, line, exprtext) << "Must not be null";
  }
  return std::forward<T>(t);
}

}  // namespace internal
}  // namespace crubit

#define CRUBIT_CHECK(cond) \
  switch (0)               \
  case 0:                  \
  default:                 \
    if (cond) {            \
    } else                 \
      ::crubit::internal::CheckFail(__FILE__, __LINE__, #cond)

#ifndef NDEBUG
#define CRUBIT_DCHECK(cond) CRUBIT_CHECK(cond)
#else
#define CRUBIT_DCHECK(cond) \
  while (false) CRUBIT_CHECK(cond)
#endif

#define CRUBIT_DIE_IF_NULL(val) \
  ::crubit::internal::DieIfNull(__FILE__, __LINE__, #val, (val))

#endif  // CRUBIT_COMMON_CHECK_H_
