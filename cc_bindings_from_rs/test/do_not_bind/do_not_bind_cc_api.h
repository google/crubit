// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// do_not_bind_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_DO_NOT_BIND_DO_NOT_BIND_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_DO_NOT_BIND_DO_NOT_BIND_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace do_not_bind {

struct CRUBIT_INTERNAL_RUST_TYPE(":: do_not_bind_golden :: Struct") alignas(4)
    [[clang::trivial_abi]] Struct final {
 public:
  ::std::int32_t bound_inherent_method() const;

  ::std::int32_t value{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: do_not_bind_golden :: Trait") Trait {
  template <typename T>
  using impl = rs_std::impl<T, Trait>;
};

::std::int32_t bound_free_fn();

}  // namespace do_not_bind

template <>
struct rs_std::impl<::do_not_bind::Struct, ::do_not_bind::Trait> {
  static constexpr bool kIsImplemented = true;

  static ::std::int32_t bound_trait_method(::do_not_bind::Struct const& self);
};

namespace do_not_bind {

static_assert(
    sizeof(Struct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Struct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Struct>);
static_assert(::std::is_trivially_move_constructible_v<::do_not_bind::Struct>);
static_assert(::std::is_trivially_move_assignable_v<::do_not_bind::Struct>);
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_bound_uinherent_umethod(
    ::do_not_bind::Struct const&);
}
inline ::std::int32_t Struct::bound_inherent_method() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_bound_uinherent_umethod(self);
}
inline void Struct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Struct, value));
}
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_bound_ufree_ufn();
}
inline ::std::int32_t bound_free_fn() {
  return __crubit_internal::__crubit_thunk_bound_ufree_ufn();
}

}  // namespace do_not_bind

namespace do_not_bind {
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_Trait_ubound_utrait_umethod_udo_unot_ubind_ugolden_x0000003a_x0000003aStruct(
    ::do_not_bind::Struct const&);
}
}  // namespace do_not_bind
inline ::std::int32_t
rs_std::impl<::do_not_bind::Struct, ::do_not_bind::Trait>::bound_trait_method(
    ::do_not_bind::Struct const& self) {
  return do_not_bind::__crubit_internal::
      __crubit_thunk_Trait_ubound_utrait_umethod_udo_unot_ubind_ugolden_x0000003a_x0000003aStruct(
          self);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_DO_NOT_BIND_DO_NOT_BIND_GOLDEN
