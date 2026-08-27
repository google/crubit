// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rust_lib_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
extern crate foo_rust_proto as foo_proto;
const _: () = assert!(::std::mem::size_of::<::rust_lib_golden::FooService>() == 8);
const _: () = assert!(::std::mem::align_of::<::rust_lib_golden::FooService>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_urust_ulib_ugolden_x0000003a_x0000003aFooService(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::rust_lib_golden::FooService as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_urust_ulib_ugolden_x0000003a_x0000003aFooService(
    __self: *mut ::rust_lib_golden::FooService,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_handle_urequest(
    __self: &'static mut ::rust_lib_golden::FooService,
    req: *const core::ffi::c_void,
    rsp: *const core::ffi::c_void,
) -> bool {
    unsafe {
        let req = {
            let mut __crubit_temp =
                ::core::mem::MaybeUninit::<::foo_proto::FooRequestView<'static>>::uninit();
            __crubit_temp.write(::core::mem::transmute(req));
            __crubit_temp.assume_init()
        };
        let rsp = {
            let mut __crubit_temp =
                ::core::mem::MaybeUninit::<::foo_proto::FooResponseMut<'static>>::uninit();
            __crubit_temp.write(::core::mem::transmute(rsp));
            __crubit_temp.assume_init()
        };
        ::rust_lib_golden::FooService::handle_request(__self, req, rsp)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_request_ustats(
    __self: &'static ::rust_lib_golden::FooService,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rust_lib_golden::FooService::request_stats(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_urequest_ustats(
    __self: &'static ::rust_lib_golden::FooService,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rust_lib_golden::FooService::clone_request_stats(__self);
        proto2_rust_thunk_Message_foo_service_FooRequestStats_crubit_rust_to_cpp_converter(
            std::ptr::from_ref(&__rs_return_value) as *const core::ffi::c_void,
            __ret_ptr,
        );
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_update_urequest_ustats(
    __self: &'static mut ::rust_lib_golden::FooService,
    updated_stats: *const core::ffi::c_void,
) -> () {
    unsafe {
        let updated_stats = {
            let mut __crubit_temp =
                ::core::mem::MaybeUninit::<::foo_proto::FooRequestStats>::uninit();
            proto2_rust_thunk_Message_foo_service_FooRequestStats_crubit_cpp_to_rust_converter(
                updated_stats,
                __crubit_temp.as_mut_ptr() as *mut core::ffi::c_void,
            );
            __crubit_temp.assume_init()
        };
        ::rust_lib_golden::FooService::update_request_stats(__self, updated_stats)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_enum_uin_usignature(_e: *mut ::foo_proto::FooEnum) -> () {
    unsafe {
        let _e = _e.read();
        ::rust_lib_golden::FooService::enum_in_signature(_e)
    }
}
const _: () = assert!(::std::mem::size_of::<::rust_lib_golden::StructWithProto>() == 8);
const _: () = assert!(::std::mem::align_of::<::rust_lib_golden::StructWithProto>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_urust_ulib_ugolden_x0000003a_x0000003aStructWithProto(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::rust_lib_golden::StructWithProto as ::core::default::Default>::default();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_urust_ulib_ugolden_x0000003a_x0000003aStructWithProto(
    __self: *mut ::rust_lib_golden::StructWithProto,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () = assert!(::core::mem::offset_of!(::rust_lib_golden::StructWithProto, stats) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_uproto_ustatus_uor(
    num: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rust_lib_golden::create_proto_status_or(num);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_uproto_uvec(
    num: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rust_lib_golden::create_proto_vec(num);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_ustruct_uwith_uproto(
    num: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rust_lib_golden::create_struct_with_proto(num);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_read_uproto_upointer(
    p: *const ::foo_proto::FooRequestStats,
) -> i32 {
    unsafe { ::rust_lib_golden::read_proto_pointer(p) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_read_uproto_uref(
    p: &'static ::foo_proto::FooRequestStats,
) -> i32 {
    unsafe { ::rust_lib_golden::read_proto_ref(p) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::alloc::vec::Vec<::foo_proto::FooRequestStats> as ::core::default::Default>::default(
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
    __self: &'static ::alloc::vec::Vec<::foo_proto::FooRequestStats>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::alloc::vec::Vec<::foo_proto::FooRequestStats> as ::core::clone::Clone>::clone(
                __self,
            );
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
    __self: &'static mut ::alloc::vec::Vec<::foo_proto::FooRequestStats>,
    source: &'static ::alloc::vec::Vec<::foo_proto::FooRequestStats>,
) -> () {
    unsafe {
        <::alloc::vec::Vec<::foo_proto::FooRequestStats> as ::core::clone::Clone>::clone_from(
            __self, source,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Drop_udrop_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
    vec: *mut ::alloc::vec::Vec<::foo_proto::FooRequestStats>,
) {
    unsafe { ::core::ptr::drop_in_place(vec) };
}
unsafe extern "C" {
    fn proto2_rust_thunk_Message_foo_service_FooRequestStats_crubit_cpp_to_rust_converter(
        cpp_in: *const core::ffi::c_void,
        rs_out: *mut core::ffi::c_void,
    );
    fn proto2_rust_thunk_Message_foo_service_FooRequestStats_crubit_rust_to_cpp_converter(
        rs_in: *const core::ffi::c_void,
        cpp_out: *mut core::ffi::c_void,
    );
}
