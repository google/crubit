// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// Note: a real example would require that Crubit implements CrubitAbiTrait in
// order for the generated code to properly compile. This example just serves to
// illustrate what the generated code will look like.

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithBridgeField
pub struct StructWithBridgeField {
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/bridge_field: 'crate::RustStruct' is a bridge type, but fields must be layout compatible between Rust and C++.
    pub(crate) bridge_field: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for StructWithBridgeField {}
impl !Sync for StructWithBridgeField {}
unsafe impl ::cxx::ExternType for StructWithBridgeField {
    type Id = ::cxx::type_id!("StructWithBridgeField");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for StructWithBridgeField {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21StructWithBridgeFieldC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn ReturnCppStruct() -> crate::RustStruct {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::RustStructAbi,crate::RustStructAbi,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z15ReturnCppStructv(__crubit_return_abi_buffer,); })
    }
}

#[inline(always)]
pub fn TakeCppStruct(__param_0: crate::RustStruct) {
    unsafe {
        crate::detail::__rust_thunk___Z13TakeCppStruct9CppStruct(
            ::bridge_rust::unstable_encode!(@crate::RustStructAbi,crate::RustStructAbi,__param_0)
                .as_ptr() as *const u8,
        )
    }
}

// error: class `MyOption` could not be bound
//   Class templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Vec3
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl !Send for Vec3 {}
impl !Sync for Vec3 {}
unsafe impl ::cxx::ExternType for Vec3 {
    type Id = ::cxx::type_id!("Vec3");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Vec3 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN4Vec3C1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn MakeOptionalVec3(x: f32, y: f32, z: f32, is_present: bool) -> crate::MyOption<crate::Vec3> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z16MakeOptionalVec3fffb(__crubit_return_abi_buffer,x,y,z,is_present); })
    }
}

#[inline(always)]
pub fn MapMultiply(v: crate::MyOption<crate::Vec3>, factor: f32) -> crate::MyOption<crate::Vec3> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(__crubit_return_abi_buffer,::bridge_rust::unstable_encode!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,v).as_ptr()as*const u8,factor); })
    }
}

// Type bindings for MyI8Struct suppressed due to being mapped to an existing Rust type (i8)

#[inline(always)]
pub fn MakeMyI8Struct() -> crate::MyOption<i8> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<i8>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<i8>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z14MakeMyI8Structv(__crubit_return_abi_buffer,); })
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `slice`: raw pointer
#[inline(always)]
pub unsafe fn InspectStringViews(slice: *mut [::cc_std::std::__u::raw_string_view]) {
    unsafe {
        crate::detail::__rust_thunk___Z18InspectStringViewsN6rs_std8SliceRefINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEEE(slice)
    }
}

#[inline(always)]
pub fn MaybeVoidPtr() -> crate::MyOption<*mut ::ffi_11::c_void> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<*mut::ffi_11::c_void>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<*mut::ffi_11::c_void>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z12MaybeVoidPtrv(__crubit_return_abi_buffer,); })
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `slice`: raw pointer
#[inline(always)]
pub unsafe fn AcceptsSliceAndReturnsStatusErrorIfEmpty(
    slice: *const [::ffi_11::c_int],
) -> crate::MyOption<*const [::ffi_11::c_int]> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<*const[::ffi_11::c_int]>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<*const[::ffi_11::c_int]>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z40AcceptsSliceAndReturnsStatusErrorIfEmptyN6rs_std8SliceRefIKiEE(__crubit_return_abi_buffer,slice); })
    }
}

#[inline(always)]
pub fn ReturnsCStrArray() -> crate::MyOption<*mut *const ::ffi_11::c_char> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<*mut*const::ffi_11::c_char>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<*mut*const::ffi_11::c_char>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z16ReturnsCStrArrayv(__crubit_return_abi_buffer,); })
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[cfi_encoding = "11DefaultEnum"]
///CRUBIT_ANNOTATE: cpp_type=DefaultEnum
pub struct DefaultEnum(::ffi_11::c_int);
impl DefaultEnum {
    pub const kZero: DefaultEnum = DefaultEnum(::ffi_11::new_c_int(0));
    pub const kOne: DefaultEnum = DefaultEnum(::ffi_11::new_c_int(1));
    pub const kTwo: DefaultEnum = DefaultEnum(::ffi_11::new_c_int(2));
}
impl From<::ffi_11::c_int> for DefaultEnum {
    fn from(value: ::ffi_11::c_int) -> DefaultEnum {
        DefaultEnum(value)
    }
}
impl From<DefaultEnum> for ::ffi_11::c_int {
    fn from(value: DefaultEnum) -> ::ffi_11::c_int {
        value.0
    }
}

#[inline(always)]
pub fn ReturnsDefaultEnumInComposableBridgeType() -> crate::MyOption<crate::DefaultEnum> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::DefaultEnum>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::DefaultEnum>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z40ReturnsDefaultEnumInComposableBridgeTypev(__crubit_return_abi_buffer,); })
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[cfi_encoding = "7I64Enum"]
///CRUBIT_ANNOTATE: cpp_type=I64Enum
pub struct I64Enum(::ffi_11::c_long);
impl I64Enum {
    pub const kNegOne: I64Enum = I64Enum(::ffi_11::new_c_long(-1));
    pub const kZero: I64Enum = I64Enum(::ffi_11::new_c_long(0));
    pub const kOne: I64Enum = I64Enum(::ffi_11::new_c_long(1));
}
impl From<::ffi_11::c_long> for I64Enum {
    fn from(value: ::ffi_11::c_long) -> I64Enum {
        I64Enum(value)
    }
}
impl From<I64Enum> for ::ffi_11::c_long {
    fn from(value: I64Enum) -> ::ffi_11::c_long {
        value.0
    }
}

#[inline(always)]
pub fn ReturnsI64EnumInComposableBridgeType() -> crate::MyOption<crate::I64Enum> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::I64Enum>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::I64Enum>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z36ReturnsI64EnumInComposableBridgeTypev(__crubit_return_abi_buffer,); })
    }
}

pub mod some_namespace {
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
    #[cfi_encoding = "N14some_namespace15EnumInNamespaceE"]
    ///CRUBIT_ANNOTATE: cpp_type=some_namespace :: EnumInNamespace
    pub struct EnumInNamespace(::ffi_11::c_int);
    impl EnumInNamespace {
        pub const kZero: EnumInNamespace = EnumInNamespace(::ffi_11::new_c_int(0));
        pub const kOne: EnumInNamespace = EnumInNamespace(::ffi_11::new_c_int(1));
        pub const kTwo: EnumInNamespace = EnumInNamespace(::ffi_11::new_c_int(2));
    }
    impl From<::ffi_11::c_int> for EnumInNamespace {
        fn from(value: ::ffi_11::c_int) -> EnumInNamespace {
            EnumInNamespace(value)
        }
    }
    impl From<EnumInNamespace> for ::ffi_11::c_int {
        fn from(value: EnumInNamespace) -> ::ffi_11::c_int {
            value.0
        }
    }
}

#[inline(always)]
pub fn ReturnsEnumInNamespaceInComposableBridgeType(
) -> crate::MyOption<crate::some_namespace::EnumInNamespace> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::some_namespace::EnumInNamespace>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::some_namespace::EnumInNamespace>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z44ReturnsEnumInNamespaceInComposableBridgeTypev(__crubit_return_abi_buffer,); })
    }
}

// error: class `std::basic_istream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_filebuf<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_fstream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>` could not be bound
//   incomplete type

// error: class `std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>` could not be bound
//   incomplete type

// Type bindings for rs_std::SliceRef<const int> suppressed due to being mapped to an existing Rust type (*const[::ffi_11::c_int])

// Type bindings for rs_std::SliceRef<std::string_view> suppressed due to being mapped to an existing Rust type (*mut[::cc_std::std::__u::raw_string_view])

// Type bindings for rs_std::SliceRef<int> suppressed due to being mapped to an existing Rust type (*mut[::ffi_11::c_int])

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN21StructWithBridgeFieldC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Z15ReturnCppStructv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z13TakeCppStruct9CppStruct(
            __param_0: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4Vec3C1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z16MakeOptionalVec3fffb(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            x: f32,
            y: f32,
            z: f32,
            is_present: bool,
        );
        pub(crate) unsafe fn __rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            v: *const ::core::ffi::c_uchar,
            factor: f32,
        );
        pub(crate) unsafe fn __rust_thunk___Z14MakeMyI8Structv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        #[link_name = "_Z18InspectStringViewsN6rs_std8SliceRefINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEEE"]
        pub(crate) unsafe fn __rust_thunk___Z18InspectStringViewsN6rs_std8SliceRefINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEEE(
            slice: *mut [::cc_std::std::__u::raw_string_view],
        );
        pub(crate) unsafe fn __rust_thunk___Z12MaybeVoidPtrv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z40AcceptsSliceAndReturnsStatusErrorIfEmptyN6rs_std8SliceRefIKiEE(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            slice: *const [::ffi_11::c_int],
        );
        pub(crate) unsafe fn __rust_thunk___Z16ReturnsCStrArrayv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z40ReturnsDefaultEnumInComposableBridgeTypev(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z36ReturnsI64EnumInComposableBridgeTypev(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z44ReturnsEnumInNamespaceInComposableBridgeTypev(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::StructWithBridgeField>() == 1);
    assert!(::core::mem::align_of::<crate::StructWithBridgeField>() == 1);
    static_assertions::assert_impl_all!(crate::StructWithBridgeField: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithBridgeField: Drop);
    assert!(::core::mem::offset_of!(crate::StructWithBridgeField, bridge_field) == 0);
    assert!(::core::mem::size_of::<crate::Vec3>() == 12);
    assert!(::core::mem::align_of::<crate::Vec3>() == 4);
    static_assertions::assert_impl_all!(crate::Vec3: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Vec3: Drop);
    assert!(::core::mem::offset_of!(crate::Vec3, x) == 0);
    assert!(::core::mem::offset_of!(crate::Vec3, y) == 4);
    assert!(::core::mem::offset_of!(crate::Vec3, z) == 8);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<*const [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*const [::ffi_11::c_int]>() == 8);
    assert!(::core::mem::size_of::<*mut [::cc_std::std::__u::raw_string_view]>() == 16);
    assert!(::core::mem::align_of::<*mut [::cc_std::std::__u::raw_string_view]>() == 8);
    assert!(::core::mem::size_of::<*mut [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*mut [::ffi_11::c_int]>() == 8);
};
