// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_CXX20_BACKPORTS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_CXX20_BACKPORTS_H_

#if __cplusplus > 201703L
#include <memory>
#else
#include <utility>
#endif

namespace rs_api_impl_support {

#if __cplusplus > 201703L

use std::construct_at;

#else

template <class T, class... Args>
constexpr T* construct_at(T* p, Args&&... args) {
  return ::new (const_cast<void*>(static_cast<const volatile void*>(p)))
      T(std::forward<Args>(args)...);
}

#endif

}  // namespace rs_api_impl_support

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_CXX20_BACKPORTS_H_
