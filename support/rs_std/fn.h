// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_H_

#include <cstddef>
#include <functional>
#include <type_traits>
#include <utility>

// NOLINTBEGIN(google-explicit-constructor)
namespace rs {

template <typename Sig>
class Fn;

namespace internal {

struct FnPayload {
  void* data;
  void (*invoker)();
  void (*destroyer)(void*);
};

inline void NoOpDestroyer(void*) noexcept {}

template <typename R, typename F, typename... Args>
R FnInvokeHelper(F&& f, Args&&... args) {
  if constexpr (std::is_void_v<R>) {
    std::invoke(std::forward<F>(f), std::forward<Args>(args)...);
  } else {
    return std::invoke(std::forward<F>(f), std::forward<Args>(args)...);
  }
}

}  // namespace internal

// Non-const Fn: matches `Box<dyn FnMut(...)>`
template <typename R, typename... Args>
class Fn<R(Args...)> {
 public:
  constexpr Fn() noexcept = default;
  constexpr Fn(std::nullptr_t) noexcept {}

  template <typename F>
    requires(!std::is_same_v<std::remove_cvref_t<F>, Fn> &&
             !std::is_same_v<std::remove_cvref_t<F>, std::nullptr_t> &&
             std::is_invocable_r_v<R, std::decay_t<F>&, Args...>)
  Fn(F&& f) {
    using DecayedF = std::decay_t<F>;
    data_ = new DecayedF(std::forward<F>(f));
    invoker_ = [](void* data, Args... args) -> R {
      return internal::FnInvokeHelper<R>(*reinterpret_cast<DecayedF*>(data),
                                         std::forward<Args>(args)...);
    };
    destroyer_ = [](void* data) noexcept {
      delete reinterpret_cast<DecayedF*>(data);
    };
  }

  ~Fn() {
    if (data_ && destroyer_) {
      destroyer_(data_);
    }
  }

  Fn(const Fn&) = delete;
  Fn& operator=(const Fn&) = delete;

  Fn(Fn&& other) noexcept
      : data_(std::exchange(other.data_, nullptr)),
        invoker_(std::exchange(other.invoker_, nullptr)),
        destroyer_(std::exchange(other.destroyer_, nullptr)) {}

  Fn& operator=(Fn&& other) noexcept {
    if (this != &other) {
      if (data_ && destroyer_) {
        destroyer_(data_);
      }
      data_ = std::exchange(other.data_, nullptr);
      invoker_ = std::exchange(other.invoker_, nullptr);
      destroyer_ = std::exchange(other.destroyer_, nullptr);
    }
    return *this;
  }

  explicit operator bool() const noexcept { return data_ != nullptr; }

  R operator()(Args... args) {
    return invoker_(data_, std::forward<Args>(args)...);
  }

  internal::FnPayload release_payload() && noexcept {
    internal::FnPayload p{
        data_,
        reinterpret_cast<void (*)()>(invoker_),
        destroyer_ ? destroyer_ : internal::NoOpDestroyer,
    };
    data_ = nullptr;
    invoker_ = nullptr;
    destroyer_ = nullptr;
    return p;
  }

 private:
  void* data_ = nullptr;
  R (*invoker_)(void*, Args...) = nullptr;
  void (*destroyer_)(void*) noexcept = nullptr;
};

// Const Fn: matches `Box<dyn Fn(...)>`
template <typename R, typename... Args>
class Fn<R(Args...) const> {
 public:
  constexpr Fn() noexcept = default;
  constexpr Fn(std::nullptr_t) noexcept {}

  template <typename F>
    requires(!std::is_same_v<std::remove_cvref_t<F>, Fn> &&
             !std::is_same_v<std::remove_cvref_t<F>, std::nullptr_t> &&
             std::is_invocable_r_v<R, const std::decay_t<F>&, Args...>)
  Fn(F&& f) {
    using DecayedF = std::decay_t<F>;
    data_ = new DecayedF(std::forward<F>(f));
    invoker_ = [](const void* data, Args... args) -> R {
      return internal::FnInvokeHelper<R>(
          *reinterpret_cast<const DecayedF*>(data),
          std::forward<Args>(args)...);
    };
    destroyer_ = [](void* data) noexcept {
      delete reinterpret_cast<DecayedF*>(data);
    };
  }

  ~Fn() {
    if (data_ && destroyer_) {
      destroyer_(data_);
    }
  }

  Fn(const Fn&) = delete;
  Fn& operator=(const Fn&) = delete;

  Fn(Fn&& other) noexcept
      : data_(std::exchange(other.data_, nullptr)),
        invoker_(std::exchange(other.invoker_, nullptr)),
        destroyer_(std::exchange(other.destroyer_, nullptr)) {}

  Fn& operator=(Fn&& other) noexcept {
    if (this != &other) {
      if (data_ && destroyer_) {
        destroyer_(data_);
      }
      data_ = std::exchange(other.data_, nullptr);
      invoker_ = std::exchange(other.invoker_, nullptr);
      destroyer_ = std::exchange(other.destroyer_, nullptr);
    }
    return *this;
  }

  explicit operator bool() const noexcept { return data_ != nullptr; }

  R operator()(Args... args) const {
    return invoker_(data_, std::forward<Args>(args)...);
  }

  internal::FnPayload release_payload() && noexcept {
    internal::FnPayload p{
        data_,
        reinterpret_cast<void (*)()>(invoker_),
        destroyer_ ? destroyer_ : internal::NoOpDestroyer,
    };
    data_ = nullptr;
    invoker_ = nullptr;
    destroyer_ = nullptr;
    return p;
  }

 private:
  void* data_ = nullptr;
  R (*invoker_)(const void*, Args...) = nullptr;
  void (*destroyer_)(void*) noexcept = nullptr;
};

// Rvalue-ref Fn: matches `Box<dyn FnOnce(...)>`
template <typename R, typename... Args>
class Fn<R(Args...) &&> {
 public:
  constexpr Fn() noexcept = default;
  constexpr Fn(std::nullptr_t) noexcept {}

  template <typename F>
    requires(!std::is_same_v<std::remove_cvref_t<F>, Fn> &&
             !std::is_same_v<std::remove_cvref_t<F>, std::nullptr_t> &&
             std::is_invocable_r_v<R, std::decay_t<F> &&, Args...>)
  Fn(F&& f) {
    using DecayedF = std::decay_t<F>;
    data_ = new DecayedF(std::forward<F>(f));
    invoker_ = [](void* data, Args... args) -> R {
      auto* typed_data = reinterpret_cast<DecayedF*>(data);
      if constexpr (std::is_void_v<R>) {
        std::invoke(std::move(*typed_data), std::forward<Args>(args)...);
      } else {
        return std::invoke(std::move(*typed_data), std::forward<Args>(args)...);
      }
    };
    destroyer_ = [](void* data) noexcept {
      delete reinterpret_cast<DecayedF*>(data);
    };
  }

  ~Fn() {
    if (data_ && destroyer_) {
      destroyer_(data_);
    }
  }

  Fn(const Fn&) = delete;
  Fn& operator=(const Fn&) = delete;

  Fn(Fn&& other) noexcept
      : data_(std::exchange(other.data_, nullptr)),
        invoker_(std::exchange(other.invoker_, nullptr)),
        destroyer_(std::exchange(other.destroyer_, nullptr)) {}

  Fn& operator=(Fn&& other) noexcept {
    if (this != &other) {
      if (data_ && destroyer_) {
        destroyer_(data_);
      }
      data_ = std::exchange(other.data_, nullptr);
      invoker_ = std::exchange(other.invoker_, nullptr);
      destroyer_ = std::exchange(other.destroyer_, nullptr);
    }
    return *this;
  }

  explicit operator bool() const noexcept { return data_ != nullptr; }

  R operator()(Args... args) && {
    void* d = data_;
    auto inv = invoker_;
    auto dest = destroyer_;
    data_ = nullptr;
    invoker_ = nullptr;
    destroyer_ = nullptr;
    struct Cleanup {
      void* d;
      void (*dest)(void*) noexcept;
      ~Cleanup() {
        if (d && dest) dest(d);
      }
    } cleanup{d, dest};
    return inv(d, std::forward<Args>(args)...);
  }

  internal::FnPayload release_payload() && noexcept {
    internal::FnPayload p{
        data_,
        reinterpret_cast<void (*)()>(invoker_),
        destroyer_ ? destroyer_ : internal::NoOpDestroyer,
    };
    data_ = nullptr;
    invoker_ = nullptr;
    destroyer_ = nullptr;
    return p;
  }

 private:
  void* data_ = nullptr;
  R (*invoker_)(void*, Args...) = nullptr;
  void (*destroyer_)(void*) noexcept = nullptr;
};

}  // namespace rs
// NOLINTEND(google-explicit-constructor)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_FN_H_
