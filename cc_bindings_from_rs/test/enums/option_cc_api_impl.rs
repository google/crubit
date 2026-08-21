// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::option_golden::CloneNoDefault>() == 1);
const _: () = assert!(::std::mem::align_of::<::option_golden::CloneNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_uoption_ugolden_x0000003a_x0000003aCloneNoDefault(
    __self: &'static ::option_golden::CloneNoDefault,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::option_golden::CloneNoDefault as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_uoption_ugolden_x0000003a_x0000003aCloneNoDefault(
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
unsafe extern "C" fn __crubit_thunk_Default_udefault_uoption_ugolden_x0000003a_x0000003aHasDefault(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::option_golden::HasDefault as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uoption_ugolden_x0000003a_x0000003aHasDefault(
    __self: *mut ::option_golden::HasDefault,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(s: &'static str, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasDefault::new(s);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
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
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasHasOptions, me) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::HasNoDefault>() == 32);
const _: () = assert!(::std::mem::align_of::<::option_golden::HasNoDefault>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uoption_ugolden_x0000003a_x0000003aHasNoDefault(
    __self: *mut ::option_golden::HasNoDefault,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(s: &'static str, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasNoDefault::new(s);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
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
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_with_uoption(
    value: *mut ::core::option::Option<u8>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.read();
        let __rs_return_value = ::option_golden::HasOptions::with_option(value);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_uref(
    value: &'static ::core::option::Option<u8>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasOptions::from_ref(value);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_with_unone(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::HasOptions::with_none();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, direct) == 0);
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, niche) == 2);
const _: () = assert!(::core::mem::offset_of!(::option_golden::HasOptions, nested) == 3);
const _: () = assert!(::std::mem::size_of::<::option_golden::LessThan20U8>() == 1);
const _: () = assert!(::std::mem::align_of::<::option_golden::LessThan20U8>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(value: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::LessThan20U8::new(value);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_value(__self: *mut ::option_golden::LessThan20U8) -> u8 {
    unsafe {
        let __self = __self.read();
        ::option_golden::LessThan20U8::value(__self)
    }
}
const _: () = assert!(::std::mem::size_of::<::option_golden::OptCloneNoDefault>() == 2);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptCloneNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_uoption_ugolden_x0000003a_x0000003aOptCloneNoDefault(
    __self: &'static ::option_golden::OptCloneNoDefault,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::option_golden::OptCloneNoDefault as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_uoption_ugolden_x0000003a_x0000003aOptCloneNoDefault(
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
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptCloneNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::OptCopyNoDefault>() == 2);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptCopyNoDefault>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: u8, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::OptCopyNoDefault::new(x);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptCopyNoDefault, val) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::OptDefaultWithDrop>() == 24);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptDefaultWithDrop>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uoption_ugolden_x0000003a_x0000003aOptDefaultWithDrop(
    __self: *mut ::option_golden::OptDefaultWithDrop,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(s: &'static str, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::OptDefaultWithDrop::new(s);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptDefaultWithDrop, opt) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::OptNoDefaultWithDrop>() == 32);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptNoDefaultWithDrop>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uoption_ugolden_x0000003a_x0000003aOptNoDefaultWithDrop(
    __self: *mut ::option_golden::OptNoDefaultWithDrop,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(s: &'static str, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::OptNoDefaultWithDrop::new(s);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
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
unsafe extern "C" fn __crubit_thunk_Default_udefault_uoption_ugolden_x0000003a_x0000003aOptZst(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <::option_golden::OptZst as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptZst, val) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::OptionWithSizeTypes>() == 32);
const _: () = assert!(::std::mem::align_of::<::option_golden::OptionWithSizeTypes>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    uval: *mut ::core::option::Option<usize>,
    ival: *mut ::core::option::Option<isize>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let uval = uval.read();
        let ival = ival.read();
        let __rs_return_value = ::option_golden::OptionWithSizeTypes::new(uval, ival);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptionWithSizeTypes, uval) == 0);
const _: () = assert!(::core::mem::offset_of!(::option_golden::OptionWithSizeTypes, ival) == 16);
const _: () = assert!(::std::mem::size_of::<::option_golden::UnitOptionField>() == 1);
const _: () = assert!(::std::mem::align_of::<::option_golden::UnitOptionField>() == 1);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_uoption_ugolden_x0000003a_x0000003aUnitOptionField(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::option_golden::UnitOptionField as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new_uwith_usome(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::UnitOptionField::new_with_some();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::option_golden::UnitOptionField, unit) == 0);
const _: () = assert!(::std::mem::size_of::<::option_golden::ZStream>() == 8);
const _: () = assert!(::std::mem::align_of::<::option_golden::ZStream>() == 8);
const _: () = assert!(::core::mem::offset_of!(::option_golden::ZStream, zfree) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_pass_uoption_uptr(
    x: *mut ::core::option::Option<*const i32>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let x = x.read();
        let __rs_return_value = ::option_golden::pass_option_ptr(x);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uoption_uresult(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::return_option_result();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_return_uoption_uresult_uunmovable(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::return_option_result_unmovable();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_stress_utesting_unested_utypes(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::stress_testing_nested_types();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_stringify_ulen(
    x: &'static ::core::option::Option<::option_golden::HasDefault>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::option_golden::stringify_len(x);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_take_uoption_uresult_uunmovable(
    _x: *mut ::core::option::Option<
        ::core::result::Result<::option_golden::HasNoDefault, ::alloc::string::String>,
    >,
) -> () {
    unsafe {
        let _x = _x.read();
        ::option_golden::take_option_result_unmovable(_x)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    __self: &'static ::core::option::Option<::option_golden::CloneNoDefault>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <std::option::Option<::option_golden::CloneNoDefault> as ::core::clone::Clone>::clone(
                __self,
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aCloneNoDefault_x0000003e(
    __self: &'static mut ::core::option::Option<::option_golden::CloneNoDefault>,
    source: &'static ::core::option::Option<::option_golden::CloneNoDefault>,
) -> () {
    unsafe {
        <std::option::Option<::option_golden::CloneNoDefault> as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
    __self: &'static ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <std::option::Option<
            ::core::result::Result<i32, ::alloc::string::String>,
        > as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e(
    __self: &'static mut ::core::option::Option<
        ::core::result::Result<i32, ::alloc::string::String>,
    >,
    source: &'static ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
) -> () {
    unsafe {
        <std::option::Option<::core::result::Result<i32,::alloc::string::String>>as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
    __self: &'static ::core::option::Option<
        ::core::result::Result<
            ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
            ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
        >,
    >,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <std::option::Option<
            ::core::result::Result<
                ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
                ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
            >,
        > as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e_x0000003e(
    __self: &'static mut ::core::option::Option<
        ::core::result::Result<
            ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
            ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
        >,
    >,
    source: &'static ::core::option::Option<
        ::core::result::Result<
            ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
            ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
        >,
    >,
) -> () {
    unsafe {
        <std::option::Option<
            ::core::result::Result<
                ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
                ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
            >,
        > as ::core::clone::Clone>::clone_from(__self, source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    __self: &'static ::core::result::Result<i32, ::alloc::string::String>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <std::result::Result<i32, ::alloc::string::String> as ::core::clone::Clone>::clone(
                __self,
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e(
    __self: &'static mut ::core::result::Result<i32, ::alloc::string::String>,
    source: &'static ::core::result::Result<i32, ::alloc::string::String>,
) -> () {
    unsafe {
        <std::result::Result<i32, ::alloc::string::String> as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
    __self: &'static ::core::result::Result<
        ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
        ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
    >,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = <std::result::Result<
            ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
            ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
        > as ::core::clone::Clone>::clone(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003ci32_x0000002c_x00000020std_x0000003a_x0000003astring_x0000003a_x0000003aString_x0000003e_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aresult_x0000003a_x0000003aResult_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x0000003e_x0000003e(
    __self: &'static mut ::core::result::Result<
        ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
        ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
    >,
    source: &'static ::core::result::Result<
        ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
        ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
    >,
) -> () {
    unsafe {
        <std::result::Result<
            ::core::option::Option<::core::result::Result<i32, ::alloc::string::String>>,
            ::core::result::Result<::core::option::Option<i32>, ::core::option::Option<i32>>,
        > as ::core::clone::Clone>::clone_from(__self, source)
    }
}
