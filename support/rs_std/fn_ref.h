// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs/fn_ref.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_REF_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_REF_H_

#include <functional>
#include <memory>
#include <type_traits>
#include <utility>

// NOLINTBEGIN(google-explicit-constructor)
namespace rs {

// A non-owning reference to a callable with signature `Sig`, representing
// Rust's borrowed closure types (`&dyn Fn` / `&mut dyn FnMut`).
//
// Threading and lifetime contract:
// - Unlike `absl::FunctionRef` and `std::function_ref` (which require `Sync` on
//   the Rust side when bound by `rs_bindings_from_cc`), `rs::FnRef` does NOT
//   require the underlying Rust closure to implement `Sync` or `Send` (so it
//   can capture `!Sync` state such as `Cell` or `Rc`). Consequently, an
//   `rs::FnRef` must ONLY be invoked on the calling thread and never accessed
//   from other threads.
// - `rs::FnRef` does not own its target callable; it must not be retained or
//   invoked after the call that received it returns.
template <typename Sig>
class FnRef;

namespace internal {

struct FnRefPayload {
  void* data;
  void (*invoker)();
};

struct PrivateTag {
  explicit PrivateTag() = default;
};

template <typename R, typename F, typename... Args>
R InvokeHelper(F&& f, Args&&... args) {
  if constexpr (::std::is_void_v<R>) {
    ::std::invoke(::std::forward<F>(f), ::std::forward<Args>(args)...);
  } else {
    return ::std::invoke(::std::forward<F>(f), ::std::forward<Args>(args)...);
  }
}

// Internal factory used by Crubit-generated C++ bindings to build an rs::FnRef
// straight from a fat pointer components.
template <typename Sig, typename R, typename... Args>
FnRef<Sig> MakeFnRef(void* data, R (*invoker)(void*, Args...)) noexcept {
  return FnRef<Sig>(PrivateTag{}, data, invoker);
}

}  // namespace internal

// Non-const FnRef: matches `&mut dyn FnMut(...)` in `cc_bindings_from_rs` and
// `&dyn Fn(...)` (without `Sync`) in `rs_bindings_from_cc`.
template <typename R, typename... Args>
class FnRef<R(Args...)> {
 public:
  constexpr FnRef() noexcept = delete;

  template <typename F>
    requires(!::std::is_same_v<::std::remove_cvref_t<F>, FnRef> &&
             ::std::is_invocable_r_v<R, F&, Args...>)
  FnRef(F&& f) noexcept
      : data_(const_cast<void*>(
            reinterpret_cast<const void*>(::std::addressof(f)))),
        invoker_([](void* data, Args... args) -> R {
          return internal::InvokeHelper<R>(
              *reinterpret_cast<::std::remove_reference_t<F>*>(data),
              ::std::forward<Args>(args)...);
        }) {}

  template <typename F>
    requires(::std::is_function_v<F> && ::std::is_invocable_r_v<R, F&, Args...>)
  FnRef(F* f) noexcept
      : data_(reinterpret_cast<void*>(f)),
        invoker_([](void* data, Args... args) -> R {
          return internal::InvokeHelper<R>(reinterpret_cast<F*>(data),
                                           ::std::forward<Args>(args)...);
        }) {}

  FnRef(const FnRef&) noexcept = default;
  FnRef& operator=(const FnRef&) noexcept = default;

  R operator()(Args... args) const {
    return invoker_(data_, ::std::forward<Args>(args)...);
  }

  internal::FnRefPayload payload() const noexcept {
    return internal::FnRefPayload{
        data_,
        reinterpret_cast<void (*)()>(invoker_),
    };
  }

  void* data() const noexcept { return data_; }
  auto invoker() const noexcept { return invoker_; }

 private:
  template <typename Sig, typename R_, typename... Args_>
  friend FnRef<Sig> internal::MakeFnRef(void* data,
                                        R_ (*invoker)(void*,
                                                      Args_...)) noexcept;

  FnRef(internal::PrivateTag, void* data, R (*invoker)(void*, Args...)) noexcept
      : data_(data), invoker_(invoker) {}

  void* data_ = nullptr;
  R (*invoker_)(void*, Args...) = nullptr;
};

// Const FnRef: matches `&dyn Fn(...)`
template <typename R, typename... Args>
class FnRef<R(Args...) const> {
 public:
  constexpr FnRef() noexcept = delete;

  template <typename F>
    requires(!::std::is_same_v<::std::remove_cvref_t<F>, FnRef> &&
             ::std::is_invocable_r_v<R, const F&, Args...>)
  FnRef(const F& f) noexcept
      : data_(const_cast<void*>(
            reinterpret_cast<const void*>(::std::addressof(f)))),
        // NOTE: the parameter is `void*`, not `const void*`, so that this
        // function's CFI type id matches the signature that the generated Rust
        // thunk transmutes to (`extern "C" fn(*mut c_void, ...)`). Clang's
        // `-fsanitize-cfi-icall-generalize-pointers` preserves the pointee's
        // cv-qualifiers, so `const void*` and `void*` are *not*
        // interchangeable for CFI purposes.
        invoker_([](void* data, Args... args) -> R {
          return internal::InvokeHelper<R>(
              *reinterpret_cast<const F*>(const_cast<const void*>(data)),
              ::std::forward<Args>(args)...);
        }) {}

  template <typename F>
    requires(::std::is_function_v<F> &&
             ::std::is_invocable_r_v<R, const F&, Args...>)
  FnRef(F* f) noexcept
      : data_(reinterpret_cast<void*>(f)),
        invoker_([](void* data, Args... args) -> R {
          return internal::InvokeHelper<R>(reinterpret_cast<F*>(data),
                                           ::std::forward<Args>(args)...);
        }) {}

  FnRef(const FnRef&) noexcept = default;
  FnRef& operator=(const FnRef&) noexcept = default;

  R operator()(Args... args) const {
    return invoker_(data_, ::std::forward<Args>(args)...);
  }

  internal::FnRefPayload payload() const noexcept {
    return internal::FnRefPayload{
        data_,
        reinterpret_cast<void (*)()>(invoker_),
    };
  }

  void* data() const noexcept { return data_; }
  auto invoker() const noexcept { return invoker_; }

 private:
  template <typename Sig, typename R_, typename... Args_>
  friend FnRef<Sig> internal::MakeFnRef(void* data,
                                        R_ (*invoker)(void*,
                                                      Args_...)) noexcept;

  FnRef(internal::PrivateTag, void* data, R (*invoker)(void*, Args...)) noexcept
      : data_(data), invoker_(invoker) {}

  void* data_ = nullptr;
  R (*invoker_)(void*, Args...) = nullptr;
};

}  // namespace rs
// NOLINTEND(google-explicit-constructor)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_REF_H_
