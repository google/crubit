// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_ERASED_FUTURE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_ERASED_FUTURE_H_

#include <concepts>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/rs_std/waker.h"

namespace crubit {

// A customization point that allows third-party coroutine libraries to
// register an implicit conversion from crubit::DynErasedFuture.
//
// To enable implicit conversion from crubit::DynErasedFuture to MyTask,
// specialize this struct:
//
// template <>
// struct FromDynErasedFuture<MyTask> {
//   static MyTask Convert(crubit::DynErasedFuture fut) { ... }
// };
//
// This adds `operator TargetType()` to `crubit::DynErasedFuture`, allowing
// implicit conversion to `MyTask`.
template <typename TargetType>
struct FromDynErasedFuture {
  // By default, no conversion is defined.
};

namespace internal_dyn_erased_future {

extern "C" {
void rs_std_dyn_erased_future_init(void* storage);
void rs_std_dyn_erased_future_drop(void* storage);
bool rs_std_dyn_erased_future_is_completed_or_discarded(const void* storage);
void rs_std_dyn_erased_future_discard(void* storage);
bool rs_std_dyn_erased_future_poll(void* storage,
                                   const rs_std::Waker* waker_ptr, void* out);
}

}  // namespace internal_dyn_erased_future

template <typename T>
class alignas(8) [[clang::trivial_abi]] DynErasedFuture final {
 public:
  using result_type = T;

  DynErasedFuture() {
    internal_dyn_erased_future::rs_std_dyn_erased_future_init(storage_);
  }
  ~DynErasedFuture() {
    internal_dyn_erased_future::rs_std_dyn_erased_future_drop(storage_);
  }

  DynErasedFuture(DynErasedFuture&& other) noexcept {
    std::memcpy(storage_, other.storage_, sizeof(storage_));
    internal_dyn_erased_future::rs_std_dyn_erased_future_init(other.storage_);
  }
  DynErasedFuture& operator=(DynErasedFuture&& other) noexcept {
    if (this != &other) {
      internal_dyn_erased_future::rs_std_dyn_erased_future_drop(storage_);
      std::memcpy(storage_, other.storage_, sizeof(storage_));
      internal_dyn_erased_future::rs_std_dyn_erased_future_init(other.storage_);
    }
    return *this;
  }

  DynErasedFuture(const DynErasedFuture&) = delete;
  DynErasedFuture& operator=(const DynErasedFuture&) = delete;

  // Templated implicit conversion operator.
  // This allows `crubit::DynErasedFuture` to be implicitly converted to any
  // coroutine task type that has specialized `crubit::FromDynErasedFuture`.
  template <typename TargetType>
    requires requires(DynErasedFuture&& fut) {
      {
        FromDynErasedFuture<std::remove_cvref_t<TargetType>>::Convert(
            std::move(fut))
      } -> std::same_as<TargetType>;
    }
  // NOLINTNEXTLINE(google-explicit-constructor)
  [[clang::coro_wrapper]] operator TargetType() && {
    return FromDynErasedFuture<std::remove_cvref_t<TargetType>>::Convert(
        std::move(*this));
  }

  bool is_completed_or_discarded() const {
    return internal_dyn_erased_future::
        rs_std_dyn_erased_future_is_completed_or_discarded(storage_);
  }
  void discard() {
    internal_dyn_erased_future::rs_std_dyn_erased_future_discard(storage_);
  }
  bool poll(const rs_std::Waker* waker_ptr, T* out) {
    return internal_dyn_erased_future::rs_std_dyn_erased_future_poll(
        storage_, waker_ptr, out);
  }

 private:
  alignas(8) unsigned char storage_[16];
};

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_ERASED_FUTURE_H_
