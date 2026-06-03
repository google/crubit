// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// from_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::from_golden::CloneAllocSource>() == 24);
const _: () = assert!(::std::mem::align_of::<::from_golden::CloneAllocSource>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::CloneAllocSource>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::from_golden::CloneAllocSource,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::from_golden::CloneAllocSource as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::from_golden::CloneAllocSource).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::from_golden::CloneAllocSource,
    source: &'static ::from_golden::CloneAllocSource,
) -> () {
    unsafe { <::from_golden::CloneAllocSource as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::from_golden::CloneAllocSource::create(s);
        (__ret_ptr as *mut ::from_golden::CloneAllocSource).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::from_golden::CloneAllocSource,
) -> &'static str {
    unsafe { ::from_golden::CloneAllocSource::get_value(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aCloneAllocType(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::CloneAllocSource>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::from_golden::CloneAllocSource as ::core::convert::Into<
            ::from_golden::CloneAllocType,
        >>::into(__self);
        (__ret_ptr as *mut ::from_golden::CloneAllocType).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::CloneAllocSource, value) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::CloneAllocType>() == 24);
const _: () = assert!(::std::mem::align_of::<::from_golden::CloneAllocType>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_drop(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::CloneAllocType>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone(
    __self: &'static ::from_golden::CloneAllocType,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::from_golden::CloneAllocType as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::from_golden::CloneAllocType).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_clone_ufrom(
    __self: &'static mut ::from_golden::CloneAllocType,
    source: &'static ::from_golden::CloneAllocType,
) -> () {
    unsafe { <::from_golden::CloneAllocType as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::from_golden::CloneAllocType,
) -> &'static str {
    unsafe { ::from_golden::CloneAllocType::get_value(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aCloneAllocSource(
    value: &'static mut ::core::mem::MaybeUninit<::from_golden::CloneAllocSource>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.assume_init_read();
        let __rs_return_value = <::from_golden::CloneAllocType as ::core::convert::From<
            ::from_golden::CloneAllocSource,
        >>::from(value);
        (__ret_ptr as *mut ::from_golden::CloneAllocType).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::CloneAllocType, value) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::CloneCopySource>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::CloneCopySource>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aCloneCopyType(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::CloneCopySource>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::from_golden::CloneCopySource as ::core::convert::Into<
            ::from_golden::CloneCopyType,
        >>::into(__self);
        (__ret_ptr as *mut ::from_golden::CloneCopyType).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::CloneCopySource, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::CloneCopyType>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::CloneCopyType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::from_golden::CloneCopyType as ::core::default::Default>::default();
        (__ret_ptr as *mut ::from_golden::CloneCopyType).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aCloneCopySource(
    value: &'static mut ::core::mem::MaybeUninit<::from_golden::CloneCopySource>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.assume_init_read();
        let __rs_return_value = <::from_golden::CloneCopyType as ::core::convert::From<
            ::from_golden::CloneCopySource,
        >>::from(value);
        (__ret_ptr as *mut ::from_golden::CloneCopyType).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::CloneCopyType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::LoopA>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::LoopA>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aLoopB(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::LoopA>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            <::from_golden::LoopA as ::core::convert::Into<::from_golden::LoopB>>::into(__self);
        (__ret_ptr as *mut ::from_golden::LoopB).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aLoopB(
    value: &'static mut ::core::mem::MaybeUninit<::from_golden::LoopB>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.assume_init_read();
        let __rs_return_value =
            <::from_golden::LoopA as ::core::convert::From<::from_golden::LoopB>>::from(value);
        (__ret_ptr as *mut ::from_golden::LoopA).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::LoopA, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::LoopB>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::LoopB>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aLoopA(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::LoopB>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            <::from_golden::LoopB as ::core::convert::Into<::from_golden::LoopA>>::into(__self);
        (__ret_ptr as *mut ::from_golden::LoopA).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aLoopA(
    value: &'static mut ::core::mem::MaybeUninit<::from_golden::LoopA>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.assume_init_read();
        let __rs_return_value =
            <::from_golden::LoopB as ::core::convert::From<::from_golden::LoopA>>::from(value);
        (__ret_ptr as *mut ::from_golden::LoopB).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::LoopB, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NoCloneCopyDropSource>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NoCloneCopyDropSource>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropType(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::NoCloneCopyDropSource>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::from_golden::NoCloneCopyDropSource as ::core::convert::Into<
            ::from_golden::NoCloneCopyDropType,
        >>::into(__self);
        (__ret_ptr as *mut ::from_golden::NoCloneCopyDropType).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::NoCloneCopyDropSource, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NoCloneCopyDropType>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NoCloneCopyDropType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropSource(
    value: &'static mut ::core::mem::MaybeUninit<::from_golden::NoCloneCopyDropSource>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.assume_init_read();
        let __rs_return_value = <::from_golden::NoCloneCopyDropType as ::core::convert::From<
            ::from_golden::NoCloneCopyDropSource,
        >>::from(value);
        (__ret_ptr as *mut ::from_golden::NoCloneCopyDropType).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::NoCloneCopyDropType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NoCloneDefaultSource>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NoCloneDefaultSource>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::from_golden::NoCloneDefaultSource as ::core::default::Default>::default();
        (__ret_ptr as *mut ::from_golden::NoCloneDefaultSource).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultType(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::NoCloneDefaultSource>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::from_golden::NoCloneDefaultSource as ::core::convert::Into<
            ::from_golden::NoCloneDefaultType,
        >>::into(__self);
        (__ret_ptr as *mut ::from_golden::NoCloneDefaultType).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::NoCloneDefaultSource, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NoCloneDefaultType>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NoCloneDefaultType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_default(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value =
            <::from_golden::NoCloneDefaultType as ::core::default::Default>::default();
        (__ret_ptr as *mut ::from_golden::NoCloneDefaultType).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultSource(
    value: &'static mut ::core::mem::MaybeUninit<::from_golden::NoCloneDefaultSource>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.assume_init_read();
        let __rs_return_value = <::from_golden::NoCloneDefaultType as ::core::convert::From<
            ::from_golden::NoCloneDefaultSource,
        >>::from(value);
        (__ret_ptr as *mut ::from_golden::NoCloneDefaultType).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::NoCloneDefaultType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NotFfiSafe>() == 8);
const _: () = assert!(::std::mem::align_of::<::from_golden::NotFfiSafe>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::from_golden::NotFfiSafe::create();
        (__ret_ptr as *mut ::from_golden::NotFfiSafe).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::NotFfiSafe>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::from_golden::NotFfiSafe as ::core::convert::Into<i32>>::into(__self)
    }
}
const _: () = assert!(::std::mem::size_of::<::from_golden::Opaque>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::Opaque>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::Opaque>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::from_golden::Opaque as ::core::convert::Into<i32>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui64(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::Opaque>,
) -> i64 {
    unsafe {
        let __self = __self.assume_init_read();
        <::from_golden::Opaque as ::core::convert::Into<i64>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_u_x00000026_x00000027static_x00000020str(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::Opaque>,
) -> &'static str {
    unsafe {
        let __self = __self.assume_init_read();
        <::from_golden::Opaque as ::core::convert::Into<&'static str>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ui16(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::Opaque>,
) -> i16 {
    unsafe {
        let __self = __self.assume_init_read();
        <::from_golden::Opaque as ::core::convert::Into<i16>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027static_x0000003e(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::Opaque>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::from_golden::Opaque as ::core::convert::Into<
            ::from_golden::OpaqueRef<'static>,
        >>::into(__self);
        (__ret_ptr as *mut ::from_golden::OpaqueRef<'static>).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::Opaque, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::OpaqueRef>() == 16);
const _: () = assert!(::std::mem::align_of::<::from_golden::OpaqueRef>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::from_golden::OpaqueRef::create(s);
        (__ret_ptr as *mut ::from_golden::OpaqueRef<'static>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uarg(
    __self: &'static ::from_golden::OpaqueRef<'static>,
) -> &'static str {
    unsafe { ::from_golden::OpaqueRef::get_arg(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_u_x00000026_x00000027a_x00000020str(
    __self: &'static mut ::core::mem::MaybeUninit<::from_golden::OpaqueRef<'static>>,
) -> &'static str {
    unsafe {
        let __self = __self.assume_init_read();
        <::from_golden::OpaqueRef as ::core::convert::Into<&'static str>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_from_ufrom_ugolden_x0000003a_x0000003aOpaque(
    value: &'static mut ::core::mem::MaybeUninit<::from_golden::Opaque>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.assume_init_read();
        let __rs_return_value =
            <::from_golden::OpaqueRef as ::core::convert::From<::from_golden::Opaque>>::from(value);
        (__ret_ptr as *mut ::from_golden::OpaqueRef<'static>).write(__rs_return_value);
    }
}
