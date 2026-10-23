// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/tuple.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_

#include <cstddef>
#include <tuple>
#include <type_traits>
#include <utility>

#include "support/internal/cpp_config.h"

#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
#define CRUBIT_RS_STD_PRIMARY_NS rs
#else
#define CRUBIT_RS_STD_PRIMARY_NS rs_std
#endif

namespace CRUBIT_RS_STD_PRIMARY_NS {

/**
 * A layout-compatible type for Rust tuples.
 *
 * This type should only be used via a generated specialization.
 */
template <typename... Ts>
struct Tuple final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

template <::std::size_t I, typename... Ts>
constexpr decltype(auto) get(Tuple<Ts...>& t) noexcept {
  return t.template get<I>();
}

template <::std::size_t I, typename... Ts>
constexpr decltype(auto) get(const Tuple<Ts...>& t) noexcept {
  return t.template get<I>();
}

template <::std::size_t I, typename... Ts>
constexpr decltype(auto) get(Tuple<Ts...>&& t) noexcept {
  return ::std::move(t).template get<I>();
}

template <::std::size_t I, typename... Ts>
constexpr decltype(auto) get(const Tuple<Ts...>&& t) noexcept {
  return ::std::move(t).template get<I>();
}

}  // namespace CRUBIT_RS_STD_PRIMARY_NS

namespace rs_std {
#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
template <typename... Ts>
using Tuple [[deprecated("Use rs::Tuple instead")]] = ::rs::Tuple<Ts...>;
template <::std::size_t I, typename... Ts>
[[deprecated("Use rs::get instead")]] constexpr decltype(auto) get(
    ::rs::Tuple<Ts...>& t) noexcept {
  return ::rs::get<I>(t);
}
template <::std::size_t I, typename... Ts>
[[deprecated("Use rs::get instead")]] constexpr decltype(auto) get(
    const ::rs::Tuple<Ts...>& t) noexcept {
  return ::rs::get<I>(t);
}
template <::std::size_t I, typename... Ts>
[[deprecated("Use rs::get instead")]] constexpr decltype(auto) get(
    ::rs::Tuple<Ts...>&& t) noexcept {
  return ::rs::get<I>(::std::move(t));
}
template <::std::size_t I, typename... Ts>
[[deprecated("Use rs::get instead")]] constexpr decltype(auto) get(
    const ::rs::Tuple<Ts...>&& t) noexcept {
  return ::rs::get<I>(::std::move(t));
}
#endif
}  // namespace rs_std

#ifndef CRUBIT_INTERNAL_RS_NAMESPACE
namespace rs {
template <typename... Ts>
using Tuple = ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>;
template <::std::size_t I, typename... Ts>
constexpr decltype(auto) get(
    ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>& t) noexcept {
  return ::rs_std::get<I>(t);
}
template <::std::size_t I, typename... Ts>
constexpr decltype(auto) get(
    const ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>& t) noexcept {
  return ::rs_std::get<I>(t);
}
template <::std::size_t I, typename... Ts>
constexpr decltype(auto) get(
    ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>&& t) noexcept {
  return ::rs_std::get<I>(::std::move(t));
}
template <::std::size_t I, typename... Ts>
constexpr decltype(auto) get(
    const ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>&& t) noexcept {
  return ::rs_std::get<I>(::std::move(t));
}
}  // namespace rs
#endif

namespace std {

template <typename... Ts>
struct tuple_size<::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>>
    : std::integral_constant<std::size_t, sizeof...(Ts)> {};

template <std::size_t I, typename... Ts>
struct tuple_element<I, ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>> {
  using type = std::tuple_element_t<I, std::tuple<Ts...>>;
};

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(
    ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>& t) noexcept {
  return t.template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(
    const ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>& t) noexcept {
  return t.template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(
    ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>&& t) noexcept {
  return std::move(t).template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(
    const ::CRUBIT_RS_STD_PRIMARY_NS::Tuple<Ts...>&& t) noexcept {
  return std::move(t).template get<I>();
}

}  // namespace std

#undef CRUBIT_RS_STD_PRIMARY_NS

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_
