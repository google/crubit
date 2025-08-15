// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// enums_golden
// Features: do_not_hardcode_status_bridge, experimental, infer_operator_lifetimes, supported, unsafe_types, wrapper

#![allow(unused_unsafe)]
#![allow(improper_ctypes_definitions)]

const _: () = assert!(::std::mem::size_of::<::enums_golden::repr_c::MyEnum>() == 40);
const _: () = assert!(::std::mem::align_of::<::enums_golden::repr_c::MyEnum>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::enums_golden::repr_c::MyEnum as ::core::default::Default>::default();
        (__ret_ptr as *mut ::enums_golden::repr_c::MyEnum).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::enums_golden::repr_c::MyEnum>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () = assert!(::std::mem::size_of::<::enums_golden::repr_c_drop::DropMe>() == 16);
const _: () = assert!(::std::mem::align_of::<::enums_golden::repr_c_drop::DropMe>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::enums_golden::repr_c_drop::DropMe as ::core::default::Default>::default();
        (__ret_ptr as *mut ::enums_golden::repr_c_drop::DropMe).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::enums_golden::repr_c_drop::DropMe>,
) {
    unsafe { __self.assume_init_drop() };
}
const _: () =
    assert!(::std::mem::size_of::<::enums_golden::repr_c_clone_counter::CloneCount>() == 16);
const _: () =
    assert!(::std::mem::align_of::<::enums_golden::repr_c_clone_counter::CloneCount>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::enums_golden::repr_c_clone_counter::CloneCount as ::core::default::Default>::default(
            );
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_counter::CloneCount)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::enums_golden::repr_c_clone_counter::CloneCount,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::enums_golden::repr_c_clone_counter::CloneCount as ::core::clone::Clone>::clone(
                __self,
            );
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_counter::CloneCount)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::enums_golden::repr_c_clone_counter::CloneCount,
    source: &'static ::enums_golden::repr_c_clone_counter::CloneCount,
) -> () {
    unsafe {
        <::enums_golden::repr_c_clone_counter::CloneCount as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
const _: () = assert!(
    ::std::mem::size_of::<::enums_golden::repr_c_clone_active_variant::CloneActiveVariant>() == 8
);
const _: () = assert!(
    ::std::mem::align_of::<::enums_golden::repr_c_clone_active_variant::CloneActiveVariant>() == 4
);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value=<::enums_golden::repr_c_clone_active_variant::CloneActiveVariant as::core::default::Default>::default();
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<::enums_golden::repr_c_clone_active_variant::CloneActiveVariant as::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
    source: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
) -> () {
    unsafe {
        <::enums_golden::repr_c_clone_active_variant::CloneActiveVariant as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_ua(
    e: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
) -> bool {
    unsafe { ::enums_golden::repr_c_clone_active_variant::is_a(e) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_ub(
    e: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
) -> bool {
    unsafe { ::enums_golden::repr_c_clone_active_variant::is_b(e) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_is_uc(
    e: &'static ::enums_golden::repr_c_clone_active_variant::CloneActiveVariant,
) -> bool {
    unsafe { ::enums_golden::repr_c_clone_active_variant::is_c(e) }
}
