// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_STD_ALLOCATOR_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_STD_ALLOCATOR_H_

#include <stdio.h>

#include <cstddef>
#include <new>

// TODO: Remove this namespace.
namespace crubit_internal {

// TODO: Refactor to a constant when Crubit supports them.
enum StdCppDefaultNewAlignment : size_t {
  Value = __STDCPP_DEFAULT_NEW_ALIGNMENT__,
};

inline void* cpp_new(size_t n) { return operator new(n); }

inline void* cpp_new_with_alignment(size_t n, size_t align) {
  return operator new(n, static_cast<std::align_val_t>(align));
}

inline void cpp_delete(void* ptr, size_t n) { operator delete(ptr); }

inline void cpp_delete_with_alignment(void* ptr, size_t n, size_t align) {
  operator delete(ptr, static_cast<std::align_val_t>(align));
}

}  // namespace crubit_internal

namespace crubit_cc_std_internal::std_allocator {

// Copy from above. `using ns::Function` is not supported yet by Crubit. So
// instead of aliasing, I copy everything here so that we can use it after the
// release.
// TODO: Refactor to a constant when Crubit supports them.
enum StdCppDefaultNewAlignment : size_t {
  Value = __STDCPP_DEFAULT_NEW_ALIGNMENT__,
};

inline void* cpp_new(size_t n) { return operator new(n); }

inline void* cpp_new_with_alignment(size_t n, size_t align) {
  return operator new(n, static_cast<std::align_val_t>(align));
}

inline void cpp_delete(void* ptr, size_t n) { operator delete(ptr); }

inline void cpp_delete_with_alignment(void* ptr, size_t n, size_t align) {
  operator delete(ptr, static_cast<std::align_val_t>(align));
}
}  // namespace crubit_cc_std_internal::std_allocator

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_STD_ALLOCATOR_H_
