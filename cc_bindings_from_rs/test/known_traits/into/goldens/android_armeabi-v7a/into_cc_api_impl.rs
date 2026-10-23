// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// into_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::core::mem::size_of::<::into_golden::CloneAllocTarget>() == 12);
const _: () = assert!(::core::mem::align_of::<::into_golden::CloneAllocTarget>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::into_golden::CloneAllocTarget,
    __ret_ptr: *mut &'static str,
) -> () {
    unsafe {
        __ret_ptr.write(::into_golden::CloneAllocTarget::get_value(__self));
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CloneAllocTarget, value) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::CloneAllocType>() == 12);
const _: () = assert!(::core::mem::align_of::<::into_golden::CloneAllocType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: *mut &'static str,
    __ret_ptr: *mut ::into_golden::CloneAllocType,
) -> () {
    unsafe {
        let s = s.read();
        __ret_ptr.write(::into_golden::CloneAllocType::create(s));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::into_golden::CloneAllocType,
    __ret_ptr: *mut &'static str,
) -> () {
    unsafe {
        __ret_ptr.write(::into_golden::CloneAllocType::get_value(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCloneAllocType_uinto_ugolden_x0000003a_x0000003aCloneAllocTarget(
    __self: *mut ::into_golden::CloneAllocType,
    __ret_ptr: *mut ::into_golden::CloneAllocTarget,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::into_golden::CloneAllocType as ::core::convert::Into<
            ::into_golden::CloneAllocTarget,
        >>::into(__self));
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CloneAllocType, value) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::CloneCopyTarget>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::CloneCopyTarget>() == 4);
const _: () = assert!(::core::mem::offset_of!(::into_golden::CloneCopyTarget, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::CloneCopyType>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::CloneCopyType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCloneCopyType_uinto_ugolden_x0000003a_x0000003aCloneCopyTarget(
    __self: *mut ::into_golden::CloneCopyType,
    __ret_ptr: *mut ::into_golden::CloneCopyTarget,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::into_golden::CloneCopyType as ::core::convert::Into<
            ::into_golden::CloneCopyTarget,
        >>::into(__self));
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CloneCopyType, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::CollidingOperators>() == 8);
const _: () = assert!(::core::mem::align_of::<::into_golden::CollidingOperators>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCollidingOperators_uusize(
    __self: *mut ::into_golden::CollidingOperators,
) -> usize {
    unsafe {
        let __self = __self.read();
        <::into_golden::CollidingOperators as ::core::convert::Into<usize>>::into(__self)
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CollidingOperators, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::Convert>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::Convert>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui32(
    __self: *mut ::into_golden::Convert,
) -> i32 {
    unsafe {
        let __self = __self.read();
        <::into_golden::Convert as ::core::convert::Into<i32>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui64(
    __self: *mut ::into_golden::Convert,
) -> i64 {
    unsafe {
        let __self = __self.read();
        <::into_golden::Convert as ::core::convert::Into<i64>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_u_x00000026_x00000027static_x00000020str(
    __self: *mut ::into_golden::Convert,
    __ret_ptr: *mut &'static str,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr
            .write(<::into_golden::Convert as ::core::convert::Into<&'static str>>::into(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui16(
    __self: *mut ::into_golden::Convert,
) -> i16 {
    unsafe {
        let __self = __self.read();
        <::into_golden::Convert as ::core::convert::Into<i16>>::into(__self)
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::Convert, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::ConvertModule>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::ConvertModule>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertModule_ui32(
    __self: *mut ::into_golden::ConvertModule,
) -> i32 {
    unsafe {
        let __self = __self.read();
        <::into_golden::ConvertModule as ::core::convert::Into<i32>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertModule_ui64(
    __self: *mut ::into_golden::ConvertModule,
) -> i64 {
    unsafe {
        let __self = __self.read();
        <::into_golden::ConvertModule as ::core::convert::Into<i64>>::into(__self)
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::ConvertModule, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::ConvertRef>() == 8);
const _: () = assert!(::core::mem::align_of::<::into_golden::ConvertRef>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create_u_x00000027a(
    s: *mut &'static str,
    __ret_ptr: *mut ::into_golden::ConvertRef<'static>,
) -> () {
    unsafe {
        let s = s.read();
        __ret_ptr.write(::into_golden::ConvertRef::create(s));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_transmigrate_u_x00000027a(
    __self: *mut ::into_golden::ConvertRef<'static>,
    __ret_ptr: *mut ::into_golden::Convert,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(::into_golden::ConvertRef::transmigrate(__self));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_u_x00000026_x00000027a_x00000020str(
    __self: *mut ::into_golden::ConvertRef<'static>,
    __ret_ptr: *mut &'static str,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::into_golden::ConvertRef as ::core::convert::Into<&'static str>>::into(
            __self,
        ));
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_uinto_ugolden_x0000003a_x0000003aConvert(
    __self: *mut ::into_golden::ConvertRef<'static>,
    __ret_ptr: *mut ::into_golden::Convert,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::into_golden::ConvertRef as ::core::convert::Into<
            ::into_golden::Convert,
        >>::into(__self));
    }
}
const _: () = assert!(::core::mem::size_of::<::into_golden::LoopA>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::LoopA>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aLoopA_uinto_ugolden_x0000003a_x0000003aLoopB(
    __self: *mut ::into_golden::LoopA,
    __ret_ptr: *mut ::into_golden::LoopB,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(
            <::into_golden::LoopA as ::core::convert::Into<::into_golden::LoopB>>::into(__self),
        );
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::LoopA, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::LoopB>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::LoopB>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aLoopB_uinto_ugolden_x0000003a_x0000003aLoopA(
    __self: *mut ::into_golden::LoopB,
    __ret_ptr: *mut ::into_golden::LoopA,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(
            <::into_golden::LoopB as ::core::convert::Into<::into_golden::LoopA>>::into(__self),
        );
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::LoopB, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::NoCloneCopyDropTarget>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::NoCloneCopyDropTarget>() == 4);
const _: () = assert!(::core::mem::offset_of!(::into_golden::NoCloneCopyDropTarget, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::NoCloneCopyDropType>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::NoCloneCopyDropType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aNoCloneCopyDropType_uinto_ugolden_x0000003a_x0000003aNoCloneCopyDropTarget(
    __self: *mut ::into_golden::NoCloneCopyDropType,
    __ret_ptr: *mut ::into_golden::NoCloneCopyDropTarget,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::into_golden::NoCloneCopyDropType as ::core::convert::Into<
            ::into_golden::NoCloneCopyDropTarget,
        >>::into(__self));
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::NoCloneCopyDropType, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::NoCloneDefaultTarget>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::NoCloneDefaultTarget>() == 4);
const _: () = assert!(::core::mem::offset_of!(::into_golden::NoCloneDefaultTarget, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::NoCloneDefaultType>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::NoCloneDefaultType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultType_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultTarget(
    __self: *mut ::into_golden::NoCloneDefaultType,
    __ret_ptr: *mut ::into_golden::NoCloneDefaultTarget,
) -> () {
    unsafe {
        let __self = __self.read();
        __ret_ptr.write(<::into_golden::NoCloneDefaultType as ::core::convert::Into<
            ::into_golden::NoCloneDefaultTarget,
        >>::into(__self));
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::NoCloneDefaultType, 0) == 0);
const _: () = assert!(::core::mem::size_of::<::into_golden::NotFfiSafe>() == 4);
const _: () = assert!(::core::mem::align_of::<::into_golden::NotFfiSafe>() == 4);
