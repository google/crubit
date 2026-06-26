// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/special_linking:special_linking

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// error: function `weak_import_func` could not be bound
//   Function is weakly imported

/// This test function is only available in Android API 9999+.
/// If we compile targeting Android API 23, this is a weak import
/// (so the caveats of `weak_import_func` apply).
///
/// This problem was discovered when trying to enable build tests targeting
/// Android in cl/937668344.
///
/// Original scenario: Functions like `sighold`, `sigignore`, `sigpause`,
/// `sigrelse`, and `sigset` in Bionic's `<signal.h>` are introduced in API 26.
/// When compiling targeting API 23 with
/// `__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__` defined (standard for NDK), Clang
/// treats them as weakly imported (unavailable, but compile-able as weak).
/// See Bionic's signal.h:
/// https://github.com/aosp-mirror/platform_bionic/blob/android-14.0.0_r1/libc/include/signal.h
#[inline(always)]
pub fn min_android_version_9999() {
    unsafe { crate::detail::__rust_thunk___Z24min_android_version_9999v() }
}

/// The problem replicated by the functions below was originally discovered when
/// trying to enable build tests targeting Android in cl/937668344.
///
/// This test replicates Android's Bionic `mallinfo` / `mallinfo2` symbol
/// redirection conflict. Original scenario: Bionic's NDK header `<malloc.h>`
/// declares (with the help of a `__RENAME` macro):
///   struct mallinfo mallinfo(void);
///   struct mallinfo2 mallinfo2(void) __asm__("mallinfo");
/// This renames `mallinfo2` at the assembler level to `mallinfo` to reuse the
/// same underlying symbol. See Bionic's malloc.h:
/// https://github.com/aosp-mirror/platform_bionic/blob/android-14.0.0_r1/libc/include/malloc.h
///
/// See also GCC documentation about controlling names in assembler code via
/// `__asm__`: https://gcc.gnu.org/onlinedocs/gcc/Asm-Labels.html
///
/// Crubit's rs_bindings_from_cc generates C++ thunk names based on the assembler
/// linkage name of the C++ functions. If two functions share the same assembler
/// name but have different signatures (e.g. different return types), Crubit
/// generates thunks with the same name, which may lead to "conflicting types for
/// '__rust_thunk__/*...*/'" compile errors in the generated C++ implementation.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: SimpleStruct
pub struct SimpleStruct {
    pub x: ::ffi_11::c_int,
}
impl !Send for SimpleStruct {}
impl !Sync for SimpleStruct {}
unsafe impl ::cxx::ExternType for SimpleStruct {
    type Id = ::cxx::type_id!(":: SimpleStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: SimpleStruct"), crate::SimpleStruct);

impl Default for SimpleStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12SimpleStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: OtherStruct
pub struct OtherStruct {
    pub y: ::ffi_11::c_int,
}
impl !Send for OtherStruct {}
impl !Sync for OtherStruct {}
unsafe impl ::cxx::ExternType for OtherStruct {
    type Id = ::cxx::type_id!(":: OtherStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: OtherStruct"), crate::OtherStruct);

impl Default for OtherStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11OtherStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn my_asm_conflict_func1() -> crate::SimpleStruct {
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::SimpleStruct>::uninit();
        crate::detail::__rust_thunk__my_asm_conflict_func(
            &raw mut __crubit_return as *mut ::core::ffi::c_void,
        );
        __crubit_return.assume_init()
    }
}

// error: function `my_asm_conflict_func2` could not be bound
//   Function uses a linkage name that is already used by another function

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z24min_android_version_9999v"]
        pub(crate) unsafe fn __rust_thunk___Z24min_android_version_9999v();
        pub(crate) unsafe fn __rust_thunk___ZN12SimpleStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN11OtherStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk__my_asm_conflict_func(__return: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SimpleStruct>() == 4);
    assert!(::core::mem::align_of::<crate::SimpleStruct>() == 4);
    static_assertions::assert_impl_all!(crate::SimpleStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SimpleStruct: Drop);
    assert!(::core::mem::offset_of!(crate::SimpleStruct, x) == 0);
    assert!(::core::mem::size_of::<crate::OtherStruct>() == 4);
    assert!(::core::mem::align_of::<crate::OtherStruct>() == 4);
    static_assertions::assert_impl_all!(crate::OtherStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::OtherStruct: Drop);
    assert!(::core::mem::offset_of!(crate::OtherStruct, y) == 0);
};
