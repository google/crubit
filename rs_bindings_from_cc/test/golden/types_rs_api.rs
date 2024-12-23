// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:types_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

// Error while generating bindings for item 'PtrDiff':
// Unsupported type 'decltype(static_cast<int *>(nullptr) - static_cast<int *>(nullptr))': Unsupported clang::Type class 'Decltype'

// Error while generating bindings for item 'Size':
// Unsupported type 'decltype(sizeof (0))': Unsupported clang::Type class 'Decltype'

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "SomeStruct")]
pub struct SomeStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeStruct {}
impl !Sync for SomeStruct {}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSEOS_(self, __param_0);
        }
    }
}

forward_declare::forward_declare!(pub ForwardDeclaredStruct = forward_declare::symbol!("ForwardDeclaredStruct"));

#[derive(Clone, Copy)]
#[repr(C, align(8))]
#[__crubit::annotate(cpp_type = "FieldTypeTestStruct")]
pub struct FieldTypeTestStruct {
    pub bool_field: bool,
    pub char_field: ::core::ffi::c_char,
    pub unsigned_char_field: ::core::ffi::c_uchar,
    pub signed_char_field: ::core::ffi::c_schar,
    pub char16_t_field: u16,
    pub char32_t_field: u32,
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'wchar_t': Unsupported builtin type
    pub(crate) wchar_t_field: [::core::mem::MaybeUninit<u8>; 4],
    pub short_field: ::core::ffi::c_short,
    pub int_field: ::core::ffi::c_int,
    pub long_field: ::core::ffi::c_long,
    pub long_long_field: ::core::ffi::c_longlong,
    pub unsigned_short_field: ::core::ffi::c_ushort,
    pub unsigned_int_field: ::core::ffi::c_uint,
    pub unsigned_long_field: ::core::ffi::c_ulong,
    pub unsigned_long_long_field: ::core::ffi::c_ulonglong,
    pub signed_short_field: ::core::ffi::c_short,
    pub signed_int_field: ::core::ffi::c_int,
    pub signed_long_field: ::core::ffi::c_long,
    pub signed_long_long_field: ::core::ffi::c_longlong,
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'PtrDiff': No generated bindings found for 'PtrDiff'
    pub(crate) ptrdiff_t_field: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'Size': No generated bindings found for 'Size'
    pub(crate) size_t_field: [::core::mem::MaybeUninit<u8>; 8],
    pub float_field: f32,
    pub double_field: f64,
    pub ptr_field: *mut ::core::ffi::c_int,
    pub void_ptr_field: *mut ::core::ffi::c_void,
    pub const_void_ptr_field: *const ::core::ffi::c_void,
    pub void_double_ptr_field: *mut *mut ::core::ffi::c_void,
    pub struct_field: crate::SomeStruct,
    pub struct_ptr_field: *mut crate::SomeStruct,
    pub const_struct_ptr_field: *const crate::SomeStruct,
    pub struct_ref_field: *mut crate::SomeStruct,
    pub const_struct_ref_field: *const crate::SomeStruct,
    /// TODO(b/226580208): Uncomment when these don't cause struct import to fail.
    /// SomeStruct&& struct_rvalue_ref_field;
    /// const SomeStruct&& const_struct_rvalue_ref_field;
    pub forward_declared_ptr_field: *mut crate::ForwardDeclaredStruct,
    pub cyclic_ptr_field: *mut crate::FieldTypeTestStruct,
}
impl !Send for FieldTypeTestStruct {}
impl !Sync for FieldTypeTestStruct {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("FieldTypeTestStruct"),
    crate::FieldTypeTestStruct
);

impl<'b> From<::ctor::RvalueReference<'b, Self>> for FieldTypeTestStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19FieldTypeTestStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn FunctionTakingPointersAndReferences<'a, 'b, 'c, 'd>(
    const_ref_param: &'a ::core::ffi::c_int,
    mut_ref_param: &'b mut ::core::ffi::c_int,
    const_ptr_param: Option<&'c ::core::ffi::c_int>,
    mut_ptr_param: Option<&'d mut ::core::ffi::c_int>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z35FunctionTakingPointersAndReferencesRKiRiPS_Pi(
            const_ref_param,
            mut_ref_param,
            const_ptr_param,
            mut_ptr_param,
        )
    }
}

#[inline(always)]
pub fn VoidReturningFunction() {
    unsafe { crate::detail::__rust_thunk___Z21VoidReturningFunctionv() }
}

/// Note especially the use of references. If we convert those to pointers,
/// this becomes un-compilable. The syntax here is awful, but this is a function
/// returning a function. In ML-like syntax:
/// FunctionPointerReturningFunction : () -> (const int&, int*) -> int&
#[inline(always)]
pub fn FunctionPointerReturningFunction() -> Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_int,
        *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_int,
> {
    unsafe { crate::detail::__rust_thunk___Z32FunctionPointerReturningFunctionv() }
}

#[inline(always)]
pub unsafe fn FunctionWithVoidPointers(
    __param_0: *mut ::core::ffi::c_void,
    __param_1: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    crate::detail::__rust_thunk___Z24FunctionWithVoidPointersPvPKv(__param_0, __param_1)
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeStruct>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeStruct,
            __param_0: &'b crate::SomeStruct,
        ) -> &'a mut crate::SomeStruct;
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        ) -> &'a mut crate::SomeStruct;
        pub(crate) unsafe fn __rust_thunk___ZN19FieldTypeTestStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::FieldTypeTestStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::FieldTypeTestStruct>,
        );
        #[link_name = "_Z35FunctionTakingPointersAndReferencesRKiRiPS_Pi"]
        pub(crate) unsafe fn __rust_thunk___Z35FunctionTakingPointersAndReferencesRKiRiPS_Pi<
            'a,
            'b,
            'c,
            'd,
        >(
            const_ref_param: &'a ::core::ffi::c_int,
            mut_ref_param: &'b mut ::core::ffi::c_int,
            const_ptr_param: Option<&'c ::core::ffi::c_int>,
            mut_ptr_param: Option<&'d mut ::core::ffi::c_int>,
        );
        pub(crate) unsafe fn __rust_thunk___Z21VoidReturningFunctionv();
        pub(crate) unsafe fn __rust_thunk___Z32FunctionPointerReturningFunctionv() -> Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *mut ::core::ffi::c_int,
            ) -> *mut ::core::ffi::c_int,
        >;
        pub(crate) unsafe fn __rust_thunk___Z24FunctionWithVoidPointersPvPKv(
            __param_0: *mut ::core::ffi::c_void,
            __param_1: *const ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeStruct>() == 1);
    assert!(::core::mem::align_of::<crate::SomeStruct>() == 1);
    static_assertions::assert_impl_all!(crate::SomeStruct: Clone);
    static_assertions::assert_impl_all!(crate::SomeStruct: Copy);
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);

    assert!(::core::mem::size_of::<crate::FieldTypeTestStruct>() == 208);
    assert!(::core::mem::align_of::<crate::FieldTypeTestStruct>() == 8);
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Clone);
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Copy);
    static_assertions::assert_not_impl_any!(crate::FieldTypeTestStruct: Drop);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, bool_field) == 0);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, char_field) == 1);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_char_field) == 2);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_char_field) == 3);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, char16_t_field) == 4);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, char32_t_field) == 8);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, wchar_t_field) == 12);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, short_field) == 16);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, int_field) == 20);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, long_field) == 24);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, long_long_field) == 32);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_short_field) == 40);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_int_field) == 44);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_long_field) == 48);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, unsigned_long_long_field) == 56);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_short_field) == 64);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_int_field) == 68);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_long_field) == 72);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, signed_long_long_field) == 80);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, ptrdiff_t_field) == 88);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, size_t_field) == 96);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, float_field) == 104);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, double_field) == 112);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, ptr_field) == 120);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, void_ptr_field) == 128);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, const_void_ptr_field) == 136);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, void_double_ptr_field) == 144);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, struct_field) == 152);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, struct_ptr_field) == 160);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, const_struct_ptr_field) == 168);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, struct_ref_field) == 176);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, const_struct_ref_field) == 184);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, forward_declared_ptr_field) == 192);
    assert!(::core::mem::offset_of!(crate::FieldTypeTestStruct, cyclic_ptr_field) == 200);
};
