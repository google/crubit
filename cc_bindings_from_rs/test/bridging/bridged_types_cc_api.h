// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// bridged_types_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_BRIDGED_TYPES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_BRIDGED_TYPES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "cpp_ns/cpp_type.h"

namespace bridged_types::test_format_bridged_func_arg_by_pointer {

// Error generating bindings for struct
// `bridged_types_golden::test_format_bridged_func_arg_by_pointer::RustTypeView`
// defined at
// cc_bindings_from_rs/test/bridging/bridged_types.rs;l=11:
// Type bindings for
// bridged_types_golden::test_format_bridged_func_arg_by_pointer::RustTypeView
// suppressed due to being mapped to an existing C++ type (CppType const*)

void test_format_bridged_func_arg_by_pointer(CppType const* __param_0);

}  // namespace bridged_types::test_format_bridged_func_arg_by_pointer

namespace bridged_types::test_format_bridged_func_arg_by_value {

// Error generating bindings for struct
// `bridged_types_golden::test_format_bridged_func_arg_by_value::RustType`
// defined at
// cc_bindings_from_rs/test/bridging/bridged_types.rs;l=24:
// Type bindings for
// bridged_types_golden::test_format_bridged_func_arg_by_value::RustType
// suppressed due to being mapped to an existing C++ type (cpp_ns::CppType)

void test_format_bridged_func_arg_by_value(cpp_ns::CppType _a);

}  // namespace bridged_types::test_format_bridged_func_arg_by_value

namespace bridged_types::test_format_bridged_return_type_by_pointer {

// Error generating bindings for struct
// `bridged_types_golden::test_format_bridged_return_type_by_pointer::RustTypeOwned`
// defined at
// cc_bindings_from_rs/test/bridging/bridged_types.rs;l=37:
// Type bindings for
// bridged_types_golden::test_format_bridged_return_type_by_pointer::RustTypeOwned
// suppressed due to being mapped to an existing C++ type (CppType*)

CppType* test_format_bridged_return_type_by_pointer();

}  // namespace bridged_types::test_format_bridged_return_type_by_pointer

namespace bridged_types::test_format_bridged_return_type_by_value {

// Error generating bindings for struct
// `bridged_types_golden::test_format_bridged_return_type_by_value::RustType`
// defined at
// cc_bindings_from_rs/test/bridging/bridged_types.rs;l=52:
// Type bindings for
// bridged_types_golden::test_format_bridged_return_type_by_value::RustType
// suppressed due to being mapped to an existing C++ type (cpp_ns::CppType)

cpp_ns::CppType test_format_bridged_return_type_by_value();

}  // namespace bridged_types::test_format_bridged_return_type_by_value

namespace bridged_types::test_format_bridged_func_arg_by_pointer {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_test_uformat_ubridged_ufunc_uarg_uby_upointer(
    CppType const*);
/// \endcond
}  // namespace __crubit_internal
inline void test_format_bridged_func_arg_by_pointer(CppType const* __param_0) {
  return __crubit_internal::
      __crubit_thunk_test_uformat_ubridged_ufunc_uarg_uby_upointer(__param_0);
}

}  // namespace bridged_types::test_format_bridged_func_arg_by_pointer

namespace bridged_types::test_format_bridged_func_arg_by_value {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_test_uformat_ubridged_ufunc_uarg_uby_uvalue(
    cpp_ns::CppType*);
/// \endcond
}  // namespace __crubit_internal
inline void test_format_bridged_func_arg_by_value(cpp_ns::CppType _a) {
  return __crubit_internal::
      __crubit_thunk_test_uformat_ubridged_ufunc_uarg_uby_uvalue(&_a);
}

}  // namespace bridged_types::test_format_bridged_func_arg_by_value

namespace bridged_types::test_format_bridged_return_type_by_pointer {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_test_uformat_ubridged_ureturn_utype_uby_upointer(
    CppType** __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline CppType* test_format_bridged_return_type_by_pointer() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    CppType* val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::
      __crubit_thunk_test_uformat_ubridged_ureturn_utype_uby_upointer(
          __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

}  // namespace bridged_types::test_format_bridged_return_type_by_pointer

namespace bridged_types::test_format_bridged_return_type_by_value {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_test_uformat_ubridged_ureturn_utype_uby_uvalue(
    cpp_ns::CppType* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline cpp_ns::CppType test_format_bridged_return_type_by_value() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    cpp_ns::CppType val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::
      __crubit_thunk_test_uformat_ubridged_ureturn_utype_uby_uvalue(
          __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

}  // namespace bridged_types::test_format_bridged_return_type_by_value

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_BRIDGED_TYPES_GOLDEN
