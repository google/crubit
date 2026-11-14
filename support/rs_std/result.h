// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs/result.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_RESULT_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_RESULT_H_

#include <initializer_list>
#include <memory>
#include <type_traits>
#include <utility>

#include "support/internal/check.h"
#include "support/internal/cpp_config.h"
#include "support/internal/move_assign.h"
#include "support/rs_std/unit.h"

#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
#define CRUBIT_RS_STD_PRIMARY_NS rs
#else
#define CRUBIT_RS_STD_PRIMARY_NS rs_std
#endif

namespace CRUBIT_RS_STD_PRIMARY_NS {

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
    requires(::std::is_constructible_v<E, G>)
  constexpr explicit unexpected(G&& err) : err_(::std::forward<G>(err)) {}

  template <typename... Args>
    requires(::std::is_constructible_v<E, Args...>)
  constexpr explicit unexpected(::std::in_place_t, Args&&... args)
      : err_(::std::forward<Args>(args)...) {}

  template <class U, class... Args>
    requires(::std::is_constructible_v<E, ::std::initializer_list<U>&, Args...>)
  constexpr explicit unexpected(::std::in_place_t,
                                ::std::initializer_list<U> il, Args&&... args)
      : err_(il, ::std::forward<Args>(args)...) {}

  constexpr const E& error() const& noexcept { return err_; }
  constexpr E& error() & noexcept { return err_; }
  constexpr const E&& error() const&& noexcept { return ::std::move(err_); }
  constexpr E&& error() && noexcept { return ::std::move(err_); }

 private:
  E err_;
};

template <typename T>
struct is_unexpected : ::std::false_type {};
template <typename E>
struct is_unexpected<unexpected<E>> : ::std::true_type {};
template <typename T>
inline constexpr bool is_unexpected_v = is_unexpected<T>::value;

template <typename ResultType, typename T, typename U>
inline constexpr bool ResultForwardConstructible =
    ::std::negation_v<::std::disjunction<
        ::std::is_base_of<ResultType, ::std::decay_t<U>>,
        ::CRUBIT_RS_STD_PRIMARY_NS::is_unexpected<::std::decay_t<U>>,
        ::std::is_same<::std::decay_t<U>,
                       ::CRUBIT_RS_STD_PRIMARY_NS::unexpect_t>,
        ::std::is_same<::std::decay_t<U>, ::std::in_place_t>,
        ::std::negation<::std::is_constructible<T, U>>>>;

template <typename ErrType, typename F>
inline constexpr bool ResultUnexpectedConstructible =
    ::std::is_constructible_v<ErrType, F>;

template <typename Derived, typename T, typename E>
class ResultBase {
 public:
  using OkType = T;
  using ErrType = E;

  ResultBase() = default;

  template <typename U = T>
    requires(ResultForwardConstructible<ResultBase, T, U>)
  explicit constexpr ResultBase(U&& ok) noexcept {
    derived().set_ok_tag();
    ::std::construct_at(derived().ok_ptr(), ::std::forward<U>(ok));
  }

  template <typename U = T>
    requires(ResultForwardConstructible<ResultBase, T, U>)
  constexpr ResultBase& operator=(U&& ok) noexcept {
    // If we're moving an Ok value into a Result hat currently holds an Err,
    // we can't move assign the underlying ok_ptr as normal. We need to destroy
    // the value at the err_ptr, and then construct our ok value at ok_ptr.
    // This can't use crubit::MoveAssignOrDestroyAndConstruct because the ok and
    // err pointers are different types (and possibly different pointers).
    if (!has_value()) {
      ::std::destroy_at(derived().err_ptr());
      derived().set_ok_tag();
      ::std::construct_at(derived().ok_ptr(), ::std::forward<U>(ok));
      return *this;
    }
    derived().set_ok_tag();
    ::crubit::MoveAssignOrDestroyAndConstruct(derived().ok_ptr(),
                                              ::std::forward<U>(ok));
    return *this;
  }

  template <typename F = E>
    requires(ResultUnexpectedConstructible<E, F>)
  explicit constexpr ResultBase(
      ::CRUBIT_RS_STD_PRIMARY_NS::unexpected<F>&& err) noexcept {
    derived().set_err_tag();
    ::std::construct_at(derived().err_ptr(), ::std::move(err.error()));
  }

  template <typename F = E>
    requires(ResultUnexpectedConstructible<E, F>)
  constexpr ResultBase& operator=(
      ::CRUBIT_RS_STD_PRIMARY_NS::unexpected<F>&& err) noexcept {
    if (has_value()) {
      ::std::destroy_at(derived().ok_ptr());
      derived().set_err_tag();
      ::std::construct_at(derived().err_ptr(), ::std::move(err.error()));
      return *this;
    }
    derived().set_err_tag();
    ::crubit::MoveAssignOrDestroyAndConstruct(derived().err_ptr(),
                                              ::std::move(err.error()));
    return *this;
  }

  template <typename... Args>
  explicit constexpr ResultBase(::std::in_place_t, Args&&... args) noexcept {
    derived().set_ok_tag();
    ::std::construct_at(derived().ok_ptr(), ::std::forward<Args>(args)...);
  }

  template <typename... Args>
  explicit constexpr ResultBase(::CRUBIT_RS_STD_PRIMARY_NS::unexpect_t,
                                Args&&... args) noexcept {
    derived().set_err_tag();
    ::std::construct_at(derived().err_ptr(), ::std::forward<Args>(args)...);
  }

  explicit constexpr operator bool() const noexcept { return has_value(); }

  /// Returns true if this Result contains an Ok variant (or value in C++
  /// terms), and false if it contains an Err variant (or unexpected in C++
  /// terms).
  constexpr bool has_value() const noexcept {
    return derived().has_value_impl();
  }

  constexpr void CheckHasOk() const {
    CRUBIT_CHECK(has_value()) << "Bad value access on rs::Result";
  }
  constexpr void CheckHasErr() const {
    CRUBIT_CHECK(!has_value()) << "Bad error access on rs::Result";
  }

  constexpr T& value() &
    requires(!::rs::is_unit_t_v<T>)
  {
    CheckHasOk();
    return *derived().ok_ptr();
  }
  constexpr const T& value() const&
    requires(!::rs::is_unit_t_v<T>)
  {
    CheckHasOk();
    return *derived().ok_const_ptr();
  }
  constexpr T&& value() &&
    requires(!::rs::is_unit_t_v<T>)
  {
    CheckHasOk();
    return ::std::move(*derived().ok_ptr());
  }
  constexpr const T&& value() const&&
    requires(!::rs::is_unit_t_v<T>)
  {
    CheckHasOk();
    return ::std::move(*derived().ok_const_ptr());
  }

  constexpr E& error() &
    requires(!::rs::is_unit_t_v<E>)
  {
    CheckHasErr();
    return *derived().err_ptr();
  }
  constexpr const E& error() const&
    requires(!::rs::is_unit_t_v<E>)
  {
    CheckHasErr();
    return *derived().err_const_ptr();
  }
  constexpr E&& error() &&
    requires(!::rs::is_unit_t_v<E>)
  {
    CheckHasErr();
    return ::std::move(*derived().err_ptr());
  }
  constexpr const E&& error() const&&
    requires(!::rs::is_unit_t_v<E>)
  {
    CheckHasErr();
    return ::std::move(*derived().err_const_ptr());
  }

  constexpr E& err() &
    requires(!::rs::is_unit_t_v<E>)
  {
    return error();
  }
  constexpr const E& err() const&
    requires(!::rs::is_unit_t_v<E>)
  {
    return error();
  }
  constexpr E&& err() &&
    requires(!::rs::is_unit_t_v<E>)
  {
    return ::std::move(error());
  }
  constexpr const E&& err() const&&
    requires(!::rs::is_unit_t_v<E>)
  {
    return ::std::move(error());
  }

  constexpr T& operator*() &
    requires(!::rs::is_unit_t_v<T>)
  {
    return value();
  }
  constexpr const T& operator*() const&
    requires(!::rs::is_unit_t_v<T>)
  {
    return value();
  }
  constexpr T&& operator*() &&
    requires(!::rs::is_unit_t_v<T>)
  {
    return ::std::move(value());
  }
  constexpr const T&& operator*() const&&
    requires(!::rs::is_unit_t_v<T>)
  {
    return ::std::move(value());
  }

  constexpr T* operator->()
    requires(!::rs::is_unit_t_v<T>)
  {
    CheckHasOk();
    return derived().ok_ptr();
  }
  constexpr const T* operator->() const
    requires(!::rs::is_unit_t_v<T>)
  {
    CheckHasOk();
    return derived().ok_const_ptr();
  }

 protected:
  constexpr void Reset() noexcept {
    if (has_value()) {
      ::std::destroy_at(derived().ok_ptr());
    } else {
      ::std::destroy_at(derived().err_ptr());
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

}  // namespace CRUBIT_RS_STD_PRIMARY_NS

#undef CRUBIT_RS_STD_PRIMARY_NS

namespace rs_std {
#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
using unexpect_t [[deprecated("Use rs::unexpect_t instead")]] =
    ::rs::unexpect_t;
inline constexpr unexpect_t unexpect{};
template <typename E>
using unexpected [[deprecated("Use rs::unexpected instead")]] =
    ::rs::unexpected<E>;
template <typename T>
using is_unexpected [[deprecated("Use rs::is_unexpected instead")]] =
    ::rs::is_unexpected<T>;
template <typename T>
inline constexpr bool is_unexpected_v = ::rs::is_unexpected_v<T>;
template <typename ResultType, typename T, typename U>
inline constexpr bool ResultForwardConstructible =
    ::rs::ResultForwardConstructible<ResultType, T, U>;
template <typename ErrType, typename F>
inline constexpr bool ResultUnexpectedConstructible =
    ::rs::ResultUnexpectedConstructible<ErrType, F>;
template <typename Derived, typename T, typename E>
using ResultBase [[deprecated("Use rs::ResultBase instead")]] =
    ::rs::ResultBase<Derived, T, E>;
template <typename T, typename E>
using Result [[deprecated("Use rs::Result instead")]] = ::rs::Result<T, E>;
#endif
}  // namespace rs_std

#ifndef CRUBIT_INTERNAL_RS_NAMESPACE
namespace rs {
using unexpect_t = ::rs_std::unexpect_t;
inline constexpr unexpect_t unexpect{};
template <typename E>
using unexpected = ::rs_std::unexpected<E>;
template <typename T>
using is_unexpected = ::rs_std::is_unexpected<T>;
template <typename T>
inline constexpr bool is_unexpected_v = ::rs_std::is_unexpected_v<T>;
template <typename ResultType, typename T, typename U>
inline constexpr bool ResultForwardConstructible =
    ::rs_std::ResultForwardConstructible<ResultType, T, U>;
template <typename ErrType, typename F>
inline constexpr bool ResultUnexpectedConstructible =
    ::rs_std::ResultUnexpectedConstructible<ErrType, F>;
template <typename Derived, typename T, typename E>
using ResultBase = ::rs_std::ResultBase<Derived, T, E>;
template <typename T, typename E>
using Result = ::rs_std::Result<T, E>;
}  // namespace rs
#endif

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_RESULT_H_
