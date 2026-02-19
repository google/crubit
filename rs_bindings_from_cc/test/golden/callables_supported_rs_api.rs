// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_supported_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for function 'invoke_once':
// while generating bridge param 'f': Callables require the `callables` feature, but target `BazelLabel("//rs_bindings_from_cc/test/golden:callables_supported_cc")` does not have it enabled.

// Error while generating bindings for function 'invoke':
// while generating bridge param 'f': Callables require the `callables` feature, but target `BazelLabel("//rs_bindings_from_cc/test/golden:callables_supported_cc")` does not have it enabled.

// Error while generating bindings for function 'invoke_const':
// while generating bridge param 'f': Callables require the `callables` feature, but target `BazelLabel("//rs_bindings_from_cc/test/golden:callables_supported_cc")` does not have it enabled.

// Error while generating bindings for function 'map_int':
// while generating bridge param 'f': Callables require the `callables` feature, but target `BazelLabel("//rs_bindings_from_cc/test/golden:callables_supported_cc")` does not have it enabled.

// Error while generating bindings for function 'map_bridged':
// while generating bridge param 'f': Callables require the `callables` feature, but target `BazelLabel("//rs_bindings_from_cc/test/golden:callables_supported_cc")` does not have it enabled.

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ABICompatible
pub struct ABICompatible {
    pub x: ::ffi_11::c_int,
}
impl !Send for ABICompatible {}
impl !Sync for ABICompatible {}
unsafe impl ::cxx::ExternType for ABICompatible {
    type Id = ::cxx::type_id!("ABICompatible");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ABICompatible {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ABICompatibleC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for function 'map_abi_compatible':
// while generating bridge param 'f': Callables require the `callables` feature, but target `BazelLabel("//rs_bindings_from_cc/test/golden:callables_supported_cc")` does not have it enabled.

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=LayoutCompatible
pub struct LayoutCompatible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) private_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for LayoutCompatible {}
impl !Sync for LayoutCompatible {}
unsafe impl ::cxx::ExternType for LayoutCompatible {
    type Id = ::cxx::type_id!("LayoutCompatible");
    type Kind = ::cxx::kind::Trivial;
}
impl LayoutCompatible {
    #[inline(always)]
    pub fn Create(x: ::ffi_11::c_int) -> crate::LayoutCompatible {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN16LayoutCompatible6CreateEi(
                &raw mut __return as *mut ::core::ffi::c_void,
                x,
            );
            __return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn get(__this: *const Self) -> ::ffi_11::c_int {
        crate::detail::__rust_thunk___ZNK16LayoutCompatible3getEv(__this)
    }
}

// Error while generating bindings for function 'map_layout_compatible':
// while generating bridge param 'f': Callables require the `callables` feature, but target `BazelLabel("//rs_bindings_from_cc/test/golden:callables_supported_cc")` does not have it enabled.

// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

// Error while generating bindings for class 'std::initializer_list<char32_t>':
// Can't generate bindings for std::initializer_list<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::initializer_list<char32_t> (crate::__CcTemplateInstSt16initializer_listIDiE is a template instantiation)

// Error while generating bindings for class 'std::initializer_list<char16_t>':
// Can't generate bindings for std::initializer_list<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::initializer_list<char16_t> (crate::__CcTemplateInstSt16initializer_listIDsE is a template instantiation)

// Error while generating bindings for class 'std::initializer_list<char>':
// Can't generate bindings for std::initializer_list<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::initializer_list<char> (crate::__CcTemplateInstSt16initializer_listIcE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<char32_t *>':
// Can't generate bindings for std::iterator_traits<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::iterator_traits<char32_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPDiEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<char16_t *>':
// Can't generate bindings for std::iterator_traits<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::iterator_traits<char16_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPDsEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<const char32_t *>':
// Can't generate bindings for std::iterator_traits<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::iterator_traits<const char32_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKDiEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<const char16_t *>':
// Can't generate bindings for std::iterator_traits<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::iterator_traits<const char16_t *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKDsEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<const char *>':
// Can't generate bindings for std::iterator_traits<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::iterator_traits<const char *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPKcEE is a template instantiation)

// Error while generating bindings for struct 'std::iterator_traits<char *>':
// Can't generate bindings for std::iterator_traits<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::iterator_traits<char *> (crate::__CcTemplateInstNSt3__u15iterator_traitsIPcEE is a template instantiation)

// Error while generating bindings for struct 'std::pointer_traits<char32_t *>':
// Can't generate bindings for std::pointer_traits<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::pointer_traits<char32_t *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPDiEE is a template instantiation)

// Error while generating bindings for struct 'std::pointer_traits<char16_t *>':
// Can't generate bindings for std::pointer_traits<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::pointer_traits<char16_t *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPDsEE is a template instantiation)

// Error while generating bindings for struct 'std::pointer_traits<char *>':
// Can't generate bindings for std::pointer_traits<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::pointer_traits<char *> (crate::__CcTemplateInstNSt3__u14pointer_traitsIPcEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<10>':
// Can't generate bindings for std::placeholders::__ph<10>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<10> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi10EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<1>':
// Can't generate bindings for std::placeholders::__ph<1>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<1> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi1EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<2>':
// Can't generate bindings for std::placeholders::__ph<2>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<2> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi2EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<3>':
// Can't generate bindings for std::placeholders::__ph<3>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<3> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi3EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<4>':
// Can't generate bindings for std::placeholders::__ph<4>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<4> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi4EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<5>':
// Can't generate bindings for std::placeholders::__ph<5>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<5> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi5EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<6>':
// Can't generate bindings for std::placeholders::__ph<6>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<6> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi6EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<7>':
// Can't generate bindings for std::placeholders::__ph<7>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<7> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi7EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<8>':
// Can't generate bindings for std::placeholders::__ph<8>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<8> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi8EEE is a template instantiation)

// Error while generating bindings for struct 'std::placeholders::__ph<9>':
// Can't generate bindings for std::placeholders::__ph<9>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::placeholders::__ph<9> (crate::__CcTemplateInstNSt3__u12placeholders4__phILi9EEE is a template instantiation)

// Error while generating bindings for class 'std::allocator<char32_t>':
// Can't generate bindings for std::allocator<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator<char32_t> (crate::__CcTemplateInstNSt3__u9allocatorIDiEE is a template instantiation)

// Error while generating bindings for class 'std::allocator<char16_t>':
// Can't generate bindings for std::allocator<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator<char16_t> (crate::__CcTemplateInstNSt3__u9allocatorIDsEE is a template instantiation)

// Error while generating bindings for class 'std::allocator<char>':
// Can't generate bindings for std::allocator<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator<char> (crate::__CcTemplateInstNSt3__u9allocatorIcEE is a template instantiation)

// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char32_t>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::pmr::polymorphic_allocator<char32_t> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE is a template instantiation)

// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char16_t>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::pmr::polymorphic_allocator<char16_t> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE is a template instantiation)

// Error while generating bindings for class 'std::pmr::polymorphic_allocator<char>':
// Can't generate bindings for std::pmr::polymorphic_allocator<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::pmr::polymorphic_allocator<char> (crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::pmr::polymorphic_allocator<char32_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>':
// Can't generate bindings for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string<char32_t, std::char_traits<char32_t>, std::allocator<char32_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::pmr::polymorphic_allocator<char16_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>':
// Can't generate bindings for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string<char16_t, std::char_traits<char16_t>, std::allocator<char16_t>> (crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'char8_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_string':
// Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for class 'std::basic_ostream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ostream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ostream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ostream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char32_t>, char32_t *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIDiEEPDivEE is a template instantiation)

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char16_t>, char16_t *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIDsEEPDsvEE is a template instantiation)

// Error while generating bindings for struct 'std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>':
// Can't generate bindings for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__alloc_traits_difference_type<std::pmr::polymorphic_allocator<char>, char *, void> (crate::__CcTemplateInstNSt3__u30__alloc_traits_difference_typeINS_3pmr21polymorphic_allocatorIcEEPcvEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char32_t>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_3pmr21polymorphic_allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char16_t>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_3pmr21polymorphic_allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>>':
// Can't generate bindings for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocator_traits_base<std::pmr::polymorphic_allocator<char>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_3pmr21polymorphic_allocatorIcEEEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocator_traits_base<std::allocator<char32_t>>':
// Can't generate bindings for std::__allocator_traits_base<std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocator_traits_base<std::allocator<char32_t>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_9allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocator_traits_base<std::allocator<char16_t>>':
// Can't generate bindings for std::__allocator_traits_base<std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocator_traits_base<std::allocator<char16_t>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_9allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocator_traits_base<std::allocator<char>>':
// Can't generate bindings for std::__allocator_traits_base<std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocator_traits_base<std::allocator<char>> (crate::__CcTemplateInstNSt3__u23__allocator_traits_baseINS_9allocatorIcEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char32_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char16_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::pmr::polymorphic_allocator<char>>':
// Can't generate bindings for std::allocator_traits<std::pmr::polymorphic_allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator_traits<std::pmr::polymorphic_allocator<char>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_3pmr21polymorphic_allocatorIcEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char32_t>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator_traits<std::allocator<char32_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIDiEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char16_t>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator_traits<std::allocator<char16_t>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIDsEEEE is a template instantiation)

// Error while generating bindings for struct 'std::allocator_traits<std::allocator<char>>':
// Can't generate bindings for std::allocator_traits<std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::allocator_traits<std::allocator<char>> (crate::__CcTemplateInstNSt3__u16allocator_traitsINS_9allocatorIcEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char32_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char32_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char32_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char16_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char16_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char16_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char32_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char32_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char32_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char16_t *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char16_t *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char16_t *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<const char *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<const char *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<const char *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<std::__wrap_iter<char *>>':
// Can't generate bindings for std::reverse_iterator<std::__wrap_iter<char *>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<std::__wrap_iter<char *>> (crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const char32_t *>':
// Can't generate bindings for std::reverse_iterator<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<const char32_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const char16_t *>':
// Can't generate bindings for std::reverse_iterator<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<const char16_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const char8_t *>':
// Can't generate bindings for std::reverse_iterator<const char8_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<const char8_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const char *>':
// Can't generate bindings for std::reverse_iterator<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<const char *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE is a template instantiation)

// Error while generating bindings for class 'std::reverse_iterator<const wchar_t *>':
// Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<char32_t *>':
// Can't generate bindings for std::__wrap_iter<char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__wrap_iter<char32_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<char16_t *>':
// Can't generate bindings for std::__wrap_iter<char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__wrap_iter<char16_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<const char32_t *>':
// Can't generate bindings for std::__wrap_iter<const char32_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__wrap_iter<const char32_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<const char16_t *>':
// Can't generate bindings for std::__wrap_iter<const char16_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__wrap_iter<const char16_t *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<const char *>':
// Can't generate bindings for std::__wrap_iter<const char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__wrap_iter<const char *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE is a template instantiation)

// Error while generating bindings for class 'std::__wrap_iter<char *>':
// Can't generate bindings for std::__wrap_iter<char *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__wrap_iter<char *> (crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocation_result<char32_t *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char32_t *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocation_result<char32_t *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocation_result<char16_t *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char16_t *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocation_result<char16_t *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE is a template instantiation)

// Error while generating bindings for struct 'std::__allocation_result<char *, unsigned long>':
// Can't generate bindings for std::__allocation_result<char *, unsigned long>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__allocation_result<char *, unsigned long> (crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE is a template instantiation)

// Error while generating bindings for class 'std::basic_filebuf<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_filebuf<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_filebuf<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_filebuf<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_filebuf<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ifstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ifstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ifstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ifstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ifstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ofstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ofstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ofstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ofstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ofstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_fstream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_fstream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_fstream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_fstream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_fstream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_fstream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ios<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_ios<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ios<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ios<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_ios<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ios<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_istream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_istream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_istream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_istream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_istream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_istream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_iostream<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_iostream<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_iostream<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_iostream<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_iostream<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_iostream<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_stringbuf<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_stringbuf<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_istringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_istringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ostringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_ostringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>':
// Can't generate bindings for std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_stringstream<char, std::char_traits<char>, std::allocator<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>':
// Can't generate bindings for std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_stringstream<wchar_t, std::char_traits<wchar_t>, std::allocator<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::basic_streambuf<char, std::char_traits<char>>':
// Can't generate bindings for std::basic_streambuf<char, std::char_traits<char>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_streambuf<char, std::char_traits<char>> (incomplete type)

// Error while generating bindings for class 'std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_streambuf<wchar_t, std::char_traits<wchar_t>> (incomplete type)

// Error while generating bindings for class 'std::fpos<__mbstate_t>':
// Can't generate bindings for std::fpos<__mbstate_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::fpos<__mbstate_t> (incomplete type)

// Error while generating bindings for struct 'std::char_traits<char>':
// Can't generate bindings for std::char_traits<char>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::char_traits<char> (crate::__CcTemplateInstNSt3__u11char_traitsIcEE is a template instantiation)

// Error while generating bindings for struct 'std::__char_traits_base<char32_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<char32_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__char_traits_base<char32_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDijLj4294967295EEE is a template instantiation)

// Error while generating bindings for struct 'std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>':
// Can't generate bindings for std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__char_traits_base<char16_t, unsigned short, (unsigned short)65535> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDstLt65535EEE is a template instantiation)

// Error while generating bindings for struct 'std::__char_traits_base<char8_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<char8_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__char_traits_base<char8_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIDujLj4294967295EEE is a template instantiation)

// Error while generating bindings for struct 'std::__char_traits_base<wchar_t, unsigned int, 4294967295U>':
// Can't generate bindings for std::__char_traits_base<wchar_t, unsigned int, 4294967295U>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::__char_traits_base<wchar_t, unsigned int, 4294967295U> (crate::__CcTemplateInstNSt3__u18__char_traits_baseIwjLj4294967295EEE is a template instantiation)

// Error while generating bindings for struct 'std::char_traits<wchar_t>':
// Can't generate bindings for std::char_traits<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::char_traits<wchar_t> (crate::__CcTemplateInstNSt3__u11char_traitsIwEE is a template instantiation)

// Error while generating bindings for struct 'std::char_traits<char8_t>':
// Can't generate bindings for std::char_traits<char8_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::char_traits<char8_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDuEE is a template instantiation)

// Error while generating bindings for struct 'std::char_traits<char16_t>':
// Can't generate bindings for std::char_traits<char16_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::char_traits<char16_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDsEE is a template instantiation)

// Error while generating bindings for struct 'std::char_traits<char32_t>':
// Can't generate bindings for std::char_traits<char32_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::char_traits<char32_t> (crate::__CcTemplateInstNSt3__u11char_traitsIDiEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string_view<char32_t, std::char_traits<char32_t>>':
// Can't generate bindings for std::basic_string_view<char32_t, std::char_traits<char32_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string_view<char32_t, std::char_traits<char32_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string_view<char16_t, std::char_traits<char16_t>>':
// Can't generate bindings for std::basic_string_view<char16_t, std::char_traits<char16_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string_view<char16_t, std::char_traits<char16_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE is a template instantiation)

// Error while generating bindings for class 'std::basic_string_view<char8_t, std::char_traits<char8_t>>':
// Can't generate bindings for std::basic_string_view<char8_t, std::char_traits<char8_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string_view<char8_t, std::char_traits<char8_t>> (crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE is a template instantiation)

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
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::traits_type (error: Can't generate bindings for std::char_traits<wchar_t>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::char_traits<wchar_t> (crate::__CcTemplateInstNSt3__u11char_traitsIwEE is a template instantiation))

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::pointer':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::pointer due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_pointer':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_pointer due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reference':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reference due to missing bindings for its dependency: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reference':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reference due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_iterator due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::iterator due to missing bindings for its dependency: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator (error: Can't generate bindings for std::reverse_iterator<const wchar_t *>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::reverse_iterator<const wchar_t *> (crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE is a template instantiation))

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:wrapper] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::reverse_iterator (error: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator)

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size_type, because it is unsupported: b/485949049: type definitions nested inside templated records are not yet supported

// Error while generating bindings for type alias 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::difference_type':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::difference_type, because it is unsupported: b/485949049: type definitions nested inside templated records are not yet supported

// Error while generating bindings for global variable 'std::basic_string_view<wchar_t>::npos':
// static data members are not supported

// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator=':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator= (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t>::basic_string_view':
// Function templates are not supported yet

// Error while generating bindings for constructor 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::std::basic_string_view<wchar_t, std::char_traits<wchar_t>>':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::begin':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::end':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::cbegin':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::cend':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rbegin':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rend':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::crbegin':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::crend':
// Cannot use an error type by value: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::const_reverse_iterator

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::size':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::length':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::max_size':
// Return type is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::empty ([[nodiscard]] attribute)

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::operator[]':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::at':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::front':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::back':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::data':
// Return type is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::remove_prefix':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::remove_suffix':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::swap (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::copy':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::substr':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare ([[nodiscard]] attribute)

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #3 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #4 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::compare':
// Parameter #0 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #3 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::rfind':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_first_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'
// Parameter #2 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::find_last_not_of':
// Parameter #0 is not supported: Unsupported type 'const wchar_t': Unsupported type 'wchar_t': Unsupported builtin type
// Parameter #1 is not supported: Unsupported type 'std::basic_string_view::size_type': No generated bindings found for 'size_type'

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with ([[nodiscard]] attribute)

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::starts_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Can't generate bindings for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_supported_cc needs [//features:experimental] for std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with ([[nodiscard]] attribute)

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

// Error while generating bindings for function 'std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::ends_with':
// Parameter #0 is not supported: Unsupported type alias std::basic_string_view<wchar_t, std::char_traits<wchar_t>>::value_type

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN13ABICompatibleC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN16LayoutCompatible6CreateEi(
            __return: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK16LayoutCompatible3getEv(
            __this: *const crate::LayoutCompatible,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ABICompatible>() == 4);
    assert!(::core::mem::align_of::<crate::ABICompatible>() == 4);
    static_assertions::assert_impl_all!(crate::ABICompatible: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ABICompatible: Drop);
    assert!(::core::mem::offset_of!(crate::ABICompatible, x) == 0);
    assert!(::core::mem::size_of::<crate::LayoutCompatible>() == 4);
    assert!(::core::mem::align_of::<crate::LayoutCompatible>() == 4);
    static_assertions::assert_impl_all!(crate::LayoutCompatible: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::LayoutCompatible: Drop);
    assert!(::core::mem::offset_of!(crate::LayoutCompatible, private_) == 0);
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
};
