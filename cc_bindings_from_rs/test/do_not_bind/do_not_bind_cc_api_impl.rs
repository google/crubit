// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// do_not_bind_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::do_not_bind_golden::Struct>() == 4);
const _: () = assert!(::std::mem::align_of::<::do_not_bind_golden::Struct>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_bound_uinherent_umethod(
    __self: &'static ::do_not_bind_golden::Struct,
) -> i32 {
    unsafe { ::do_not_bind_golden::Struct::bound_inherent_method(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::do_not_bind_golden::Struct, value) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_bound_ufree_ufn() -> i32 {
    unsafe { ::do_not_bind_golden::bound_free_fn() }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Trait_ubound_utrait_umethod_udo_unot_ubind_ugolden_x0000003a_x0000003aStruct(
    __self: &'static ::do_not_bind_golden::Struct,
) -> i32 {
    unsafe {
        <::do_not_bind_golden::Struct as ::do_not_bind_golden::Trait>::bound_trait_method(__self)
    }
}
