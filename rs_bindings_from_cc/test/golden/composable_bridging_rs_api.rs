// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Note: a real example would require that Crubit implements CrubitAbiTrait in
// order for the generated code to properly compile. This example just serves to
// illustrate what the generated code will look like.

#[inline(always)]
pub fn ReturnCppStruct() -> crate::RustStruct {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::RustStructAbi,crate::RustStructAbi,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z15ReturnCppStructv(__return_abi_buffer,); })
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

// Error while generating bindings for class 'MyOption':
// Class templates are not supported yet

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
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z16MakeOptionalVec3fffb(__return_abi_buffer,x,y,z,is_present); })
    }
}

#[inline(always)]
pub fn MapMultiply(v: crate::MyOption<crate::Vec3>, factor: f32) -> crate::MyOption<crate::Vec3> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(__return_abi_buffer,::bridge_rust::unstable_encode!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<crate::Vec3>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<crate::Vec3>>,v).as_ptr()as*const u8,factor); })
    }
}

// Type bindings for MyI8Struct suppressed due to being mapped to an existing Rust type (i8)

#[inline(always)]
pub fn MakeMyI8Struct() -> crate::MyOption<i8> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<i8>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<i8>>,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z14MakeMyI8Structv(__return_abi_buffer,); })
    }
}

#[inline(always)]
pub unsafe fn InspectStringViews(
    slice: *mut [::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE],
) {
    crate::detail::__rust_thunk___Z18InspectStringViewsN6rs_std8SliceRefINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEEE(slice)
}

#[inline(always)]
pub fn MaybeVoidPtr() -> crate::MyOption<*mut ::ffi_11::c_void> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<*mut::ffi_11::c_void>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<*mut::ffi_11::c_void>>,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z12MaybeVoidPtrv(__return_abi_buffer,); })
    }
}

#[inline(always)]
pub unsafe fn AcceptsSliceAndReturnsStatusErrorIfEmpty(
    slice: *const [::ffi_11::c_int],
) -> crate::MyOption<*const [::ffi_11::c_int]> {
    ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<*const[::ffi_11::c_int]>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<*const[::ffi_11::c_int]>>,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z40AcceptsSliceAndReturnsStatusErrorIfEmptyN6rs_std8SliceRefIKiEE(__return_abi_buffer,slice); })
}

#[inline(always)]
pub fn ReturnsCStrArray() -> crate::MyOption<*mut *const ::ffi_11::c_char> {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<*mut*const::ffi_11::c_char>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<*mut*const::ffi_11::c_char>>,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z16ReturnsCStrArrayv(__return_abi_buffer,); })
    }
}

// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<char32_t *>':
// Can't generate bindings for std::iterator_traits<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::iterator_traits<char32_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPDiEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<char16_t *>':
// Can't generate bindings for std::iterator_traits<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::iterator_traits<char16_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPDsEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<const char32_t *>':
// Can't generate bindings for std::iterator_traits<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::iterator_traits<const char32_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKDiEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<const char16_t *>':
// Can't generate bindings for std::iterator_traits<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::iterator_traits<const char16_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKDsEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<const char *>':
// Can't generate bindings for std::iterator_traits<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::iterator_traits<const char *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKcEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<char *>':
// Can't generate bindings for std::iterator_traits<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::iterator_traits<char *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPcEE is a template instantiation)

// Error while generating bindings for struct 'std::pointer_traits<char32_t *>':
// Can't generate bindings for std::pointer_traits<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::pointer_traits<char32_t *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPDiEE is a template instantiation)

// Error while generating bindings for struct 'std::pointer_traits<char16_t *>':
// Can't generate bindings for std::pointer_traits<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::pointer_traits<char16_t *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPDsEE is a template instantiation)

// Error while generating bindings for struct 'std::pointer_traits<char *>':
// Can't generate bindings for std::pointer_traits<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::pointer_traits<char *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPcEE is a template instantiation)

// Error while generating bindings for class 'std::initializer_list<char32_t>':
// Can't generate bindings for std::initializer_list<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::initializer_list<char32_t> (crate::__CcTemplateInstSt16initializer_listIDiE is a template instantiation)

// Error while generating bindings for class 'std::initializer_list<char16_t>':
// Can't generate bindings for std::initializer_list<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::initializer_list<char16_t> (crate::__CcTemplateInstSt16initializer_listIDsE is a template instantiation)

// Error while generating bindings for class 'std::initializer_list<char>':
// Can't generate bindings for std::initializer_list<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::initializer_list<char> (crate::__CcTemplateInstSt16initializer_listIcE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char32_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char32_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char32_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char16_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char16_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char16_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char32_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char32_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char32_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char16_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char16_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char16_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const char32_t *>':
// Can't generate bindings for std::reverse_iterator<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const char32_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const char16_t *>':
// Can't generate bindings for std::reverse_iterator<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const char16_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const char8_t *>':
// Can't generate bindings for std::reverse_iterator<const char8_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const char8_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const std::basic_string_view<char, std::char_traits<char>> *>':
// Can't generate bindings for std::reverse_iterator<const std::basic_string_view<char, std::char_traits<char>> *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const std::basic_string_view<char, std::char_traits<char>> *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const char *>':
// Can't generate bindings for std::reverse_iterator<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const char *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const int *>':
// Can't generate bindings for std::reverse_iterator<const int *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const int *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const wchar_t *>':
// Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::basic_string_view<char, std::char_traits<char>> *>':
// Can't generate bindings for std::reverse_iterator<std::basic_string_view<char, std::char_traits<char>> *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<std::basic_string_view<char, std::char_traits<char>> *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPNS_17basic_string_viewIcNS_11char_traitsIcEEEEEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<char32_t *>':
// Can't generate bindings for std::__wrap_iter<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__wrap_iter<char32_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<char16_t *>':
// Can't generate bindings for std::__wrap_iter<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__wrap_iter<char16_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<const char32_t *>':
// Can't generate bindings for std::__wrap_iter<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__wrap_iter<const char32_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<const char16_t *>':
// Can't generate bindings for std::__wrap_iter<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__wrap_iter<const char16_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<const char *>':
// Can't generate bindings for std::__wrap_iter<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__wrap_iter<const char *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<char *>':
// Can't generate bindings for std::__wrap_iter<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__wrap_iter<char *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE is a template instantiation)

// Error while generating bindings for class 'std::allocator<char32_t>':
// Can't generate bindings for std::allocator<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator<char32_t> (crate::__CcTemplateInstNSt3__u9allocatorIDiEE is a template instantiation)

// Error while generating bindings for class 'std::allocator<char16_t>':
// Can't generate bindings for std::allocator<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator<char16_t> (crate::__CcTemplateInstNSt3__u9allocatorIDsEE is a template instantiation)

// Error while generating bindings for class 'std::allocator<char>':
// Can't generate bindings for std::allocator<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator<char> (crate::__CcTemplateInstNSt3__u9allocatorIcEE is a template instantiation)

// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char32_t>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::pmr::polymorphic_allocator<char32_t> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE is a template instantiation)

// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char16_t>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::pmr::polymorphic_allocator<char16_t> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE is a template instantiation)

// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::pmr::polymorphic_allocator<char> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIDiEEPDivEE is a template instantiation)

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIDsEEPDsvEE is a template instantiation)

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIcEEPcvEE is a template instantiation)

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::allocator<char32_t>, char32_t *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::allocator<char32_t>, char32_t *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::allocator<char32_t>, char32_t *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_9allocatorIDiEEPDivEE is a template instantiation)

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::allocator<char16_t>, char16_t *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::allocator<char16_t>, char16_t *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::allocator<char16_t>, char16_t *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_9allocatorIDsEEPDsvEE is a template instantiation)

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::allocator<char>, char *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::allocator<char>, char *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::allocator<char>, char *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_9allocatorIcEEPcvEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIcEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char32_t>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator_traits<std::allocator<char32_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char16_t>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator_traits<std::allocator<char16_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::allocator_traits<std::allocator<char>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIcEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_filebuf<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_filebuf<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_filebuf<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_filebuf<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ifstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ifstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ifstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ifstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ofstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ofstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ofstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ofstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_fstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_fstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_fstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_fstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_fstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_fstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ios<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ios<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ios<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ios<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ios<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ios<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_istream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_istream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_istream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_istream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_istream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_istream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_iostream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_iostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_iostream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_iostream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_iostream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_iostream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ostream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ostream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ostream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_streambuf<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_streambuf<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_streambuf<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_streambuf<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::fpos<__mbstate_t>':
// Can't generate bindings for std::fpos<__mbstate_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::fpos<__mbstate_t> (incomplete type)

// Error while generating bindings for struct 'std::char_traits<char>':
// Can't generate bindings for std::char_traits<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::char_traits<char> (crate::__CcTemplateInstNSt3__u11char_traitsIcEE is a template instantiation)

// Error while generating bindings for struct 'std::__char_traits_base<char32_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<char32_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__char_traits_base<char32_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDijLj4294967295EEE is a template instantiation)

// Error while generating bindings for struct 'std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>':
// Can't generate bindings for std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDstLt65535EEE is a template instantiation)

// Error while generating bindings for struct 'std::__char_traits_base<char8_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<char8_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__char_traits_base<char8_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDujLj4294967295EEE is a template instantiation)

// Error while generating bindings for struct 'std::__char_traits_base<wchar_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<wchar_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__char_traits_base<wchar_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIwjLj4294967295EEE is a template instantiation)

// Error while generating bindings for struct 'std::char_traits<wchar_t>':
// Can't generate bindings for std::char_traits<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::char_traits<wchar_t> (crate::__CcTemplateInstNSt3__u11char_traitsIwEE is a template instantiation)

// Error while generating bindings for struct 'std::char_traits<char8_t>':
// Can't generate bindings for std::char_traits<char8_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::char_traits<char8_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDuEE is a template instantiation)

// Error while generating bindings for struct 'std::char_traits<char16_t>':
// Can't generate bindings for std::char_traits<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::char_traits<char16_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDsEE is a template instantiation)

// Error while generating bindings for struct 'std::char_traits<char32_t>':
// Can't generate bindings for std::char_traits<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::char_traits<char32_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDiEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocation_result<char32_t *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char32_t *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__allocation_result<char32_t *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocation_result<char16_t *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char16_t *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__allocation_result<char16_t *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocation_result<char *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::__allocation_result<char *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string_view<char32_t, std::char_traits<char32_t>>':
// Can't generate bindings for std::basic_string_view<char32_t, std::char_traits<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<char32_t, std::char_traits<char32_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string_view<char16_t, std::char_traits<char16_t>>':
// Can't generate bindings for std::basic_string_view<char16_t, std::char_traits<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<char16_t, std::char_traits<char16_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string_view<char8_t, std::char_traits<char8_t>>':
// Can't generate bindings for std::basic_string_view<char8_t, std::char_traits<char8_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<char8_t, std::char_traits<char8_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE is a template instantiation)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < wchar_t , std :: char_traits < wchar_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        "std :: basic_string_view < wchar_t , std :: char_traits < wchar_t >>"
    ),
    crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE
);

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type (error: Can't generate bindings for std::char_traits<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::char_traits<wchar_t> (crate::__CcTemplateInstNSt3__u11char_traitsIwEE is a template instantiation))

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t>::value_type':
// Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t>::pointer':
// Unsupported type 'wchar_t *': Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t>::const_pointer':
// Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t>::reference':
// Unsupported type 'wchar_t &': Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t>::const_reference':
// Unsupported type 'const wchar_t &': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t>::const_iterator':
// Unsupported type 'std::basic_string_view<wchar_t>::const_pointer': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t>::iterator':
// Unsupported type 'std::basic_string_view<wchar_t>::const_iterator': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation))

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator (error: Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation)))

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size_type, because it is unsupported: b/200067824: type definitions nested inside templated records are not yet supported

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::difference_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::difference_type, because it is unsupported: b/200067824: type definitions nested inside templated records are not yet supported

// Error while generating bindings for global variable 'std::basic_string_view<wchar_t>::npos':
// static data members are not supported

// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator=':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator= (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for constructor 'std::basic_string_view<wchar_t>::basic_string_view':
// Parameter #0 is not supported: Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::basic_string_view':
// Function templates are not supported yet

// Error while generating bindings for constructor 'std::basic_string_view<wchar_t>::basic_string_view':
// Parameter #0 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::begin':
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_iterator': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::end':
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_iterator': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::cbegin':
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_iterator': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::cend':
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_iterator': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rbegin':
// Cannot use an error type by value: Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation))

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rend':
// Cannot use an error type by value: Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation))

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::crbegin':
// Cannot use an error type by value: Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation))

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::crend':
// Cannot use an error type by value: Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation))

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::size':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::length':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::max_size':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty ([[nodiscard]] attribute)

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::operator[]':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_reference': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::at':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_reference': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::front':
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_reference': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::back':
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_reference': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::data':
// Return type is not supported: Unsupported type 'std::basic_string_view<wchar_t>::const_pointer': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::remove_prefix':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::remove_suffix':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::copy':
// Parameter #0 is not supported: Unsupported type 'wchar_t *': Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::substr':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare ([[nodiscard]] attribute)

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #3 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #4 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::compare':
// Parameter #0 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #3 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find':
// Parameter #0 is not supported: Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find':
// Parameter #0 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::rfind':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::rfind':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::rfind':
// Parameter #0 is not supported: Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::rfind':
// Parameter #0 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_first_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_last_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_first_not_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_last_not_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t *': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
//
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t * _Nonnull': Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
//
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with ([[nodiscard]] attribute)

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::starts_with':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view<wchar_t>::value_type': No generated bindings found for 'value_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::starts_with':
// Parameter #0 is not supported: Unsupported type 'const std::basic_string_view<wchar_t>::value_type * _Nonnull': Unsupported type 'const std::basic_string_view<wchar_t>::value_type': No generated bindings found for 'value_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with ([[nodiscard]] attribute)

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::ends_with':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view<wchar_t>::value_type': No generated bindings found for 'value_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::ends_with':
// Parameter #0 is not supported: Unsupported type 'const std::basic_string_view<wchar_t>::value_type * _Nonnull': Unsupported type 'const std::basic_string_view<wchar_t>::value_type': No generated bindings found for 'value_type'

// Error while generating bindings for struct 'std::placeholders::__ph<10>':
// Can't generate bindings for std::placeholders::__ph<10>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<10> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi10EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<1>':
// Can't generate bindings for std::placeholders::__ph<1>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<1> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi1EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<2>':
// Can't generate bindings for std::placeholders::__ph<2>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<2> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi2EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<3>':
// Can't generate bindings for std::placeholders::__ph<3>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<3> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi3EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<4>':
// Can't generate bindings for std::placeholders::__ph<4>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<4> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi4EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<5>':
// Can't generate bindings for std::placeholders::__ph<5>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<5> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi5EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<6>':
// Can't generate bindings for std::placeholders::__ph<6>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<6> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi6EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<7>':
// Can't generate bindings for std::placeholders::__ph<7>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<7> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi7EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<8>':
// Can't generate bindings for std::placeholders::__ph<8>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<8> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi8EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<9>':
// Can't generate bindings for std::placeholders::__ph<9>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:composable_bridging_cc needs [//features:wrapper] for std::placeholders::__ph<9> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi9EEE is a template instantiation)

// Type bindings for rs_std::SliceRef<const int> suppressed due to being mapped to an existing Rust type (*const[::ffi_11::c_int])

// Type bindings for rs_std::SliceRef<std::string_view> suppressed due to being mapped to an existing Rust type (*mut[::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE])

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
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
            slice: *mut[::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE],
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
    }
}

const _: () = {
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
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIwNS_11char_traitsIwEEEE,
            __size_
        ) == 8
    );
    assert!(::core::mem::size_of::<*const [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*const [::ffi_11::c_int]>() == 8);
    assert!(
        ::core::mem::size_of::<
            *mut [::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE],
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            *mut [::cc_std::__CcTemplateInstNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE],
        >() == 8
    );
};
