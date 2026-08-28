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

namespace rs_std {

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

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(Tuple<Ts...>& t) noexcept {
  return t.template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(const Tuple<Ts...>& t) noexcept {
  return t.template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(Tuple<Ts...>&& t) noexcept {
  return std::move(t).template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(const Tuple<Ts...>&& t) noexcept {
  return std::move(t).template get<I>();
}

}  // namespace rs_std

namespace std {

template <typename... Ts>
struct tuple_size<::rs_std::Tuple<Ts...>>
    : std::integral_constant<std::size_t, sizeof...(Ts)> {};

template <std::size_t I, typename... Ts>
struct tuple_element<I, ::rs_std::Tuple<Ts...>> {
  using type = std::tuple_element_t<I, std::tuple<Ts...>>;
};

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(::rs_std::Tuple<Ts...>& t) noexcept {
  return t.template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(const ::rs_std::Tuple<Ts...>& t) noexcept {
  return t.template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(::rs_std::Tuple<Ts...>&& t) noexcept {
  return std::move(t).template get<I>();
}

template <std::size_t I, typename... Ts>
constexpr decltype(auto) get(const ::rs_std::Tuple<Ts...>&& t) noexcept {
  return std::move(t).template get<I>();
}

}  // namespace std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TUPLE_H_
