// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// function_pointers_golden
// Features: do_not_hardcode_status_bridge, experimental,
// infer_operator_lifetimes, std_vector, supported, unsafe_types, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTION_POINTERS_FUNCTION_POINTERS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTION_POINTERS_FUNCTION_POINTERS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace function_pointers {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=12
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: function_pointers_golden :: HasFnPtrField") alignas(8)
    [[clang::trivial_abi]] HasFnPtrField final {
 public:
  // `HasFnPtrField` doesn't implement the `Default` trait
  HasFnPtrField() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~HasFnPtrField() = default;
  HasFnPtrField(HasFnPtrField&&) = default;
  HasFnPtrField& operator=(HasFnPtrField&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  HasFnPtrField(const HasFnPtrField&) = default;
  HasFnPtrField& operator=(const HasFnPtrField&) = default;
  HasFnPtrField(::crubit::UnsafeRelocateTag, HasFnPtrField&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=17
  static ::function_pointers::HasFnPtrField with_add_ten();

 public:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=13
    crubit::type_identity_t<std::int32_t(std::int32_t)>* ptr;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for `CONST_RUST_FN_PTR_ADD_TEN` defined at
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=26:
// Function pointers can't have a thunk: Any calling convention other than
// `extern "C"` requires a thunk

// Error generating bindings for `CONST_C_FN_PTR_ADD_TEN` defined at
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=27:
// Pointer values cannot be used as scalar constants.

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=30
void call_fn_ptr_no_args_or_return(crubit::type_identity_t<void()>& fn_ptr);

// Error generating bindings for `call_rust_fn_ptr` defined at
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=35:
// Error handling parameter #0 of type `fn()`: Function pointers can't have a
// thunk: Any calling convention other than `extern "C"` requires a thunk

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=40
std::int32_t call_fn_ptr_with_five(
    crubit::type_identity_t<std::int32_t(std::int32_t)>& fn_ptr);

// Error generating bindings for `call_fn_ptr_with_five_reference` defined at
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=45:
// Error handling parameter #0 of type `for<'a> extern "C" fn(&'a i32) -> i32`:
// Generic function pointers are not supported yet (b/259749023)

// Error generating bindings for `call_fn_ptr_with_five_reference_hrtb` defined
// at
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=50:
// Error handling parameter #0 of type `for<'a> extern "C" fn(&'a i32) -> i32`:
// Generic function pointers are not supported yet (b/259749023)

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=57
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: function_pointers_golden :: CStruct") alignas(4) [[clang::trivial_abi]]
CStruct final {
 public:
  // Default::default
  CStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~CStruct() = default;
  CStruct(CStruct&&) = default;
  CStruct& operator=(CStruct&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  CStruct(const CStruct&) = default;
  CStruct& operator=(const CStruct&) = default;
  CStruct(::crubit::UnsafeRelocateTag, CStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

 public:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=58
    std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=62
std::int32_t call_fn_ptr_with_repr_c_struct_ptr_containing_seven(
    crubit::type_identity_t<std::int32_t(::function_pointers::CStruct const*)>&
        fn_ptr);

// Error generating bindings for `call_fn_ptr_with_repr_c_struct` defined at
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=69:
// Error handling parameter #0 of type `extern "C" fn(CStruct) -> i32`: Function
// pointers can't have a thunk: Type of parameter #0 requires a thunk

// Error generating bindings for `call_fn_ptr_with_repr_c_struct_ref` defined at
// cc_bindings_from_rs/test/function_pointers/function_pointers.rs;l=74:
// Error handling parameter #0 of type `for<'a> extern "C" fn(&'a CStruct) ->
// i32`: Generic function pointers are not supported yet (b/259749023)

static_assert(
    sizeof(HasFnPtrField) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasFnPtrField) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<HasFnPtrField>);
static_assert(std::is_trivially_move_constructible_v<HasFnPtrField>);
static_assert(std::is_trivially_move_assignable_v<HasFnPtrField>);
static_assert(std::is_trivially_copy_constructible_v<HasFnPtrField>);
static_assert(std::is_trivially_copy_assignable_v<HasFnPtrField>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_with_uadd_uten(
    ::function_pointers::HasFnPtrField* __ret_ptr);
}
inline ::function_pointers::HasFnPtrField HasFnPtrField::with_add_ten() {
  crubit::Slot<::function_pointers::HasFnPtrField>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_uadd_uten(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasFnPtrField::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasFnPtrField, ptr));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_call_ufn_uptr_uno_uargs_uor_ureturn(
    crubit::type_identity_t<void()>&);
}
inline void call_fn_ptr_no_args_or_return(
    crubit::type_identity_t<void()>& fn_ptr) {
  return __crubit_internal::__crubit_thunk_call_ufn_uptr_uno_uargs_uor_ureturn(
      fn_ptr);
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_call_ufn_uptr_uwith_ufive(
    crubit::type_identity_t<std::int32_t(std::int32_t)>&);
}
inline std::int32_t call_fn_ptr_with_five(
    crubit::type_identity_t<std::int32_t(std::int32_t)>& fn_ptr) {
  return __crubit_internal::__crubit_thunk_call_ufn_uptr_uwith_ufive(fn_ptr);
}

static_assert(
    sizeof(CStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::function_pointers::CStruct* __ret_ptr);
}
inline CStruct::CStruct() { __crubit_internal::__crubit_thunk_default(this); }
static_assert(std::is_trivially_destructible_v<CStruct>);
static_assert(std::is_trivially_move_constructible_v<CStruct>);
static_assert(std::is_trivially_move_assignable_v<CStruct>);
static_assert(std::is_trivially_copy_constructible_v<CStruct>);
static_assert(std::is_trivially_copy_assignable_v<CStruct>);
inline void CStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CStruct, field));
}
namespace __crubit_internal {
extern "C" std::int32_t
__crubit_thunk_call_ufn_uptr_uwith_urepr_uc_ustruct_uptr_ucontaining_useven(
    crubit::type_identity_t<
        std::int32_t(::function_pointers::CStruct const*)>&);
}
inline std::int32_t call_fn_ptr_with_repr_c_struct_ptr_containing_seven(
    crubit::type_identity_t<std::int32_t(::function_pointers::CStruct const*)>&
        fn_ptr) {
  return __crubit_internal::
      __crubit_thunk_call_ufn_uptr_uwith_urepr_uc_ustruct_uptr_ucontaining_useven(
          fn_ptr);
}

}  // namespace function_pointers
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTION_POINTERS_FUNCTION_POINTERS_GOLDEN
