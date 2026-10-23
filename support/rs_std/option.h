// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/option.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTION_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTION_H_

#include <memory>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/internal/check.h"
#include "support/internal/move_assign.h"
#include "support/internal/slot.h"
#include "support/rs_std/unit.h"

namespace rs_std {

template <typename OptionType, typename T, typename U>
inline constexpr bool OptionForwardConstructible = std::negation_v<
    std::disjunction<std::is_base_of<OptionType, std::decay_t<U>>,
                     std::is_same<std::decay_t<U>, std::nullopt_t>,
                     std::is_same<std::decay_t<U>, std::in_place_t>,
                     std::negation<std::is_constructible<T, U>>>>;

template <typename ArgType, typename Opt>
inline constexpr bool OptionFromStdOptional = std::conjunction_v<
    std::is_same<std::decay_t<Opt>, ::std::optional<ArgType>>,
    std::negation<std::is_lvalue_reference<Opt>>>;

template <typename Derived, typename T>
class OptionBase {
 private:
  // Sets `option` to `None` when destroyed. This lets a `return` statement
  // relocate the value out of `option` before `option` is marked empty.
  struct SetNoneOnDestruction {
    Derived* option;
    ~SetNoneOnDestruction() { option->set_none_tag(); }
  };

  Derived& derived() { return *static_cast<Derived*>(this); }
  const Derived& derived() const { return *static_cast<const Derived*>(this); }

 protected:
  constexpr void reset() noexcept {
    if (has_value()) {
      ::std::destroy_at(derived().some_ptr());
    }
  }

 public:
  using value_type = T;

  constexpr OptionBase() noexcept { derived().set_none_tag(); }
  // Intentionally implicit to support `return std::nullopt;` and ternary
  // expressions (`cond ? opt : std::nullopt`), mirroring `std::optional<T>`.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr OptionBase(::std::nullopt_t) noexcept { derived().set_none_tag(); }
  constexpr OptionBase& operator=(::std::nullopt_t) noexcept {
    reset();
    derived().set_none_tag();
    return *this;
  }

  template <typename U>
    requires(OptionForwardConstructible<OptionBase, T, U>)
  // Intentionally implicit to support seamless copy-initialization and
  // conversion from the wrapped value type T, mirroring std::optional<T>.
  // NOLINTNEXTLINE(google-explicit-constructor)
  OptionBase(U&& value) noexcept {
    derived().set_some_tag();
    std::construct_at(derived().some_ptr(), std::forward<U>(value));
  }

  template <typename U>
    requires(OptionForwardConstructible<OptionBase, T, U>)
  OptionBase& operator=(U&& value) noexcept {
    if (has_value()) {
      ::crubit::MoveAssignOrDestroyAndConstruct(derived().some_ptr(),
                                                std::forward<U>(value));
    } else {
      derived().set_some_tag();
      ::std::construct_at(derived().some_ptr(), std::forward<U>(value));
    }
    return *this;
  }

  template <typename Opt>
    requires(OptionFromStdOptional<T, Opt>)
  // Intentionally implicit to support conversion from std::optional<T>.
  // NOLINTNEXTLINE(google-explicit-constructor)
  OptionBase(Opt&& value) noexcept {
    if (!value.has_value()) {
      derived().set_none_tag();
      return;
    }
    derived().set_some_tag();
    T* some = derived().some_ptr();
    if constexpr (requires(T x) {
                    T(::crubit::UnsafeRelocateTag{}, std::move(x));
                  }) {
      std::construct_at(some, ::crubit::UnsafeRelocateTag{},
                        ::std::move(*value));
      std::construct_at(&value, ::std::nullopt);
    } else if constexpr (std::is_move_constructible_v<T>) {
      std::construct_at(some, ::std::move(*value));
    } else {
      std::construct_at(some, *value);
    }
  }

  template <typename Opt>
    requires(OptionFromStdOptional<T, Opt>)
  OptionBase& operator=(Opt&& value) noexcept {
    reset();
    if (!value.has_value()) {
      derived().set_none_tag();
      return *this;
    }
    derived().set_some_tag();
    T* some = derived().some_ptr();
    if constexpr (requires(T x) {
                    T(::crubit::UnsafeRelocateTag{}, std::move(x));
                  }) {
      std::construct_at(some, ::crubit::UnsafeRelocateTag{},
                        ::std::move(*value));
      std::construct_at(&value, ::std::nullopt);
    } else if constexpr (std::is_move_constructible_v<T>) {
      std::construct_at(some, ::std::move(*value));
    } else {
      std::construct_at(some, *value);
    }
    return *this;
  }

  template <typename... Args>
  explicit OptionBase(::std::in_place_t, Args&&... args) noexcept {
    derived().set_some_tag();
    std::construct_at(derived().some_ptr(), ::std::forward<Args>(args)...);
  }

  // Intentionally implicit to support conversion to std::optional<T>.
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator ::std::optional<T>() && noexcept
    requires(!is_unit_t_v<T>)
  {
    if (!has_value()) {
      return std::nullopt;
    }
    T* some = derived().some_ptr();
    if constexpr (requires(T x) {
                    T(::crubit::UnsafeRelocateTag{}, std::move(x));
                  }) {
      SetNoneOnDestruction set_none{&derived()};
      return std::make_optional<T>(::crubit::UnsafeRelocateTag{},
                                   std::move(*some));
    } else {
      T& value = *some;
      std::optional<T> return_value(std::move(value));
      std::destroy_at(&value);
      derived().set_none_tag();
      return return_value;
    }
  }

  bool has_value() const noexcept { return !derived().is_none(); }
  explicit operator bool() const noexcept { return has_value(); }
  void check_has_value() const {
    CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Option";
  }

  T& operator*() &
    requires(!is_unit_t_v<T>)
  {
    check_has_value();
    return *derived().some_ptr();
  }
  const T& operator*() const&
    requires(!is_unit_t_v<T>)
  {
    check_has_value();
    return *derived().some_const_ptr();
  }
  T&& operator*() &&
    requires(!is_unit_t_v<T>)
  {
    check_has_value();
    return std::move(*derived().some_ptr());
  }

  T* operator->()
    requires(!is_unit_t_v<T>)
  {
    check_has_value();
    return derived().some_ptr();
  }
  const T* operator->() const
    requires(!is_unit_t_v<T>)
  {
    check_has_value();
    return derived().some_const_ptr();
  }

  // TODO(b/523234159): Remove temporary `std::optional` compatibility
  // `.value()` and `.value_or()` overloads once existing callers are migrated.
  //
  // Unlike `std::optional::value()`, which throws `std::bad_optional_access`,
  // `value()` on a `None` option CHECK-fails.
  T& value() &
    requires(!is_unit_t_v<T>)
  {
    return **this;
  }
  const T& value() const&
    requires(!is_unit_t_v<T>)
  {
    return **this;
  }
  T&& value() &&
    requires(!is_unit_t_v<T>)
  {
    return *std::move(*this);
  }

  template <typename U = std::remove_cv_t<T>>
    requires(!is_unit_t_v<T> && std::is_copy_constructible_v<T> &&
             std::is_convertible_v<U &&, T>)
  T value_or(U&& default_value) const& {
    return has_value() ? **this
                       : static_cast<T>(std::forward<U>(default_value));
  }

  // Unlike `std::optional::value_or() &&`, which leaves a moved-from value in
  // the optional, this leaves `*this` as `None`.
  template <typename U = std::remove_cv_t<T>>
    requires(!is_unit_t_v<T> &&
             (std::is_move_constructible_v<T> ||
              requires(T x) {
                T(::crubit::UnsafeRelocateTag{}, std::move(x));
              }) &&
             std::is_convertible_v<U &&, T>)
  T value_or(U&& default_value) && {
    if (!has_value()) {
      return static_cast<T>(std::forward<U>(default_value));
    }
    T* some = derived().some_ptr();
    if constexpr (requires(T x) {
                    T(::crubit::UnsafeRelocateTag{}, std::move(x));
                  }) {
      SetNoneOnDestruction set_none{&derived()};
      return T(::crubit::UnsafeRelocateTag{}, std::move(*some));
    } else {
      T return_value(std::move(*some));
      std::destroy_at(some);
      derived().set_none_tag();
      return return_value;
    }
  }
};

// TODO(b/523234159): Remove temporary `std::optional` compatibility
// `operator==` overloads once existing callers are migrated.
template <typename Derived, typename T>
  requires requires(const T& a, const T& b) {
    requires std::is_convertible_v<decltype(a == b), bool>;
  }
bool operator==(const OptionBase<Derived, T>& lhs,
                const OptionBase<Derived, T>& rhs) {
  if (lhs.has_value() != rhs.has_value()) {
    return false;
  }
  if constexpr (is_unit_t_v<T>) {
    return true;
  } else {
    return !lhs.has_value() || static_cast<bool>(*lhs == *rhs);
  }
}

template <typename Derived, typename T>
bool operator==(const OptionBase<Derived, T>& opt, ::std::nullopt_t) noexcept {
  return !opt.has_value();
}

template <typename Derived, typename T, typename U>
  requires(!is_unit_t_v<T> &&
           !std::is_base_of_v<OptionBase<Derived, T>, std::decay_t<U>> &&
           !std::is_same_v<std::decay_t<U>, ::std::nullopt_t> &&
           requires(const T& a, const U& b) {
             requires std::is_convertible_v<decltype(a == b), bool>;
           })
bool operator==(const OptionBase<Derived, T>& opt, const U& v) {
  return opt.has_value() && static_cast<bool>(*opt == v);
}

template <typename T>
struct Option final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

}  // namespace rs_std

namespace rs {

// Shorthand for `rs_std::Option<T>`, mirroring `Option`'s place in the Rust
// prelude.
template <typename T>
using Option = ::rs_std::Option<T>;

}  // namespace rs

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTION_H_
