// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// boxes_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::core::mem::size_of::<::boxes_golden::Holder>() == 8);
const _: () = assert!(::core::mem::align_of::<::boxes_golden::Holder>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_uboxes_ugolden_x0000003a_x0000003aHolder(
    __ret_ptr: *mut ::boxes_golden::Holder,
) -> () {
    unsafe {
        __ret_ptr.write(<::boxes_golden::Holder as ::core::default::Default>::default());
    }
}
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aHolder(
    __self: *mut ::boxes_golden::Holder,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () = assert!(::core::mem::offset_of!(::boxes_golden::Holder, b) == 0);
const _: () = assert!(::core::mem::size_of::<::boxes_golden::NotUnpin>() == 4);
const _: () = assert!(::core::mem::align_of::<::boxes_golden::NotUnpin>() == 4);
const _: () = assert!(::core::mem::offset_of!(::boxes_golden::NotUnpin, val) == 0);
const _: () = assert!(::core::mem::size_of::<::boxes_golden::OptHolder>() == 8);
const _: () = assert!(::core::mem::align_of::<::boxes_golden::OptHolder>() == 8);
const _: () = assert!(::core::mem::offset_of!(::boxes_golden::OptHolder, o) == 0);
const _: () = assert!(::core::mem::size_of::<::boxes_golden::Point>() == 8);
const _: () = assert!(::core::mem::align_of::<::boxes_golden::Point>() == 4);
const _: () = assert!(::core::mem::offset_of!(::boxes_golden::Point, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::boxes_golden::Point, y) == 4);
const _: () = assert!(::core::mem::size_of::<::boxes_golden::Tracked>() == 8);
const _: () = assert!(::core::mem::align_of::<::boxes_golden::Tracked>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aTracked(
    __self: *mut ::boxes_golden::Tracked,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_uboxes_ugolden_x0000003a_x0000003aTracked(
    __self: &'static ::boxes_golden::Tracked,
    __ret_ptr: *mut ::boxes_golden::Tracked,
) -> () {
    unsafe {
        __ret_ptr.write(<::boxes_golden::Tracked as ::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_uboxes_ugolden_x0000003a_x0000003aTracked(
    __self: &'static mut ::boxes_golden::Tracked,
    source: &'static ::boxes_golden::Tracked,
) -> () {
    unsafe { <::boxes_golden::Tracked as ::core::clone::Clone>::clone_from(__self, source) }
}
const _: () = assert!(::core::mem::size_of::<::boxes_golden::UnmovableHolder>() == 8);
const _: () = assert!(::core::mem::align_of::<::boxes_golden::UnmovableHolder>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aUnmovableHolder(
    __self: *mut ::boxes_golden::UnmovableHolder,
) {
    unsafe { ::core::ptr::drop_in_place(__self) };
}
const _: () = assert!(::core::mem::offset_of!(::boxes_golden::UnmovableHolder, b) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_uhandle(
    t: &'static ::boxes_golden::Tracked,
    __ret_ptr: *mut ::boxes_golden::Tracked,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::clone_handle(t));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_ubox(
    b: *mut ::alloc::boxed::Box<i32, ::alloc::alloc::Global>,
) -> i32 {
    unsafe {
        let b = b.read();
        ::boxes_golden::consume_box(b)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_uopt_ubox(
    o: *mut ::core::option::Option<::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
) -> i32 {
    unsafe {
        let o = o.read();
        ::boxes_golden::consume_opt_box(o)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_uslice_ubox(
    b: *mut ::alloc::boxed::Box<[i32], ::alloc::alloc::Global>,
) -> usize {
    unsafe {
        let b = b.read();
        ::boxes_golden::consume_slice_box(b)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_ustr_ubox(
    s: *mut ::alloc::boxed::Box<str, ::alloc::alloc::Global>,
) -> usize {
    unsafe {
        let s = s.read();
        ::boxes_golden::consume_str_box(s)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_consume_utracked(
    _b: *mut ::alloc::boxed::Box<::boxes_golden::Tracked, ::alloc::alloc::Global>,
) -> () {
    unsafe {
        let _b = _b.read();
        ::boxes_golden::consume_tracked(_b)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_holder_uvalue(h: &'static ::boxes_golden::Holder) -> i32 {
    unsafe { ::boxes_golden::holder_value(h) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_ubox(
    x: i32,
    __ret_ptr: *mut ::alloc::boxed::Box<i32, ::alloc::alloc::Global>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_box(x));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_ubox_unot_uunpin(
    __ret_ptr: *mut ::alloc::boxed::Box<::boxes_golden::NotUnpin, ::alloc::alloc::Global>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_box_not_unpin());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uboxed_upoint(
    __ret_ptr: *mut ::alloc::boxed::Box<::boxes_golden::Point, ::alloc::alloc::Global>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_boxed_point());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uholder(__ret_ptr: *mut ::boxes_golden::Holder) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_holder());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uopt_ubox(
    some: bool,
    __ret_ptr: *mut ::core::option::Option<::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_opt_box(some));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uopt_ubox_uslice(
    some: bool,
    __ret_ptr: *mut ::core::option::Option<::alloc::boxed::Box<[i32], ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_opt_box_slice(some));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uopt_ubox_ustr(
    some: bool,
    __ret_ptr: *mut ::core::option::Option<::alloc::boxed::Box<str, ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_opt_box_str(some));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uopt_upin_ubox(
    some: bool,
    __ret_ptr: *mut ::core::option::Option<
        ::core::pin::Pin<::alloc::boxed::Box<::boxes_golden::NotUnpin, ::alloc::alloc::Global>>,
    >,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_opt_pin_box(some));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_upin_ubox(
    __ret_ptr: *mut ::core::pin::Pin<::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_pin_box());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_upin_ubox_unot_uunpin(
    __ret_ptr: *mut ::core::pin::Pin<
        ::alloc::boxed::Box<::boxes_golden::NotUnpin, ::alloc::alloc::Global>,
    >,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_pin_box_not_unpin());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uslice_ubox(
    n: usize,
    __ret_ptr: *mut ::alloc::boxed::Box<[i32], ::alloc::alloc::Global>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_slice_box(n));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_ustr_ubox(
    __ret_ptr: *mut ::alloc::boxed::Box<str, ::alloc::alloc::Global>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_str_box());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_utracked_ubox(
    __ret_ptr: *mut ::alloc::boxed::Box<::boxes_golden::Tracked, ::alloc::alloc::Global>,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_tracked_box());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uunmovable_uholder(
    __ret_ptr: *mut ::boxes_golden::UnmovableHolder,
) -> () {
    unsafe {
        __ret_ptr.write(::boxes_golden::make_unmovable_holder());
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_mutate_ubox(
    b: &'static mut ::alloc::boxed::Box<i32, ::alloc::alloc::Global>,
) -> () {
    unsafe { ::boxes_golden::mutate_box(b) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_opt_ubox_uvalue(
    o: *mut ::core::option::Option<&'static ::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
) -> i32 {
    unsafe {
        let o = o.read();
        ::boxes_golden::opt_box_value(o)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_point_ux(
    b: *mut ::alloc::boxed::Box<::boxes_golden::Point, ::alloc::alloc::Global>,
) -> i32 {
    unsafe {
        let b = b.read();
        ::boxes_golden::point_x(b)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_read_ubox(
    b: &'static ::alloc::boxed::Box<i32, ::alloc::alloc::Global>,
) -> i32 {
    unsafe { ::boxes_golden::read_box(b) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_read_upin_ubox(
    b: &'static ::core::pin::Pin<::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
) -> i32 {
    unsafe { ::boxes_golden::read_pin_box(b) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_roundtrip_ubox(
    b: *mut ::alloc::boxed::Box<i32, ::alloc::alloc::Global>,
    __ret_ptr: *mut ::alloc::boxed::Box<i32, ::alloc::alloc::Global>,
) -> () {
    unsafe {
        let b = b.read();
        __ret_ptr.write(::boxes_golden::roundtrip_box(b));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_str_ubox_ulen(
    s: &'static ::alloc::boxed::Box<str, ::alloc::alloc::Global>,
) -> usize {
    unsafe { ::boxes_golden::str_box_len(s) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_sum_uslice_ubox(
    b: &'static ::alloc::boxed::Box<[i32], ::alloc::alloc::Global>,
) -> i32 {
    unsafe { ::boxes_golden::sum_slice_box(b) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_tracked_ustrong_ucount(
    t: &'static ::boxes_golden::Tracked,
) -> usize {
    unsafe { ::boxes_golden::tracked_strong_count(t) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003c_x0000005bi32_x0000005d_x0000003e_x0000003e(
    __self: &'static ::core::option::Option<::alloc::boxed::Box<[i32], ::alloc::alloc::Global>>,
    __ret_ptr: *mut ::core::option::Option<::alloc::boxed::Box<[i32], ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        __ret_ptr.write(<core::option::Option<::alloc::boxed::Box<[i32],::alloc::alloc::Global>>as::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003c_x0000005bi32_x0000005d_x0000003e_x0000003e(
    __self: &'static mut ::core::option::Option<::alloc::boxed::Box<[i32], ::alloc::alloc::Global>>,
    source: &'static ::core::option::Option<::alloc::boxed::Box<[i32], ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        <core::option::Option<::alloc::boxed::Box<[i32],::alloc::alloc::Global>>as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003ci32_x0000003e_x0000003e(
    __self: &'static ::core::option::Option<::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
    __ret_ptr: *mut ::core::option::Option<::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        __ret_ptr.write(<core::option::Option<::alloc::boxed::Box<i32,::alloc::alloc::Global>>as::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003ci32_x0000003e_x0000003e(
    __self: &'static mut ::core::option::Option<::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
    source: &'static ::core::option::Option<::alloc::boxed::Box<i32, ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        <core::option::Option<::alloc::boxed::Box<i32,::alloc::alloc::Global>>as::core::clone::Clone>::clone_from(__self,source)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003cstr_x0000003e_x0000003e(
    __self: &'static ::core::option::Option<::alloc::boxed::Box<str, ::alloc::alloc::Global>>,
    __ret_ptr: *mut ::core::option::Option<::alloc::boxed::Box<str, ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        __ret_ptr.write(<core::option::Option<::alloc::boxed::Box<str,::alloc::alloc::Global>>as::core::clone::Clone>::clone(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003cstr_x0000003e_x0000003e(
    __self: &'static mut ::core::option::Option<::alloc::boxed::Box<str, ::alloc::alloc::Global>>,
    source: &'static ::core::option::Option<::alloc::boxed::Box<str, ::alloc::alloc::Global>>,
) -> () {
    unsafe {
        <core::option::Option<::alloc::boxed::Box<str,::alloc::alloc::Global>>as::core::clone::Clone>::clone_from(__self,source)
    }
}
