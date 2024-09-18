// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate
// Features: experimental, supported

// clang-format off
#pragma once

#include "support/internal/attribute_macros.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace example_crate {

// Generated from:
// examples/rust/struct/example.rs;l=5
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate :: Struct") alignas(4)
    [[clang::trivial_abi]] Struct final {
 public:
  // `Struct` doesn't implement the `Default` trait
  Struct() = delete;

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~Struct() = default;
  Struct(Struct&&) = default;
  Struct& operator=(Struct&&) = default;

  // `Struct` doesn't implement the `Clone` trait
  Struct(const Struct&) = delete;
  Struct& operator=(const Struct&) = delete;

 public:
  union {
    // Generated from:
    // examples/rust/struct/example.rs;l=6
    std::int32_t a;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// examples/rust/struct/example.rs;l=11
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate :: StructWithDefault") alignas(4)
    [[clang::trivial_abi]] StructWithDefault final {
 public:
  // Default::default
  StructWithDefault();

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~StructWithDefault() = default;
  StructWithDefault(StructWithDefault&&) = default;
  StructWithDefault& operator=(StructWithDefault&&) = default;

  // `StructWithDefault` doesn't implement the `Clone` trait
  StructWithDefault(const StructWithDefault&) = delete;
  StructWithDefault& operator=(const StructWithDefault&) = delete;

 public:
  union {
    // Generated from:
    // examples/rust/struct/example.rs;l=12
    std::int32_t a;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// examples/rust/struct/example.rs;l=17
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate :: StructWithClone") alignas(
    4) [[clang::trivial_abi]] StructWithClone final {
 public:
  // Default::default
  StructWithClone();

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~StructWithClone() = default;
  StructWithClone(StructWithClone&&) = default;
  StructWithClone& operator=(StructWithClone&&) = default;

  // Clone::clone
  StructWithClone(const StructWithClone&);

  // Clone::clone_from
  StructWithClone& operator=(const StructWithClone&);

 public:
  union {
    // Generated from:
    // examples/rust/struct/example.rs;l=18
    std::int32_t a;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(Struct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Struct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Struct>);
static_assert(std::is_trivially_move_constructible_v<Struct>);
static_assert(std::is_trivially_move_assignable_v<Struct>);
inline void Struct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Struct, a));
}
static_assert(
    sizeof(StructWithDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate17StructWithDefaultNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
    ::example_crate::StructWithDefault* __ret_ptr);
}
inline StructWithDefault::StructWithDefault() {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate17StructWithDefaultNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
          this);
}
static_assert(std::is_trivially_destructible_v<StructWithDefault>);
static_assert(std::is_trivially_move_constructible_v<StructWithDefault>);
static_assert(std::is_trivially_move_assignable_v<StructWithDefault>);
inline void StructWithDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithDefault, a));
}
static_assert(
    sizeof(StructWithClone) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithClone) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate15StructWithCloneNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
    ::example_crate::StructWithClone* __ret_ptr);
}
inline StructWithClone::StructWithClone() {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate15StructWithCloneNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
          this);
}
static_assert(std::is_trivially_destructible_v<StructWithClone>);
static_assert(std::is_trivially_move_constructible_v<StructWithClone>);
static_assert(std::is_trivially_move_assignable_v<StructWithClone>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate15StructWithCloneNtNtCs8sGNUgcxoFi_u4core5clone5Clone5cloneB4_u(
    ::example_crate::StructWithClone const& [[clang::annotate_type("lifetime",
                                                                   "__anon1")]],
    ::example_crate::StructWithClone* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate15StructWithCloneNtNtCs8sGNUgcxoFi_u4core5clone5Clone10clone_ufromB4_u(
    ::example_crate::StructWithClone& [[clang::annotate_type("lifetime",
                                                             "__anon1")]],
    ::example_crate::StructWithClone const& [[clang::annotate_type(
        "lifetime", "__anon2")]]);
}
inline StructWithClone::StructWithClone(const StructWithClone& other) {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate15StructWithCloneNtNtCs8sGNUgcxoFi_u4core5clone5Clone5cloneB4_u(
          other, this);
}
inline StructWithClone& StructWithClone::operator=(
    const StructWithClone& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate15StructWithCloneNtNtCs8sGNUgcxoFi_u4core5clone5Clone10clone_ufromB4_u(
            *this, other);
  }
  return *this;
}
inline void StructWithClone::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithClone, a));
}
}  // namespace example_crate
