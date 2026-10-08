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
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// Note: a real example would require that Crubit implements CrubitAbiTrait in
// order for the generated code to properly compile. This example just serves to
// illustrate what the generated code will look like.

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "21StructWithBridgeField"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=StructWithBridgeField
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct StructWithBridgeField {
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/bridge_field: 'crate::RustStruct' is not layout-compatible between Rust and C++.
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
        ::crubit_support::bridge::unstable_return!(@crate::RustStructAbi,crate::RustStructAbi,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z15ReturnCppStructv(__crubit_return_abi_buffer,); })
    }
}

#[inline(always)]
pub fn TakeCppStruct(__param_0: crate::RustStruct) {
    unsafe {
        crate::detail::__rust_thunk___Z13TakeCppStruct9CppStruct(::crubit_support::bridge::unstable_encode!(@crate::RustStructAbi,crate::RustStructAbi,__param_0).as_ptr()as*const u8)
    }
}

/// An alias to a bridge type is bound as an alias to the bridged Rust type.
pub type CppStructAlias = crate::RustStruct;

// error: class `MyOption` could not be bound
//   Class templates are not yet supported

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "4Vec3"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Vec3
///CRUBIT_ANNOTATE: cpp_move_constructible=
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
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<crate::Vec3>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z16MakeOptionalVec3fffb(__crubit_return_abi_buffer,x,y,z,is_present); })
    }
}

#[inline(always)]
pub fn MapMultiply(v: crate::MyOption<crate::Vec3>, factor: f32) -> crate::MyOption<crate::Vec3> {
    unsafe {
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<crate::Vec3>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(__crubit_return_abi_buffer,::crubit_support::bridge::unstable_encode!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<crate::Vec3>>,v).as_ptr()as*const u8,factor); })
    }
}

// Type bindings for MyI8Struct suppressed due to being mapped to an existing Rust type (i8)

#[inline(always)]
pub fn MakeMyI8Struct() -> crate::MyOption<i8> {
    unsafe {
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<i8>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<i8>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z14MakeMyI8Structv(__crubit_return_abi_buffer,); })
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
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<*mut::ffi_11::c_void>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<*mut::ffi_11::c_void>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z12MaybeVoidPtrv(__crubit_return_abi_buffer,); })
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
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<*const[::ffi_11::c_int]>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<*const[::ffi_11::c_int]>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z40AcceptsSliceAndReturnsStatusErrorIfEmptyN6rs_std8SliceRefIKiEE(__crubit_return_abi_buffer,slice); })
    }
}

#[inline(always)]
pub fn ReturnsCStrArray() -> crate::MyOption<*mut *const ::ffi_11::c_char> {
    unsafe {
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<*mut*const::ffi_11::c_char>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<*mut*const::ffi_11::c_char>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z16ReturnsCStrArrayv(__crubit_return_abi_buffer,); })
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
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<crate::DefaultEnum>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<crate::DefaultEnum>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z40ReturnsDefaultEnumInComposableBridgeTypev(__crubit_return_abi_buffer,); })
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
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<crate::I64Enum>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<crate::I64Enum>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z36ReturnsI64EnumInComposableBridgeTypev(__crubit_return_abi_buffer,); })
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
        ::crubit_support::bridge::unstable_return!(@crate::MyOptionAbi(::crubit_support::bridge::transmute_abi::<crate::some_namespace::EnumInNamespace>()),crate::MyOptionAbi<::crubit_support::bridge::TransmuteAbi<crate::some_namespace::EnumInNamespace>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z44ReturnsEnumInNamespaceInComposableBridgeTypev(__crubit_return_abi_buffer,); })
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < class std :: __wrap_iter < const int *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_ki_eeee::base(self)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE) -> Self {
        let mut __x = args;
        let mut __x = ::core::mem::MaybeUninit::new(__x);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(&raw mut tmp as*mut _,__x.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_ki_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < class std :: __wrap_iter < class std :: basic_string_view < char , struct std :: char_traits < char >> *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE
{
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl!Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{  }
impl!Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{  }
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{ #[must_use]#[inline(always)]pub fn base<'__this>(&'__this self)->crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE{ unsafe{ self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipns_17basic_string_view_ic_ns_11char_traits_ic_eeeeeeee::base(self) } } }

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{ #[inline(always)]fn default()->Self{ let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEC1Ev(&raw mut tmp as*mut _);tmp.assume_init() } } }

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{ #[inline(always)]fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE)->Self{ let mut __x=args;let mut __x=::core::mem::MaybeUninit::new(__x);let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEC1ES7_(&raw mut tmp as*mut _,__x.as_mut_ptr());tmp.assume_init() } } }
impl::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{ type CtorType=Self;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE)->Self::CtorType{ <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>>::from(args) } }

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipns_17basic_string_view_ic_ns_11char_traits_ic_eeeeeeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE
    {
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>::uninit();
            crate::detail::__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < class std :: __wrap_iter < int *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_i_pi_eeee::base(self)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE) -> Self {
        let mut __x = args;
        let mut __x = ::core::mem::MaybeUninit::new(__x);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(&raw mut tmp as*mut _,__x.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_i_pi_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> *const ::ffi_11::c_char {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ip_kc_ee::base(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const ::ffi_11::c_char>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *const ::ffi_11::c_char) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const ::ffi_11::c_char>
    for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const ::ffi_11::c_char) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const ::ffi_11::c_char>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ip_kc_ee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
    ) -> *const ::ffi_11::c_char {
        unsafe {
            crate::detail::__rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const wchar_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPKiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < const int *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {}

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPKiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Add<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPKiEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPKiEpLEl(self, __n);
        }
    }
}

impl<'__this> ::core::ops::Sub<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPKiEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPKiEmIEl(self, __n);
        }
    }
}

impl ::operator::CcIndex<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    type Output<'ctnr> = &'ctnr ::ffi_11::c_int;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __n: isize) -> Self::Output<'ctnr> {
        unsafe { crate::detail::__rust_thunk__6dc0ff60__ZNKSt3__u11__wrap_iterIPKiEixEl(self, __n) }
    }
}
impl ::core::ops::Index<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    type Output = ::ffi_11::c_int;
    #[inline(always)]
    fn index(&self, index: isize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < class std :: basic_string_view < char , struct std :: char_traits < char >> *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send
    for __CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE
{
}
impl !Sync
    for __CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE
{
}

impl Default
    for __CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE
{
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl<'__this>::core::ops::Add<isize>for&'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE{ type Output=crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE;#[inline(always)]fn add(self,__n: isize)->Self::Output{ unsafe{ let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>::uninit();crate::detail::__rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEplEl(&raw mut __crubit_return as*mut::core::ffi::c_void,self,__n);__crubit_return.assume_init() } } }

impl ::core::ops::AddAssign<isize>
    for __CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE
{
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEpLEl(self,__n);
        }
    }
}

impl<'__this>::core::ops::Sub<isize>for&'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE{ type Output=crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE;#[inline(always)]fn sub(self,__n: isize)->Self::Output{ unsafe{ let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>::uninit();crate::detail::__rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEmiEl(&raw mut __crubit_return as*mut::core::ffi::c_void,self,__n);__crubit_return.assume_init() } } }

impl ::core::ops::SubAssign<isize>
    for __CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE
{
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEmIEl(self,__n);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < int *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {}

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Add<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPiEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPiEpLEl(self, __n);
        }
    }
}

impl<'__this> ::core::ops::Sub<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPiEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPiEmIEl(self, __n);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: span < const int , 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {}
impl !Sync for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {}
impl __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    #[must_use]
    #[inline(always)]
    pub fn first<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::first(
                self, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn last<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::last(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn subspan<'__this>(
        &'__this self,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::subspan(
                self, __offset, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::size(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn size_bytes<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::size_bytes(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::empty(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::front(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::back(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::data(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::begin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::end(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::rbegin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::rend(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9ef48370__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::operator::CcIndex<usize> for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    type Output<'ctnr> = &'ctnr ::ffi_11::c_int;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __idx: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__0b050f23__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm(
                self, __idx,
            )
        }
    }
}
impl ::core::ops::Index<usize> for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    type Output = ::ffi_11::c_int;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

pub mod cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn first<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__eb5fa8a1__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn last<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__cb04abd4__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn subspan<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__f8d72bf1__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__e58d956f__ZNKSt3__u4spanIKiLm18446744073709551615EE4sizeEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__1a2eb8d0__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__5eda390c__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__02898003__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__f4827081__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> *const ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk__e6274c04__ZNKSt3__u4spanIKiLm18446744073709551615EE4dataEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__4d8e3070__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__b3c9f034__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__3e61f7cc__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__dc1b110a__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: span < class std :: basic_string_view < char , struct std :: char_traits < char >>, 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE
{
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl!Send for __CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{  }
impl!Sync for __CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{  }
impl __CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{ #[must_use]#[inline(always)]pub fn first<'__this>(&'__this self,__count: usize)->crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::first(self,__count) } }#[must_use]#[inline(always)]pub fn last<'__this>(&'__this self,__count: usize)->crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::last(self,__count) } }#[must_use]#[inline(always)]pub fn subspan<'__this>(&'__this self,__offset: usize,__count: usize)->crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::subspan(self,__offset,__count) } }#[must_use]#[inline(always)]pub fn size<'__this>(&'__this self)->usize{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::size(self) } }#[must_use]#[inline(always)]pub fn size_bytes<'__this>(&'__this self)->usize{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::size_bytes(self) } }#[must_use]#[inline(always)]pub fn empty<'__this>(&'__this self)->bool{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::empty(self) } }#[must_use]#[inline(always)]pub fn front<'__this>(&'__this self)->::cref::CMut<'__this,::cc_std::std::__u::raw_string_view>{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::front(self) } }#[must_use]#[inline(always)]pub fn back<'__this>(&'__this self)->::cref::CMut<'__this,::cc_std::std::__u::raw_string_view>{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::back(self) } }#[must_use]#[inline(always)]pub fn data<'__this>(&'__this self)->*mut::cc_std::std::__u::raw_string_view{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::data(self) } }#[must_use]#[inline(always)]pub fn begin<'__this>(&'__this self)->crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::begin(self) } }#[must_use]#[inline(always)]pub fn end<'__this>(&'__this self)->crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::end(self) } }#[must_use]#[inline(always)]pub fn rbegin<'__this>(&'__this self)->crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::rbegin(self) } }#[must_use]#[inline(always)]pub fn rend<'__this>(&'__this self)->crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{ unsafe{ self::cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee::rend(self) } } }

impl Default for __CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{ #[inline(always)]fn default()->Self{ let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__9ef48370__ZNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEC1Ev(&raw mut tmp as*mut _);tmp.assume_init() } } }

pub mod cc_template_inst_n_st3_u4span_ins_17basic_string_view_ic_ns_11char_traits_ic_eeee_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]    pub(crate)fn first<'__this>(__this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,__count: usize)->crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE>::uninit();
            crate::detail::__rust_thunk__eb5fa8a1__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5firstEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]    pub(crate)fn last<'__this>(__this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,__count: usize)->crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE>::uninit();
            crate::detail::__rust_thunk__cb04abd4__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4lastEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]    pub(crate)fn subspan<'__this>(__this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,__offset: usize,__count: usize)->crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE{
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE>::uninit();
            crate::detail::__rust_thunk__f8d72bf1__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__e58d956f__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4sizeEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__1a2eb8d0__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__5eda390c__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5emptyEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, ::cc_std::std::__u::raw_string_view> {
        unsafe {
            crate::detail::__rust_thunk__02898003__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5frontEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, ::cc_std::std::__u::raw_string_view> {
        unsafe {
            crate::detail::__rust_thunk__f4827081__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4backEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
    ) -> *mut ::cc_std::std::__u::raw_string_view {
        unsafe {
            crate::detail::__rust_thunk__e6274c04__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4dataEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE
    {
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>::uninit();
            crate::detail::__rust_thunk__4d8e3070__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5beginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE
    {
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>::uninit();
            crate::detail::__rust_thunk__b3c9f034__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE3endEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]    pub(crate)fn rbegin<'__this>(__this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE)->crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE>::uninit();
            crate::detail::__rust_thunk__3e61f7cc__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]    pub(crate)fn rend<'__this>(__this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE)->crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE{
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE>::uninit();
            crate::detail::__rust_thunk__dc1b110a__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: span < int , 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {}
impl !Sync for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {}
impl __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    #[must_use]
    #[inline(always)]
    pub fn first<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::first(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn last<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::last(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn subspan<'__this>(
        &'__this self,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::subspan(
                self, __offset, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::size(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn size_bytes<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::size_bytes(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::empty(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::front(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::back(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *mut ::ffi_11::c_int {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::data(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::begin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::end(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::rbegin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::rend(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9ef48370__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn first<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__eb5fa8a1__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn last<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__cb04abd4__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn subspan<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__f8d72bf1__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__e58d956f__ZNKSt3__u4spanIiLm18446744073709551615EE4sizeEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__1a2eb8d0__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__5eda390c__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__02898003__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__f4827081__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> *mut ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk__e6274c04__ZNKSt3__u4spanIiLm18446744073709551615EE4dataEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__4d8e3070__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__b3c9f034__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__3e61f7cc__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__dc1b110a__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

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
        pub(crate) unsafe fn __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        );
        pub(crate) unsafe fn __rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEE4baseEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEC1ES7_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEE4baseEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
        );
        pub(crate) unsafe fn __rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEE4baseEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__b1a7fc4d__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk__44ef6cb1__ZNKSt3__u16reverse_iteratorIPKcE4baseEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__12254c19__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPKiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPKiEplEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPKiEpLEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        pub(crate) unsafe fn __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPKiEmiEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPKiEmIEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        pub(crate) unsafe fn __rust_thunk__6dc0ff60__ZNKSt3__u11__wrap_iterIPKiEixEl<'__this>(
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEplEl<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE,
            __n: isize,
        );
        pub(crate)unsafe fn __rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEpLEl<'__this>(__this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE,__n: isize)->&'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE;
        pub(crate) unsafe fn __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEmiEl<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE,
            __n: isize,
        );
        pub(crate)unsafe fn __rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEmIEl<'__this>(__this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE,__n: isize)->&'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE;
        pub(crate) unsafe fn __rust_thunk__b4336fca__ZNSt3__u11__wrap_iterIPiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__2f32272b__ZNKSt3__u11__wrap_iterIPiEplEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ebd93561__ZNSt3__u11__wrap_iterIPiEpLEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
        pub(crate) unsafe fn __rust_thunk__6caa0065__ZNKSt3__u11__wrap_iterIPiEmiEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__b6912148__ZNSt3__u11__wrap_iterIPiEmIEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
        pub(crate) unsafe fn __rust_thunk__9ef48370__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__eb5fa8a1__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__cb04abd4__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__f8d72bf1__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__e58d956f__ZNKSt3__u4spanIKiLm18446744073709551615EE4sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1a2eb8d0__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__5eda390c__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__0b050f23__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __idx: usize,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__02898003__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__f4827081__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__e6274c04__ZNKSt3__u4spanIKiLm18446744073709551615EE4dataEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__4d8e3070__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__b3c9f034__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__3e61f7cc__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__dc1b110a__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__9ef48370__ZNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__eb5fa8a1__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__cb04abd4__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__f8d72bf1__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__e58d956f__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1a2eb8d0__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__5eda390c__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__02898003__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, ::cc_std::std::__u::raw_string_view>;
        pub(crate) unsafe fn __rust_thunk__f4827081__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, ::cc_std::std::__u::raw_string_view>;
        pub(crate) unsafe fn __rust_thunk__e6274c04__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4dataEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        ) -> *mut ::cc_std::std::__u::raw_string_view;
        pub(crate) unsafe fn __rust_thunk__4d8e3070__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__b3c9f034__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__3e61f7cc__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__dc1b110a__ZNKSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__9ef48370__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__eb5fa8a1__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__cb04abd4__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__f8d72bf1__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__e58d956f__ZNKSt3__u4spanIiLm18446744073709551615EE4sizeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1a2eb8d0__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__5eda390c__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__02898003__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__f4827081__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__e6274c04__ZNKSt3__u4spanIiLm18446744073709551615EE4dataEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> *mut ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__4d8e3070__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__b3c9f034__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__3e61f7cc__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__dc1b110a__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
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
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            current
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE>()==8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEEEE,current)==0);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            current
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE, __i_) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>()==8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE,__i_)==0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE, __i_) == 0);
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __size_
        ) == 8
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE>()==16);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,__data_)==0);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u4spanINS_17basic_string_viewIcNS_11char_traitsIcEEEELm18446744073709551615EEE,__size_)==8);
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __size_
        ) == 8
    );
    assert!(::core::mem::size_of::<*const [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*const [::ffi_11::c_int]>() == 8);
    assert!(::core::mem::size_of::<*mut [::cc_std::std::__u::raw_string_view]>() == 16);
    assert!(::core::mem::align_of::<*mut [::cc_std::std::__u::raw_string_view]>() == 8);
    assert!(::core::mem::size_of::<*mut [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*mut [::ffi_11::c_int]>() == 8);
};
