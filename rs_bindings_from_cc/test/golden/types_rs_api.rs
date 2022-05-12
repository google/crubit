// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:types_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

use ::std as rust_std;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeStruct {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::SomeStruct>> for SomeStruct {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::SomeStruct>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/types.h;l=13
// Error while generating bindings for item 'SomeStruct::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/types.h;l=13
// Error while generating bindings for item 'SomeStruct::operator=':
// Bindings for this kind of operator are not supported

forward_declare::forward_declare!(pub ForwardDeclaredStruct = forward_declare::symbol!("ForwardDeclaredStruct"));

#[derive(Clone, Copy)]
#[repr(C)]
pub union EmptyUnion {
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("EmptyUnion"), crate::EmptyUnion);

impl Default for EmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::EmptyUnion>> for EmptyUnion {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::EmptyUnion>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10EmptyUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/types.h;l=17
// Error while generating bindings for item 'EmptyUnion::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/types.h;l=17
// Error while generating bindings for item 'EmptyUnion::operator=':
// Bindings for this kind of operator are not supported

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

impl<'b> From<ctor::RvalueReference<'b, crate::FieldTypeTestStruct>> for FieldTypeTestStruct {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::FieldTypeTestStruct>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19FieldTypeTestStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union NonEmptyUnion {
    pub bool_field: bool,
    pub char_field: u8,
    pub int16_field: i16,
    pub int_field: i32,
    pub int32_field: i32,
    pub int64_field: i64,
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonEmptyUnion"), crate::NonEmptyUnion);

impl Default for NonEmptyUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::NonEmptyUnion>> for NonEmptyUnion {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::NonEmptyUnion>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NonEmptyUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/types.h;l=88
// Error while generating bindings for item 'NonEmptyUnion::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/types.h;l=88
// Error while generating bindings for item 'NonEmptyUnion::operator=':
// Bindings for this kind of operator are not supported

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

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10SomeStructC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::SomeStruct>,
            __param_0: ctor::RvalueReference<'b, crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10EmptyUnionC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::EmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN10EmptyUnionC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::EmptyUnion>,
            __param_0: ctor::RvalueReference<'b, crate::EmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN19FieldTypeTestStructC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::FieldTypeTestStruct>,
            __param_0: ctor::RvalueReference<'b, crate::FieldTypeTestStruct>,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NonEmptyUnion>,
        );
        pub(crate) fn __rust_thunk___ZN13NonEmptyUnionC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NonEmptyUnion>,
            __param_0: ctor::RvalueReference<'b, crate::NonEmptyUnion>,
        );
        pub(crate) fn __rust_thunk___Z21VoidReturningFunctionv();
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::SomeStruct>() == 1usize);
const _: () = assert!(rust_std::mem::align_of::<crate::SomeStruct>() == 1usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::SomeStruct: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::EmptyUnion>() == 1usize);
const _: () = assert!(rust_std::mem::align_of::<crate::EmptyUnion>() == 1usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::EmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::EmptyUnion: Drop);
};

const _: () = assert!(rust_std::mem::size_of::<crate::FieldTypeTestStruct>() == 288usize);
const _: () = assert!(rust_std::mem::align_of::<crate::FieldTypeTestStruct>() == 8usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::FieldTypeTestStruct: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, bool_field) * 8 == 0usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, char_field) * 8 == 8usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_char_field) * 8
        == 16usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_char_field) * 8
        == 24usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, char16_t_field) * 8 == 32usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, char32_t_field) * 8 == 64usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, wchar_t_field) * 8 == 96usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, short_field) * 8 == 128usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int_field) * 8 == 160usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, long_field) * 8 == 192usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, long_long_field) * 8
        == 256usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_short_field) * 8
        == 320usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_int_field) * 8
        == 352usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_long_field) * 8
        == 384usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, unsigned_long_long_field) * 8
        == 448usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_short_field) * 8
        == 512usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_int_field) * 8
        == 544usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_long_field) * 8
        == 576usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, signed_long_long_field) * 8
        == 640usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int8_t_field) * 8 == 704usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int16_t_field) * 8 == 720usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int32_t_field) * 8 == 736usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, int64_t_field) * 8 == 768usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_int8_t_field) * 8
        == 832usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_int16_t_field) * 8
        == 848usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_int32_t_field) * 8
        == 864usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_int64_t_field) * 8
        == 896usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uint8_t_field) * 8 == 960usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uint16_t_field) * 8
        == 976usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uint32_t_field) * 8
        == 992usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uint64_t_field) * 8
        == 1024usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uint8_t_field) * 8
        == 1088usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uint16_t_field) * 8
        == 1104usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uint32_t_field) * 8
        == 1120usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uint64_t_field) * 8
        == 1152usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, ptrdiff_t_field) * 8
        == 1216usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, size_t_field) * 8 == 1280usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, intptr_t_field) * 8
        == 1344usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, uintptr_t_field) * 8
        == 1408usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_ptrdiff_t_field) * 8
        == 1472usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_size_t_field) * 8
        == 1536usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_intptr_t_field) * 8
        == 1600usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, std_uintptr_t_field) * 8
        == 1664usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, float_field) * 8 == 1728usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, double_field) * 8 == 1792usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, ptr_field) * 8 == 1856usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, struct_field) * 8 == 1920usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, struct_ptr_field) * 8
        == 1984usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, const_struct_ptr_field) * 8
        == 2048usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, struct_ref_field) * 8
        == 2112usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, const_struct_ref_field) * 8
        == 2176usize
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::FieldTypeTestStruct, forward_declared_ptr_field)
        * 8
        == 2240usize
);

const _: () = assert!(rust_std::mem::size_of::<crate::NonEmptyUnion>() == 8usize);
const _: () = assert!(rust_std::mem::align_of::<crate::NonEmptyUnion>() == 8usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NonEmptyUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NonEmptyUnion: Drop);
};
const _: () = {
    static_assertions::assert_impl_all!(bool: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(u8: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i16: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(i64: Copy);
};
