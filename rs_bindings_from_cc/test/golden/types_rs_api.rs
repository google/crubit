// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:types_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=15
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeStruct {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=15
impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=15
impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=15
impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=15
impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSEOS_(self, __param_0);
        }
    }
}

forward_declare::forward_declare!(pub ForwardDeclaredStruct = forward_declare::symbol!("ForwardDeclaredStruct"));

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=19
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct FieldTypeTestStruct {
    pub bool_field: bool,
    pub char_field: u8,
    pub unsigned_char_field: u8,
    pub signed_char_field: i8,
    pub char16_t_field: u16,
    pub char32_t_field: u32,
    pub wchar_t_field: i32,
    pub short_field: i16,
    pub int_field: i32,
    pub long_field: i64,
    pub long_long_field: i64,
    pub unsigned_short_field: u16,
    pub unsigned_int_field: u32,
    pub unsigned_long_field: u64,
    pub unsigned_long_long_field: u64,
    pub signed_short_field: i16,
    pub signed_int_field: i32,
    pub signed_long_field: i64,
    pub signed_long_long_field: i64,
    pub int8_t_field: i8,
    pub int16_t_field: i16,
    pub int32_t_field: i32,
    pub int64_t_field: i64,
    pub std_int8_t_field: i8,
    pub std_int16_t_field: i16,
    pub std_int32_t_field: i32,
    pub std_int64_t_field: i64,
    pub uint8_t_field: u8,
    pub uint16_t_field: u16,
    pub uint32_t_field: u32,
    pub uint64_t_field: u64,
    pub std_uint8_t_field: u8,
    pub std_uint16_t_field: u16,
    pub std_uint32_t_field: u32,
    pub std_uint64_t_field: u64,
    pub ptrdiff_t_field: isize,
    pub size_t_field: usize,
    pub intptr_t_field: isize,
    pub uintptr_t_field: usize,
    pub std_ptrdiff_t_field: isize,
    pub std_size_t_field: usize,
    pub std_intptr_t_field: isize,
    pub std_uintptr_t_field: usize,
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'class rs_std::rs_char': No generated bindings found for 'rs_char'
    pub(crate) rs_char_field: [::std::mem::MaybeUninit<u8>; 4],
    pub float_field: f32,
    pub double_field: f64,
    pub ptr_field: *mut i32,
    pub void_ptr_field: *mut ::std::os::raw::c_void,
    pub const_void_ptr_field: *const ::std::os::raw::c_void,
    pub void_double_ptr_field: *mut *mut ::std::os::raw::c_void,
    pub struct_field: crate::SomeStruct,
    pub struct_ptr_field: *mut crate::SomeStruct,
    pub const_struct_ptr_field: *const crate::SomeStruct,
    pub struct_ref_field: *mut crate::SomeStruct,
    pub const_struct_ref_field: *const crate::SomeStruct,
    /// TODO(b/226580208): Uncomment when these don't cause struct import to fail.
    /// SomeStruct&& struct_rvalue_ref_field;
    /// const SomeStruct&& const_struct_rvalue_ref_field;
    pub forward_declared_ptr_field: *mut crate::ForwardDeclaredStruct,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("FieldTypeTestStruct"),
    crate::FieldTypeTestStruct
);

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=19
impl<'b> From<::ctor::RvalueReference<'b, Self>> for FieldTypeTestStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19FieldTypeTestStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=93
#[inline(always)]
pub fn VoidReturningFunction() {
    unsafe { crate::detail::__rust_thunk___Z21VoidReturningFunctionv() }
}

/// Note especially the use of references. If we convert those to pointers,
/// this becomes un-compilable. The syntax here is awful, but this is a function
/// returning a function. In ML-like syntax:
/// FunctionPointerReturningFunction : () -> (const int&, int*) -> int&
///
/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=99
#[inline(always)]
pub fn FunctionPointerReturningFunction() -> Option<extern "C" fn(*const i32, *mut i32) -> *mut i32>
{
    unsafe { crate::detail::__rust_thunk___Z32FunctionPointerReturningFunctionv() }
}

/// Generated from: rs_bindings_from_cc/test/golden/types.h;l=103
#[inline(always)]
pub unsafe fn FunctionWithVoidPointers(
    __param_0: *mut ::std::os::raw::c_void,
    __param_1: *const ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    crate::detail::__rust_thunk___Z24FunctionWithVoidPointersPvPKv(__param_0, __param_1)
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_

/// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstNSt3__u17integral_constantIbLb0EEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std::integral_constant<bool, false>"),
    crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE
);

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::std::integral_constant<bool, false>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::std::integral_constant<bool, false>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::integral_constant':
// Parameter #0 is not supported: Unsupported type 'integral_constant<_Bool, false> &&': Unsupported type: && without lifetime

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::operator=':
// Parameter #0 is not supported: Unsupported type 'integral_constant<_Bool, false> &&': Unsupported type: && without lifetime

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=24
// Error while generating bindings for item 'value_type':
// Typedefs nested in classes are not supported yet

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=25
// Error while generating bindings for item 'type':
// Typedefs nested in classes are not supported yet

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__config;l=553
// Expanded at: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=26
// Error while generating bindings for item 'std::integral_constant<bool, false>::operator bool':
// TODO(b/248542210,b/248577708): as a temporary workaround for un-instantiable function templates, template functions from the STL cannot be instantiated in user crates

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__config;l=553
// Expanded at: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=29
// Error while generating bindings for item 'std::integral_constant<bool, false>::operator()':
// TODO(b/248542210,b/248577708): as a temporary workaround for un-instantiable function templates, template functions from the STL cannot be instantiated in user crates

/// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstNSt3__u17integral_constantIbLb1EEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std::integral_constant<bool, true>"),
    crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE
);

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::std::integral_constant<bool, true>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::std::integral_constant<bool, true>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::integral_constant':
// Parameter #0 is not supported: Unsupported type 'integral_constant<_Bool, true> &&': Unsupported type: && without lifetime

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::operator=':
// Parameter #0 is not supported: Unsupported type 'integral_constant<_Bool, true> &&': Unsupported type: && without lifetime

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=24
// Error while generating bindings for item 'value_type':
// Typedefs nested in classes are not supported yet

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=25
// Error while generating bindings for item 'type':
// Typedefs nested in classes are not supported yet

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__config;l=553
// Expanded at: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=26
// Error while generating bindings for item 'std::integral_constant<bool, true>::operator bool':
// TODO(b/248542210,b/248577708): as a temporary workaround for un-instantiable function templates, template functions from the STL cannot be instantiated in user crates

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__config;l=553
// Expanded at: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=29
// Error while generating bindings for item 'std::integral_constant<bool, true>::operator()':
// TODO(b/248542210,b/248577708): as a temporary workaround for un-instantiable function templates, template functions from the STL cannot be instantiated in user crates

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIwNS_11char_traitsIwEENS_3pmr21polymorphic_allocatorIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12basic_stringIwNS_11char_traitsIwEENS_3pmr21polymorphic_allocatorIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12basic_stringIwNS_11char_traitsIwEENS_9allocatorIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u9basic_iosIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u9basic_iosIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u9basic_iosIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u9basic_iosIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u15basic_streambufIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u15basic_streambufIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u15basic_streambufIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u15basic_streambufIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_istreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u13basic_istreamIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_istreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u13basic_istreamIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u13basic_ostreamIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_ostreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u13basic_ostreamIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_iostreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u14basic_iostreamIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_iostreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u14basic_iostreamIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u15basic_stringbufIcNS_11char_traitsIcEENS_9allocatorIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u15basic_stringbufIwNS_11char_traitsIwEENS_9allocatorIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u19basic_istringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u19basic_istringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u19basic_ostringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u19basic_ostringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u18basic_stringstreamIcNS_11char_traitsIcEENS_9allocatorIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u18basic_stringstreamIwNS_11char_traitsIwEENS_9allocatorIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_filebufIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u13basic_filebufIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_filebufIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u13basic_filebufIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_ifstreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u14basic_ifstreamIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_ifstreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u14basic_ifstreamIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_ofstreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u14basic_ofstreamIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u14basic_ofstreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u14basic_ofstreamIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_fstreamIcNS_11char_traitsIcEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u13basic_fstreamIcNS_11char_traitsIcEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u13basic_fstreamIwNS_11char_traitsIwEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u13basic_fstreamIwNS_11char_traitsIwEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u4fposI11__mbstate_tEE = forward_declare::symbol!("__CcTemplateInstNSt3__u4fposI11__mbstate_tEE"));

/// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/type_list.h;l=22
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstNSt3__u11__type_listINS_12__align_typeIhEENS0_INS1_ItEENS0_INS1_IjEENS0_INS1_ImEENS0_INS1_IyEENS0_INS1_IdEENS0_INS1_IeEENS0_INS1_INS_15__struct_doubleEEENS0_INS1_INS_16__struct_double4EEENS0_INS1_IPiEENS_5__natEEEEEEEEEEEEEEEEEEEEE
{
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("std::__type_list<std::__align_type<unsigned char>, std::__type_list<std::__align_type<unsigned short>, std::__type_list<std::__align_type<unsigned int>, std::__type_list<std::__align_type<unsigned long>, std::__type_list<std::__align_type<unsigned long long>, std::__type_list<std::__align_type<double>, std::__type_list<std::__align_type<long double>, std::__type_list<std::__align_type<std::__struct_double>, std::__type_list<std::__align_type<std::__struct_double4>, std::__type_list<std::__align_type<int *>, std::__nat>>>>>>>>>>"),crate::__CcTemplateInstNSt3__u11__type_listINS_12__align_typeIhEENS0_INS1_ItEENS0_INS1_IjEENS0_INS1_ImEENS0_INS1_IyEENS0_INS1_IdEENS0_INS1_IeEENS0_INS1_INS_15__struct_doubleEEENS0_INS1_INS_16__struct_double4EEENS0_INS1_IPiEENS_5__natEEEEEEEEEEEEEEEEEEEEE);

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/type_list.h;l=22
// Error while generating bindings for item 'std::__type_list<std::__align_type<unsigned char>, std::__type_list<std::__align_type<unsigned short>, std::__type_list<std::__align_type<unsigned int>, std::__type_list<std::__align_type<unsigned long>, std::__type_list<std::__align_type<unsigned long long>, std::__type_list<std::__align_type<double>, std::__type_list<std::__align_type<long double>, std::__type_list<std::__align_type<std::__struct_double>, std::__type_list<std::__align_type<std::__struct_double4>, std::__type_list<std::__align_type<int *>, std::__nat>>>>>>>>>>::std::__type_list<std::__align_type<unsigned char>, std::__type_list<std::__align_type<unsigned short>, std::__type_list<std::__align_type<unsigned int>, std::__type_list<std::__align_type<unsigned long>, std::__type_list<std::__align_type<unsigned long long>, std::__type_list<std::__align_type<double>, std::__type_list<std::__align_type<long double>, std::__type_list<std::__align_type<std::__struct_double>, std::__type_list<std::__align_type<std::__struct_double4>, std::__type_list<std::__align_type<int *>, std::__nat>>>>>>>>>>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/type_list.h;l=22
// Error while generating bindings for item 'std::__type_list<std::__align_type<unsigned char>, std::__type_list<std::__align_type<unsigned short>, std::__type_list<std::__align_type<unsigned int>, std::__type_list<std::__align_type<unsigned long>, std::__type_list<std::__align_type<unsigned long long>, std::__type_list<std::__align_type<double>, std::__type_list<std::__align_type<long double>, std::__type_list<std::__align_type<std::__struct_double>, std::__type_list<std::__align_type<std::__struct_double4>, std::__type_list<std::__align_type<int *>, std::__nat>>>>>>>>>>::std::__type_list<std::__align_type<unsigned char>, std::__type_list<std::__align_type<unsigned short>, std::__type_list<std::__align_type<unsigned int>, std::__type_list<std::__align_type<unsigned long>, std::__type_list<std::__align_type<unsigned long long>, std::__type_list<std::__align_type<double>, std::__type_list<std::__align_type<long double>, std::__type_list<std::__align_type<std::__struct_double>, std::__type_list<std::__align_type<std::__struct_double4>, std::__type_list<std::__align_type<int *>, std::__nat>>>>>>>>>>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/type_list.h;l=22
// Error while generating bindings for item 'std::__type_list<std::__align_type<unsigned char>, std::__type_list<std::__align_type<unsigned short>, std::__type_list<std::__align_type<unsigned int>, std::__type_list<std::__align_type<unsigned long>, std::__type_list<std::__align_type<unsigned long long>, std::__type_list<std::__align_type<double>, std::__type_list<std::__align_type<long double>, std::__type_list<std::__align_type<std::__struct_double>, std::__type_list<std::__align_type<std::__struct_double4>, std::__type_list<std::__align_type<int *>, std::__nat>>>>>>>>>>::__type_list':
// Parameter #0 is not supported: Unsupported type '__type_list<__align_type<unsigned char>, __type_list<__align_type<unsigned short>, __type_list<__align_type<unsigned int>, __type_list<__align_type<unsigned long>, __type_list<__align_type<unsigned long long>, __type_list<__align_type<double>, __type_list<__align_type<long double>, __type_list<__align_type<__struct_double>, __type_list<__align_type<__struct_double4>, __type_list<__align_type<int *>, __nat> > > > > > > > > > &&': Unsupported type: && without lifetime

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/type_list.h;l=22
// Error while generating bindings for item 'std::__type_list<std::__align_type<unsigned char>, std::__type_list<std::__align_type<unsigned short>, std::__type_list<std::__align_type<unsigned int>, std::__type_list<std::__align_type<unsigned long>, std::__type_list<std::__align_type<unsigned long long>, std::__type_list<std::__align_type<double>, std::__type_list<std::__align_type<long double>, std::__type_list<std::__align_type<std::__struct_double>, std::__type_list<std::__align_type<std::__struct_double4>, std::__type_list<std::__align_type<int *>, std::__nat>>>>>>>>>>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/type_list.h;l=22
// Error while generating bindings for item 'std::__type_list<std::__align_type<unsigned char>, std::__type_list<std::__align_type<unsigned short>, std::__type_list<std::__align_type<unsigned int>, std::__type_list<std::__align_type<unsigned long>, std::__type_list<std::__align_type<unsigned long long>, std::__type_list<std::__align_type<double>, std::__type_list<std::__align_type<long double>, std::__type_list<std::__align_type<std::__struct_double>, std::__type_list<std::__align_type<std::__struct_double4>, std::__type_list<std::__align_type<int *>, std::__nat>>>>>>>>>>::operator=':
// Parameter #0 is not supported: Unsupported type '__type_list<__align_type<unsigned char>, __type_list<__align_type<unsigned short>, __type_list<__align_type<unsigned int>, __type_list<__align_type<unsigned long>, __type_list<__align_type<unsigned long long>, __type_list<__align_type<double>, __type_list<__align_type<long double>, __type_list<__align_type<__struct_double>, __type_list<__align_type<__struct_double4>, __type_list<__align_type<int *>, __nat> > > > > > > > > > &&': Unsupported type: && without lifetime

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/type_list.h;l=24
// Error while generating bindings for item '_Head':
// Typedefs nested in classes are not supported yet

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/type_list.h;l=25
// Error while generating bindings for item '_Tail':
// Typedefs nested in classes are not supported yet

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u11__type_listINS_12__align_typeItEENS0_INS1_IjEENS0_INS1_ImEENS0_INS1_IyEENS0_INS1_IdEENS0_INS1_IeEENS0_INS1_INS_15__struct_doubleEEENS0_INS1_INS_16__struct_double4EEENS0_INS1_IPiEENS_5__natEEEEEEEEEEEEEEEEEEE = forward_declare::symbol!("__CcTemplateInstNSt3__u11__type_listINS_12__align_typeItEENS0_INS1_IjEENS0_INS1_ImEENS0_INS1_IyEENS0_INS1_IdEENS0_INS1_IeEENS0_INS1_INS_15__struct_doubleEEENS0_INS1_INS_16__struct_double4EEENS0_INS1_IPiEENS_5__natEEEEEEEEEEEEEEEEEEE"));

forward_declare::forward_declare!(pub __CcTemplateInstNSt3__u12__align_typeIhEE = forward_declare::symbol!("__CcTemplateInstNSt3__u12__align_typeIhEE"));

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10SomeStructC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SomeStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeStruct,
            __param_0: &'b crate::SomeStruct,
        ) -> &'a mut crate::SomeStruct;
        pub(crate) fn __rust_thunk___ZN10SomeStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        ) -> &'a mut crate::SomeStruct;
        pub(crate) fn __rust_thunk___ZN19FieldTypeTestStructC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::FieldTypeTestStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::FieldTypeTestStruct>,
        );
        pub(crate) fn __rust_thunk___Z21VoidReturningFunctionv();
        pub(crate) fn __rust_thunk___Z32FunctionPointerReturningFunctionv()
        -> Option<extern "C" fn(*const i32, *mut i32) -> *mut i32>;
        pub(crate) fn __rust_thunk___Z24FunctionWithVoidPointersPvPKv(
            __param_0: *mut ::std::os::raw::c_void,
            __param_1: *const ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void;
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::SomeStruct>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::SomeStruct>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::FieldTypeTestStruct>() == 312);
const _: () = assert!(::std::mem::align_of::<crate::FieldTypeTestStruct>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::FieldTypeTestStruct: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, bool_field) == 0);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, char_field) == 1);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_char_field) == 2);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_char_field) == 3);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, char16_t_field) == 4);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, char32_t_field) == 8);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, wchar_t_field) == 12);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, short_field) == 16);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, int_field) == 20);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, long_field) == 24);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, long_long_field) == 32);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_short_field) == 40);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_int_field) == 44);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_long_field) == 48);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_long_long_field) == 56);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_short_field) == 64);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_int_field) == 68);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_long_field) == 72);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_long_long_field) == 80);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, int8_t_field) == 88);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, int16_t_field) == 90);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, int32_t_field) == 92);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, int64_t_field) == 96);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_int8_t_field) == 104);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_int16_t_field) == 106);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_int32_t_field) == 108);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_int64_t_field) == 112);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, uint8_t_field) == 120);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, uint16_t_field) == 122);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, uint32_t_field) == 124);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, uint64_t_field) == 128);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_uint8_t_field) == 136);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_uint16_t_field) == 138);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_uint32_t_field) == 140);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_uint64_t_field) == 144);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, ptrdiff_t_field) == 152);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, size_t_field) == 160);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, intptr_t_field) == 168);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, uintptr_t_field) == 176);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_ptrdiff_t_field) == 184);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_size_t_field) == 192);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_intptr_t_field) == 200);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, std_uintptr_t_field) == 208);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, rs_char_field) == 216);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, float_field) == 220);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, double_field) == 224);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, ptr_field) == 232);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, void_ptr_field) == 240);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, const_void_ptr_field) == 248);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, void_double_ptr_field) == 256);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, struct_field) == 264);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, struct_ptr_field) == 272);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, const_struct_ptr_field) == 280);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, struct_ref_field) == 288);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, const_struct_ref_field) == 296);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, forward_declared_ptr_field) == 304);

const _: () = assert!(
    ::std::mem::size_of::<crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE>() == 1
);
const _: () = assert!(
    ::std::mem::align_of::<crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE>() == 1
);
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE: Drop
    );
};

const _: () = assert!(
    ::std::mem::size_of::<crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE>() == 1
);
const _: () = assert!(
    ::std::mem::align_of::<crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE>() == 1
);
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE: Drop
    );
};

const _:()=assert!(::std::mem::size_of::<crate::__CcTemplateInstNSt3__u11__type_listINS_12__align_typeIhEENS0_INS1_ItEENS0_INS1_IjEENS0_INS1_ImEENS0_INS1_IyEENS0_INS1_IdEENS0_INS1_IeEENS0_INS1_INS_15__struct_doubleEEENS0_INS1_INS_16__struct_double4EEENS0_INS1_IPiEENS_5__natEEEEEEEEEEEEEEEEEEEEE>()==1);
const _:()=assert!(::std::mem::align_of::<crate::__CcTemplateInstNSt3__u11__type_listINS_12__align_typeIhEENS0_INS1_ItEENS0_INS1_IjEENS0_INS1_ImEENS0_INS1_IyEENS0_INS1_IdEENS0_INS1_IeEENS0_INS1_INS_15__struct_doubleEEENS0_INS1_INS_16__struct_double4EEENS0_INS1_IPiEENS_5__natEEEEEEEEEEEEEEEEEEEEE>()==1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__type_listINS_12__align_typeIhEENS0_INS1_ItEENS0_INS1_IjEENS0_INS1_ImEENS0_INS1_IyEENS0_INS1_IdEENS0_INS1_IeEENS0_INS1_INS_15__struct_doubleEEENS0_INS1_INS_16__struct_double4EEENS0_INS1_IPiEENS_5__natEEEEEEEEEEEEEEEEEEEEE:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__type_listINS_12__align_typeIhEENS0_INS1_ItEENS0_INS1_IjEENS0_INS1_ImEENS0_INS1_IyEENS0_INS1_IdEENS0_INS1_IeEENS0_INS1_INS_15__struct_doubleEEENS0_INS1_INS_16__struct_double4EEENS0_INS1_IPiEENS_5__natEEEEEEEEEEEEEEEEEEEEE:Drop);
};
