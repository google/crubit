// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_REF_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_REF_H_

#include <functional>
#include <memory>
#include <type_traits>
#include <utility>

// NOLINTBEGIN(google-explicit-constructor)
namespace rs {

template <typename Sig>
class FnRef;

namespace internal {

struct FnRefPayload {
  void* data;
  void (*invoker)();
};

template <typename R, typename F, typename... Args>
R InvokeHelper(F&& f, Args&&... args) {
  if constexpr (std::is_void_v<R>) {
    std::invoke(std::forward<F>(f), std::forward<Args>(args)...);
  } else {
    return std::invoke(std::forward<F>(f), std::forward<Args>(args)...);
  }
}

}  // namespace internal

// Non-const FnRef: matches `&mut dyn FnMut(...)`
template <typename R, typename... Args>
class FnRef<R(Args...)> {
 public:
  constexpr FnRef() noexcept = delete;

  template <typename F>
    requires(!std::is_same_v<std::remove_cvref_t<F>, FnRef> &&
             std::is_invocable_r_v<R, F&, Args...>)
  FnRef(F&& f) noexcept
      : data_(const_cast<void*>(
            reinterpret_cast<const void*>(std::addressof(f)))),
        invoker_([](void* data, Args... args) -> R {
          return internal::InvokeHelper<R>(
              *reinterpret_cast<std::remove_reference_t<F>*>(data),
              std::forward<Args>(args)...);
        }) {}

  template <typename F>
    requires(std::is_function_v<F> && std::is_invocable_r_v<R, F&, Args...>)
  FnRef(F* f) noexcept
      : data_(reinterpret_cast<void*>(f)),
        invoker_([](void* data, Args... args) -> R {
          return internal::InvokeHelper<R>(reinterpret_cast<F*>(data),
                                           std::forward<Args>(args)...);
        }) {}

  FnRef(const FnRef&) noexcept = default;
  FnRef& operator=(const FnRef&) noexcept = default;

  R operator()(Args... args) const {
    return invoker_(data_, std::forward<Args>(args)...);
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
  void* data_ = nullptr;
  R (*invoker_)(void*, Args...) = nullptr;
};

// Const FnRef: matches `&dyn Fn(...)`
template <typename R, typename... Args>
class FnRef<R(Args...) const> {
 public:
  constexpr FnRef() noexcept = delete;

  template <typename F>
    requires(!std::is_same_v<std::remove_cvref_t<F>, FnRef> &&
             std::is_invocable_r_v<R, const F&, Args...>)
  FnRef(const F& f) noexcept
      : data_(const_cast<void*>(
            reinterpret_cast<const void*>(std::addressof(f)))),
        invoker_([](const void* data, Args... args) -> R {
          return internal::InvokeHelper<R>(*reinterpret_cast<const F*>(data),
                                           std::forward<Args>(args)...);
        }) {}

  template <typename F>
    requires(std::is_function_v<F> &&
             std::is_invocable_r_v<R, const F&, Args...>)
  FnRef(F* f) noexcept
      : data_(reinterpret_cast<void*>(f)),
        invoker_([](const void* data, Args... args) -> R {
          return internal::InvokeHelper<R>(
              reinterpret_cast<F*>(const_cast<void*>(data)),
              std::forward<Args>(args)...);
        }) {}

  FnRef(const FnRef&) noexcept = default;
  FnRef& operator=(const FnRef&) noexcept = default;

  R operator()(Args... args) const {
    return invoker_(data_, std::forward<Args>(args)...);
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
  void* data_ = nullptr;
  R (*invoker_)(const void*, Args...) = nullptr;
};

}  // namespace rs
// NOLINTEND(google-explicit-constructor)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_REF_H_
