// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// pass_by_value_unmovable_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PASS_BY_VALUE_UNMOVABLE_PASS_BY_VALUE_UNMOVABLE_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PASS_BY_VALUE_UNMOVABLE_PASS_BY_VALUE_UNMOVABLE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <utility>

namespace pass_by_value_unmovable {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: pass_by_value_unmovable_golden :: CppMovable") alignas(4)
    [[clang::trivial_abi]] CppMovable final {
 public:
  // Default::default
  CppMovable();

  // Synthesized tuple constructor
  explicit CppMovable(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // Drop::drop
  ~CppMovable();

  CppMovable(CppMovable&&);
  ::pass_by_value_unmovable::CppMovable& operator=(CppMovable&&);

  // `pass_by_value_unmovable_golden::CppMovable` doesn't implement the `Clone`
  // trait
  CppMovable(const CppMovable&) = delete;
  CppMovable& operator=(const CppMovable&) = delete;
  CppMovable(::crubit::UnsafeRelocateTag, CppMovable&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: pass_by_value_unmovable_golden :: NotCppMovable") alignas(4)
    [[clang::trivial_abi]] NotCppMovable final {
 public:
  // `pass_by_value_unmovable_golden::NotCppMovable` doesn't implement the
  // `Default` trait
  NotCppMovable() = delete;

  // Synthesized tuple constructor
  explicit NotCppMovable(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // Drop::drop
  ~NotCppMovable();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  NotCppMovable(NotCppMovable&&) = delete;
  ::pass_by_value_unmovable::NotCppMovable& operator=(NotCppMovable&&) = delete;
  // `pass_by_value_unmovable_golden::NotCppMovable` doesn't implement the
  // `Clone` trait
  NotCppMovable(const NotCppMovable&) = delete;
  NotCppMovable& operator=(const NotCppMovable&) = delete;
  NotCppMovable(::crubit::UnsafeRelocateTag, NotCppMovable&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

void takes_val_movable(::pass_by_value_unmovable::CppMovable _val);

// Error generating bindings for function
// `pass_by_value_unmovable_golden::takes_val_unmovable` defined at
// cc_bindings_from_rs/test/pass_by_value_unmovable/pass_by_value_unmovable.rs;l=23:
// Error handling parameter #0 of type
// `pass_by_value_unmovable_golden::NotCppMovable`: Can't pass a type by value
// without a move constructor. See crubit.rs/rust/movable_types for what types
// are C++ movable.

static_assert(
    sizeof(CppMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CppMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::pass_by_value_unmovable::CppMovable __crubit_thunk_default();
/// \endcond
}  // namespace __crubit_internal
inline ::pass_by_value_unmovable::CppMovable::CppMovable() {
  *this = __crubit_internal::__crubit_thunk_default();
}
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_drop(::pass_by_value_unmovable::CppMovable&);
/// \endcond
}  // namespace __crubit_internal
inline CppMovable::~CppMovable() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline ::pass_by_value_unmovable::CppMovable::CppMovable(CppMovable&& other)
    : CppMovable() {
  *this = ::std::move(other);
}
inline ::pass_by_value_unmovable::CppMovable& ::pass_by_value_unmovable::
    CppMovable::operator=(CppMovable&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void CppMovable::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CppMovable, __field0));
}
static_assert(
    sizeof(NotCppMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NotCppMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_drop(::pass_by_value_unmovable::NotCppMovable&);
/// \endcond
}  // namespace __crubit_internal
inline NotCppMovable::~NotCppMovable() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline void NotCppMovable::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NotCppMovable, __field0));
}
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_takes_uval_umovable(
    ::pass_by_value_unmovable::CppMovable);
/// \endcond
}  // namespace __crubit_internal
inline void takes_val_movable(::pass_by_value_unmovable::CppMovable _val) {
  return __crubit_internal::__crubit_thunk_takes_uval_umovable(_val);
}

}  // namespace pass_by_value_unmovable

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PASS_BY_VALUE_UNMOVABLE_PASS_BY_VALUE_UNMOVABLE_GOLDEN
