// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/result.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_RESULT_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_RESULT_H_

#include <initializer_list>
#include <memory>
#include <type_traits>
#include <utility>

#include "support/internal/check.h"
#include "support/internal/move_assign.h"

namespace rs_std {

struct unexpect_t {
  explicit unexpect_t() = default;
};
inline constexpr unexpect_t unexpect{};

template <typename E>
class unexpected final {
 public:
  constexpr unexpected(const unexpected& err) = default;
  constexpr unexpected(unexpected&& err) = default;

  constexpr unexpected& operator=(const unexpected& err) = default;
  constexpr unexpected& operator=(unexpected&& err) = default;

  constexpr ~unexpected() = default;

  template <typename G = E>
    requires(std::is_constructible_v<E, G>)
  constexpr explicit unexpected(G&& err) : err_(std::forward<G>(err)) {}

  template <typename... Args>
    requires(std::is_constructible_v<E, Args...>)
  constexpr explicit unexpected(std::in_place_t, Args&&... args)
      : err_(std::forward<Args>(args)...) {}

  template <class U, class... Args>
    requires(std::is_constructible_v<E, std::initializer_list<U>&, Args...>)
  constexpr explicit unexpected(std::in_place_t, std::initializer_list<U> il,
                                Args&&... args)
      : err_(il, std::forward<Args>(args)...) {}

  constexpr const E& error() const& noexcept { return err_; }
  constexpr E& error() & noexcept { return err_; }
  constexpr const E&& error() const&& noexcept { return std::move(err_); }
  constexpr E&& error() && noexcept { return std::move(err_); }

 private:
  E err_;
};

template <typename T>
struct is_unexpected : std::false_type {};
template <typename E>
struct is_unexpected<unexpected<E>> : std::true_type {};
template <typename T>
inline constexpr bool is_unexpected_v = is_unexpected<T>::value;

template <typename Derived, typename T, typename E>
class ResultBase {
 public:
  using OkType = T;
  using ErrType = E;

  ResultBase() = default;

  template <typename U = T>
    requires(!std::is_base_of_v<ResultBase, std::decay_t<U>> &&
             !is_unexpected_v<std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
             !std::is_same_v<std::decay_t<U>, std::in_place_t> &&
             std::is_constructible_v<T, U>)
  explicit constexpr ResultBase(U&& ok) noexcept {
    derived().set_ok_tag();
    std::construct_at(derived().ok_ptr(), std::forward<U>(ok));
  }

  template <typename U = T>
    requires(!std::is_base_of_v<ResultBase, std::decay_t<U>> &&
             !is_unexpected_v<std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, rs_std::unexpect_t> &&
             std::is_constructible_v<T, U>)
  constexpr ResultBase& operator=(U&& ok) noexcept {
    // If we're moving an Ok value into a Result hat currently holds an Err,
    // we can't move assign the underlying ok_ptr as normal. We need to destroy
    // the value at the err_ptr, and then construct our ok value at ok_ptr.
    // This can't use crubit::MoveAssignOrDestroyAndConstruct because the ok and
    // err pointers are different types (and possibly different pointers).
    if (!has_value()) {
      std::destroy_at(derived().err_ptr());
      derived().set_ok_tag();
      std::construct_at(derived().ok_ptr(), std::forward<U>(ok));
      return *this;
    }
    derived().set_ok_tag();
    crubit::MoveAssignOrDestroyAndConstruct(derived().ok_ptr(),
                                            std::forward<U>(ok));
    return *this;
  }

  template <typename F = E>
    requires(std::is_constructible_v<E, F>)
  explicit constexpr ResultBase(rs_std::unexpected<F>&& err) noexcept {
    derived().set_err_tag();
    std::construct_at(derived().err_ptr(), std::move(err.error()));
  }

  template <typename F = E>
    requires(std::is_constructible_v<E, F>)
  constexpr ResultBase& operator=(rs_std::unexpected<F>&& err) noexcept {
    if (has_value()) {
      std::destroy_at(derived().ok_ptr());
      derived().set_err_tag();
      std::construct_at(derived().err_ptr(), std::move(err.error()));
      return *this;
    }
    derived().set_err_tag();
    crubit::MoveAssignOrDestroyAndConstruct(derived().err_ptr(),
                                            std::move(err.error()));
    return *this;
  }

  template <typename... Args>
  explicit constexpr ResultBase(std::in_place_t, Args&&... args) noexcept {
    derived().set_ok_tag();
    std::construct_at(derived().ok_ptr(), std::forward<Args>(args)...);
  }

  template <typename... Args>
  explicit constexpr ResultBase(rs_std::unexpect_t, Args&&... args) noexcept {
    derived().set_err_tag();
    std::construct_at(derived().err_ptr(), std::forward<Args>(args)...);
  }

  explicit constexpr operator bool() const noexcept { return has_value(); }

  /// Returns true if this Result contains an Ok variant (or value in C++
  /// terms), and false if it contains an Err variant (or unexpected in C++
  /// terms).
  constexpr bool has_value() const noexcept {
    return derived().has_value_impl();
  }

  constexpr void CheckHasOk() const {
    CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Result";
  }
  constexpr void CheckHasErr() const {
    CRUBIT_CHECK(!has_value()) << "Bad error access on rs_std::Result";
  }

  constexpr T& value() & {
    CheckHasOk();
    return *derived().ok_ptr();
  }
  constexpr const T& value() const& {
    CheckHasOk();
    return *derived().ok_const_ptr();
  }
  constexpr T&& value() && {
    CheckHasOk();
    return std::move(*derived().ok_ptr());
  }
  constexpr const T&& value() const&& {
    CheckHasOk();
    return std::move(*derived().ok_const_ptr());
  }

  constexpr E& error() & {
    CheckHasErr();
    return *derived().err_ptr();
  }
  constexpr const E& error() const& {
    CheckHasErr();
    return *derived().err_const_ptr();
  }
  constexpr E&& error() && {
    CheckHasErr();
    return std::move(*derived().err_ptr());
  }
  constexpr const E&& error() const&& {
    CheckHasErr();
    return std::move(*derived().err_const_ptr());
  }

  constexpr E& err() & { return error(); }
  constexpr const E& err() const& { return error(); }
  constexpr E&& err() && { return std::move(error()); }
  constexpr const E&& err() const&& { return std::move(error()); }

  constexpr T& operator*() & { return value(); }
  constexpr const T& operator*() const& {
    CheckHasOk();
    return *derived().ok_const_ptr();
  }
  constexpr T&& operator*() && { return std::move(value()); }
  constexpr const T&& operator*() const&& { return std::move(value()); }

  constexpr T* operator->() {
    CheckHasOk();
    return derived().ok_ptr();
  }
  constexpr const T* operator->() const {
    CheckHasOk();
    return derived().ok_const_ptr();
  }

 protected:
  constexpr void Reset() noexcept {
    if (has_value()) {
      std::destroy_at(derived().ok_ptr());
    } else {
      std::destroy_at(derived().err_ptr());
    }
  }

 private:
  Derived& derived() { return *static_cast<Derived*>(this); }
  const Derived& derived() const { return *static_cast<const Derived*>(this); }
};

template <typename T, typename E>
struct Result final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_RESULT_H_
