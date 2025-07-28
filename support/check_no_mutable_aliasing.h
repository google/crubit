// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CHECK_NO_MUTABLE_ALIASING_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CHECK_NO_MUTABLE_ALIASING_H_

#include <array>
#include <cstdint>
#include <type_traits>

#include "absl/types/span.h"

namespace crubit::internal {

struct PtrData {
  uintptr_t start;
  uintptr_t end;
};

// Converts a reference or pointer to const data into a `PtrData`.
template <typename T>
PtrData AsPtrData(T t) {
  if constexpr (std::is_pointer_v<T>) {
    static_assert(std::is_const_v<std::remove_pointer_t<T>>);
    uintptr_t start = reinterpret_cast<uintptr_t>(t);
    return {
        .start = start,
        .end = start + sizeof(std::remove_pointer_t<T>),
    };
  } else if constexpr (std::is_reference_v<T>) {
    static_assert(std::is_const_v<std::remove_reference_t<T>>);
    uintptr_t start = reinterpret_cast<uintptr_t>(&t);
    return {
        .start = start,
        .end = start + sizeof(std::remove_reference_t<T>),
    };
  } else {
    static_assert(false, "Expected pointer or reference type");
  }
}

// Converts a reference or pointer to mutable data into a `PtrData`.
template <typename T>
PtrData AsMutPtrData(T t) {
  if constexpr (std::is_pointer_v<T>) {
    static_assert(!std::is_const_v<std::remove_pointer_t<T>>);
    uintptr_t start = reinterpret_cast<uintptr_t>(t);
    return {
        .start = start,
        .end = start + sizeof(std::remove_pointer_t<T>),
    };
  } else if constexpr (std::is_reference_v<T>) {
    static_assert(!std::is_const_v<std::remove_reference_t<T>>);
    uintptr_t start = reinterpret_cast<uintptr_t>(&t);
    return {
        .start = start,
        .end = start + sizeof(std::remove_reference_t<T>),
    };
  } else {
    static_assert(false, "Expected pointer or reference type");
  }
}

template <typename... Ts>
std::array<PtrData, sizeof...(Ts)> AsPtrDatas(Ts... ts) {
  return {AsPtrData<Ts>(ts)...};
}

template <typename... Ts>
std::array<PtrData, sizeof...(Ts)> AsMutPtrDatas(Ts... ts) {
  return {AsMutPtrData<Ts>(ts)...};
}

// CHECKs that none of the mutable pointers alias with either each other or
// with any of the const pointers.
void CheckNoMutableAliasingSpans(absl::Span<PtrData> mut_ptrs,
                                 absl::Span<PtrData> const_ptrs);

// Convenience alias to allow calls with rvalue arrays.
template <auto M = 0, auto N = 0>
bool CheckNoMutableAliasing(std::array<PtrData, M>&& mut_ptrs,
                            std::array<PtrData, N>&& const_ptrs) {
  return CheckNoMutableAliasingSpans(absl::MakeSpan(mut_ptrs),
                                     absl::MakeSpan(const_ptrs));
}

// Returns `true` if any of the mutable pointers alias with either each other or
// with any of the const pointers.
bool HasMutableAliasingSpans(absl::Span<PtrData> mut_ptrs,
                             absl::Span<PtrData> const_ptrs);

// Convenience alias to allow calls with rvalue arrays.
template <auto M = 0, auto N = 0>
bool HasMutableAliasing(std::array<PtrData, M>&& mut_ptrs,
                        std::array<PtrData, N>&& const_ptrs) {
  return HasMutableAliasingSpans(absl::MakeSpan(mut_ptrs),
                                 absl::MakeSpan(const_ptrs));
}

}  // namespace crubit::internal

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CHECK_NO_MUTABLE_ALIASING_H_
