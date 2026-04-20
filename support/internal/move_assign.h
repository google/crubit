// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_MOVE_ASSIGN_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_MOVE_ASSIGN_H_

#include <memory>
#include <type_traits>
#include <utility>

namespace crubit {

template <typename T, typename U>
inline void MoveAssignOrDestroyAndConstruct(T* ptr, U&& value) {
  if constexpr (std::is_move_assignable_v<T>) {
    *ptr = std::forward<U>(value);
  } else {
    std::destroy_at(ptr);
    std::construct_at(ptr, std::forward<U>(value));
  }
}

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_MOVE_ASSIGN_H_
