// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

const _: () = assert!(::std::mem::size_of::<::option_golden::CloneNoDefault>() == 1);
const _: () = assert!(::std::mem::align_of::<::option_golden::CloneNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::option_golden::CloneNoDefault,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::option_golden::CloneNoDefault as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::option_golden::CloneNoDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::option_golden::CloneNoDefault,
    source: &'static ::option_golden::CloneNoDefault,
) -> () {
    unsafe { <::option_golden::CloneNoDefault as ::core::clone::Clone>::clone_from(__self, source) }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::CloneNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::CopyNoDefault>() == 1);
const _: () = assert!(::std::mem::align_of::<::option_golden::CopyNoDefault>() == 1);
const _: () = assert!(::core::mem::offset_of!(::option_golden::CopyNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::HasDefault>() == 24);
const _: () = assert!(::std::mem::align_of::<::option_golden::HasDefault>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::option_golden::HasDefault as ::core::default::Default>::default();
        (__ret_ptr as *mut ::option_golden::HasDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::option_golden::HasDefault>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(s: &'static str, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasDefault::new(s);
        (__ret_ptr as *mut ::option_golden::HasDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustring_uinside_uoption(
    __self: &'static ::option_golden::HasDefault,
) -> &'static str {
    unsafe { ::option_golden::HasDefault::get_string_inside_option(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasDefault, foo) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::HasHasOptions>() == 4);
const _: () = assert!(::std::mem::align_of::<::option_golden::HasHasOptions>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(value: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasHasOptions::new(value);
        (__ret_ptr as *mut ::option_golden::HasHasOptions).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasHasOptions, me) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::HasNoDefault>() == 32);
const _: () = assert!(::std::mem::align_of::<::option_golden::HasNoDefault>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::option_golden::HasNoDefault>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(s: &'static str, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasNoDefault::new(s);
        (__ret_ptr as *mut ::option_golden::HasNoDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustring_uinside_uoption(
    __self: &'static ::option_golden::HasNoDefault,
) -> &'static str {
    unsafe { ::option_golden::HasNoDefault::get_string_inside_option(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasNoDefault, foo) == 0);
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasNoDefault, a) == 24);
const _: () = assert!(::std::mem::size_of::<::option_golden::HasOptions>() == 4);
const _: () = assert!(::std::mem::align_of::<::option_golden::HasOptions>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(value: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasOptions::new(value);
        (__ret_ptr as *mut ::option_golden::HasOptions).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_with_uoption(
    value: *const core::ffi::c_uchar,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = unsafe {
            ::bridge_rust::internal::decode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<u8>()),
                value,
            )
        };
        let __rs_return_value = ::option_golden::HasOptions::with_option(value);
        (__ret_ptr as *mut ::option_golden::HasOptions).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_uref(
    value: &'static ::core::option::Option<u8>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasOptions::from_ref(value);
        (__ret_ptr as *mut ::option_golden::HasOptions).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_with_unone(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasOptions::with_none();
        (__ret_ptr as *mut ::option_golden::HasOptions).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, direct) == 0);
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, niche) == 2);
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, nested) == 3);
const _: () = assert!(::std::mem::size_of::<::option_golden::NonMaxU8>() == 1);
const _: () = assert!(::std::mem::align_of::<::option_golden::NonMaxU8>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_value(__self: &'static ::option_golden::NonMaxU8) -> u8 {
    unsafe { ::option_golden::NonMaxU8::value(__self) }
}
const _: () = assert!(::std::mem::size_of::<::option_golden::OptCloneNoDefault>() == 2);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptCloneNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::option_golden::OptCloneNoDefault,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::option_golden::OptCloneNoDefault as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::option_golden::OptCloneNoDefault).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::option_golden::OptCloneNoDefault,
    source: &'static ::option_golden::OptCloneNoDefault,
) -> () {
    unsafe {
        <::option_golden::OptCloneNoDefault as ::core::clone::Clone>::clone_from(__self, source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::OptCloneNoDefault::new(x);
        (__ret_ptr as *mut ::option_golden::OptCloneNoDefault).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptCloneNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::OptCopyNoDefault>() == 2);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptCopyNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::OptCopyNoDefault::new(x);
        (__ret_ptr as *mut ::option_golden::OptCopyNoDefault).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptCopyNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::OptDefaultWithDrop>() == 24);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptDefaultWithDrop>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::option_golden::OptDefaultWithDrop>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(s: &'static str, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::OptDefaultWithDrop::new(s);
        (__ret_ptr as *mut ::option_golden::OptDefaultWithDrop).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptDefaultWithDrop, opt) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::OptNoDefaultWithDrop>() == 32);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptNoDefaultWithDrop>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::option_golden::OptNoDefaultWithDrop>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(s: &'static str, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::OptNoDefaultWithDrop::new(s);
        (__ret_ptr as *mut ::option_golden::OptNoDefaultWithDrop).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_ustring_uinside_uoption(
    __self: &'static ::option_golden::OptNoDefaultWithDrop,
) -> &'static str {
    unsafe { ::option_golden::OptNoDefaultWithDrop::get_string_inside_option(__self) }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptNoDefaultWithDrop, val) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::OptZst>() == 1);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptZst>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = <::option_golden::OptZst as ::core::default::Default>::default();
        (__ret_ptr as *mut ::option_golden::OptZst).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptZst, val) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_stringify_ulen(
    x: &'static ::core::option::Option<::option_golden::HasDefault>,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::stringify_len(x);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<usize>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::core::option::Option<::option_golden::CloneNoDefault>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <std::option::Option<::option_golden::CloneNoDefault> as ::core::clone::Clone>::clone(
                __self,
            );
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<
                    ::option_golden::CloneNoDefault,
                >()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::core::option::Option<::option_golden::CloneNoDefault>,
    source: &'static ::core::option::Option<::option_golden::CloneNoDefault>,
) -> () {
    unsafe {
        <std::option::Option<::option_golden::CloneNoDefault> as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
