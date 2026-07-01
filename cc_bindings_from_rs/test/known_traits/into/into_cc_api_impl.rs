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
const _: () = assert!(::std::mem::size_of::<::into_golden::CloneAllocTarget>() == 24);
const _: () = assert!(::std::mem::align_of::<::into_golden::CloneAllocTarget>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uinto_ugolden_x0000003a_x0000003aCloneAllocTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::CloneAllocTarget>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_uinto_ugolden_x0000003a_x0000003aCloneAllocTarget(
    __self: &'static ::into_golden::CloneAllocTarget,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_golden::CloneAllocTarget as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::into_golden::CloneAllocTarget).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_uinto_ugolden_x0000003a_x0000003aCloneAllocTarget(
    __self: &'static mut ::into_golden::CloneAllocTarget,
    source: &'static ::into_golden::CloneAllocTarget,
) -> () {
    unsafe { <::into_golden::CloneAllocTarget as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::into_golden::CloneAllocTarget,
) -> &'static str {
    unsafe { ::into_golden::CloneAllocTarget::get_value(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_into_ugolden_x0000003a_x0000003aCloneAllocType_as_into_ugolden_x0000003a_x0000003aCloneAllocTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::CloneAllocType>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::CloneAllocType as ::core::convert::Into<
            ::into_golden::CloneAllocTarget,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::CloneAllocTarget).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CloneAllocTarget, value) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::CloneAllocType>() == 24);
const _: () = assert!(::std::mem::align_of::<::into_golden::CloneAllocType>() == 8);
#[unsafe(no_mangle)]
extern "C" fn __crubit_thunk_Drop_udrop_uinto_ugolden_x0000003a_x0000003aCloneAllocType(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::CloneAllocType>,
) {
    unsafe { __self.assume_init_drop() };
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_uinto_ugolden_x0000003a_x0000003aCloneAllocType(
    __self: &'static ::into_golden::CloneAllocType,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_golden::CloneAllocType as ::core::clone::Clone>::clone(__self);
        (__ret_ptr as *mut ::into_golden::CloneAllocType).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Clone_uclone_ufrom_uinto_ugolden_x0000003a_x0000003aCloneAllocType(
    __self: &'static mut ::into_golden::CloneAllocType,
    source: &'static ::into_golden::CloneAllocType,
) -> () {
    unsafe { <::into_golden::CloneAllocType as ::core::clone::Clone>::clone_from(__self, source) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::into_golden::CloneAllocType::create(s);
        (__ret_ptr as *mut ::into_golden::CloneAllocType).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_get_uvalue(
    __self: &'static ::into_golden::CloneAllocType,
) -> &'static str {
    unsafe { ::into_golden::CloneAllocType::get_value(__self) }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCloneAllocType_uinto_ugolden_x0000003a_x0000003aCloneAllocTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::CloneAllocType>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::CloneAllocType as ::core::convert::Into<
            ::into_golden::CloneAllocTarget,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::CloneAllocTarget).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CloneAllocType, value) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::CloneCopyTarget>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::CloneCopyTarget>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_into_ugolden_x0000003a_x0000003aCloneCopyType_as_into_ugolden_x0000003a_x0000003aCloneCopyTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::CloneCopyType>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::CloneCopyType as ::core::convert::Into<
            ::into_golden::CloneCopyTarget,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::CloneCopyTarget).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CloneCopyTarget, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::CloneCopyType>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::CloneCopyType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_uinto_ugolden_x0000003a_x0000003aCloneCopyType(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_golden::CloneCopyType as ::core::default::Default>::default();
        (__ret_ptr as *mut ::into_golden::CloneCopyType).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCloneCopyType_uinto_ugolden_x0000003a_x0000003aCloneCopyTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::CloneCopyType>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::CloneCopyType as ::core::convert::Into<
            ::into_golden::CloneCopyTarget,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::CloneCopyTarget).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CloneCopyType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::CollidingOperators>() == 8);
const _: () = assert!(::std::mem::align_of::<::into_golden::CollidingOperators>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCollidingOperators_uusize(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::CollidingOperators>,
) -> usize {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::CollidingOperators as ::core::convert::Into<usize>>::into(__self)
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::CollidingOperators, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::Convert>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::Convert>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::Convert>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::Convert as ::core::convert::Into<i32>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui64(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::Convert>,
) -> i64 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::Convert as ::core::convert::Into<i64>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_u_x00000026_x00000027static_x00000020str(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::Convert>,
) -> &'static str {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::Convert as ::core::convert::Into<&'static str>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui16(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::Convert>,
) -> i16 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::Convert as ::core::convert::Into<i16>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_into_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_as_into_ugolden_x0000003a_x0000003aConvert(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertRef<'static>>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::ConvertRef<'static> as ::core::convert::Into<
            ::into_golden::Convert,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::Convert).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::Convert, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::ConvertModule>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::ConvertModule>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertModule_ui32(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertModule>,
) -> i32 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::ConvertModule as ::core::convert::Into<i32>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertModule_ui64(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertModule>,
) -> i64 {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::ConvertModule as ::core::convert::Into<i64>>::into(__self)
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::ConvertModule, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::ConvertRef>() == 16);
const _: () = assert!(::std::mem::align_of::<::into_golden::ConvertRef>() == 8);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_create(
    s: &'static str,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::into_golden::ConvertRef::create(s);
        (__ret_ptr as *mut ::into_golden::ConvertRef<'static>).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_transmigrate(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertRef<'static>>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = ::into_golden::ConvertRef::transmigrate(__self);
        (__ret_ptr as *mut ::into_golden::Convert).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_u_x00000026_x00000027a_x00000020str(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertRef<'static>>,
) -> &'static str {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_golden::ConvertRef as ::core::convert::Into<&'static str>>::into(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_uinto_ugolden_x0000003a_x0000003aConvert(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::ConvertRef<'static>>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::ConvertRef as ::core::convert::Into<
            ::into_golden::Convert,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::Convert).write(__rs_return_value);
    }
}
const _: () = assert!(::std::mem::size_of::<::into_golden::LoopA>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::LoopA>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aLoopA_uinto_ugolden_x0000003a_x0000003aLoopB(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::LoopA>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            <::into_golden::LoopA as ::core::convert::Into<::into_golden::LoopB>>::into(__self);
        (__ret_ptr as *mut ::into_golden::LoopB).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_into_ugolden_x0000003a_x0000003aLoopB_as_into_ugolden_x0000003a_x0000003aLoopA(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::LoopB>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            <::into_golden::LoopB as ::core::convert::Into<::into_golden::LoopA>>::into(__self);
        (__ret_ptr as *mut ::into_golden::LoopA).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::LoopA, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::LoopB>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::LoopB>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aLoopB_uinto_ugolden_x0000003a_x0000003aLoopA(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::LoopB>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            <::into_golden::LoopB as ::core::convert::Into<::into_golden::LoopA>>::into(__self);
        (__ret_ptr as *mut ::into_golden::LoopA).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_into_ugolden_x0000003a_x0000003aLoopA_as_into_ugolden_x0000003a_x0000003aLoopB(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::LoopA>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            <::into_golden::LoopA as ::core::convert::Into<::into_golden::LoopB>>::into(__self);
        (__ret_ptr as *mut ::into_golden::LoopB).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::LoopB, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::NoCloneCopyDropTarget>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::NoCloneCopyDropTarget>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_into_ugolden_x0000003a_x0000003aNoCloneCopyDropType_as_into_ugolden_x0000003a_x0000003aNoCloneCopyDropTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::NoCloneCopyDropType>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::NoCloneCopyDropType as ::core::convert::Into<
            ::into_golden::NoCloneCopyDropTarget,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::NoCloneCopyDropTarget).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::NoCloneCopyDropTarget, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::NoCloneCopyDropType>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::NoCloneCopyDropType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aNoCloneCopyDropType_uinto_ugolden_x0000003a_x0000003aNoCloneCopyDropTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::NoCloneCopyDropType>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::NoCloneCopyDropType as ::core::convert::Into<
            ::into_golden::NoCloneCopyDropTarget,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::NoCloneCopyDropTarget).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::NoCloneCopyDropType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::NoCloneDefaultTarget>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::NoCloneDefaultTarget>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultTarget(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_golden::NoCloneDefaultTarget as ::core::default::Default>::default();
        (__ret_ptr as *mut ::into_golden::NoCloneDefaultTarget).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_into_ugolden_x0000003a_x0000003aNoCloneDefaultType_as_into_ugolden_x0000003a_x0000003aNoCloneDefaultTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::NoCloneDefaultType>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::NoCloneDefaultType as ::core::convert::Into<
            ::into_golden::NoCloneDefaultTarget,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::NoCloneDefaultTarget).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::NoCloneDefaultTarget, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::NoCloneDefaultType>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_golden::NoCloneDefaultType>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Default_udefault_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultType(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_golden::NoCloneDefaultType as ::core::default::Default>::default();
        (__ret_ptr as *mut ::into_golden::NoCloneDefaultType).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultType_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultTarget(
    __self: &'static mut ::core::mem::MaybeUninit<::into_golden::NoCloneDefaultType>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value = <::into_golden::NoCloneDefaultType as ::core::convert::Into<
            ::into_golden::NoCloneDefaultTarget,
        >>::into(__self);
        (__ret_ptr as *mut ::into_golden::NoCloneDefaultTarget).write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_golden::NoCloneDefaultType, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::into_golden::NotFfiSafe>() == 8);
const _: () = assert!(::std::mem::align_of::<::into_golden::NotFfiSafe>() == 8);
