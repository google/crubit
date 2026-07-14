// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/option.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTION_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTION_H_

#include <optional>
#include <type_traits>
#include <utility>

#include "support/internal/check.h"
#include "support/internal/move_assign.h"
#include "support/internal/slot.h"

namespace rs_std {

template <typename Derived, typename T>
class OptionBase {
 private:
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
  explicit constexpr OptionBase(::std::nullopt_t) noexcept {
    derived().set_none_tag();
  }
  constexpr OptionBase& operator=(::std::nullopt_t) noexcept {
    reset();
    derived().set_none_tag();
    return *this;
  }

  template <typename U>
    requires(!std::is_base_of_v<OptionBase, std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, ::std::nullopt_t> &&
             !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
             std::is_constructible_v<T, U>)
  // Intentionally implicit to support seamless copy-initialization and
  // conversion from the wrapped value type T, mirroring std::optional<T>.
  // NOLINTNEXTLINE(google-explicit-constructor)
  OptionBase(U&& value) noexcept {
    derived().set_some_tag();
    std::construct_at(derived().some_ptr(), std::forward<U>(value));
  }

  template <typename U>
    requires(!std::is_base_of_v<OptionBase, std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, ::std::nullopt_t> &&
             !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
             std::is_constructible_v<T, U>)
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
    requires(std::is_same_v<std::decay_t<Opt>, ::std::optional<T>> &&
             !std::is_lvalue_reference_v<Opt>)
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
    requires(std::is_same_v<std::decay_t<Opt>, ::std::optional<T>> &&
             !std::is_lvalue_reference_v<Opt>)
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
  operator ::std::optional<T>() && noexcept {
    if (!has_value()) {
      return std::nullopt;
    }
    T* some = derived().some_ptr();
    if constexpr (requires(T x) {
                    T(::crubit::UnsafeRelocateTag{}, std::move(x));
                  }) {
      struct DeferSetTagNone {
        Derived* value;
        ~DeferSetTagNone() { value->set_none_tag(); }
      } defer(&derived());
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
  void check_has_value() const {
    CRUBIT_CHECK(has_value()) << "Bad value access on rs_std::Option";
  }

  T& operator*() & {
    check_has_value();
    return *derived().some_ptr();
  }
  const T& operator*() const& {
    check_has_value();
    return *derived().some_const_ptr();
  }
  T&& operator*() && {
    check_has_value();
    return std::move(*derived().some_ptr());
  }

  T* operator->() {
    check_has_value();
    return derived().some_ptr();
  }
  const T* operator->() const {
    check_has_value();
    return derived().some_const_ptr();
  }
};

template <typename T>
struct Option final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_OPTION_H_
