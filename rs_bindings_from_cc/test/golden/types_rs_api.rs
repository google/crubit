// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:types_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeStruct {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

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

impl<'b> From<::ctor::RvalueReference<'b, crate::SomeStruct>> for SomeStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/types.h;l=13
// Error while generating bindings for item 'SomeStruct::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/types.h;l=13
// Error while generating bindings for item 'SomeStruct::operator=':
// operator= for Unpin types is not yet supported.

forward_declare::forward_declare!(pub ForwardDeclaredStruct = forward_declare::symbol!("ForwardDeclaredStruct"));

#[derive(Clone, Copy)]
#[repr(C)]
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
    pub float_field: f32,
    pub double_field: f64,
    pub ptr_field: *mut i32,
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

impl<'b> From<::ctor::RvalueReference<'b, crate::FieldTypeTestStruct>> for FieldTypeTestStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::FieldTypeTestStruct>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19FieldTypeTestStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn VoidReturningFunction() {
    unsafe { crate::detail::__rust_thunk___Z21VoidReturningFunctionv() }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Color(u32);
impl Color {
    pub const kRed: Color = Color(0);
    pub const kBlue: Color = Color(1);
    pub const kLimeGreen: Color = Color(4294967295);
}
impl From<u32> for Color {
    fn from(value: u32) -> Color {
        Color(value)
    }
}
impl From<Color> for u32 {
    fn from(value: Color) -> u32 {
        value.0
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstNSt3__u17integral_constantIbLb0EEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std::integral_constant<bool, false>"),
    crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE
);

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::std::integral_constant<bool, false>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::std::integral_constant<bool, false>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::integral_constant':
// Parameter #0 is not supported: Unsupported type 'struct std::integral_constant<_Bool, false> &&': Unsupported type: && without lifetime

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>::operator=':
// Parameter #0 is not supported: Unsupported type 'struct std::integral_constant<_Bool, false> &&': Unsupported type: && without lifetime

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=24
// Error while generating bindings for item 'std::integral_constant<bool, false>::value_type':
// Typedefs nested in classes are not supported yet

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=25
// Error while generating bindings for item 'std::integral_constant<bool, false>::type':
// Typedefs nested in classes are not supported yet

// <unknown location>
// Error while generating bindings for item 'std::integral_constant<bool, false>::operator bool':
// Return type is not supported: Unsupported type 'std::integral_constant<_Bool, false>::value_type': No generated bindings found for 'value_type'

// <unknown location>
// Error while generating bindings for item 'std::integral_constant<bool, false>::operator()':
// Return type is not supported: Unsupported type 'std::integral_constant<_Bool, false>::value_type': No generated bindings found for 'value_type'

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInstNSt3__u17integral_constantIbLb1EEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std::integral_constant<bool, true>"),
    crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE
);

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::std::integral_constant<bool, true>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::std::integral_constant<bool, true>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::integral_constant':
// Parameter #0 is not supported: Unsupported type 'struct std::integral_constant<_Bool, true> &&': Unsupported type: && without lifetime

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>::operator=':
// Parameter #0 is not supported: Unsupported type 'struct std::integral_constant<_Bool, true> &&': Unsupported type: && without lifetime

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=24
// Error while generating bindings for item 'std::integral_constant<bool, true>::value_type':
// Typedefs nested in classes are not supported yet

// google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=25
// Error while generating bindings for item 'std::integral_constant<bool, true>::type':
// Typedefs nested in classes are not supported yet

// <unknown location>
// Error while generating bindings for item 'std::integral_constant<bool, true>::operator bool':
// Return type is not supported: Unsupported type 'std::integral_constant<_Bool, true>::value_type': No generated bindings found for 'value_type'

// <unknown location>
// Error while generating bindings for item 'std::integral_constant<bool, true>::operator()':
// Return type is not supported: Unsupported type 'std::integral_constant<_Bool, true>::value_type': No generated bindings found for 'value_type'

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
        pub(crate) fn __rust_thunk___ZN19FieldTypeTestStructC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::FieldTypeTestStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::FieldTypeTestStruct>,
        );
        pub(crate) fn __rust_thunk___Z21VoidReturningFunctionv();
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

const _: () = assert!(::std::mem::size_of::<crate::FieldTypeTestStruct>() == 288);
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
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, bool_field) == 0);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, char_field) == 1);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_char_field) == 2
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_char_field) == 3
);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, char16_t_field) == 4);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, char32_t_field) == 8);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, wchar_t_field) == 12);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, short_field) == 16);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int_field) == 20);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, long_field) == 24);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, long_long_field) == 32
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_short_field) == 40
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_int_field) == 44
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_long_field) == 48
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_long_long_field)
        == 56
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_short_field) == 64
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_int_field) == 68
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_long_field) == 72
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_long_long_field) == 80
);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int8_t_field) == 88);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int16_t_field) == 90);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int32_t_field) == 92);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int64_t_field) == 96);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_int8_t_field) == 104
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_int16_t_field) == 106
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_int32_t_field) == 108
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_int64_t_field) == 112
);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uint8_t_field) == 120);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uint16_t_field) == 122
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uint32_t_field) == 124
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uint64_t_field) == 128
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uint8_t_field) == 136
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uint16_t_field) == 138
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uint32_t_field) == 140
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uint64_t_field) == 144
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, ptrdiff_t_field) == 152
);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, size_t_field) == 160);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, intptr_t_field) == 168
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uintptr_t_field) == 176
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_ptrdiff_t_field) == 184
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_size_t_field) == 192
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_intptr_t_field) == 200
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uintptr_t_field) == 208
);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, float_field) == 216);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, double_field) == 224);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, ptr_field) == 232);
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, struct_field) == 240);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, struct_ptr_field) == 248
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, const_struct_ptr_field) == 256
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, struct_ref_field) == 264
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, const_struct_ref_field) == 272
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, forward_declared_ptr_field)
        == 280
);

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
