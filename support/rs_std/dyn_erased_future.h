// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_ERASED_FUTURE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_ERASED_FUTURE_H_

#include <cstring>

#include "support/rs_std/waker.h"

namespace crubit {
namespace internal_dyn_erased_future {
extern "C" {
// These functions are defined in dyn_erased_future.rs and exposed by the
// linker.
void rs_std_dyn_erased_future_init(void* storage);
void rs_std_dyn_erased_future_drop(void* storage);
bool rs_std_dyn_erased_future_poll(void* storage,
                                   const rs_std::Waker* waker_ptr, void* out);
}
}  // namespace internal_dyn_erased_future

// A thin wrapper around a Rust Pin<Box<dyn ErasedFuture + Send + 'a>>.
//
// This is the return type of Rust async functions when wrapped by Crubit,
// and is not associated with any particular async C++ library. Instead,
// it offers just the bare minimum functionality of being able to poll the
// underlying future, and it is the responsibility of the user to do something
// with this. This allows Crubit to be agnostic to the async library used.
//
// For example, an async C++ library might implement an `await_transform`
// overload that allows them to co_await a DynErasedFuture.
//
// SAFETY / LIFETIME REQUIREMENTS:
// `DynErasedFuture` instances MUST be used entirely within the expression in
// which they are created (e.g., `co_await my_rust_fn()`). It is critical that
// `DynErasedFuture` instances are never stored or persisted across
// expressions because any temporary C++ arguments bound to the underlying Rust
// future will be destroyed at the end of the full-expression; if the future
// were stored and polled later, it would result in a use-after-free.
template <typename T>
class alignas(void*) [[clang::trivial_abi]] DynErasedFuture final {
 public:
  DynErasedFuture() = delete;
  DynErasedFuture(DynErasedFuture&& other) {
    std::memcpy(storage_, other.storage_, sizeof(storage_));
    internal_dyn_erased_future::rs_std_dyn_erased_future_init(other.storage_);
  }
  DynErasedFuture& operator=(DynErasedFuture&&) = delete;
  DynErasedFuture(const DynErasedFuture&) = delete;
  DynErasedFuture& operator=(const DynErasedFuture&) = delete;

  ~DynErasedFuture() {
    internal_dyn_erased_future::rs_std_dyn_erased_future_drop(storage_);
  }

  // Polls the underlying Rust future, attempting to advance it to completion.
  //
  // If the future completes, its output is written to `out`, and `true` is
  // returned. If the future is pending, `out` is unmodified, and `false` is
  // returned.
  //
  // REQUIRES: `Poll` has not previously returned `true`. Polling a future
  //           after it has completed is undefined behavior.
  //
  // REQUIRES: `out` points to an uninitialized, aligned T slot that is valid
  //           for writes.
  //
  // REQUIRES: If the `DynErasedFuture` was created with temporaries, they must
  //           still be alive and valid.
  bool Poll(const rs_std::Waker* waker_ptr, T* out) {
    return internal_dyn_erased_future::rs_std_dyn_erased_future_poll(
        storage_, waker_ptr, out);
  }

 private:
  // Much of the correctness of DynErasedFuture relies on it having the same
  // memory layout as the Rust definition, which supports both 32-bit and
  // 64-bit platforms. We check that we're on one of these platforms.
  static_assert((sizeof(void*) == 8 && alignof(void*) == 8)          // 64-bit
                    || (sizeof(void*) == 4 && alignof(void*) == 4),  // 32-bit
                "Only 64-bit and 32-bit platforms are supported.");

  // Stores the raw bytes of the Rust `DynErasedFuture` value, which is two
  // pointers and has the alignment of a pointer.
  alignas(void*) unsigned char storage_[sizeof(void*) * 2];
};

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_DYN_ERASED_FUTURE_H_
