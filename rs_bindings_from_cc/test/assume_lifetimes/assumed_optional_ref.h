// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_OPTIONAL_REF_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_OPTIONAL_REF_H_

#include <memory>
#include <optional>

namespace absl {

// Minimal implementation of absl::optional_ref to test code generation and ABI
// compatibility without depending on @abseil-cpp//absl/types:optional_ref.
template <typename T>
class optional_ref {
 public:
  constexpr optional_ref() : ptr_(nullptr) {}
  constexpr optional_ref(std::nullopt_t) : ptr_(nullptr) {}
  constexpr optional_ref(T& input [[clang::lifetimebound]])
      : ptr_(std::addressof(input)) {}
  constexpr optional_ref(T* input [[clang::lifetimebound]]) : ptr_(input) {}

  constexpr bool has_value() const { return ptr_ != nullptr; }
  constexpr T& operator*() const { return *ptr_; }
  constexpr T* as_pointer() const { return ptr_; }

 private:
  T* const ptr_;
};

}  // namespace absl

int ValueOrDefault(absl::optional_ref<const int> x, int default_val);

void IncrementIfPresent(absl::optional_ref<int> x);

absl::optional_ref<const int> IdentityConstOptionalRef(
    absl::optional_ref<const int> x);

absl::optional_ref<int> IdentityMutOptionalRef(absl::optional_ref<int> x);

absl::optional_ref<const int> FilterPositive(absl::optional_ref<const int> x);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_OPTIONAL_REF_H_
