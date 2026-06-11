// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// layout_equivalent_pointers_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_LAYOUT_EQUIVALENT_POINTERS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_LAYOUT_EQUIVALENT_POINTERS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"

#include "cpp_ns/cpp_type.h"

namespace layout_equivalent_pointers::test_format_func_arg_pointer_like {

// Error generating bindings for struct
// `layout_equivalent_pointers_golden::test_format_func_arg_pointer_like::RustTypeView`
// defined at
// cc_bindings_from_rs/test/bridging/layout_equivalent_pointers.rs;l=9:
// Type bindings for
// layout_equivalent_pointers_golden::test_format_func_arg_pointer_like::RustTypeView
// suppressed due to being mapped to an existing C++ type (const CppType*)

void test_format_func_arg_pointer_like(const CppType* __param_0);

}  // namespace layout_equivalent_pointers::test_format_func_arg_pointer_like

namespace layout_equivalent_pointers::test_format_return_type_pointer_like {

// Error generating bindings for struct
// `layout_equivalent_pointers_golden::test_format_return_type_pointer_like::RustTypeOwned`
// defined at
// cc_bindings_from_rs/test/bridging/layout_equivalent_pointers.rs;l=21:
// Type bindings for
// layout_equivalent_pointers_golden::test_format_return_type_pointer_like::RustTypeOwned
// suppressed due to being mapped to an existing C++ type (CppType*)

CppType* test_format_return_type_pointer_like();

}  // namespace layout_equivalent_pointers::test_format_return_type_pointer_like

namespace layout_equivalent_pointers::test_format_func_arg_pointer_like {

namespace __crubit_internal {
extern "C" void __crubit_thunk_test_uformat_ufunc_uarg_upointer_ulike(
    const CppType*);
}
inline void test_format_func_arg_pointer_like(const CppType* __param_0) {
  return __crubit_internal::
      __crubit_thunk_test_uformat_ufunc_uarg_upointer_ulike(__param_0);
}

}  // namespace layout_equivalent_pointers::test_format_func_arg_pointer_like

namespace layout_equivalent_pointers::test_format_return_type_pointer_like {

namespace __crubit_internal {
extern "C" void __crubit_thunk_test_uformat_ureturn_utype_upointer_ulike(
    CppType** crubit_nonnull __ret_ptr);
}
inline CppType* test_format_return_type_pointer_like() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    CppType* val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_test_uformat_ureturn_utype_upointer_ulike(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

}  // namespace layout_equivalent_pointers::test_format_return_type_pointer_like

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_LAYOUT_EQUIVALENT_POINTERS_GOLDEN
