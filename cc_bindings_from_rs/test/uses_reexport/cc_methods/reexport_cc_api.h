// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// reexport_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_CC_METHODS_REEXPORT_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_CC_METHODS_REEXPORT_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstdint>
#include <utility>

#include "cc_bindings_from_rs/test/uses_reexport/cc_methods/cc_type.h"  // IWYU pragma: export

namespace reexport {

//  Calls the free function that rs_bindings_from_cc generates for
//  `CcType::GetValue`, to check that it is usable from other crates.
::std::int32_t get_value_via_free_fn(::cc_ns::CcType const& value);

::cc_ns::CcType make_value(::std::int32_t value);

}  // namespace reexport

namespace reexport {

// CRUBIT_ANNOTATE: cpp_type=cc_ns :: CcType
// CRUBIT_ANNOTATE: cpp_move_constructible=
using CcType CRUBIT_INTERNAL_RUST_TYPE(":: cc_type :: cc_ns :: CcType") =
    ::cc_ns::CcType;
}  // namespace reexport

namespace reexport {

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uvalue_uvia_ufree_ufn(
    ::cc_ns::CcType const&);
}
inline ::std::int32_t get_value_via_free_fn(::cc_ns::CcType const& value) {
  return __crubit_internal::__crubit_thunk_get_uvalue_uvia_ufree_ufn(value);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uvalue(::std::int32_t,
                                           ::cc_ns::CcType* __ret_ptr);
}
inline ::cc_ns::CcType make_value(::std::int32_t value) {
  crubit::Slot<::cc_ns::CcType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uvalue(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace reexport

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_CC_METHODS_REEXPORT_GOLDEN
