// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/traits.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TRAITS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TRAITS_H_

#include <type_traits>

#include "support/internal/cpp_config.h"

namespace rs::core::marker {
struct Unpin;
}  // namespace rs::core::marker

#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
#define CRUBIT_RS_STD_PRIMARY_NS rs
#else
#define CRUBIT_RS_STD_PRIMARY_NS rs_std
#endif

namespace CRUBIT_RS_STD_PRIMARY_NS {

// `impl<Self, Trait>` is a template that is specialized when `Trait` is
// implemented for `Self`. Trait associated items are only available on
// specializations, so that users can only reference trait items for
// implementations that exist.
template <typename Self, typename Trait>
struct impl {
  static constexpr bool kIsImplemented = false;
};

// By default, Rust types are `Unpin`. Types that are not `Unpin` explicitly
// specialize `impl<Self, ::rs::core::marker::Unpin>` with `kIsImplemented =
// false`.
template <typename Self>
struct impl<Self, ::rs::core::marker::Unpin> {
  static constexpr bool kIsImplemented = true;
};

// `where<T, Trait1, Trait2, ...>` is a type trait that is true if
// `Trait1, Trait2, ...` are implemented for `T`.
template <typename T, typename... Traits>
struct where
    : ::std::conjunction<
          ::std::integral_constant<bool, impl<T, Traits>::kIsImplemented>...> {
};

// `where_v` is a convenience alias for `where<T, Traits...>::value`.
template <typename T, typename... Traits>
constexpr bool where_v = where<T, Traits...>::value;

}  // namespace CRUBIT_RS_STD_PRIMARY_NS

#undef CRUBIT_RS_STD_PRIMARY_NS

namespace rs_std {
#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
template <typename Self, typename Trait>
using impl [[deprecated("Use rs::impl instead")]] = ::rs::impl<Self, Trait>;

template <typename T, typename... Traits>
using where [[deprecated("Use rs::where instead")]] = ::rs::where<T, Traits...>;

#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#endif

template <typename T, typename... Traits>
[[deprecated("Use rs::where_v instead")]]
inline constexpr bool where_v = ::rs::where_v<T, Traits...>;

#ifdef __clang__
#pragma clang diagnostic pop
#endif
#endif
}  // namespace rs_std

#ifndef CRUBIT_INTERNAL_RS_NAMESPACE
namespace rs {
template <typename Self, typename Trait>
using impl = ::rs_std::impl<Self, Trait>;

template <typename T, typename... Traits>
using where = ::rs_std::where<T, Traits...>;

template <typename T, typename... Traits>
inline constexpr bool where_v = ::rs_std::where_v<T, Traits...>;
}  // namespace rs
#endif

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_TRAITS_H_
