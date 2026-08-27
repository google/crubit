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
const _: () = assert!(::std::mem::size_of::<::from_golden::CloneAllocSource>() == 12);
const _: () = assert!(::std::mem::align_of::<::from_golden::CloneAllocSource>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: *mut &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let s = s.read();
        let __rs_return_value = ::from_golden::CloneAllocSource::create(s);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::from_golden::CloneAllocSource,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::from_golden::CloneAllocSource::get_value(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aCloneAllocSource_ufrom_ugolden_x0000003a_x0000003aCloneAllocType(
    __self: *mut ::from_golden::CloneAllocSource,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value = <::from_golden::CloneAllocSource as ::core::convert::Into<
            ::from_golden::CloneAllocType,
        >>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::CloneAllocSource, value) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::CloneAllocType>() == 12);
const _: () = assert!(::std::mem::align_of::<::from_golden::CloneAllocType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::from_golden::CloneAllocType,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::from_golden::CloneAllocType::get_value(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::CloneAllocType, value) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::CloneCopySource>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::CloneCopySource>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aCloneCopySource_ufrom_ugolden_x0000003a_x0000003aCloneCopyType(
    __self: *mut ::from_golden::CloneCopySource,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value = <::from_golden::CloneCopySource as ::core::convert::Into<
            ::from_golden::CloneCopyType,
        >>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::CloneCopySource, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::CloneCopyType>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::CloneCopyType>() == 4);
const _: () = assert!(::core::mem::offset_of!(::from_golden::CloneCopyType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::CollidingConstructor>() == 8);
const _: () = assert!(::std::mem::align_of::<::from_golden::CollidingConstructor>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_From_ufrom_ufrom_ugolden_x0000003a_x0000003aCollidingConstructor_uusize(
    value: usize,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::from_golden::CollidingConstructor as ::core::convert::From<usize>>::from(value);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::from_golden::LoopA>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::LoopA>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aLoopA_ufrom_ugolden_x0000003a_x0000003aLoopB(
    __self: *mut ::from_golden::LoopA,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value =
            <::from_golden::LoopA as ::core::convert::Into<::from_golden::LoopB>>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::LoopA, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::LoopB>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::LoopB>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aLoopB_ufrom_ugolden_x0000003a_x0000003aLoopA(
    __self: *mut ::from_golden::LoopB,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value =
            <::from_golden::LoopB as ::core::convert::Into<::from_golden::LoopA>>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::LoopB, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NoCloneCopyDropSource>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NoCloneCopyDropSource>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropSource_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropType(
    __self: *mut ::from_golden::NoCloneCopyDropSource,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value = <::from_golden::NoCloneCopyDropSource as ::core::convert::Into<
            ::from_golden::NoCloneCopyDropType,
        >>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::NoCloneCopyDropSource, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NoCloneCopyDropType>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NoCloneCopyDropType>() == 4);
const _: () = assert!(::core::mem::offset_of!(::from_golden::NoCloneCopyDropType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NoCloneDefaultSource>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NoCloneDefaultSource>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultSource_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultType(
    __self: *mut ::from_golden::NoCloneDefaultSource,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value = <::from_golden::NoCloneDefaultSource as ::core::convert::Into<
            ::from_golden::NoCloneDefaultType,
        >>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::NoCloneDefaultSource, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NoCloneDefaultType>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NoCloneDefaultType>() == 4);
const _: () = assert!(::core::mem::offset_of!(::from_golden::NoCloneDefaultType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::NotFfiSafe>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::NotFfiSafe>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(__ret_ptr: *mut core::ffi::c_void) -> () {
    unsafe {
        let __rs_return_value = ::from_golden::NotFfiSafe::create();
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNotFfiSafe_ui32(
    __self: *mut ::from_golden::NotFfiSafe,
) -> i32 {
    unsafe {
        let __self = __self.read();
        <::from_golden::NotFfiSafe as ::core::convert::Into<i32>>::into(__self)
    }
}
const _: () = assert!(::std::mem::size_of::<::from_golden::Opaque>() == 4);
const _: () = assert!(::std::mem::align_of::<::from_golden::Opaque>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui32(
    __self: *mut ::from_golden::Opaque,
) -> i32 {
    unsafe {
        let __self = __self.read();
        <::from_golden::Opaque as ::core::convert::Into<i32>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui64(
    __self: *mut ::from_golden::Opaque,
) -> i64 {
    unsafe {
        let __self = __self.read();
        <::from_golden::Opaque as ::core::convert::Into<i64>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_u_x00000026_x00000027static_x00000020str(
    __self: *mut ::from_golden::Opaque,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value =
            <::from_golden::Opaque as ::core::convert::Into<&'static str>>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui16(
    __self: *mut ::from_golden::Opaque,
) -> i16 {
    unsafe {
        let __self = __self.read();
        <::from_golden::Opaque as ::core::convert::Into<i16>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027static_x0000003e(
    __self: *mut ::from_golden::Opaque,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value = <::from_golden::Opaque as ::core::convert::Into<
            ::from_golden::OpaqueRef<'static>,
        >>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::from_golden::Opaque, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::from_golden::OpaqueRef>() == 8);
const _: () = assert!(::std::mem::align_of::<::from_golden::OpaqueRef>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: *mut &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let s = s.read();
        let __rs_return_value = ::from_golden::OpaqueRef::create(s);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uarg(
    __self: &'static ::from_golden::OpaqueRef<'static>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::from_golden::OpaqueRef::get_arg(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027_u_x0000003e_u_x00000026_x00000027a_x00000020str(
    __self: *mut ::from_golden::OpaqueRef<'static>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.read();
        let __rs_return_value =
            <::from_golden::OpaqueRef as ::core::convert::Into<&'static str>>::into(__self);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_From_ufrom_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027_u_x0000003e_ufrom_ugolden_x0000003a_x0000003aOpaque(
    value: *mut ::from_golden::Opaque,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let value = value.read();
        let __rs_return_value =
            <::from_golden::OpaqueRef as ::core::convert::From<::from_golden::Opaque>>::from(value);
        ::core::ptr::write(__ret_ptr as *mut _, __rs_return_value);
    }
}
