// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rust_lib_golden
// Features: do_not_hardcode_status_bridge, std_unique_ptr, std_vector, supported

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate foo_rust_proto as foo_proto;
const _: () = assert!(::std::mem::size_of::<::rust_lib_golden::FooService>() == 8);
const _: () = assert!(::std::mem::align_of::<::rust_lib_golden::FooService>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::rust_lib_golden::FooService as ::core::default::Default>::default();
        (__ret_ptr as *mut ::rust_lib_golden::FooService).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::rust_lib_golden::FooService>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_handle_urequest(
    __self: &'static mut ::rust_lib_golden::FooService,
    req: *const core::ffi::c_void,
    rsp: *const core::ffi::c_void,
) -> bool {
    unsafe {
        let req = {
            let mut __crubit_temp=::core::mem::MaybeUninit::<::foo_proto::internal_do_not_use_third__party_scrubit_scc__bindings__from__rs_stest_sbridging_sprotobuf_sfoo::FooRequestView<'static>>::uninit();
            __crubit_temp.write(::core::mem::transmute(req));
            __crubit_temp.assume_init()
        };
        let rsp = {
            let mut __crubit_temp=::core::mem::MaybeUninit::<::foo_proto::internal_do_not_use_third__party_scrubit_scc__bindings__from__rs_stest_sbridging_sprotobuf_sfoo::FooResponseMut<'static>>::uninit();
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
        (__ret_ptr as*mut::foo_proto::internal_do_not_use_third__party_scrubit_scc__bindings__from__rs_stest_sbridging_sprotobuf_sfoo::FooRequestStatsView<'static>).write(__rs_return_value);
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
            let mut __crubit_temp=::core::mem::MaybeUninit::<::foo_proto::internal_do_not_use_third__party_scrubit_scc__bindings__from__rs_stest_sbridging_sprotobuf_sfoo::FooRequestStats>::uninit();
            proto2_rust_thunk_Message_foo_service_FooRequestStats_crubit_cpp_to_rust_converter(
                updated_stats,
                __crubit_temp.as_mut_ptr() as *mut core::ffi::c_void,
            );
            __crubit_temp.assume_init()
        };
        ::rust_lib_golden::FooService::update_request_stats(__self, updated_stats)
    }
}
extern "C" {
    fn proto2_rust_thunk_Message_foo_service_FooRequestStats_crubit_cpp_to_rust_converter(
        cpp_in: *const core::ffi::c_void,
        rs_out: *mut core::ffi::c_void,
    );
    fn proto2_rust_thunk_Message_foo_service_FooRequestStats_crubit_rust_to_cpp_converter(
        rs_in: *const core::ffi::c_void,
        cpp_out: *mut core::ffi::c_void,
    );
}
