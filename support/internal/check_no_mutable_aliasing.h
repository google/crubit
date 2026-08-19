// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CHECK_NO_MUTABLE_ALIASING_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CHECK_NO_MUTABLE_ALIASING_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <span>  // NOLINT(build/c++20)
#include <type_traits>

#include "support/internal/check.h"

namespace crubit::internal {

struct PtrData {
  uintptr_t start;
  uintptr_t end;
};

// Typeclass for types that are pointer-like. Specifically, C++ types that may
// to a Rust reference should specialize this template to add `kIsConst` and
// `AsPtrData` functionality.
//
// This is used to convert a reference or pointer to a `PtrData` for checking
// for illegal mutable aliasing.
template <typename T>
struct PtrLike {
  static_assert(false, "Expected pointer or reference type");
};

template <typename T>
struct PtrLike<T*> {
  static constexpr bool kIsConst = std::is_const_v<T>;
  static PtrData AsPtrData(T* t) {
    uintptr_t start = reinterpret_cast<uintptr_t>(t);
    return {
        .start = start,
        .end = start + sizeof(T),
    };
  }
};

template <typename T>
struct PtrLike<T&> {
  static constexpr bool kIsConst = std::is_const_v<T>;
  static PtrData AsPtrData(T& t) {
    uintptr_t start = reinterpret_cast<uintptr_t>(&t);
    return {
        .start = start,
        .end = start + sizeof(T),
    };
  }
};

// Converts a reference or pointer to const data into a `PtrData`.
template <typename T>
PtrData AsPtrData(T t) {
  static_assert(PtrLike<T>::kIsConst,
                "Expected pointer or reference to be const");
  return PtrLike<T>::AsPtrData(t);
}

// Converts a reference or pointer to mutable data into a `PtrData`.
template <typename T>
PtrData AsMutPtrData(T t) {
  static_assert(!PtrLike<T>::kIsConst,
                "Expected pointer or reference to be mutable");
  return PtrLike<T>::AsPtrData(t);
}

template <typename... Ts>
std::array<PtrData, sizeof...(Ts)> AsPtrDatas(Ts... ts) {
  return {AsPtrData<Ts>(ts)...};
}

template <typename... Ts>
std::array<PtrData, sizeof...(Ts)> AsMutPtrDatas(Ts... ts) {
  return {AsMutPtrData<Ts>(ts)...};
}

// Returns `true` if the two `PtrData` overlap in memory. An overlap means the
// pointers point at the same region of memory.
constexpr bool Overlaps(PtrData a, PtrData b) {
  return a.start < b.end && b.start < a.end;
}

// CHECKs that none of the mutable pointers alias with either each other or
// with any of the const pointers.
void CheckNoMutableAliasingSpans(std::span<PtrData> mut_ptrs,
                                 std::span<PtrData> const_ptrs);

// Convenience alias to allow calls with rvalue arrays.
template <size_t M = 0, size_t N = 0>
void CheckNoMutableAliasing(std::array<PtrData, M>&& mut_ptrs,
                            std::array<PtrData, N>&& const_ptrs) {
  if constexpr (M == 0 || (M == 1 && N == 0)) {
    return;
  } else if constexpr (M <= 3 && N <= 3) {
    CRUBIT_CHECK(
        !HasMutableAliasing(std::move(mut_ptrs), std::move(const_ptrs)));
  } else {
    CheckNoMutableAliasingSpans(mut_ptrs, const_ptrs);
  }
}

// Returns `true` if any of the mutable pointers alias with either each other or
// with any of the const pointers.
bool HasMutableAliasingSpans(std::span<PtrData> mut_ptrs,
                             std::span<PtrData> const_ptrs);

// Convenience alias to allow calls with rvalue arrays.
template <size_t M = 0, size_t N = 0>
bool HasMutableAliasing(std::array<PtrData, M>&& mut_ptrs,
                        std::array<PtrData, N>&& const_ptrs) {
  // Case 0: 0 mutable pointers or at most 1 pointer total -> impossible to
  // alias.
  if constexpr (M == 0 || (M == 1 && N == 0)) {
    return false;
  } else if constexpr (M <= 3 && N <= 3) {
    // Because M and N are small and compile-time constants, we expect these
    // loops to be unrolled.
    for (size_t i = 0; i < M; ++i) {
      for (size_t j = i + 1; j < M; ++j) {
        if (Overlaps(mut_ptrs[i], mut_ptrs[j])) {
          return true;
        }
      }
    }
    for (const auto& mut_ptr : mut_ptrs) {
      for (const auto& const_ptr : const_ptrs) {
        if (Overlaps(mut_ptr, const_ptr)) {
          return true;
        }
      }
    }
    return false;
  } else {
    return HasMutableAliasingSpans(mut_ptrs, const_ptrs);
  }
}

}  // namespace crubit::internal

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CHECK_NO_MUTABLE_ALIASING_H_
