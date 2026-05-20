// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_index_golden
// Features: callables, supported, types

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::rs_index_golden::CustomIndex>() == 8);
const _: () = assert!(::std::mem::align_of::<::rs_index_golden::CustomIndex>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(index: usize, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::rs_index_golden::CustomIndex::new(index);
        (__ret_ptr as *mut ::rs_index_golden::CustomIndex).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::rs_index_golden::CustomIndex, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::rs_index_golden::Id>() == 4);
const _: () = assert!(::std::mem::align_of::<::rs_index_golden::Id>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(id: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::rs_index_golden::Id::new(id);
        (__ret_ptr as *mut ::rs_index_golden::Id).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::rs_index_golden::Id, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::rs_index_golden::IntPair>() == 8);
const _: () = assert!(::std::mem::align_of::<::rs_index_golden::IntPair>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(x: i32, y: i32, __ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::rs_index_golden::IntPair::new(x, y);
        (__ret_ptr as *mut ::rs_index_golden::IntPair).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_index_uusize(
    __self: &'static ::rs_index_golden::IntPair,
    index: usize,
) -> &'static i32 {
    unsafe { <::rs_index_golden::IntPair as ::core::ops::Index<usize>>::index(__self, index) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_index_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
    __self: &'static ::rs_index_golden::IntPair,
    index: &'static mut ::core::mem::MaybeUninit<::rs_index_golden::CustomIndex>,
) -> &'static i32 {
    unsafe {
        let index = index.assume_init_read();
        <::rs_index_golden::IntPair as ::core::ops::Index<::rs_index_golden::CustomIndex>>::index(
            __self, index,
        )
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_index_umut_uusize(
    __self: &'static mut ::rs_index_golden::IntPair,
    index: usize,
) -> &'static mut i32 {
    unsafe {
        <::rs_index_golden::IntPair as ::core::ops::IndexMut<usize>>::index_mut(__self, index)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_index_umut_urs_uindex_ugolden_x0000003a_x0000003aCustomIndex(
    __self: &'static mut ::rs_index_golden::IntPair,
    index: &'static mut ::core::mem::MaybeUninit<::rs_index_golden::CustomIndex>,
) -> &'static mut i32 {
    unsafe {
        let index = index.assume_init_read();
        <::rs_index_golden::IntPair as::core::ops::IndexMut<::rs_index_golden::CustomIndex>>::index_mut(__self,index)
    }
}
const _: () = assert!(::core::mem::offset_of!(::rs_index_golden::IntPair, x) == 0);
const _: () = assert!(::core::mem::offset_of!(::rs_index_golden::IntPair, y) == 4);
const _: () = assert!(::std::mem::size_of::<::rs_index_golden::Map>() == 32);
const _: () = assert!(::std::mem::align_of::<::rs_index_golden::Map>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::rs_index_golden::Map>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_new(
    row_size: usize,
    col_size: usize,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::rs_index_golden::Map::new(row_size, col_size);
        (__ret_ptr as *mut ::rs_index_golden::Map).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_index_u_x00000028usize_x0000002c_x00000020usize_x00000029(
    __self: &'static ::rs_index_golden::Map,
    index: *const [*const core::ffi::c_void; 2usize],
) -> &'static str {
    unsafe {
        let index = (
            {
                let index_0: usize = ((*index)[0usize] as *const usize).read();
                index_0
            },
            {
                let index_1: usize = ((*index)[1usize] as *const usize).read();
                index_1
            },
        );
        <::rs_index_golden::Map as ::core::ops::Index<(usize, usize)>>::index(__self, index)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_index_u_x00000026rs_uindex_ugolden_x0000003a_x0000003aId(
    __self: &'static ::rs_index_golden::Map,
    index: &'static ::rs_index_golden::Id,
) -> &'static str {
    unsafe {
        <::rs_index_golden::Map as ::core::ops::Index<&'static ::rs_index_golden::Id>>::index(
            __self, index,
        )
    }
}
