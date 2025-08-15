// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// enums_golden
// Features: do_not_hardcode_status_bridge, experimental,
// infer_operator_lifetimes, supported, unsafe_types, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace enums {

namespace classless_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum
//
// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=11
enum CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: classless_enum :: Color") Color : std::int32_t {
  RED = INT32_C(0),
  BLUE = INT32_C(2),
};

}  // namespace classless_enum

namespace cpp_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=22
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: cpp_enum :: Color") Color : std::int32_t {
  RED = INT32_C(0),
  BLUE = INT32_C(2),
};

}  // namespace cpp_enum

namespace repr_c {

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=33
struct CRUBIT_INTERNAL_RUST_TYPE(":: enums_golden :: repr_c :: MyEnum") alignas(
    8) [[clang::trivial_abi]] MyEnum final {
 public:
  // Default::default
  MyEnum();

  // Drop::drop
  ~MyEnum();

  MyEnum(MyEnum&&);
  MyEnum& operator=(MyEnum&&);

  // `repr_c::MyEnum` doesn't implement the `Clone` trait
  MyEnum(const MyEnum&) = delete;
  MyEnum& operator=(const MyEnum&) = delete;
  MyEnum(::crubit::UnsafeRelocateTag, MyEnum&& value) {
    memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_E_struct {
   private:
    // Field type has been replaced with a blob of bytes: Type
    // `std::string::String` comes from the `alloc` crate, but no
    // `--crate-header` was specified for this crate
    unsigned char __field0[24];

   public:
    std::int32_t __field1;
  };
  struct alignas(0) __crubit_A_struct {
   public:
    std::int32_t __field0;
    std::int64_t __field1;
  };
  // Variant F has no size, so no struct is generated.

  // Variant Z has no size, so no struct is generated.

  // Variant G has no size, so no struct is generated.

  struct alignas(0) __crubit_B_struct {
   public:
    bool h;
    bool i;
  };
  struct alignas(0) __crubit_C_struct {
   public:
    std::int32_t a;
    std::int32_t b;
    std::int32_t c;
  };
  // Variant D has no size, so no struct is generated.

  enum class Tag : std::int64_t {
    E = 0,
    A = 1,
    F = 2,
    Z = 3,
    G = 4,
    B = 10000,
    C = 10001,
    D = 10002,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_E_struct E;
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_c

namespace repr_c_drop {

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=53
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_drop :: DropMe") alignas(8)
    [[clang::trivial_abi]] DropMe final {
 public:
  // Default::default
  DropMe();

  // Drop::drop
  ~DropMe();

  DropMe(DropMe&&);
  DropMe& operator=(DropMe&&);

  // `repr_c_drop::DropMe` doesn't implement the `Clone` trait
  DropMe(const DropMe&) = delete;
  DropMe& operator=(const DropMe&) = delete;
  DropMe(::crubit::UnsafeRelocateTag, DropMe&& value) {
    memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    std::int32_t __field0;
  };
  struct alignas(0) __crubit_B_struct {
   public:
    std::int64_t __field0;
  };
  // Variant Q has no size, so no struct is generated.

  struct alignas(0) __crubit_C_struct {
   public:
    std::int32_t* p;
  };

  enum class Tag : std::uint32_t {
    A = 0,
    B = 1,
    Q = 2,
    C = 3,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_c_drop

namespace repr_c_clone_counter {

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=77
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_clone_counter :: CloneCount") alignas(8)
    [[clang::trivial_abi]] CloneCount final {
 public:
  // Default::default
  CloneCount();

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneCount() = default;
  CloneCount(CloneCount&&) = default;
  CloneCount& operator=(CloneCount&&) = default;

  // Clone::clone
  CloneCount(const CloneCount&);

  // Clone::clone_from
  CloneCount& operator=(const CloneCount&);

  CloneCount(::crubit::UnsafeRelocateTag, CloneCount&& value) {
    memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    std::int32_t* p;
  };

  enum class Tag : std::int8_t {
    A = 0,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_c_clone_counter

namespace repr_c_clone_active_variant {

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=101
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_clone_active_variant :: "
    "CloneActiveVariant") alignas(4) [[clang::trivial_abi]]
CloneActiveVariant final {
 public:
  // Default::default
  CloneActiveVariant();

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneActiveVariant() = default;
  CloneActiveVariant(CloneActiveVariant&&) = default;
  CloneActiveVariant& operator=(CloneActiveVariant&&) = default;

  // Clone::clone
  CloneActiveVariant(const CloneActiveVariant&);

  // Clone::clone_from
  CloneActiveVariant& operator=(const CloneActiveVariant&);

  CloneActiveVariant(::crubit::UnsafeRelocateTag, CloneActiveVariant&& value) {
    memcpy(this, &value, sizeof(value));
  }
  struct alignas(0) __crubit_A_struct {
   public:
    std::int32_t __field0;
  };
  struct alignas(0) __crubit_B_struct {
   public:
    std::int32_t __field0;
  };
  struct alignas(0) __crubit_C_struct {
   public:
    std::int32_t __field0;
  };

  enum class Tag : std::int8_t {
    A = 0,
    B = 1,
    C = 2,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=123
bool is_a(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=127
bool is_b(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=131
bool is_c(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

}  // namespace repr_c_clone_active_variant

namespace deprecated_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// cc_bindings_from_rs/test/enums/enums.rs;l=144
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: deprecated_enum :: Color")
    [[nodiscard]] [[deprecated("Use NewColor")]] Color : std::int32_t{
        RED = INT32_C(0),
        BLUE = INT32_C(2),
    };

}  // namespace deprecated_enum

namespace classless_enum {}

namespace cpp_enum {}

namespace repr_c {

static_assert(
    sizeof(MyEnum) == 40,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyEnum) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::enums::repr_c::MyEnum* __ret_ptr);
}
inline MyEnum::MyEnum() { __crubit_internal::__crubit_thunk_default(this); }
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::enums::repr_c::MyEnum&);
}
inline MyEnum::~MyEnum() { __crubit_internal::__crubit_thunk_drop(*this); }
inline MyEnum::MyEnum(MyEnum&& other) : MyEnum() { *this = std::move(other); }
inline MyEnum& MyEnum::operator=(MyEnum&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void MyEnum::__crubit_field_offset_assertions() {
  static_assert(8 == offsetof(MyEnum, E));
  static_assert(8 == offsetof(MyEnum, A));
  static_assert(8 == offsetof(MyEnum, B));
  static_assert(8 == offsetof(MyEnum, C));
  static_assert(24 == offsetof(MyEnum::__crubit_E_struct, __field1));
  static_assert(0 == offsetof(MyEnum::__crubit_A_struct, __field0));
  static_assert(8 == offsetof(MyEnum::__crubit_A_struct, __field1));
  static_assert(0 == offsetof(MyEnum::__crubit_B_struct, h));
  static_assert(1 == offsetof(MyEnum::__crubit_B_struct, i));
  static_assert(0 == offsetof(MyEnum::__crubit_C_struct, a));
  static_assert(4 == offsetof(MyEnum::__crubit_C_struct, b));
  static_assert(8 == offsetof(MyEnum::__crubit_C_struct, c));
}
}  // namespace repr_c

namespace repr_c_drop {

static_assert(
    sizeof(DropMe) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(DropMe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::enums::repr_c_drop::DropMe* __ret_ptr);
}
inline DropMe::DropMe() { __crubit_internal::__crubit_thunk_default(this); }
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::enums::repr_c_drop::DropMe&);
}
inline DropMe::~DropMe() { __crubit_internal::__crubit_thunk_drop(*this); }
inline DropMe::DropMe(DropMe&& other) : DropMe() { *this = std::move(other); }
inline DropMe& DropMe::operator=(DropMe&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void DropMe::__crubit_field_offset_assertions() {
  static_assert(8 == offsetof(DropMe, A));
  static_assert(8 == offsetof(DropMe, B));
  static_assert(8 == offsetof(DropMe, C));
  static_assert(0 == offsetof(DropMe::__crubit_A_struct, __field0));
  static_assert(0 == offsetof(DropMe::__crubit_B_struct, __field0));
  static_assert(0 == offsetof(DropMe::__crubit_C_struct, p));
}
}  // namespace repr_c_drop

namespace repr_c_clone_counter {

static_assert(
    sizeof(CloneCount) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneCount) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::enums::repr_c_clone_counter::CloneCount* __ret_ptr);
}
inline CloneCount::CloneCount() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<CloneCount>);
static_assert(std::is_trivially_move_constructible_v<CloneCount>);
static_assert(std::is_trivially_move_assignable_v<CloneCount>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::enums::repr_c_clone_counter::CloneCount const&,
    ::enums::repr_c_clone_counter::CloneCount* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    ::enums::repr_c_clone_counter::CloneCount&,
    ::enums::repr_c_clone_counter::CloneCount const&);
}
inline CloneCount::CloneCount(const CloneCount& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline CloneCount& CloneCount::operator=(const CloneCount& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void CloneCount::__crubit_field_offset_assertions() {
  static_assert(8 == offsetof(CloneCount, A));
  static_assert(0 == offsetof(CloneCount::__crubit_A_struct, p));
}
}  // namespace repr_c_clone_counter

namespace repr_c_clone_active_variant {

static_assert(
    sizeof(CloneActiveVariant) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneActiveVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline CloneActiveVariant::CloneActiveVariant() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<CloneActiveVariant>);
static_assert(std::is_trivially_move_constructible_v<CloneActiveVariant>);
static_assert(std::is_trivially_move_assignable_v<CloneActiveVariant>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant&,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline CloneActiveVariant::CloneActiveVariant(const CloneActiveVariant& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline CloneActiveVariant& CloneActiveVariant::operator=(
    const CloneActiveVariant& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void CloneActiveVariant::__crubit_field_offset_assertions() {
  static_assert(4 == offsetof(CloneActiveVariant, A));
  static_assert(4 == offsetof(CloneActiveVariant, B));
  static_assert(4 == offsetof(CloneActiveVariant, C));
  static_assert(0 == offsetof(CloneActiveVariant::__crubit_A_struct, __field0));
  static_assert(0 == offsetof(CloneActiveVariant::__crubit_B_struct, __field0));
  static_assert(0 == offsetof(CloneActiveVariant::__crubit_C_struct, __field0));
}
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_ua(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_a(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_ua(e);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_ub(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_b(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_ub(e);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uc(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_c(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_uc(e);
}

}  // namespace repr_c_clone_active_variant

namespace deprecated_enum {}

}  // namespace enums
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN
