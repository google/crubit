// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TRAITS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TRAITS_H_

#include <type_traits>

namespace rs_std {

template <typename Self, typename Trait>
struct impl {
  static constexpr bool kIsImplemented = false;
};

template <typename T, typename... Traits>
struct where
    : std::conjunction<
          std::integral_constant<bool, impl<T, Traits>::kIsImplemented>...> {};

template <typename T, typename... Traits>
constexpr bool where_v = where<T, Traits...>::value;

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TRAITS_H_
