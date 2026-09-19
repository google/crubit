// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/consume_absl:absl_functional

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
extern crate alloc;

// error: class `MyOption` could not be bound
//   Class templates are not yet supported

/// Calls the invocable and returns void.
#[inline(always)]
pub fn CallVoidVoid(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z12CallVoidVoidN4absl12AnyInvocableIFvvOEEE(::bridge_rust::unstable_encode!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move||{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _) }; }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

/// Returns an invocable that returns 42.
#[inline(always)]
pub fn ReturnIntMapper() -> ::alloc::boxed::Box<
    dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
        + ::core::marker::Send
        + ::core::marker::Sync
        + 'static,
> {
    unsafe {
        ::bridge_rust::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: ::ffi_11::c_int|->::ffi_11::c_int{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,param_0) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z15ReturnIntMapperv(__crubit_return_abi_buffer,); })
    }
}

/// Returns an AnyInvocable that takes a MyOption<int> and returns a
/// MyOption<int>.
#[inline(always)]
pub fn MyOptionIntMapper() -> ::alloc::boxed::Box<
    dyn ::core::ops::Fn(crate::MyOption<::ffi_11::c_int>) -> crate::MyOption<::ffi_11::c_int>
        + ::core::marker::Send
        + ::core::marker::Sync
        + 'static,
> {
    unsafe {
        ::bridge_rust::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::MyOption<::ffi_11::c_int>|->crate::MyOption<::ffi_11::c_int>{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: crate::MyOption<::ffi_11::c_int>|->crate::MyOption<::ffi_11::c_int>{ ::bridge_rust::unstable_return!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<::core::ffi::c_int>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<::core::ffi::c_int>>,|out|{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,::bridge_rust::unstable_encode!(@crate::MyOptionAbi(::bridge_rust::transmute_abi::<::core::ffi::c_int>()),crate::MyOptionAbi<::bridge_rust::TransmuteAbi<::core::ffi::c_int>>,param_0).as_ptr()as*const u8,out) } }) }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(crate::MyOption<::ffi_11::c_int>)->crate::MyOption<::ffi_11::c_int>+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z17MyOptionIntMapperv(__crubit_return_abi_buffer,); })
    }
}

/// Returns an AnyInvocable without const qualifier, which Crubit promotes to
/// Fn in Rust.
#[inline(always)]
pub fn ReturnNonConstIntMapper() -> ::alloc::boxed::Box<
    dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
        + ::core::marker::Send
        + ::core::marker::Sync
        + 'static,
> {
    unsafe {
        ::bridge_rust::unstable_return!(@::any_invocable::AnyInvocableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),|raw_any_invocable: ::cc_std::std::unique_ptr<::any_invocable::RawAnyInvocable>|->::alloc::boxed::Box<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>{ ::alloc::boxed::Box::new(move|param_0: ::ffi_11::c_int|->::ffi_11::c_int{ unsafe{ crate::detail::__crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(::cc_std::std::unique_ptr::as_ptr(&raw_any_invocable)as*mut _,param_0) } }) },),::any_invocable::AnyInvocableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z23ReturnNonConstIntMapperv(__crubit_return_abi_buffer,); })
    }
}

// error: struct `Incomplete` could not be bound
//   incomplete type

// error: function `ReturnIncompleteMapper` could not be bound
//   Return type is not supported: Unsupported type 'absl::AnyInvocable<Incomplete (Incomplete) const>': Failed to create bindings for template specialization type absl::AnyInvocable<Incomplete (Incomplete) const>: Return type of callable is incomplete: struct Incomplete

// error: function `CallIncompleteMapper` could not be bound
//   Parameter #0 is not supported: Unsupported type 'absl::AnyInvocable<Incomplete (Incomplete) const>': Failed to create bindings for template specialization type absl::AnyInvocable<Incomplete (Incomplete) const>: Return type of callable is incomplete: struct Incomplete

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi10EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 10 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi10EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi10EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi10EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi10EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi10EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 1 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi1EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi1EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi2EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 2 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi2EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi2EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi2EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi2EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi2EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi3EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 3 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi3EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi3EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi3EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi3EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi3EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi4EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 4 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi4EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi4EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi4EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi4EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi4EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi5EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 5 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi5EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi5EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi5EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi5EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi5EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi6EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 6 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi6EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi6EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi6EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi6EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi6EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi7EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 7 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi7EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi7EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi7EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi7EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi7EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi8EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 8 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi8EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi8EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi8EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi8EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi8EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12placeholders4__phILi9EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: placeholders :: __ph < 9 >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12placeholders4__phILi9EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u12placeholders4__phILi9EEE {}
impl !Sync for __CcTemplateInstNSt3__u12placeholders4__phILi9EEE {}

impl Default for __CcTemplateInstNSt3__u12placeholders4__phILi9EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi9EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u9allocatorIDiEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: allocator < char32_t >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u9allocatorIDiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u9allocatorIDiEE {}
impl !Sync for __CcTemplateInstNSt3__u9allocatorIDiEE {}
impl __CcTemplateInstNSt3__u9allocatorIDiEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn allocate(__this: *mut Self, __n: usize) -> *mut u32 {
        unsafe { self::cc_template_inst_n_st3_u9allocator_i_di_ee::allocate(__this, __n) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__p`: raw pointer
    #[inline(always)]
    pub unsafe fn deallocate(__this: *mut Self, __p: *mut u32, __n: usize) {
        unsafe { self::cc_template_inst_n_st3_u9allocator_i_di_ee::deallocate(__this, __p, __n) }
    }
}

impl Default for __CcTemplateInstNSt3__u9allocatorIDiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__7cb2b833__ZNSt3__u9allocatorIDiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u9allocator_i_di_ee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn allocate(
        __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        __n: usize,
    ) -> *mut u32 {
        unsafe {
            crate::detail::__rust_thunk__7b30cda6__ZNSt3__u9allocatorIDiE8allocateEm(__this, __n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn deallocate(
        __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        __p: *mut u32,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__8d7cdda6__ZNSt3__u9allocatorIDiE10deallocateEPDim(
                __this, __p, __n,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u9allocatorIDsEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: allocator < char16_t >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u9allocatorIDsEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u9allocatorIDsEE {}
impl !Sync for __CcTemplateInstNSt3__u9allocatorIDsEE {}
impl __CcTemplateInstNSt3__u9allocatorIDsEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn allocate(__this: *mut Self, __n: usize) -> *mut u16 {
        unsafe { self::cc_template_inst_n_st3_u9allocator_i_ds_ee::allocate(__this, __n) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__p`: raw pointer
    #[inline(always)]
    pub unsafe fn deallocate(__this: *mut Self, __p: *mut u16, __n: usize) {
        unsafe { self::cc_template_inst_n_st3_u9allocator_i_ds_ee::deallocate(__this, __p, __n) }
    }
}

impl Default for __CcTemplateInstNSt3__u9allocatorIDsEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__7cb2b833__ZNSt3__u9allocatorIDsEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u9allocator_i_ds_ee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn allocate(
        __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        __n: usize,
    ) -> *mut u16 {
        unsafe {
            crate::detail::__rust_thunk__7b30cda6__ZNSt3__u9allocatorIDsE8allocateEm(__this, __n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn deallocate(
        __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        __p: *mut u16,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__8d7cdda6__ZNSt3__u9allocatorIDsE10deallocateEPDsm(
                __this, __p, __n,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u9allocatorIcEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: allocator < char >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u9allocatorIcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u9allocatorIcEE {}
impl !Sync for __CcTemplateInstNSt3__u9allocatorIcEE {}
impl __CcTemplateInstNSt3__u9allocatorIcEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn allocate(__this: *mut Self, __n: usize) -> *mut ::ffi_11::c_char {
        unsafe { self::cc_template_inst_n_st3_u9allocator_ic_ee::allocate(__this, __n) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__p`: raw pointer
    #[inline(always)]
    pub unsafe fn deallocate(__this: *mut Self, __p: *mut ::ffi_11::c_char, __n: usize) {
        unsafe { self::cc_template_inst_n_st3_u9allocator_ic_ee::deallocate(__this, __p, __n) }
    }
}

impl Default for __CcTemplateInstNSt3__u9allocatorIcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__7cb2b833__ZNSt3__u9allocatorIcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u9allocator_ic_ee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn allocate(
        __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIcEE,
        __n: usize,
    ) -> *mut ::ffi_11::c_char {
        unsafe {
            crate::detail::__rust_thunk__7b30cda6__ZNSt3__u9allocatorIcE8allocateEm(__this, __n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn deallocate(
        __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIcEE,
        __p: *mut ::ffi_11::c_char,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__8d7cdda6__ZNSt3__u9allocatorIcE10deallocateEPcm(
                __this, __p, __n,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: pmr :: polymorphic_allocator < char32_t >
pub struct __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __res_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE {}
impl !Sync for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE {}

impl Default for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__2107cf07__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*mut ::cc_std::std::__u::pmr::memory_resource>
    for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *mut ::cc_std::std::__u::pmr::memory_resource) -> Self {
        let mut __r = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__227390d7__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1EPNS0_15memory_resourceE(&raw mut tmp as*mut _,__r);
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*mut ::cc_std::std::__u::pmr::memory_resource>
    for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *mut ::cc_std::std::__u::pmr::memory_resource) -> Self::CtorType {
        unsafe {
            <Self as ::ctor::UnsafeFrom<*mut ::cc_std::std::__u::pmr::memory_resource>>::unsafe_from(
                args,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: pmr :: polymorphic_allocator < char16_t >
pub struct __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __res_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE {}
impl !Sync for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE {}

impl Default for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__2107cf07__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*mut ::cc_std::std::__u::pmr::memory_resource>
    for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *mut ::cc_std::std::__u::pmr::memory_resource) -> Self {
        let mut __r = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__227390d7__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1EPNS0_15memory_resourceE(&raw mut tmp as*mut _,__r);
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*mut ::cc_std::std::__u::pmr::memory_resource>
    for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *mut ::cc_std::std::__u::pmr::memory_resource) -> Self::CtorType {
        unsafe {
            <Self as ::ctor::UnsafeFrom<*mut ::cc_std::std::__u::pmr::memory_resource>>::unsafe_from(
                args,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: pmr :: polymorphic_allocator < char >
pub struct __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __res_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE {}
impl !Sync for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE {}

impl Default for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__2107cf07__ZNSt3__u3pmr21polymorphic_allocatorIcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*mut ::cc_std::std::__u::pmr::memory_resource>
    for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: *mut ::cc_std::std::__u::pmr::memory_resource) -> Self {
        let mut __r = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__227390d7__ZNSt3__u3pmr21polymorphic_allocatorIcEC1EPNS0_15memory_resourceE(&raw mut tmp as*mut _,__r);
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*mut ::cc_std::std::__u::pmr::memory_resource>
    for __CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *mut ::cc_std::std::__u::pmr::memory_resource) -> Self::CtorType {
        unsafe {
            <Self as ::ctor::UnsafeFrom<*mut ::cc_std::std::__u::pmr::memory_resource>>::unsafe_from(
                args,
            )
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string < char32_t , std :: char_traits < char32_t >, std :: pmr :: polymorphic_allocator < char32_t >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE
{
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) __rep_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) __alloc_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl!Send for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{  }
impl!Sync for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{  }
impl __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ #[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn cbegin(__this: *const Self)->crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::cbegin(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn cend(__this: *const Self)->crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::cend(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn crbegin(__this: *const Self)->crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::crbegin(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn crend(__this: *const Self)->crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::crend(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn reserve(__this: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::reserve(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn empty(__this: *const Self)->bool{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::empty(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn c_str(__this: *const Self)->*const u32{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::c_str(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn get_allocator(__this: *const Self)->crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::get_allocator(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn push_back(__this: *mut Self,__c: u32){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::push_back(__this,__c) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn pop_back(__this: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::pop_back(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn clear(__this: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::clear(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn shrink_to_fit(__this: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::shrink_to_fit(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer\n * `__str`: raw pointer"]#[inline(always)]pub unsafe fn swap(__this: *mut Self,__str: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee::swap(__this,__str) } } }

impl::ctor::CtorNew<()>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=::ctor::Ctor![Self];type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: ())->Self::CtorType{ let()=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__6e53fd45__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1Ev(__crubit_dest as*mut::core::ffi::c_void); }) } } }

impl<'__unelided>::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE)->Self::CtorType{ let mut __a=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__f065fef8__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS5_(__crubit_dest as*mut::core::ffi::c_void,__a); }) } } }
impl<'__unelided>::ctor::CtorNew<(&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE,)>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<&'__unelided Self>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &'__unelided Self)->Self::CtorType{ let mut __str=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_(__crubit_dest as*mut::core::ffi::c_void,__str); }) } } }
impl<'__unelided>::ctor::CtorNew<(&'__unelided Self,)>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided Self,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<(&'__unelided Self,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE)>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided Self,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE))->Self::CtorType{ let(mut __str,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_RKS5_(__crubit_dest as*mut::core::ffi::c_void,__str,__a); }) } } }

impl<'__unelided>::ctor::CtorNew<::ctor::RvalueReference<'__unelided,Self>>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: ::ctor::RvalueReference<'__unelided,Self>)->Self::CtorType{ let mut __str=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__8e81d883__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_(__crubit_dest as*mut::core::ffi::c_void,__str); }) } } }
impl<'__unelided>::ctor::CtorNew<(::ctor::RvalueReference<'__unelided,Self>,)>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (::ctor::RvalueReference<'__unelided,Self>,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<::ctor::RvalueReference<'__unelided,Self>>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<(::ctor::RvalueReference<'__unelided,Self>,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE)>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (::ctor::RvalueReference<'__unelided,Self>,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE))->Self::CtorType{ let(mut __str,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__91697c2d__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_RKS5_(__crubit_dest as*mut::core::ffi::c_void,__str,__a); }) } } }

impl<'__unelided>::ctor::CtorNew<(crate::__CcTemplateInstSt16initializer_listIDiE,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE)>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (crate::__CcTemplateInstSt16initializer_listIDiE,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE))->Self::CtorType{ let(mut __il,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ESt16initializer_listIDiERKS5_(__crubit_dest as*mut::core::ffi::c_void,&mut __il,__a); }) } } }

impl::ctor::PinnedDrop for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ #[inline(always)]unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>){ unsafe{ crate::detail::__rust_thunk__3aa0a429__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEED1Ev(self) } } }

impl::ctor::Assign<::ctor::RvalueReference<'_,Self>>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__str: ::ctor::RvalueReference<'_,Self>){ unsafe{ crate::detail::__rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEOS6_(self,__str); } } }

impl::ctor::Assign<crate::__CcTemplateInstSt16initializer_listIDiE>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,mut __il: crate::__CcTemplateInstSt16initializer_listIDiE){ unsafe{ crate::detail::__rust_thunk__0e3b9316__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSESt16initializer_listIDiE(self,&mut __il); } } }

impl::ctor::Assign<*const u32>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__s: *const u32){ unsafe{ crate::detail::__rust_thunk__4cd278b5__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEPKDi(self,__s); } } }

impl::ctor::Assign<u32>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__c: u32){ unsafe{ crate::detail::__rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEDi(self,__c); } } }

impl::ctor::Assign<&Self>for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__str: &Self){ unsafe{ crate::detail::__rust_thunk__13806d1e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSERKS6_(self,__str); } } }

pub mod cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_3pmr21polymorphic_allocator_i_di_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE6cbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4cendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn reserve(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__ec767001__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7reserveEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__d528730e__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn c_str(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5c_strEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn get_allocator(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE,
            >::uninit();
            crate::detail::__rust_thunk__21b14257__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13get_allocatorEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn push_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        __c: u32,
    ) {
        unsafe {
            crate::detail::__rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE9push_backEDi(__this,__c)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn pop_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE8pop_backEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn clear(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__0df906ad__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5clearEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn shrink_to_fit(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__ca1a6967__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13shrink_to_fitEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__str`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn swap(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__2f27b851__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4swapERS6_(__this,__str)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string < char32_t , std :: char_traits < char32_t >, std :: allocator < char32_t >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) __rep_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
}
impl !Send for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE {}
impl __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::cbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cend(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::cend(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::crbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::crend(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::size(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn length(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::length(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn max_size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::max_size(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn capacity(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::capacity(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::empty(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn substr(
        __this: *const Self,
        __pos: usize,
        __n: usize,
    ) -> ::ctor::Ctor![
        crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
    ] {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::substr(__this,__pos,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn c_str(__this: *const Self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::c_str(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn get_allocator(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u9allocatorIDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::get_allocator(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn push_back(__this: *mut Self, __c: u32) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::push_back(__this,__c)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn pop_back(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::pop_back(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn clear(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::clear(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn shrink_to_fit(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::shrink_to_fit(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__s`: raw pointer
    #[inline(always)]
    pub unsafe fn copy(__this: *const Self, __s: *mut u32, __n: usize, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::copy(__this,__s,__n,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__str`: raw pointer
    #[inline(always)]
    pub unsafe fn swap(__this: *mut Self, __str: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee::swap(__this,__str)
        }
    }
}

impl ::ctor::CtorNew<()>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__6e53fd45__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1Ev(__crubit_dest as*mut::core::ffi::c_void);
            })
        }
    }
}

impl<'__unelided> ::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
    ) -> Self::CtorType {
        let mut __a = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__f065fef8__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS4_(__crubit_dest as*mut::core::ffi::c_void,__a);
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(&'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,)>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (&'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<&'__unelided Self>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
        let mut __str = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_(__crubit_dest as*mut::core::ffi::c_void,__str);
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(&'__unelided Self, &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE)>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (&'__unelided Self, &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE),
    ) -> Self::CtorType {
        let (mut __str, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_RKS4_(__crubit_dest as*mut::core::ffi::c_void,__str,__a);
            })
        }
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __str = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8e81d883__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_(__crubit_dest as*mut::core::ffi::c_void,__str);
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(
        ::ctor::RvalueReference<'__unelided, Self>,
        &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
    )> for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            ::ctor::RvalueReference<'__unelided, Self>,
            &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        ),
    ) -> Self::CtorType {
        let (mut __str, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__91697c2d__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_RKS4_(__crubit_dest as*mut::core::ffi::c_void,__str,__a);
            })
        }
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(
        &'__unelided Self,
        usize,
        usize,
        &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
    )> for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            &'__unelided Self,
            usize,
            usize,
            &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        ),
    ) -> Self::CtorType {
        let (mut __str, mut __pos, mut __n, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__ea3378b5__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mmRKS4_(__crubit_dest as*mut::core::ffi::c_void,__str,__pos,__n,__a);
            })
        }
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(
        &'__unelided Self,
        usize,
        &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
    )> for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            &'__unelided Self,
            usize,
            &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        ),
    ) -> Self::CtorType {
        let (mut __str, mut __pos, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__6c7db170__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mRKS4_(__crubit_dest as*mut::core::ffi::c_void,__str,__pos,__a);
            })
        }
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(
        crate::__CcTemplateInstSt16initializer_listIDiE,
        &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
    )> for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            crate::__CcTemplateInstSt16initializer_listIDiE,
            &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        ),
    ) -> Self::CtorType {
        let (mut __il, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ESt16initializer_listIDiERKS4_(__crubit_dest as*mut::core::ffi::c_void,&mut __il,__a);
            })
        }
    }
}

impl ::ctor::PinnedDrop
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe {
            crate::detail::__rust_thunk__3aa0a429__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEED1Ev(self)
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __str: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEOS5_(self,__str);
        }
    }
}

impl ::ctor::Assign<crate::__CcTemplateInstSt16initializer_listIDiE>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    #[inline(always)]
    fn assign(
        self: ::core::pin::Pin<&mut Self>,
        mut __il: crate::__CcTemplateInstSt16initializer_listIDiE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__0e3b9316__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSESt16initializer_listIDiE(self,&mut __il);
        }
    }
}

impl ::ctor::Assign<*const u32>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __s: *const u32) {
        unsafe {
            crate::detail::__rust_thunk__4cd278b5__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEPKDi(self,__s);
        }
    }
}

impl ::operator::CcIndex<usize>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type Output<'ctnr> = &'ctnr u32;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __pos: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__1e6d7163__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm(self,__pos)
        }
    }
}
impl<'ctnr>::core::ops::Index<usize>for::core::pin::Pin<&'ctnr mut __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>{ type Output=u32;#[inline(always)]fn index(&self,index: usize)->&Self::Output{ ::operator::CcIndex::cc_index(self.as_ref().get_ref(),index) } }

impl ::operator::CcIndexMut<usize>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    type Output<'ctnr> = &'ctnr mut u32;
    #[inline(always)]
    fn cc_index_mut<'ctnr>(
        self: ::core::pin::Pin<&'ctnr mut Self>,
        __pos: usize,
    ) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__310e3d8e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm(self,__pos)
        }
    }
}
impl<'ctnr>::core::ops::IndexMut<usize>for::core::pin::Pin<&'ctnr mut __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>{ #[inline(always)]fn index_mut(&mut self,index: usize)->&mut Self::Output{ ::operator::CcIndexMut::cc_index_mut(self.as_mut(),index) } }

impl ::ctor::Assign<u32>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __c: u32) {
        unsafe {
            crate::detail::__rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEDi(self,__c);
        }
    }
}

impl ::ctor::Assign<&Self>
    for __CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __str: &Self) {
        unsafe {
            crate::detail::__rust_thunk__13806d1e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSERKS5_(self,__str);
        }
    }
}

pub mod cc_template_inst_n_st3_u12basic_string_i_di_ns_11char_traits_i_di_eens_9allocator_i_di_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6cbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4cendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn size(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__5348ae0a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn length(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__1005b8a3__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6lengthEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn max_size(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__3730555f__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8max_sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn capacity(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__ed5beee8__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8capacityEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__d528730e__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn substr(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        __pos: usize,
        __n: usize,
    ) -> ::ctor::Ctor![
        crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE
    ] {
        unsafe {
            ::ctor::FnCtor::new(move|__crubit_dest: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE|{ crate::detail::__rust_thunk__68107f44__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6substrEmm(__crubit_dest as*mut::core::ffi::c_void,__this,__pos,__n); })
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn c_str(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5c_strEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn get_allocator(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u9allocatorIDiEE {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u9allocatorIDiEE>::uninit();
            crate::detail::__rust_thunk__21b14257__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13get_allocatorEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn push_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        __c: u32,
    ) {
        unsafe {
            crate::detail::__rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE9push_backEDi(__this,__c)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn pop_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8pop_backEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn clear(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__0df906ad__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5clearEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn shrink_to_fit(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__ca1a6967__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13shrink_to_fitEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__s`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn copy(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        __s: *mut u32,
        __n: usize,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__ac16dbb5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4copyEPDimm(__this,__s,__n,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__str`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn swap(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__2f27b851__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4swapERS5_(__this,__str)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string < char16_t , std :: char_traits < char16_t >, std :: pmr :: polymorphic_allocator < char16_t >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE
{
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) __rep_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) __alloc_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl!Send for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{  }
impl!Sync for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{  }
impl __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ #[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn cbegin(__this: *const Self)->crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::cbegin(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn cend(__this: *const Self)->crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::cend(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn crbegin(__this: *const Self)->crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::crbegin(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn crend(__this: *const Self)->crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::crend(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn reserve(__this: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::reserve(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn empty(__this: *const Self)->bool{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::empty(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn c_str(__this: *const Self)->*const u16{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::c_str(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[must_use]#[inline(always)]pub unsafe fn get_allocator(__this: *const Self)->crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE{ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::get_allocator(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn push_back(__this: *mut Self,__c: u16){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::push_back(__this,__c) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn pop_back(__this: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::pop_back(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn clear(__this: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::clear(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer"]#[inline(always)]pub unsafe fn shrink_to_fit(__this: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::shrink_to_fit(__this) } }#[doc=" # Safety\n \n The caller must ensure that the following unsafe arguments are not misused by the function:\n * `__this`: raw pointer\n * `__str`: raw pointer"]#[inline(always)]pub unsafe fn swap(__this: *mut Self,__str: *mut Self){ unsafe{ self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee::swap(__this,__str) } } }

impl::ctor::CtorNew<()>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=::ctor::Ctor![Self];type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: ())->Self::CtorType{ let()=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__6e53fd45__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1Ev(__crubit_dest as*mut::core::ffi::c_void); }) } } }

impl<'__unelided>::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE)->Self::CtorType{ let mut __a=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__f065fef8__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS5_(__crubit_dest as*mut::core::ffi::c_void,__a); }) } } }
impl<'__unelided>::ctor::CtorNew<(&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE,)>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<&'__unelided Self>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &'__unelided Self)->Self::CtorType{ let mut __str=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_(__crubit_dest as*mut::core::ffi::c_void,__str); }) } } }
impl<'__unelided>::ctor::CtorNew<(&'__unelided Self,)>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided Self,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<(&'__unelided Self,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE)>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided Self,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE))->Self::CtorType{ let(mut __str,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_RKS5_(__crubit_dest as*mut::core::ffi::c_void,__str,__a); }) } } }

impl<'__unelided>::ctor::CtorNew<::ctor::RvalueReference<'__unelided,Self>>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: ::ctor::RvalueReference<'__unelided,Self>)->Self::CtorType{ let mut __str=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__8e81d883__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_(__crubit_dest as*mut::core::ffi::c_void,__str); }) } } }
impl<'__unelided>::ctor::CtorNew<(::ctor::RvalueReference<'__unelided,Self>,)>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (::ctor::RvalueReference<'__unelided,Self>,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<::ctor::RvalueReference<'__unelided,Self>>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<(::ctor::RvalueReference<'__unelided,Self>,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE)>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (::ctor::RvalueReference<'__unelided,Self>,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE))->Self::CtorType{ let(mut __str,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__91697c2d__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_RKS5_(__crubit_dest as*mut::core::ffi::c_void,__str,__a); }) } } }

impl<'__unelided>::ctor::CtorNew<(crate::__CcTemplateInstSt16initializer_listIDsE,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE)>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (crate::__CcTemplateInstSt16initializer_listIDsE,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE))->Self::CtorType{ let(mut __il,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ESt16initializer_listIDsERKS5_(__crubit_dest as*mut::core::ffi::c_void,&mut __il,__a); }) } } }

impl::ctor::PinnedDrop for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ #[inline(always)]unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>){ unsafe{ crate::detail::__rust_thunk__3aa0a429__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEED1Ev(self) } } }

impl::ctor::Assign<::ctor::RvalueReference<'_,Self>>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__str: ::ctor::RvalueReference<'_,Self>){ unsafe{ crate::detail::__rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEOS6_(self,__str); } } }

impl::ctor::Assign<crate::__CcTemplateInstSt16initializer_listIDsE>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,mut __il: crate::__CcTemplateInstSt16initializer_listIDsE){ unsafe{ crate::detail::__rust_thunk__0e3b9316__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSESt16initializer_listIDsE(self,&mut __il); } } }

impl::ctor::Assign<*const u16>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__s: *const u16){ unsafe{ crate::detail::__rust_thunk__4cd278b5__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEPKDs(self,__s); } } }

impl::ctor::Assign<u16>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__c: u16){ unsafe{ crate::detail::__rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEDs(self,__c); } } }

impl::ctor::Assign<&Self>for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__str: &Self){ unsafe{ crate::detail::__rust_thunk__13806d1e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSERKS6_(self,__str); } } }

pub mod cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_3pmr21polymorphic_allocator_i_ds_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE6cbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4cendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
            >::uninit();
            crate::detail::__rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
            >::uninit();
            crate::detail::__rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn reserve(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__ec767001__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7reserveEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__d528730e__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn c_str(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5c_strEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn get_allocator(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE,
            >::uninit();
            crate::detail::__rust_thunk__21b14257__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13get_allocatorEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn push_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        __c: u16,
    ) {
        unsafe {
            crate::detail::__rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE9push_backEDs(__this,__c)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn pop_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE8pop_backEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn clear(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__0df906ad__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5clearEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn shrink_to_fit(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__ca1a6967__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13shrink_to_fitEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__str`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn swap(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__2f27b851__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4swapERS6_(__this,__str)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string < char16_t , std :: char_traits < char16_t >, std :: allocator < char16_t >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) __rep_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
}
impl !Send for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE {}
impl !Sync for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE {}
impl __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::cbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cend(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::cend(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::crbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::crend(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::size(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn length(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::length(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn max_size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::max_size(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn capacity(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::capacity(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::empty(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn substr(
        __this: *const Self,
        __pos: usize,
        __n: usize,
    ) -> ::ctor::Ctor![
        crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
    ] {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::substr(__this,__pos,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn c_str(__this: *const Self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::c_str(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn get_allocator(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u9allocatorIDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::get_allocator(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn push_back(__this: *mut Self, __c: u16) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::push_back(__this,__c)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn pop_back(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::pop_back(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn clear(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::clear(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn shrink_to_fit(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::shrink_to_fit(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__s`: raw pointer
    #[inline(always)]
    pub unsafe fn copy(__this: *const Self, __s: *mut u16, __n: usize, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::copy(__this,__s,__n,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__str`: raw pointer
    #[inline(always)]
    pub unsafe fn swap(__this: *mut Self, __str: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee::swap(__this,__str)
        }
    }
}

impl ::ctor::CtorNew<()>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__6e53fd45__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1Ev(__crubit_dest as*mut::core::ffi::c_void);
            })
        }
    }
}

impl<'__unelided> ::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
    ) -> Self::CtorType {
        let mut __a = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__f065fef8__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS4_(__crubit_dest as*mut::core::ffi::c_void,__a);
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(&'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,)>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (&'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<&'__unelided Self>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
        let mut __str = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_(__crubit_dest as*mut::core::ffi::c_void,__str);
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(&'__unelided Self, &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE)>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (&'__unelided Self, &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE),
    ) -> Self::CtorType {
        let (mut __str, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_RKS4_(__crubit_dest as*mut::core::ffi::c_void,__str,__a);
            })
        }
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __str = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8e81d883__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_(__crubit_dest as*mut::core::ffi::c_void,__str);
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(
        ::ctor::RvalueReference<'__unelided, Self>,
        &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
    )> for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            ::ctor::RvalueReference<'__unelided, Self>,
            &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        ),
    ) -> Self::CtorType {
        let (mut __str, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__91697c2d__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_RKS4_(__crubit_dest as*mut::core::ffi::c_void,__str,__a);
            })
        }
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(
        &'__unelided Self,
        usize,
        usize,
        &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
    )> for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            &'__unelided Self,
            usize,
            usize,
            &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        ),
    ) -> Self::CtorType {
        let (mut __str, mut __pos, mut __n, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__ea3378b5__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mmRKS4_(__crubit_dest as*mut::core::ffi::c_void,__str,__pos,__n,__a);
            })
        }
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(
        &'__unelided Self,
        usize,
        &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
    )> for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            &'__unelided Self,
            usize,
            &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        ),
    ) -> Self::CtorType {
        let (mut __str, mut __pos, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__6c7db170__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mRKS4_(__crubit_dest as*mut::core::ffi::c_void,__str,__pos,__a);
            })
        }
    }
}

impl<'__unelided>
    ::ctor::CtorNew<(
        crate::__CcTemplateInstSt16initializer_listIDsE,
        &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
    )> for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: (
            crate::__CcTemplateInstSt16initializer_listIDsE,
            &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        ),
    ) -> Self::CtorType {
        let (mut __il, mut __a) = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ESt16initializer_listIDsERKS4_(__crubit_dest as*mut::core::ffi::c_void,&mut __il,__a);
            })
        }
    }
}

impl ::ctor::PinnedDrop
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe {
            crate::detail::__rust_thunk__3aa0a429__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEED1Ev(self)
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __str: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEOS5_(self,__str);
        }
    }
}

impl ::ctor::Assign<crate::__CcTemplateInstSt16initializer_listIDsE>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    #[inline(always)]
    fn assign(
        self: ::core::pin::Pin<&mut Self>,
        mut __il: crate::__CcTemplateInstSt16initializer_listIDsE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__0e3b9316__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSESt16initializer_listIDsE(self,&mut __il);
        }
    }
}

impl ::ctor::Assign<*const u16>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __s: *const u16) {
        unsafe {
            crate::detail::__rust_thunk__4cd278b5__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEPKDs(self,__s);
        }
    }
}

impl ::operator::CcIndex<usize>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type Output<'ctnr> = &'ctnr u16;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __pos: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__1e6d7163__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm(self,__pos)
        }
    }
}
impl<'ctnr>::core::ops::Index<usize>for::core::pin::Pin<&'ctnr mut __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>{ type Output=u16;#[inline(always)]fn index(&self,index: usize)->&Self::Output{ ::operator::CcIndex::cc_index(self.as_ref().get_ref(),index) } }

impl ::operator::CcIndexMut<usize>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    type Output<'ctnr> = &'ctnr mut u16;
    #[inline(always)]
    fn cc_index_mut<'ctnr>(
        self: ::core::pin::Pin<&'ctnr mut Self>,
        __pos: usize,
    ) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__310e3d8e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm(self,__pos)
        }
    }
}
impl<'ctnr>::core::ops::IndexMut<usize>for::core::pin::Pin<&'ctnr mut __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>{ #[inline(always)]fn index_mut(&mut self,index: usize)->&mut Self::Output{ ::operator::CcIndexMut::cc_index_mut(self.as_mut(),index) } }

impl ::ctor::Assign<u16>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __c: u16) {
        unsafe {
            crate::detail::__rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEDs(self,__c);
        }
    }
}

impl ::ctor::Assign<&Self>
    for __CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __str: &Self) {
        unsafe {
            crate::detail::__rust_thunk__13806d1e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSERKS5_(self,__str);
        }
    }
}

pub mod cc_template_inst_n_st3_u12basic_string_i_ds_ns_11char_traits_i_ds_eens_9allocator_i_ds_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6cbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4cendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
            >::uninit();
            crate::detail::__rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
            >::uninit();
            crate::detail::__rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn size(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__5348ae0a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn length(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__1005b8a3__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6lengthEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn max_size(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__3730555f__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8max_sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn capacity(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__ed5beee8__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8capacityEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__d528730e__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn substr(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        __pos: usize,
        __n: usize,
    ) -> ::ctor::Ctor![
        crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE
    ] {
        unsafe {
            ::ctor::FnCtor::new(move|__crubit_dest: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE|{ crate::detail::__rust_thunk__68107f44__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6substrEmm(__crubit_dest as*mut::core::ffi::c_void,__this,__pos,__n); })
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn c_str(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5c_strEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn get_allocator(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u9allocatorIDsEE {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u9allocatorIDsEE>::uninit();
            crate::detail::__rust_thunk__21b14257__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13get_allocatorEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn push_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        __c: u16,
    ) {
        unsafe {
            crate::detail::__rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE9push_backEDs(__this,__c)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn pop_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8pop_backEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn clear(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__0df906ad__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5clearEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn shrink_to_fit(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__ca1a6967__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13shrink_to_fitEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__s`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn copy(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        __s: *mut u16,
        __n: usize,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__ac16dbb5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4copyEPDsmm(__this,__s,__n,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__str`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn swap(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__2f27b851__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4swapERS5_(__this,__str)
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string < char , std :: char_traits < char >, std :: pmr :: polymorphic_allocator < char >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE
{
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) __rep_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 24],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) __alloc_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl!Send for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{  }
impl!Sync for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{  }
impl
    __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE
{
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cbegin(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::cbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cend(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::cend(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::crbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::crend(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn reserve(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::reserve(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::empty(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn c_str(__this: *const Self) -> *const ::ffi_11::c_char {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::c_str(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn get_allocator(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::get_allocator(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn push_back(__this: *mut Self, __c: ::ffi_11::c_char) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::push_back(__this,__c)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn pop_back(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::pop_back(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn clear(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::clear(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn shrink_to_fit(__this: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::shrink_to_fit(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__str`: raw pointer
    #[inline(always)]
    pub unsafe fn swap(__this: *mut Self, __str: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee::swap(__this,__str)
        }
    }
}

impl::ctor::CtorNew<()>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=::ctor::Ctor![Self];type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: ())->Self::CtorType{ let()=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__6e53fd45__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1Ev(__crubit_dest as*mut::core::ffi::c_void); }) } } }

impl<'__unelided>::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE)->Self::CtorType{ let mut __a=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__f065fef8__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS5_(__crubit_dest as*mut::core::ffi::c_void,__a); }) } } }
impl<'__unelided>::ctor::CtorNew<(&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE,)>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<&'__unelided Self>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &'__unelided Self)->Self::CtorType{ let mut __str=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_(__crubit_dest as*mut::core::ffi::c_void,__str); }) } } }
impl<'__unelided>::ctor::CtorNew<(&'__unelided Self,)>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided Self,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<(&'__unelided Self,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE)>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (&'__unelided Self,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE))->Self::CtorType{ let(mut __str,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_RKS5_(__crubit_dest as*mut::core::ffi::c_void,__str,__a); }) } } }

impl<'__unelided>::ctor::CtorNew<::ctor::RvalueReference<'__unelided,Self>>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: ::ctor::RvalueReference<'__unelided,Self>)->Self::CtorType{ let mut __str=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__8e81d883__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_(__crubit_dest as*mut::core::ffi::c_void,__str); }) } } }
impl<'__unelided>::ctor::CtorNew<(::ctor::RvalueReference<'__unelided,Self>,)>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (::ctor::RvalueReference<'__unelided,Self>,))->Self::CtorType{ let(arg,)=args;<Self as::ctor::CtorNew<::ctor::RvalueReference<'__unelided,Self>>>::ctor_new(arg) } }

impl<'__unelided>::ctor::CtorNew<(::ctor::RvalueReference<'__unelided,Self>,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE)>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (::ctor::RvalueReference<'__unelided,Self>,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE))->Self::CtorType{ let(mut __str,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__91697c2d__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_RKS5_(__crubit_dest as*mut::core::ffi::c_void,__str,__a); }) } } }

impl<'__unelided>::ctor::CtorNew<(crate::__CcTemplateInstSt16initializer_listIcE,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE)>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ type CtorType=impl::ctor::Ctor<Output=Self,Error=::ctor::Infallible>+use<'__unelided>;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: (crate::__CcTemplateInstSt16initializer_listIcE,&'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE))->Self::CtorType{ let(mut __il,mut __a)=args;unsafe{ ::ctor::FnCtor::new(move|__crubit_dest: *mut Self|{ crate::detail::__rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ESt16initializer_listIcERKS5_(__crubit_dest as*mut::core::ffi::c_void,&mut __il,__a); }) } } }

impl::ctor::PinnedDrop for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ #[inline(always)]unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>){ unsafe{ crate::detail::__rust_thunk__3aa0a429__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEED1Ev(self) } } }

impl::ctor::Assign<::ctor::RvalueReference<'_,Self>>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__str: ::ctor::RvalueReference<'_,Self>){ unsafe{ crate::detail::__rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEOS6_(self,__str); } } }

impl::ctor::Assign<crate::__CcTemplateInstSt16initializer_listIcE>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,mut __il: crate::__CcTemplateInstSt16initializer_listIcE){ unsafe{ crate::detail::__rust_thunk__0e3b9316__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSESt16initializer_listIcE(self,&mut __il); } } }

impl::ctor::Assign<*const::ffi_11::c_char>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__s: *const::ffi_11::c_char){ unsafe{ crate::detail::__rust_thunk__4cd278b5__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEPKc(self,__s); } } }

impl::ctor::Assign<::ffi_11::c_char>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__c: ::ffi_11::c_char){ unsafe{ crate::detail::__rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEc(self,__c); } } }

impl::ctor::Assign<&Self>for __CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE{ #[inline(always)]fn assign(self: ::core::pin::Pin<&mut Self>,__str: &Self){ unsafe{ crate::detail::__rust_thunk__13806d1e__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSERKS6_(self,__str); } } }

pub mod cc_template_inst_n_st3_u12basic_string_ic_ns_11char_traits_ic_eens_3pmr21polymorphic_allocator_ic_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            >::uninit();
            crate::detail::__rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE6cbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            >::uninit();
            crate::detail::__rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4cendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crbegin(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE,
            >::uninit();
            crate::detail::__rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crend(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE,
            >::uninit();
            crate::detail::__rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn reserve(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__ec767001__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7reserveEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__d528730e__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn c_str(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) -> *const ::ffi_11::c_char {
        unsafe {
            crate::detail::__rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5c_strEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn get_allocator(
        __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) -> crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE,
            >::uninit();
            crate::detail::__rust_thunk__21b14257__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13get_allocatorEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn push_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        __c: ::ffi_11::c_char,
    ) {
        unsafe {
            crate::detail::__rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE9push_backEc(__this,__c)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn pop_back(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE8pop_backEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn clear(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__0df906ad__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5clearEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn shrink_to_fit(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__ca1a6967__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13shrink_to_fitEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__str`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn swap(
        __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__2f27b851__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4swapERS6_(__this,__str)
        }
    }
}

// error: class `std::basic_ostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < std :: __wrap_iter < char32_t *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_di_eeee::base(
                __this,
            )
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1ES3_(&raw mut tmp as*mut _,&mut __x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_di_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE,
            >::uninit();
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < std :: __wrap_iter < char16_t *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_ds_eeee::base(
                __this,
            )
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1ES3_(&raw mut tmp as*mut _,&mut __x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_ds_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE,
            >::uninit();
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < std :: __wrap_iter < const char32_t *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipk_di_eeee::base(
                __this,
            )
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1ES4_(&raw mut tmp as*mut _,&mut __x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipk_di_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < std :: __wrap_iter < const char16_t *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipk_ds_eeee::base(
                __this,
            )
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1ES4_(&raw mut tmp as*mut _,&mut __x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipk_ds_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < std :: __wrap_iter < const char *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_kc_eeee::base(
                __this,
            )
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1ES4_(&raw mut tmp as*mut _,&mut __x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_kc_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            >::uninit();
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < std :: __wrap_iter < char *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_i_pc_eeee::base(
                __this,
            )
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1ES3_(&raw mut tmp as*mut _,&mut __x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_i_pc_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE,
            >::uninit();
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char32_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> *const u32 {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ipk_di_ee::base(__this) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKDiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const u32> for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    #[inline(always)]
    unsafe fn unsafe_from(args: *const u32) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorIPKDiEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const u32> for __CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const u32) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const u32>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ipk_di_ee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorIPKDiE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char16_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {}
impl __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> *const u16 {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ipk_ds_ee::base(__this) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKDsEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::ctor::UnsafeFrom<*const u16> for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    #[inline(always)]
    unsafe fn unsafe_from(args: *const u16) -> Self {
        let mut __x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorIPKDsEC1ES2_(
                &raw mut tmp as *mut _,
                __x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<*const u16> for __CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: *const u16) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<*const u16>>::unsafe_from(args) }
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ipk_ds_ee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorIPKDsE4baseEv(__this)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char8_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKDuEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
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
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn base(__this: *const Self) -> *const ::ffi_11::c_char {
        unsafe { self::cc_template_inst_n_st3_u16reverse_iterator_ip_kc_ee::base(__this) }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
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
            crate::detail::__rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
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
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn base(
        __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
    ) -> *const ::ffi_11::c_char {
        unsafe {
            crate::detail::__rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(__this)
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
            crate::detail::__rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstSt16initializer_listIDiE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: initializer_list < char32_t >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstSt16initializer_listIDiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __begin_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstSt16initializer_listIDiE {}
impl !Sync for __CcTemplateInstSt16initializer_listIDiE {}
impl __CcTemplateInstSt16initializer_listIDiE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn data(__this: *const Self) -> *const u32 {
        unsafe { self::cc_template_inst_st16initializer_list_i_di_e::data(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn size(__this: *const Self) -> usize {
        unsafe { self::cc_template_inst_st16initializer_list_i_di_e::size(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_st16initializer_list_i_di_e::empty(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn begin(__this: *const Self) -> *const u32 {
        unsafe { self::cc_template_inst_st16initializer_list_i_di_e::begin(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn end(__this: *const Self) -> *const u32 {
        unsafe { self::cc_template_inst_st16initializer_list_i_di_e::end(__this) }
    }
}

impl Default for __CcTemplateInstSt16initializer_listIDiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__5d27f53c__ZNSt16initializer_listIDiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_st16initializer_list_i_di_e {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn data(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
    ) -> *const u32 {
        unsafe { crate::detail::__rust_thunk__9fe1b834__ZNKSt16initializer_listIDiE4dataEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn size(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
    ) -> usize {
        unsafe { crate::detail::__rust_thunk__45dfec80__ZNKSt16initializer_listIDiE4sizeEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__dcdbfbf1__ZNKSt16initializer_listIDiE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn begin(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__17e6530b__ZNKSt16initializer_listIDiE5beginEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn end(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
    ) -> *const u32 {
        unsafe { crate::detail::__rust_thunk__f0cc69a4__ZNKSt16initializer_listIDiE3endEv(__this) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstSt16initializer_listIDsE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: initializer_list < char16_t >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstSt16initializer_listIDsE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __begin_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstSt16initializer_listIDsE {}
impl !Sync for __CcTemplateInstSt16initializer_listIDsE {}
impl __CcTemplateInstSt16initializer_listIDsE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn data(__this: *const Self) -> *const u16 {
        unsafe { self::cc_template_inst_st16initializer_list_i_ds_e::data(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn size(__this: *const Self) -> usize {
        unsafe { self::cc_template_inst_st16initializer_list_i_ds_e::size(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_st16initializer_list_i_ds_e::empty(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn begin(__this: *const Self) -> *const u16 {
        unsafe { self::cc_template_inst_st16initializer_list_i_ds_e::begin(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn end(__this: *const Self) -> *const u16 {
        unsafe { self::cc_template_inst_st16initializer_list_i_ds_e::end(__this) }
    }
}

impl Default for __CcTemplateInstSt16initializer_listIDsE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__5d27f53c__ZNSt16initializer_listIDsEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_st16initializer_list_i_ds_e {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn data(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
    ) -> *const u16 {
        unsafe { crate::detail::__rust_thunk__9fe1b834__ZNKSt16initializer_listIDsE4dataEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn size(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
    ) -> usize {
        unsafe { crate::detail::__rust_thunk__45dfec80__ZNKSt16initializer_listIDsE4sizeEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__dcdbfbf1__ZNKSt16initializer_listIDsE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn begin(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__17e6530b__ZNKSt16initializer_listIDsE5beginEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn end(
        __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
    ) -> *const u16 {
        unsafe { crate::detail::__rust_thunk__f0cc69a4__ZNKSt16initializer_listIDsE3endEv(__this) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstSt16initializer_listIcE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: initializer_list < char >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstSt16initializer_listIcE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __begin_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstSt16initializer_listIcE {}
impl !Sync for __CcTemplateInstSt16initializer_listIcE {}
impl __CcTemplateInstSt16initializer_listIcE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn data(__this: *const Self) -> *const ::ffi_11::c_char {
        unsafe { self::cc_template_inst_st16initializer_list_ic_e::data(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn size(__this: *const Self) -> usize {
        unsafe { self::cc_template_inst_st16initializer_list_ic_e::size(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_st16initializer_list_ic_e::empty(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn begin(__this: *const Self) -> *const ::ffi_11::c_char {
        unsafe { self::cc_template_inst_st16initializer_list_ic_e::begin(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn end(__this: *const Self) -> *const ::ffi_11::c_char {
        unsafe { self::cc_template_inst_st16initializer_list_ic_e::end(__this) }
    }
}

impl Default for __CcTemplateInstSt16initializer_listIcE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__5d27f53c__ZNSt16initializer_listIcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_st16initializer_list_ic_e {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn data(
        __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
    ) -> *const ::ffi_11::c_char {
        unsafe { crate::detail::__rust_thunk__9fe1b834__ZNKSt16initializer_listIcE4dataEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn size(
        __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
    ) -> usize {
        unsafe { crate::detail::__rust_thunk__45dfec80__ZNKSt16initializer_listIcE4sizeEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
    ) -> bool {
        unsafe { crate::detail::__rust_thunk__dcdbfbf1__ZNKSt16initializer_listIcE5emptyEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn begin(
        __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
    ) -> *const ::ffi_11::c_char {
        unsafe { crate::detail::__rust_thunk__17e6530b__ZNKSt16initializer_listIcE5beginEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn end(
        __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
    ) -> *const ::ffi_11::c_char {
        unsafe { crate::detail::__rust_thunk__f0cc69a4__ZNKSt16initializer_listIcE3endEv(__this) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPDiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < char32_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPDiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPDiEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPDiEE {}

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPDiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPDiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::core::ops::Add<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE,
            >::uninit();
            crate::detail::__rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPDiEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPDiEE {
    #[inline(always)]
    fn add_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPDiEpLEl(self, __n);
        }
    }
}

impl ::core::ops::Sub<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE,
            >::uninit();
            crate::detail::__rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPDiEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPDiEE {
    #[inline(always)]
    fn sub_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPDiEmIEl(self, __n);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPDsEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < char16_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPDsEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPDsEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPDsEE {}

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPDsEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPDsEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::core::ops::Add<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE,
            >::uninit();
            crate::detail::__rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPDsEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPDsEE {
    #[inline(always)]
    fn add_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPDsEpLEl(self, __n);
        }
    }
}

impl ::core::ops::Sub<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE,
            >::uninit();
            crate::detail::__rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPDsEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPDsEE {
    #[inline(always)]
    fn sub_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPDsEmIEl(self, __n);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < const char32_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {}

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPKDiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::core::ops::Add<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPKDiEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
    #[inline(always)]
    fn add_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPKDiEpLEl(self, __n);
        }
    }
}

impl ::core::ops::Sub<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPKDiEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
    #[inline(always)]
    fn sub_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPKDiEmIEl(self, __n);
        }
    }
}

impl ::operator::CcIndex<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
    type Output<'ctnr> = &'ctnr u32;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __n: isize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__2bf1b1cb__ZNKSt3__u11__wrap_iterIPKDiEixEl(self, __n)
        }
    }
}
impl ::core::ops::Index<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKDiEE {
    type Output = u32;
    #[inline(always)]
    fn index(&self, index: isize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < const char16_t *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {}

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPKDsEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::core::ops::Add<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPKDsEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
    #[inline(always)]
    fn add_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPKDsEpLEl(self, __n);
        }
    }
}

impl ::core::ops::Sub<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPKDsEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
    #[inline(always)]
    fn sub_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPKDsEmIEl(self, __n);
        }
    }
}

impl ::operator::CcIndex<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
    type Output<'ctnr> = &'ctnr u16;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __n: isize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__2bf1b1cb__ZNKSt3__u11__wrap_iterIPKDsEixEl(self, __n)
        }
    }
}
impl ::core::ops::Index<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKDsEE {
    type Output = u16;
    #[inline(always)]
    fn index(&self, index: isize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPKcEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < const char *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPKcEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPKcEE {}

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPKcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::core::ops::Add<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            >::uninit();
            crate::detail::__rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPKcEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
    #[inline(always)]
    fn add_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPKcEpLEl(self, __n);
        }
    }
}

impl ::core::ops::Sub<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            >::uninit();
            crate::detail::__rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPKcEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
    #[inline(always)]
    fn sub_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPKcEmIEl(self, __n);
        }
    }
}

impl ::operator::CcIndex<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
    type Output<'ctnr> = &'ctnr ::ffi_11::c_char;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __n: isize) -> Self::Output<'ctnr> {
        unsafe { crate::detail::__rust_thunk__2bf1b1cb__ZNKSt3__u11__wrap_iterIPKcEixEl(self, __n) }
    }
}
impl ::core::ops::Index<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKcEE {
    type Output = ::ffi_11::c_char;
    #[inline(always)]
    fn index(&self, index: isize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPcEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < char *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPcEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPcEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPcEE {}

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPcEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPcEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::core::ops::Add<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE,
            >::uninit();
            crate::detail::__rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPcEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPcEE {
    #[inline(always)]
    fn add_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPcEpLEl(self, __n);
        }
    }
}

impl ::core::ops::Sub<isize> for &crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE {
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE,
            >::uninit();
            crate::detail::__rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPcEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPcEE {
    #[inline(always)]
    fn sub_assign(&mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPcEmIEl(self, __n);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1000000000000000000L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1000000000000000000ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1000000000000000L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1000000000000000ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1000000000000L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1000000000000ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1000000000L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1000000000ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1000000L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1000000ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1000ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1000L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1000ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1000ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1000ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1000ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1000ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl100ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 100L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl100ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl100ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl100ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl100ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl100ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl10ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 10L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl10ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl10ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl10ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl10ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl10ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 1000000000000000000L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000000000000000EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 1000000000000000L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000000000000EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 1000000000000L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000000000EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 1000000000L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000000EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 1000000L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl1000EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 1000L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl1000EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl1000EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl1000EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl1000EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl100EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 100L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl100EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl100EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl100EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl100EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl100EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl10EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 10L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl10EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl10EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl10EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl10EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl10EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl1ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 1L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl1ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl1ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl1ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl1ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 2629746L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl2629746ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 31556952L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl31556952ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl3600ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 3600L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl3600ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl3600ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl3600ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl3600ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl3600ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl604800ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 604800L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl604800ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl604800ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl604800ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl604800ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl604800ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl60ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 60L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl60ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl60ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl60ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl60ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl60ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u5ratioILl86400ELl1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: ratio < 86400L , 1L >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u5ratioILl86400ELl1EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstNSt3__u5ratioILl86400ELl1EEE {}
impl !Sync for __CcTemplateInstNSt3__u5ratioILl86400ELl1EEE {}

impl Default for __CcTemplateInstNSt3__u5ratioILl86400ELl1EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__806f2567__ZNSt3__u5ratioILl86400ELl1EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < int , std :: ratio < 2629746L , 1L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < int , std :: ratio < 31556952L , 1L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < int , std :: ratio < 604800L , 1L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < int , std :: ratio < 86400L , 1L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < long , std :: ratio < 3600L , 1L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < long , std :: ratio < 60L , 1L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < long long , std :: ratio < 1L , 1000000000L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < long long , std :: ratio < 1L , 1000000L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < long long , std :: ratio < 1L , 1000L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: duration < long long , std :: ratio < 1L , 1L >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __rep_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE {}
impl !Sync for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE {}

impl Default for __CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__36eb0319__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: time_point < std :: chrono :: steady_clock , std :: chrono :: duration < long long , std :: ratio < 1L , 1000000000L >>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE
{
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __d_: [::core::mem::MaybeUninit<u8>; 8],
}
impl!Send for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE{  }
impl!Sync for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE{  }

impl Default for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE{ #[inline(always)]fn default()->Self{ let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__78ef8f7e__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1Ev(&raw mut tmp as*mut _);tmp.assume_init() } } }

impl From<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE>for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE{ #[inline(always)]fn from(args: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE)->Self{ let mut __d=args;let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__bb707a65__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1ERKS6_(&raw mut tmp as*mut _,__d);tmp.assume_init() } } }
impl::ctor::CtorNew<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE>for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE{ type CtorType=Self;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE)->Self::CtorType{ <Self as From<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE>>::from(args) } }

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: time_point < std :: chrono :: system_clock , std :: chrono :: duration < int , std :: ratio < 86400L , 1L >>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE
{
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __d_: [::core::mem::MaybeUninit<u8>; 4],
}
impl!Send for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE{  }
impl!Sync for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE{  }

impl Default for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE{ #[inline(always)]fn default()->Self{ let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__78ef8f7e__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1Ev(&raw mut tmp as*mut _);tmp.assume_init() } } }

impl From<&crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE>for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE{ #[inline(always)]fn from(args: &crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE)->Self{ let mut __d=args;let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__bb707a65__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1ERKS6_(&raw mut tmp as*mut _,__d);tmp.assume_init() } } }
impl::ctor::CtorNew<&crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE>for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE{ type CtorType=Self;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE)->Self::CtorType{ <Self as From<&crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE>>::from(args) } }

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: time_point < std :: chrono :: system_clock , std :: chrono :: duration < long long , std :: ratio < 1L , 1000000L >>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE
{
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __d_: [::core::mem::MaybeUninit<u8>; 8],
}
impl!Send for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE{  }
impl!Sync for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE{  }

impl Default for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE{ #[inline(always)]fn default()->Self{ let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__78ef8f7e__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1Ev(&raw mut tmp as*mut _);tmp.assume_init() } } }

impl From<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE>for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE{ #[inline(always)]fn from(args: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE)->Self{ let mut __d=args;let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__bb707a65__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1ERKS6_(&raw mut tmp as*mut _,__d);tmp.assume_init() } } }
impl::ctor::CtorNew<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE>for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE{ type CtorType=Self;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE)->Self::CtorType{ <Self as From<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE>>::from(args) } }

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: chrono :: time_point < std :: chrono :: system_clock , std :: chrono :: duration < long long , std :: ratio < 1L , 1L >>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE
{
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __d_: [::core::mem::MaybeUninit<u8>; 8],
}
impl!Send for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE{  }
impl!Sync for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE{  }

impl Default for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE{ #[inline(always)]fn default()->Self{ let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__78ef8f7e__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1Ev(&raw mut tmp as*mut _);tmp.assume_init() } } }

impl From<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE>for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE{ #[inline(always)]fn from(args: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE)->Self{ let mut __d=args;let mut tmp=::core::mem::MaybeUninit::<Self>::zeroed();unsafe{ crate::detail::__rust_thunk__bb707a65__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1ERKS6_(&raw mut tmp as*mut _,__d);tmp.assume_init() } } }
impl::ctor::CtorNew<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE>for __CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE{ type CtorType=Self;type Error=::ctor::Infallible;#[inline(always)]fn ctor_new(args: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE)->Self::CtorType{ <Self as From<&crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE>>::from(args) } }

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<char32_t>': Failed to create bindings for template specialization type __cxx_atomic_impl<char32_t>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < char32_t , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<char32_t>': Failed to create bindings for template specialization type __cxx_atomic_impl<char32_t>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_di_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_di_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_di_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDiLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIDiLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<u32> for __CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: u32) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIDiLb0EEC1EDi(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(u32,)> for __CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (u32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<u32>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_i_di_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIDiLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIDiLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIDiLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<char16_t>': Failed to create bindings for template specialization type __cxx_atomic_impl<char16_t>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE"]
#[repr(C, align(2))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < char16_t , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<char16_t>': Failed to create bindings for template specialization type __cxx_atomic_impl<char16_t>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 2],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_ds_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_ds_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_ds_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDsLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIDsLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<u16> for __CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: u16) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIDsLb0EEC1EDs(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(u16,)> for __CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (u16,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<u16>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_i_ds_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIDsLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIDsLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIDsLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<char8_t>': Failed to create bindings for template specialization type __cxx_atomic_impl<char8_t>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < char8_t , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<char8_t>': Failed to create bindings for template specialization type __cxx_atomic_impl<char8_t>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_du_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_du_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_i_du_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDuLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIDuLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_i_du_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIDuLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIDuLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIDuLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<signed char>': Failed to create bindings for template specialization type __cxx_atomic_impl<signed char>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < signed char , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<signed char>': Failed to create bindings for template specialization type __cxx_atomic_impl<signed char>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ia_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ia_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ia_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIaLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIaLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_schar> for __CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_schar) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIaLb0EEC1Ea(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_schar,)> for __CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_schar,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_schar>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_ia_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIaLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIaLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIaLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<bool>': Failed to create bindings for template specialization type __cxx_atomic_impl<_Bool>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < bool , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<bool>': Failed to create bindings for template specialization type __cxx_atomic_impl<_Bool>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ib_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ib_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ib_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIbLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIbLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<bool> for __CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: bool) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIbLb0EEC1Eb(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(bool,)> for __CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (bool,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<bool>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_ib_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIbLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIbLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIbLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<char>': Failed to create bindings for template specialization type __cxx_atomic_impl<char>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < char , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<char>': Failed to create bindings for template specialization type __cxx_atomic_impl<char>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ic_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ic_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ic_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIcLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIcLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_char> for __CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_char) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIcLb0EEC1Ec(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_char,)> for __CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_char,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_char>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_ic_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIcLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIcLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIcLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<unsigned char>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned char>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned char , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<unsigned char>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned char>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ih_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ih_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ih_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIhLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIhLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_uchar> for __CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_uchar) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIhLb0EEC1Eh(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_uchar,)> for __CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_uchar,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_uchar>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_ih_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIhLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIhLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIhLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<int>': Failed to create bindings for template specialization type __cxx_atomic_impl<int>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < int , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<int>': Failed to create bindings for template specialization type __cxx_atomic_impl<int>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ii_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ii_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ii_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIiLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIiLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_int> for __CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIiLb0EEC1Ei(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for __CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_ii_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIiLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIiLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIiLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<unsigned int>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned int>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned int , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<unsigned int>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned int>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ij_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ij_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ij_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIjLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIjLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_uint> for __CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_uint) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIjLb0EEC1Ej(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_uint,)> for __CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_uint,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_uint>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_ij_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIjLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIjLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIjLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<long>': Failed to create bindings for template specialization type __cxx_atomic_impl<long>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < long , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<long>': Failed to create bindings for template specialization type __cxx_atomic_impl<long>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_il_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_il_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_il_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIlLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIlLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_long> for __CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_long) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIlLb0EEC1El(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_long,)> for __CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_long,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_long>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_il_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIlLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIlLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIlLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<unsigned long>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned long>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned long , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseImLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<unsigned long>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned long>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseImLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseImLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseImLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_im_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_im_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_im_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseImLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseImLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseImLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseImLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_ulong> for __CcTemplateInstNSt3__u13__atomic_baseImLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_ulong) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseImLb0EEC1Em(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_ulong,)> for __CcTemplateInstNSt3__u13__atomic_baseImLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_ulong,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_ulong>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_im_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseImLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseImLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseImLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<short>': Failed to create bindings for template specialization type __cxx_atomic_impl<short>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE"]
#[repr(C, align(2))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < short , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<short>': Failed to create bindings for template specialization type __cxx_atomic_impl<short>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 2],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_is_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_is_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_is_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIsLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIsLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_short> for __CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_short) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIsLb0EEC1Es(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_short,)> for __CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_short,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_short>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_is_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIsLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIsLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIsLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<unsigned short>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned short>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE"]
#[repr(C, align(2))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned short , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseItLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<unsigned short>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned short>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 2],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseItLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseItLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseItLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_it_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_it_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_it_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseItLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseItLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseItLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseItLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_ushort> for __CcTemplateInstNSt3__u13__atomic_baseItLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_ushort) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseItLb0EEC1Et(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_ushort,)> for __CcTemplateInstNSt3__u13__atomic_baseItLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_ushort,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_ushort>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_it_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseItLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseItLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseItLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<wchar_t>': Failed to create bindings for template specialization type __cxx_atomic_impl<wchar_t>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < wchar_t , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<wchar_t>': Failed to create bindings for template specialization type __cxx_atomic_impl<wchar_t>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_iw_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_iw_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_iw_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIwLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIwLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_iw_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIwLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIwLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIwLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<long long>': Failed to create bindings for template specialization type __cxx_atomic_impl<long long>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < long long , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<long long>': Failed to create bindings for template specialization type __cxx_atomic_impl<long long>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ix_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ix_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_ix_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIxLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIxLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_longlong> for __CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_longlong) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIxLb0EEC1Ex(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_longlong,)> for __CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_longlong,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_longlong>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_ix_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIxLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIxLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIxLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `__a_`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'std::__cxx_atomic_impl<unsigned long long>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned long long>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned long long , false >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'std::__cxx_atomic_impl<unsigned long long>': Failed to create bindings for template specialization type __cxx_atomic_impl<unsigned long long>: Class template instantiation forbidden by blocklist: std::__cxx_atomic_impl
    pub(crate) __a_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE {}
impl __CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn is_lock_free(__this: *const Self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_iy_lb0_eee::is_lock_free(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_one(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_iy_lb0_eee::notify_one(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn notify_all(__this: *mut Self) {
        unsafe { self::cc_template_inst_n_st3_u13_atomic_base_iy_lb0_eee::notify_all(__this) }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIyLb0EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIyLb0EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_ulonglong> for __CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_ulonglong) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIyLb0EEC1Ey(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_ulonglong,)> for __CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_ulonglong,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_ulonglong>>::ctor_new(arg)
    }
}

pub mod cc_template_inst_n_st3_u13_atomic_base_iy_lb0_eee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn is_lock_free(
        __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIyLb0EE12is_lock_freeEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_one(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIyLb0EE10notify_oneEv(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn notify_all(
        __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__699572a8__ZNSt3__u13__atomic_baseIyLb0EE10notify_allEv(
                __this,
            )
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < char32_t , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDiLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIDiLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIDiLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<u32> for __CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: u32) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIDiLb1EEC1EDi(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(u32,)> for __CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (u32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<u32>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE"]
#[repr(C, align(2))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < char16_t , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 2],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDsLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIDsLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIDsLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<u16> for __CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: u16) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIDsLb1EEC1EDs(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(u16,)> for __CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (u16,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<u16>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < char8_t , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDuLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIDuLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIDuLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < signed char , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIaLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIaLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIaLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_schar> for __CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_schar) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIaLb1EEC1Ea(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_schar,)> for __CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_schar,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_schar>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < char , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIcLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIcLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIcLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_char> for __CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_char) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIcLb1EEC1Ec(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_char,)> for __CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_char,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_char>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned char , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIhLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIhLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIhLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_uchar> for __CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_uchar) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIhLb1EEC1Eh(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_uchar,)> for __CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_uchar,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_uchar>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < int , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIiLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIiLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIiLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_int> for __CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIiLb1EEC1Ei(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for __CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned int , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIjLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIjLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIjLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_uint> for __CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_uint) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIjLb1EEC1Ej(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_uint,)> for __CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_uint,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_uint>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < long , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIlLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIlLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIlLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_long> for __CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_long) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIlLb1EEC1El(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_long,)> for __CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_long,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_long>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned long , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseImLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseImLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseImLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseImLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseImLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseImLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseImLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseImLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseImLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_ulong> for __CcTemplateInstNSt3__u13__atomic_baseImLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_ulong) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseImLb1EEC1Em(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_ulong,)> for __CcTemplateInstNSt3__u13__atomic_baseImLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_ulong,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_ulong>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE"]
#[repr(C, align(2))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < short , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 2],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIsLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIsLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIsLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_short> for __CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_short) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIsLb1EEC1Es(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_short,)> for __CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_short,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_short>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE"]
#[repr(C, align(2))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned short , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseItLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 2],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseItLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseItLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseItLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseItLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseItLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseItLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseItLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseItLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_ushort> for __CcTemplateInstNSt3__u13__atomic_baseItLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_ushort) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseItLb1EEC1Et(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_ushort,)> for __CcTemplateInstNSt3__u13__atomic_baseItLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_ushort,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_ushort>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < wchar_t , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIwLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIwLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIwLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < long long , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIxLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIxLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIxLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_longlong> for __CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_longlong) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIxLb1EEC1Ex(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_longlong,)> for __CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_longlong,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_longlong>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __atomic_base < unsigned long long , true >
pub struct __CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE {}
impl !Sync for __CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE {}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE
{
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIyLb1EEaSEOS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::Assign<&Self> for __CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIyLb1EEaSERKS1_(
                self, __param_0,
            );
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIyLb1EEC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_ulonglong> for __CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_ulonglong) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIyLb1EEC1Ey(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_ulonglong,)> for __CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_ulonglong,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_ulonglong>>::ctor_new(arg)
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u6atomicIDuEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: atomic < char8_t >
pub struct __CcTemplateInstNSt3__u6atomicIDuEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
}
impl !Send for __CcTemplateInstNSt3__u6atomicIDuEE {}
impl !Sync for __CcTemplateInstNSt3__u6atomicIDuEE {}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u6atomicIDuEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__112a35fb__ZNSt3__u6atomicIDuEC1EvQ26is_default_constructible_vIT_E(__crubit_dest as*mut::core::ffi::c_void);
            })
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u6atomicIlEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: atomic < long >
pub struct __CcTemplateInstNSt3__u6atomicIlEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u6atomicIlEE {}
impl !Sync for __CcTemplateInstNSt3__u6atomicIlEE {}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u6atomicIlEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__112a35fb__ZNSt3__u6atomicIlEC1EvQ26is_default_constructible_vIT_E(__crubit_dest as*mut::core::ffi::c_void);
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_long> for __CcTemplateInstNSt3__u6atomicIlEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_long) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__5aa22ed4__ZNSt3__u6atomicIlEC1El(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_long,)> for __CcTemplateInstNSt3__u6atomicIlEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_long,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_long>>::ctor_new(arg)
    }
}

impl ::ctor::Assign<::ffi_11::c_long> for __CcTemplateInstNSt3__u6atomicIlEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __d: ::ffi_11::c_long) {
        unsafe {
            crate::detail::__rust_thunk__51bb7eeb__ZNSt3__u6atomicIlEaSEl(self, __d);
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u6atomicImEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: atomic < unsigned long >
pub struct __CcTemplateInstNSt3__u6atomicImEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for __CcTemplateInstNSt3__u6atomicImEE {}
impl !Sync for __CcTemplateInstNSt3__u6atomicImEE {}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u6atomicImEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__112a35fb__ZNSt3__u6atomicImEC1EvQ26is_default_constructible_vIT_E(__crubit_dest as*mut::core::ffi::c_void);
            })
        }
    }
}

impl ::ctor::CtorNew<::ffi_11::c_ulong> for __CcTemplateInstNSt3__u6atomicImEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_ulong) -> Self::CtorType {
        let mut __d = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__5aa22ed4__ZNSt3__u6atomicImEC1Em(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __d,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_ulong,)> for __CcTemplateInstNSt3__u6atomicImEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_ulong,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_ulong>>::ctor_new(arg)
    }
}

impl ::ctor::Assign<::ffi_11::c_ulong> for __CcTemplateInstNSt3__u6atomicImEE {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __d: ::ffi_11::c_ulong) {
        unsafe {
            crate::detail::__rust_thunk__51bb7eeb__ZNSt3__u6atomicImEaSEm(self, __d);
        }
    }
}

#[::ctor::recursively_pinned]
#[cfi_encoding = "__CcTemplateInstNSt3__u6atomicIwEE"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=std :: atomic < wchar_t >
pub struct __CcTemplateInstNSt3__u6atomicIwEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 4],
}
impl !Send for __CcTemplateInstNSt3__u6atomicIwEE {}
impl !Sync for __CcTemplateInstNSt3__u6atomicIwEE {}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u6atomicIwEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk__112a35fb__ZNSt3__u6atomicIwEC1EvQ26is_default_constructible_vIT_E(__crubit_dest as*mut::core::ffi::c_void);
            })
        }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `ptr`: raw pointer
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u19__allocation_resultIPDimEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __allocation_result < char32_t *, unsigned long >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u19__allocation_resultIPDimEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub ptr: *mut u32,
    pub count: ::ffi_11::c_ulong,
}
impl !Send for __CcTemplateInstNSt3__u19__allocation_resultIPDimEE {}
impl !Sync for __CcTemplateInstNSt3__u19__allocation_resultIPDimEE {}

impl ::ctor::UnsafeFrom<(*mut u32, ::ffi_11::c_ulong)>
    for __CcTemplateInstNSt3__u19__allocation_resultIPDimEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: (*mut u32, ::ffi_11::c_ulong)) -> Self {
        let (mut __ptr, mut __count) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__ce8a09ba__ZNSt3__u19__allocation_resultIPDimEC1ES1_m(
                &raw mut tmp as *mut _,
                __ptr,
                __count,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<(*mut u32, ::ffi_11::c_ulong)>
    for __CcTemplateInstNSt3__u19__allocation_resultIPDimEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: (*mut u32, ::ffi_11::c_ulong)) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<(*mut u32, ::ffi_11::c_ulong)>>::unsafe_from(args) }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `ptr`: raw pointer
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __allocation_result < char16_t *, unsigned long >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u19__allocation_resultIPDsmEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub ptr: *mut u16,
    pub count: ::ffi_11::c_ulong,
}
impl !Send for __CcTemplateInstNSt3__u19__allocation_resultIPDsmEE {}
impl !Sync for __CcTemplateInstNSt3__u19__allocation_resultIPDsmEE {}

impl ::ctor::UnsafeFrom<(*mut u16, ::ffi_11::c_ulong)>
    for __CcTemplateInstNSt3__u19__allocation_resultIPDsmEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: (*mut u16, ::ffi_11::c_ulong)) -> Self {
        let (mut __ptr, mut __count) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__ce8a09ba__ZNSt3__u19__allocation_resultIPDsmEC1ES1_m(
                &raw mut tmp as *mut _,
                __ptr,
                __count,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<(*mut u16, ::ffi_11::c_ulong)>
    for __CcTemplateInstNSt3__u19__allocation_resultIPDsmEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: (*mut u16, ::ffi_11::c_ulong)) -> Self::CtorType {
        unsafe { <Self as ::ctor::UnsafeFrom<(*mut u16, ::ffi_11::c_ulong)>>::unsafe_from(args) }
    }
}

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `ptr`: raw pointer
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u19__allocation_resultIPcmEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=std :: __allocation_result < char *, unsigned long >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u19__allocation_resultIPcmEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub ptr: *mut ::ffi_11::c_char,
    pub count: ::ffi_11::c_ulong,
}
impl !Send for __CcTemplateInstNSt3__u19__allocation_resultIPcmEE {}
impl !Sync for __CcTemplateInstNSt3__u19__allocation_resultIPcmEE {}

impl ::ctor::UnsafeFrom<(*mut ::ffi_11::c_char, ::ffi_11::c_ulong)>
    for __CcTemplateInstNSt3__u19__allocation_resultIPcmEE
{
    #[inline(always)]
    unsafe fn unsafe_from(args: (*mut ::ffi_11::c_char, ::ffi_11::c_ulong)) -> Self {
        let (mut __ptr, mut __count) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__ce8a09ba__ZNSt3__u19__allocation_resultIPcmEC1ES1_m(
                &raw mut tmp as *mut _,
                __ptr,
                __count,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::UnsafeCtorNew<(*mut ::ffi_11::c_char, ::ffi_11::c_ulong)>
    for __CcTemplateInstNSt3__u19__allocation_resultIPcmEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    unsafe fn ctor_new(args: (*mut ::ffi_11::c_char, ::ffi_11::c_ulong)) -> Self::CtorType {
        unsafe {
            <Self as ::ctor::UnsafeFrom<(*mut ::ffi_11::c_char, ::ffi_11::c_ulong)>>::unsafe_from(
                args,
            )
        }
    }
}

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

// error: class `std::basic_ios<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_ios<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_istream<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_iostream<wchar_t, std::char_traits<wchar_t>>` could not be bound
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

// error: class `std::basic_streambuf<char, std::char_traits<char>>` could not be bound
//   incomplete type

// error: class `std::basic_streambuf<wchar_t, std::char_traits<wchar_t>>` could not be bound
//   incomplete type

// error: class `std::fpos<__mbstate_t>` could not be bound
//   incomplete type

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < char32_t , std :: char_traits < char32_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {}
impl __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn begin(__this: *const Self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::begin(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn end(__this: *const Self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::end(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cbegin(__this: *const Self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::cbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cend(__this: *const Self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::cend(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn rbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::rbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn rend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::rend(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::crbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::crend(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::size(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn length(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::length(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn max_size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::max_size(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::empty(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn at(__this: *const Self, __pos: usize) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::at(
                __this, __pos,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn front(__this: *const Self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::front(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn back(__this: *const Self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::back(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn data(__this: *const Self) -> *const u32 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::data(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn remove_prefix(__this: *mut Self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::remove_prefix(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn remove_suffix(__this: *mut Self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::remove_suffix(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__other`: raw pointer
    #[inline(always)]
    pub unsafe fn swap(__this: *mut Self, __other: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::swap(
                __this, __other,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__s`: raw pointer
    #[inline(always)]
    pub unsafe fn copy(__this: *const Self, __s: *mut u32, __n: usize, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::copy(
                __this, __s, __n, __pos,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn substr(
        __this: *const Self,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee::substr(__this,__pos,__n)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__0155bed9__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl ::operator::CcIndex<usize>
    for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE
{
    type Output<'ctnr> = &'ctnr u32;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __pos: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__c1a8ac1a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEixEm(self,__pos)
        }
    }
}
impl ::core::ops::Index<usize>
    for __CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE
{
    type Output = u32;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

pub mod cc_template_inst_n_st3_u17basic_string_view_i_di_ns_11char_traits_i_di_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn begin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__c581f3e4__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5beginEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn end(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__fc92cc82__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE3endEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cbegin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__e9654ad0__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6cbeginEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cend(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__54f567ee__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4cendEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn rbegin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__84266528__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn rend(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__baee6305__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crbegin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__54426dd4__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crend(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
            >::uninit();
            crate::detail::__rust_thunk__ec7b71a6__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn size(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__f5820f77__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn length(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__e7fcc61f__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6lengthEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn max_size(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__695013da__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE8max_sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__92fbf29d__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn at(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __pos: usize,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__d9fe6e35__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE2atEm(__this,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn front(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__2f149acd__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5frontEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn back(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__98754e6e__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4backEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn data(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) -> *const u32 {
        unsafe {
            crate::detail::__rust_thunk__3ab6e13d__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4dataEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn remove_prefix(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__14d676c6__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_prefixEm(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn remove_suffix(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__6fdcd7f7__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_suffixEm(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__other`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn swap(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __other: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__65a2cff8__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4swapERS3_(__this,__other)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__s`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn copy(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __s: *mut u32,
        __n: usize,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__7734a492__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4copyEPDimm(__this,__s,__n,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn substr(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__a47b72b2__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6substrEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__pos,__n);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < char16_t , std :: char_traits < char16_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {}
impl __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn begin(__this: *const Self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::begin(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn end(__this: *const Self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::end(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cbegin(__this: *const Self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::cbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn cend(__this: *const Self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::cend(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn rbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::rbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn rend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::rend(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::crbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::crend(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::size(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn length(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::length(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn max_size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::max_size(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::empty(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn at(__this: *const Self, __pos: usize) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::at(
                __this, __pos,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn front(__this: *const Self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::front(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn back(__this: *const Self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::back(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn data(__this: *const Self) -> *const u16 {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::data(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn remove_prefix(__this: *mut Self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::remove_prefix(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn remove_suffix(__this: *mut Self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::remove_suffix(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__other`: raw pointer
    #[inline(always)]
    pub unsafe fn swap(__this: *mut Self, __other: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::swap(
                __this, __other,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__s`: raw pointer
    #[inline(always)]
    pub unsafe fn copy(__this: *const Self, __s: *mut u16, __n: usize, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::copy(
                __this, __s, __n, __pos,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn substr(
        __this: *const Self,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee::substr(__this,__pos,__n)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__0155bed9__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl ::operator::CcIndex<usize>
    for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE
{
    type Output<'ctnr> = &'ctnr u16;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __pos: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__c1a8ac1a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEixEm(self,__pos)
        }
    }
}
impl ::core::ops::Index<usize>
    for __CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE
{
    type Output = u16;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

pub mod cc_template_inst_n_st3_u17basic_string_view_i_ds_ns_11char_traits_i_ds_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn begin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__c581f3e4__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5beginEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn end(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__fc92cc82__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE3endEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cbegin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__e9654ad0__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6cbeginEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn cend(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__54f567ee__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4cendEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn rbegin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__84266528__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn rend(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__baee6305__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crbegin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__54426dd4__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crend(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
            >::uninit();
            crate::detail::__rust_thunk__ec7b71a6__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn size(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__f5820f77__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn length(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__e7fcc61f__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6lengthEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn max_size(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__695013da__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE8max_sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__92fbf29d__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn at(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __pos: usize,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__d9fe6e35__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE2atEm(__this,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn front(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__2f149acd__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5frontEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn back(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__98754e6e__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4backEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn data(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) -> *const u16 {
        unsafe {
            crate::detail::__rust_thunk__3ab6e13d__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4dataEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn remove_prefix(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__14d676c6__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_prefixEm(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn remove_suffix(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__6fdcd7f7__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_suffixEm(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__other`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn swap(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __other: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__65a2cff8__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4swapERS3_(__this,__other)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__s`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn copy(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __s: *mut u16,
        __n: usize,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__7734a492__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4copyEPDsmm(__this,__s,__n,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn substr(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            >::uninit();
            crate::detail::__rust_thunk__a47b72b2__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6substrEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__pos,__n);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: basic_string_view < char8_t , std :: char_traits < char8_t >>
pub struct __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {}
impl !Sync for __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {}
impl __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn rbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::rbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn rend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::rend(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crbegin(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::crbegin(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn crend(
        __this: *const Self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::crend(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::size(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn length(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::length(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn max_size(__this: *const Self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::max_size(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn empty(__this: *const Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::empty(
                __this,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn remove_prefix(__this: *mut Self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::remove_prefix(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub unsafe fn remove_suffix(__this: *mut Self, __n: usize) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::remove_suffix(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__other`: raw pointer
    #[inline(always)]
    pub unsafe fn swap(__this: *mut Self, __other: *mut Self) {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::swap(
                __this, __other,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn substr(
        __this: *const Self,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::substr(__this,__pos,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn find(__this: *const Self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find(
                __this, __s, __pos,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn rfind(__this: *const Self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::rfind(
                __this, __s, __pos,
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn find_first_of(__this: *const Self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find_first_of(__this,__s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn find_last_of(__this: *const Self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find_last_of(__this,__s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn find_first_not_of(__this: *const Self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find_first_not_of(__this,__s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn find_last_not_of(__this: *const Self, mut __s: Self, __pos: usize) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::find_last_not_of(__this,__s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn starts_with(__this: *const Self, mut __s: Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::starts_with(__this,__s)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub unsafe fn ends_with(__this: *const Self, mut __s: Self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee::ends_with(__this,__s)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__0155bed9__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u17basic_string_view_i_du_ns_11char_traits_i_du_eeee {
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn rbegin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE,
            >::uninit();
            crate::detail::__rust_thunk__84266528__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn rend(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE,
            >::uninit();
            crate::detail::__rust_thunk__baee6305__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crbegin(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE,
            >::uninit();
            crate::detail::__rust_thunk__54426dd4__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE7crbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn crend(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE,
            >::uninit();
            crate::detail::__rust_thunk__ec7b71a6__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5crendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn size(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__f5820f77__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn length(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__e7fcc61f__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6lengthEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn max_size(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__695013da__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE8max_sizeEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn empty(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__92fbf29d__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5emptyEv(__this)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn remove_prefix(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__14d676c6__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_prefixEm(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn remove_suffix(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __n: usize,
    ) {
        unsafe {
            crate::detail::__rust_thunk__6fdcd7f7__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_suffixEm(__this,__n)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    /// * `__other`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn swap(
        __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __other: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) {
        unsafe {
            crate::detail::__rust_thunk__65a2cff8__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4swapERS3_(__this,__other)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn substr(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
        __n: usize,
    ) -> crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            >::uninit();
            crate::detail::__rust_thunk__a47b72b2__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6substrEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__pos,__n);
            __crubit_return.assume_init()
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn find(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__2765a73e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4findES3_m(__this,&mut __s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn rfind(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__af5e5fce__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5rfindES3_m(__this,&mut __s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn find_first_of(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__0ff1016e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13find_first_ofES3_m(__this,&mut __s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn find_last_of(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__72294100__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE12find_last_ofES3_m(__this,&mut __s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn find_first_not_of(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__aad246a1__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE17find_first_not_ofES3_m(__this,&mut __s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn find_last_not_of(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        __pos: usize,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__cfd04870__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE16find_last_not_ofES3_m(__this,&mut __s,__pos)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn starts_with(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__63374014__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE11starts_withES3_(__this,&mut __s)
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__this`: raw pointer
    #[must_use]
    #[inline(always)]
    pub(crate) unsafe fn ends_with(
        __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        mut __s: crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__3f933bd7__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE9ends_withES3_(__this,&mut __s)
        }
    }
}

// error: class `absl::internal_any_invocable::CoreImpl<false, MyOption<int>, MyOption<int>>` could not be bound
//   Crubit is not enabled on defining target:
//     third_party/absl/functional/internal/any_invocable.h
//   template instantiation is not yet supported

// error: class `absl::internal_any_invocable::CoreImpl<false, int, int>` could not be bound
//   Crubit is not enabled on defining target:
//     third_party/absl/functional/internal/any_invocable.h
//   template instantiation is not yet supported

// error: class `absl::internal_any_invocable::CoreImpl<false, void>` could not be bound
//   Crubit is not enabled on defining target:
//     third_party/absl/functional/internal/any_invocable.h
//   template instantiation is not yet supported

// error: class `absl::AnyInvocable` could not be bound
//   Return type of callable is incomplete: struct Incomplete

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z12CallVoidVoidN4absl12AnyInvocableIFvvOEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z15ReturnIntMapperv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z17MyOptionIntMapperv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z23ReturnNonConstIntMapperv(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi10EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi2EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi3EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi4EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi5EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi6EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi7EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi8EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__a965d653__ZNSt3__u12placeholders4__phILi9EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__7cb2b833__ZNSt3__u9allocatorIDiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__7b30cda6__ZNSt3__u9allocatorIDiE8allocateEm(
            __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
            __n: usize,
        ) -> *mut u32;
        pub(crate) unsafe fn __rust_thunk__8d7cdda6__ZNSt3__u9allocatorIDiE10deallocateEPDim(
            __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
            __p: *mut u32,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__7cb2b833__ZNSt3__u9allocatorIDsEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__7b30cda6__ZNSt3__u9allocatorIDsE8allocateEm(
            __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
            __n: usize,
        ) -> *mut u16;
        pub(crate) unsafe fn __rust_thunk__8d7cdda6__ZNSt3__u9allocatorIDsE10deallocateEPDsm(
            __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
            __p: *mut u16,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__7cb2b833__ZNSt3__u9allocatorIcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__7b30cda6__ZNSt3__u9allocatorIcE8allocateEm(
            __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIcEE,
            __n: usize,
        ) -> *mut ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__8d7cdda6__ZNSt3__u9allocatorIcE10deallocateEPcm(
            __this: *mut crate::__CcTemplateInstNSt3__u9allocatorIcEE,
            __p: *mut ::ffi_11::c_char,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__2107cf07__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__227390d7__ZNSt3__u3pmr21polymorphic_allocatorIDiEC1EPNS0_15memory_resourceE(
            __this: *mut ::core::ffi::c_void,
            __r: *mut ::cc_std::std::__u::pmr::memory_resource,
        );
        pub(crate) unsafe fn __rust_thunk__2107cf07__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__227390d7__ZNSt3__u3pmr21polymorphic_allocatorIDsEC1EPNS0_15memory_resourceE(
            __this: *mut ::core::ffi::c_void,
            __r: *mut ::cc_std::std::__u::pmr::memory_resource,
        );
        pub(crate) unsafe fn __rust_thunk__2107cf07__ZNSt3__u3pmr21polymorphic_allocatorIcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__227390d7__ZNSt3__u3pmr21polymorphic_allocatorIcEC1EPNS0_15memory_resourceE(
            __this: *mut ::core::ffi::c_void,
            __r: *mut ::cc_std::std::__u::pmr::memory_resource,
        );
        pub(crate) unsafe fn __rust_thunk__6e53fd45__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__f065fef8__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ERKS6_RKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__8e81d883__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>,
        );
        pub(crate) unsafe fn __rust_thunk__91697c2d__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1EOS6_RKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEC1ESt16initializer_listIDiERKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __il: &mut crate::__CcTemplateInstSt16initializer_listIDiE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__3aa0a429__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEED1Ev<
            'a,
        >(
            __this: ::core::pin::Pin<&'a mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>,
        );
        pub(crate)unsafe fn __rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEOS6_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>,__str: ::ctor::RvalueReference<'_,crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>;
        pub(crate)unsafe fn __rust_thunk__0e3b9316__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSESt16initializer_listIDiE<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>,__il: &mut crate::__CcTemplateInstSt16initializer_listIDiE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>;
        pub(crate)unsafe fn __rust_thunk__4cd278b5__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEPKDi<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>,__s: *const u32)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>;
        pub(crate) unsafe fn __rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE6cbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4cendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7crbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5crendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ec767001__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE7reserveEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__d528730e__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5emptyEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5c_strEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__21b14257__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13get_allocatorEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate)unsafe fn __rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSEDi<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>,__c: u32)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>;
        pub(crate)unsafe fn __rust_thunk__13806d1e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEaSERKS6_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>,__str: &crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>;
        pub(crate) unsafe fn __rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE9push_backEDi(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
            __c: u32,
        );
        pub(crate) unsafe fn __rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE8pop_backEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0df906ad__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE5clearEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ca1a6967__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE13shrink_to_fitEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__2f27b851__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEE4swapERS6_(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
            __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__6e53fd45__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__f065fef8__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_RKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__8e81d883__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,
        );
        pub(crate) unsafe fn __rust_thunk__91697c2d__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1EOS5_RKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__ea3378b5__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mmRKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
            __pos: usize,
            __n: usize,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__6c7db170__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ERKS5_mRKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
            __pos: usize,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEC1ESt16initializer_listIDiERKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __il: &mut crate::__CcTemplateInstSt16initializer_listIDiE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__3aa0a429__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEED1Ev<
            'a,
        >(
            __this: ::core::pin::Pin<&'a mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,
        );
        pub(crate)unsafe fn __rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEOS5_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,__str: ::ctor::RvalueReference<'_,crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>;
        pub(crate)unsafe fn __rust_thunk__0e3b9316__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSESt16initializer_listIDiE<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,__il: &mut crate::__CcTemplateInstSt16initializer_listIDiE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>;
        pub(crate)unsafe fn __rust_thunk__4cd278b5__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEPKDi<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,__s: *const u32)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>;
        pub(crate) unsafe fn __rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6cbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4cendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE7crbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5crendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__5348ae0a__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1005b8a3__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6lengthEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__3730555f__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8max_sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__ed5beee8__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8capacityEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__d528730e__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5emptyEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__1e6d7163__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm<
            '__return_lifetime,
        >(
            __this: &crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
            __pos: usize,
        ) -> &'__return_lifetime u32;
        pub(crate) unsafe fn __rust_thunk__310e3d8e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEixEm<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,
            __pos: usize,
        ) -> &'__return_lifetime mut u32;
        pub(crate) unsafe fn __rust_thunk__68107f44__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE6substrEmm(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
            __pos: usize,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5c_strEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__21b14257__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13get_allocatorEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate)unsafe fn __rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSEDi<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,__c: u32)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>;
        pub(crate)unsafe fn __rust_thunk__13806d1e__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEaSERKS5_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>,__str: &crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>;
        pub(crate) unsafe fn __rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE9push_backEDi(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
            __c: u32,
        );
        pub(crate) unsafe fn __rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE8pop_backEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0df906ad__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE5clearEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ca1a6967__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE13shrink_to_fitEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ac16dbb5__ZNKSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4copyEPDimm(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
            __s: *mut u32,
            __n: usize,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__2f27b851__ZNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEE4swapERS5_(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
            __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__6e53fd45__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__f065fef8__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ERKS6_RKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__8e81d883__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>,
        );
        pub(crate) unsafe fn __rust_thunk__91697c2d__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1EOS6_RKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEC1ESt16initializer_listIDsERKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __il: &mut crate::__CcTemplateInstSt16initializer_listIDsE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__3aa0a429__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEED1Ev<
            'a,
        >(
            __this: ::core::pin::Pin<&'a mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>,
        );
        pub(crate)unsafe fn __rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEOS6_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>,__str: ::ctor::RvalueReference<'_,crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>;
        pub(crate)unsafe fn __rust_thunk__0e3b9316__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSESt16initializer_listIDsE<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>,__il: &mut crate::__CcTemplateInstSt16initializer_listIDsE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>;
        pub(crate)unsafe fn __rust_thunk__4cd278b5__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEPKDs<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>,__s: *const u16)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>;
        pub(crate) unsafe fn __rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE6cbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4cendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7crbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5crendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ec767001__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE7reserveEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__d528730e__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5emptyEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5c_strEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__21b14257__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13get_allocatorEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate)unsafe fn __rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSEDs<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>,__c: u16)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>;
        pub(crate)unsafe fn __rust_thunk__13806d1e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEaSERKS6_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>,__str: &crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>;
        pub(crate) unsafe fn __rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE9push_backEDs(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
            __c: u16,
        );
        pub(crate) unsafe fn __rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE8pop_backEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0df906ad__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE5clearEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ca1a6967__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE13shrink_to_fitEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__2f27b851__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEE4swapERS6_(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
            __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__6e53fd45__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__f065fef8__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_RKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__8e81d883__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,
        );
        pub(crate) unsafe fn __rust_thunk__91697c2d__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1EOS5_RKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__ea3378b5__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mmRKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
            __pos: usize,
            __n: usize,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__6c7db170__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ERKS5_mRKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
            __pos: usize,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEC1ESt16initializer_listIDsERKS4_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __il: &mut crate::__CcTemplateInstSt16initializer_listIDsE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u9allocatorIDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__3aa0a429__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEED1Ev<
            'a,
        >(
            __this: ::core::pin::Pin<&'a mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,
        );
        pub(crate)unsafe fn __rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEOS5_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,__str: ::ctor::RvalueReference<'_,crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>;
        pub(crate)unsafe fn __rust_thunk__0e3b9316__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSESt16initializer_listIDsE<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,__il: &mut crate::__CcTemplateInstSt16initializer_listIDsE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>;
        pub(crate)unsafe fn __rust_thunk__4cd278b5__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEPKDs<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,__s: *const u16)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>;
        pub(crate) unsafe fn __rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6cbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4cendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE7crbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5crendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__5348ae0a__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__1005b8a3__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6lengthEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__3730555f__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8max_sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__ed5beee8__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8capacityEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__d528730e__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5emptyEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__1e6d7163__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm<
            '__return_lifetime,
        >(
            __this: &crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
            __pos: usize,
        ) -> &'__return_lifetime u16;
        pub(crate) unsafe fn __rust_thunk__310e3d8e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEixEm<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,
            __pos: usize,
        ) -> &'__return_lifetime mut u16;
        pub(crate) unsafe fn __rust_thunk__68107f44__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE6substrEmm(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
            __pos: usize,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5c_strEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__21b14257__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13get_allocatorEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate)unsafe fn __rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSEDs<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,__c: u16)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>;
        pub(crate)unsafe fn __rust_thunk__13806d1e__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEaSERKS5_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>,__str: &crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>;
        pub(crate) unsafe fn __rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE9push_backEDs(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
            __c: u16,
        );
        pub(crate) unsafe fn __rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE8pop_backEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0df906ad__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE5clearEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ca1a6967__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE13shrink_to_fitEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ac16dbb5__ZNKSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4copyEPDsmm(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
            __s: *mut u16,
            __n: usize,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__2f27b851__ZNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEE4swapERS5_(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
            __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__6e53fd45__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__f065fef8__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE,
        );
        pub(crate) unsafe fn __rust_thunk__2c6bd1e9__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__7eb4e77c__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ERKS6_RKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: &'__unelided crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE,
        );
        pub(crate) unsafe fn __rust_thunk__8e81d883__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>,
        );
        pub(crate) unsafe fn __rust_thunk__91697c2d__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1EOS6_RKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __str: ::ctor::RvalueReference<'__unelided,crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE,
        );
        pub(crate) unsafe fn __rust_thunk__e5bcc56c__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEC1ESt16initializer_listIcERKS5_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __il: &mut crate::__CcTemplateInstSt16initializer_listIcE,
            __a: &'__unelided crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE,
        );
        pub(crate) unsafe fn __rust_thunk__3aa0a429__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEED1Ev<
            'a,
        >(
            __this: ::core::pin::Pin<&'a mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>,
        );
        pub(crate)unsafe fn __rust_thunk__4d9b4d04__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEOS6_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>,__str: ::ctor::RvalueReference<'_,crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>;
        pub(crate)unsafe fn __rust_thunk__0e3b9316__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSESt16initializer_listIcE<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>,__il: &mut crate::__CcTemplateInstSt16initializer_listIcE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>;
        pub(crate)unsafe fn __rust_thunk__4cd278b5__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEPKc<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>,__s: *const::ffi_11::c_char)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>;
        pub(crate) unsafe fn __rust_thunk__c1e1cd48__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE6cbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__eb0590d4__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4cendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__dbd89a4a__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7crbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0e6cbd88__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5crendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ec767001__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE7reserveEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__d528730e__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5emptyEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__6b0e41c5__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5c_strEv(
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__21b14257__ZNKSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13get_allocatorEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate)unsafe fn __rust_thunk__11d5ea5c__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSEc<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>,__c: ::ffi_11::c_char)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>;
        pub(crate)unsafe fn __rust_thunk__13806d1e__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEaSERKS6_<'__return_lifetime>(__this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>,__str: &crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE)->::core::pin::Pin<&'__return_lifetime mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>;
        pub(crate) unsafe fn __rust_thunk__94c8dcfb__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE9push_backEc(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
            __c: ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk__3b1cebb3__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE8pop_backEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__0df906ad__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE5clearEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ca1a6967__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE13shrink_to_fitEv(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__2f27b851__ZNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEE4swapERS6_(
            __this: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
            __str: *mut crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEC1ES3_(
            __this: *mut ::core::ffi::c_void,
            __x: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEE4baseEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEC1ES3_(
            __this: *mut ::core::ffi::c_void,
            __x: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEE4baseEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEC1ES4_(
            __this: *mut ::core::ffi::c_void,
            __x: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEE4baseEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEC1ES4_(
            __this: *mut ::core::ffi::c_void,
            __x: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEE4baseEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEC1ES4_(
            __this: *mut ::core::ffi::c_void,
            __x: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEE4baseEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEC1ES3_(
            __this: *mut ::core::ffi::c_void,
            __x: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEE4baseEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKDiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorIPKDiEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const u32,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorIPKDiE4baseEv(
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKDsEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorIPKDsEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const u16,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorIPKDsE4baseEv(
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKDuEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__33a011dc__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk__b06b9ec4__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(
            __this: *const crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__86b85a07__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__5d27f53c__ZNSt16initializer_listIDiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9fe1b834__ZNKSt16initializer_listIDiE4dataEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__45dfec80__ZNKSt16initializer_listIDiE4sizeEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__dcdbfbf1__ZNKSt16initializer_listIDiE5emptyEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__17e6530b__ZNKSt16initializer_listIDiE5beginEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__f0cc69a4__ZNKSt16initializer_listIDiE3endEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDiE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__5d27f53c__ZNSt16initializer_listIDsEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9fe1b834__ZNKSt16initializer_listIDsE4dataEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__45dfec80__ZNKSt16initializer_listIDsE4sizeEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__dcdbfbf1__ZNKSt16initializer_listIDsE5emptyEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__17e6530b__ZNKSt16initializer_listIDsE5beginEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__f0cc69a4__ZNKSt16initializer_listIDsE3endEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIDsE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__5d27f53c__ZNSt16initializer_listIcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9fe1b834__ZNKSt16initializer_listIcE4dataEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__45dfec80__ZNKSt16initializer_listIcE4sizeEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__dcdbfbf1__ZNKSt16initializer_listIcE5emptyEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__17e6530b__ZNKSt16initializer_listIcE5beginEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__f0cc69a4__ZNKSt16initializer_listIcE3endEv(
            __this: *const crate::__CcTemplateInstSt16initializer_listIcE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPDiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPDiEplEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPDiEpLEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE;
        pub(crate) unsafe fn __rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPDiEmiEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPDiEmIEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE;
        pub(crate) unsafe fn __rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPDsEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPDsEplEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPDsEpLEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE;
        pub(crate) unsafe fn __rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPDsEmiEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPDsEmIEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE;
        pub(crate) unsafe fn __rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPKDiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPKDiEplEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPKDiEpLEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE;
        pub(crate) unsafe fn __rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPKDiEmiEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPKDiEmIEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE;
        pub(crate) unsafe fn __rust_thunk__2bf1b1cb__ZNKSt3__u11__wrap_iterIPKDiEixEl<
            '__return_lifetime,
        >(
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE,
            __n: isize,
        ) -> &'__return_lifetime u32;
        pub(crate) unsafe fn __rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPKDsEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPKDsEplEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPKDsEpLEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE;
        pub(crate) unsafe fn __rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPKDsEmiEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPKDsEmIEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE;
        pub(crate) unsafe fn __rust_thunk__2bf1b1cb__ZNKSt3__u11__wrap_iterIPKDsEixEl<
            '__return_lifetime,
        >(
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE,
            __n: isize,
        ) -> &'__return_lifetime u16;
        pub(crate) unsafe fn __rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPKcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPKcEplEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPKcEpLEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE;
        pub(crate) unsafe fn __rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPKcEmiEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPKcEmIEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE;
        pub(crate) unsafe fn __rust_thunk__2bf1b1cb__ZNKSt3__u11__wrap_iterIPKcEixEl<
            '__return_lifetime,
        >(
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE,
            __n: isize,
        ) -> &'__return_lifetime ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__db286e53__ZNSt3__u11__wrap_iterIPcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ab55222a__ZNKSt3__u11__wrap_iterIPcEplEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__75ad3749__ZNSt3__u11__wrap_iterIPcEpLEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE;
        pub(crate) unsafe fn __rust_thunk__4a39aa41__ZNKSt3__u11__wrap_iterIPcEmiEl(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__b385efc4__ZNSt3__u11__wrap_iterIPcEmIEl<
            '__return_lifetime,
        >(
            __this: &mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE,
            __n: isize,
        ) -> &'__return_lifetime mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE;
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1000000000000000000ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1000000000000000ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1000000000000ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1000000000ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1000000ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1000ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl100ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl10ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000000000000000EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000000000000EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000000000EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000000EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000000EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1000EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl100EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl10EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl1ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl2629746ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl31556952ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl3600ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl604800ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl60ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__806f2567__ZNSt3__u5ratioILl86400ELl1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__36eb0319__ZNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__78ef8f7e__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__bb707a65__ZNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEC1ERKS6_(
            __this: *mut ::core::ffi::c_void,
            __d: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__78ef8f7e__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__bb707a65__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEC1ERKS6_(
            __this: *mut ::core::ffi::c_void,
            __d: &crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__78ef8f7e__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__bb707a65__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEC1ERKS6_(
            __this: *mut ::core::ffi::c_void,
            __d: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__78ef8f7e__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__bb707a65__ZNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEC1ERKS6_(
            __this: *mut ::core::ffi::c_void,
            __d: &crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDiLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIDiLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIDiLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIDiLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIDiLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIDiLb0EEC1EDi(
            __this: *mut ::core::ffi::c_void,
            __d: u32,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDsLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIDsLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIDsLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIDsLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIDsLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIDsLb0EEC1EDs(
            __this: *mut ::core::ffi::c_void,
            __d: u16,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDuLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIDuLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIDuLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIDuLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIDuLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIaLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIaLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIaLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIaLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIaLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIaLb0EEC1Ea(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_schar,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIbLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIbLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIbLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIbLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIbLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIbLb0EEC1Eb(
            __this: *mut ::core::ffi::c_void,
            __d: bool,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIcLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIcLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIcLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIcLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIcLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIcLb0EEC1Ec(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIhLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIhLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIhLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIhLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIhLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIhLb0EEC1Eh(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIiLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIiLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIiLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIiLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIiLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIiLb0EEC1Ei(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIjLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIjLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIjLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIjLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIjLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIjLb0EEC1Ej(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_uint,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIlLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIlLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIlLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIlLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIlLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIlLb0EEC1El(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_long,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseImLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseImLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseImLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseImLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseImLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseImLb0EEC1Em(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_ulong,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIsLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIsLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIsLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIsLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIsLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIsLb0EEC1Es(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_short,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseItLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseItLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseItLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseItLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseItLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseItLb0EEC1Et(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_ushort,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIwLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIwLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIwLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIwLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIwLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIxLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIxLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIxLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIxLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIxLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIxLb0EEC1Ex(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_longlong,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIyLb0EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1332b893__ZNKSt3__u13__atomic_baseIyLb0EE12is_lock_freeEv(
            __this: *const crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__a005f1ef__ZNSt3__u13__atomic_baseIyLb0EE10notify_oneEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__699572a8__ZNSt3__u13__atomic_baseIyLb0EE10notify_allEv(
            __this: *mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE,
        );
        pub(crate) unsafe fn __rust_thunk__50d14bfd__ZNSt3__u13__atomic_baseIyLb0EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__8eba890f__ZNSt3__u13__atomic_baseIyLb0EEC1Ey(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_ulonglong,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDiLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIDiLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIDiLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIDiLb1EEC1EDi(
            __this: *mut ::core::ffi::c_void,
            __d: u32,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDsLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIDsLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIDsLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIDsLb1EEC1EDs(
            __this: *mut ::core::ffi::c_void,
            __d: u16,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIDuLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIDuLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIDuLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIaLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIaLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIaLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIaLb1EEC1Ea(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_schar,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIcLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIcLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIcLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIcLb1EEC1Ec(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIhLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIhLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIhLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIhLb1EEC1Eh(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIiLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIiLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIiLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIiLb1EEC1Ei(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIjLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIjLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIjLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIjLb1EEC1Ej(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_uint,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIlLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIlLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIlLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIlLb1EEC1El(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_long,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseImLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseImLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseImLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseImLb1EEC1Em(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_ulong,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIsLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIsLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIsLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIsLb1EEC1Es(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_short,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseItLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseItLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseItLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseItLb1EEC1Et(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_ushort,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIwLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIwLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIwLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIxLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIxLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIxLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIxLb1EEC1Ex(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_longlong,
        );
        pub(crate) unsafe fn __rust_thunk__705c8bf3__ZNSt3__u13__atomic_baseIyLb1EEaSEOS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE>,
            __param_0: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE,
            >,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__1dc7b320__ZNSt3__u13__atomic_baseIyLb1EEaSERKS1_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE>,
            __param_0: &crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE,
        >;
        pub(crate) unsafe fn __rust_thunk__c686b37a__ZNSt3__u13__atomic_baseIyLb1EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__dd654b09__ZNSt3__u13__atomic_baseIyLb1EEC1Ey(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_ulonglong,
        );
        pub(crate) unsafe fn __rust_thunk__112a35fb__ZNSt3__u6atomicIDuEC1EvQ26is_default_constructible_vIT_E(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__112a35fb__ZNSt3__u6atomicIlEC1EvQ26is_default_constructible_vIT_E(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__5aa22ed4__ZNSt3__u6atomicIlEC1El(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_long,
        );
        pub(crate) unsafe fn __rust_thunk__51bb7eeb__ZNSt3__u6atomicIlEaSEl(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u6atomicIlEE>,
            __d: ::ffi_11::c_long,
        ) -> ::ffi_11::c_long;
        pub(crate) unsafe fn __rust_thunk__112a35fb__ZNSt3__u6atomicImEC1EvQ26is_default_constructible_vIT_E(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__5aa22ed4__ZNSt3__u6atomicImEC1Em(
            __this: *mut ::core::ffi::c_void,
            __d: ::ffi_11::c_ulong,
        );
        pub(crate) unsafe fn __rust_thunk__51bb7eeb__ZNSt3__u6atomicImEaSEm(
            __this: ::core::pin::Pin<&mut crate::__CcTemplateInstNSt3__u6atomicImEE>,
            __d: ::ffi_11::c_ulong,
        ) -> ::ffi_11::c_ulong;
        pub(crate) unsafe fn __rust_thunk__112a35fb__ZNSt3__u6atomicIwEC1EvQ26is_default_constructible_vIT_E(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ce8a09ba__ZNSt3__u19__allocation_resultIPDimEC1ES1_m(
            __this: *mut ::core::ffi::c_void,
            __ptr: *mut u32,
            __count: ::ffi_11::c_ulong,
        );
        pub(crate) unsafe fn __rust_thunk__ce8a09ba__ZNSt3__u19__allocation_resultIPDsmEC1ES1_m(
            __this: *mut ::core::ffi::c_void,
            __ptr: *mut u16,
            __count: ::ffi_11::c_ulong,
        );
        pub(crate) unsafe fn __rust_thunk__ce8a09ba__ZNSt3__u19__allocation_resultIPcmEC1ES1_m(
            __this: *mut ::core::ffi::c_void,
            __ptr: *mut ::ffi_11::c_char,
            __count: ::ffi_11::c_ulong,
        );
        pub(crate) unsafe fn __rust_thunk__0155bed9__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__c581f3e4__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5beginEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__fc92cc82__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE3endEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__e9654ad0__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6cbeginEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__54f567ee__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4cendEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__84266528__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6rbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__baee6305__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4rendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__54426dd4__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE7crbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ec7b71a6__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5crendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__f5820f77__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__e7fcc61f__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6lengthEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__695013da__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE8max_sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__92fbf29d__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5emptyEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__c1a8ac1a__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEixEm<
            '__return_lifetime,
        >(
            __this: &crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __pos: usize,
        ) -> &'__return_lifetime u32;
        pub(crate) unsafe fn __rust_thunk__d9fe6e35__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE2atEm(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __pos: usize,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__2f149acd__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE5frontEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__98754e6e__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4backEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__3ab6e13d__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4dataEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        ) -> *const u32;
        pub(crate) unsafe fn __rust_thunk__14d676c6__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_prefixEm(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__6fdcd7f7__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE13remove_suffixEm(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__65a2cff8__ZNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4swapERS3_(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __other: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__7734a492__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE4copyEPDimm(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __s: *mut u32,
            __n: usize,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__a47b72b2__ZNKSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEE6substrEmm(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __pos: usize,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__0155bed9__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__c581f3e4__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5beginEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__fc92cc82__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE3endEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__e9654ad0__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6cbeginEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__54f567ee__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4cendEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__84266528__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6rbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__baee6305__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4rendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__54426dd4__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE7crbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ec7b71a6__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5crendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__f5820f77__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__e7fcc61f__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6lengthEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__695013da__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE8max_sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__92fbf29d__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5emptyEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__c1a8ac1a__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEixEm<
            '__return_lifetime,
        >(
            __this: &crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __pos: usize,
        ) -> &'__return_lifetime u16;
        pub(crate) unsafe fn __rust_thunk__d9fe6e35__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE2atEm(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __pos: usize,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__2f149acd__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE5frontEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__98754e6e__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4backEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__3ab6e13d__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4dataEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        ) -> *const u16;
        pub(crate) unsafe fn __rust_thunk__14d676c6__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_prefixEm(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__6fdcd7f7__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE13remove_suffixEm(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__65a2cff8__ZNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4swapERS3_(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __other: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__7734a492__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE4copyEPDsmm(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __s: *mut u16,
            __n: usize,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__a47b72b2__ZNKSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEE6substrEmm(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __pos: usize,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__0155bed9__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__84266528__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6rbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__baee6305__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4rendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__54426dd4__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE7crbeginEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__ec7b71a6__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5crendEv(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__f5820f77__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__e7fcc61f__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6lengthEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__695013da__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE8max_sizeEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__92fbf29d__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5emptyEv(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__14d676c6__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_prefixEm(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__6fdcd7f7__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13remove_suffixEm(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__65a2cff8__ZNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4swapERS3_(
            __this: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __other: *mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__a47b72b2__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE6substrEmm(
            __return: *mut ::core::ffi::c_void,
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
            __n: usize,
        );
        pub(crate) unsafe fn __rust_thunk__2765a73e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE4findES3_m(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__af5e5fce__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE5rfindES3_m(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__0ff1016e__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE13find_first_ofES3_m(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__72294100__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE12find_last_ofES3_m(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__aad246a1__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE17find_first_not_ofES3_m(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__cfd04870__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE16find_last_not_ofES3_m(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __pos: usize,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__63374014__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE11starts_withES3_(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__3f933bd7__ZNKSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEE9ends_withES3_(
            __this: *const crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __s: &mut crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        ) -> bool;
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: ::ffi_11::c_int,
    ) -> ::ffi_11::c_int {
        (unsafe { &*f })(param_0)
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe {
            ::core::ptr::replace(
                f,
                ::alloc::boxed::Box::new(|| {
                    ::core::unreachable!("Called FnOnce after it was moved");
                }),
            )
        })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            f: *mut ::any_invocable::RawAnyInvocable,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(
                    crate::MyOption<::ffi_11::c_int>,
                ) -> crate::MyOption<::ffi_11::c_int>
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::core::ffi::c_uchar,
        bridge_buffer: *mut ::core::ffi::c_uchar,
    ) {
        let param_0 = unsafe {
            ::bridge_rust::internal::decode(
                crate::MyOptionAbi(::bridge_rust::transmute_abi::<::core::ffi::c_int>()),
                param_0,
            )
        };
        unsafe {
            ::bridge_rust::internal::encode(
                crate::MyOptionAbi(::bridge_rust::transmute_abi::<::core::ffi::c_int>()),
                bridge_buffer,
                (unsafe { &*f })(param_0),
            )
        };
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(
                    crate::MyOption<::ffi_11::c_int>,
                ) -> crate::MyOption<::ffi_11::c_int>
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(
                    crate::MyOption<::ffi_11::c_int>,
                ) -> crate::MyOption<::ffi_11::c_int>
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF8MyOptionIiES2_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: *const ::core::ffi::c_uchar,
            out: *mut ::core::ffi::c_uchar,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: ::ffi_11::c_int,
    ) -> ::ffi_11::c_int {
        (unsafe { &*f })(param_0)
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fconsume_5fabsl_3aabsl_5ffunctional(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi10EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi10EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi10EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi10EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi1EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi1EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi1EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi2EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi2EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi2EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi2EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi3EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi3EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi3EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi3EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi4EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi4EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi4EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi4EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi5EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi5EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi5EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi5EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi6EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi6EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi6EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi6EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi7EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi7EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi7EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi7EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi8EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi8EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi8EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi8EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi9EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12placeholders4__phILi9EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi9EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12placeholders4__phILi9EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u9allocatorIDiEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u9allocatorIDiEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u9allocatorIDiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u9allocatorIDiEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u9allocatorIDsEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u9allocatorIDsEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u9allocatorIDsEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u9allocatorIDsEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u9allocatorIcEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u9allocatorIcEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u9allocatorIcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u9allocatorIcEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE>()
            == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDiEE,
            __res_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE>()
            == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIDsEE,
            __res_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE>()
            == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u3pmr21polymorphic_allocatorIcEE,
            __res_
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>()==32);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE: Drop);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE: Copy);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,__rep_)==0);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_3pmr21polymorphic_allocatorIDiEEEE,__alloc_)==24);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>()==24);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE: Drop);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE: Copy);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u12basic_stringIDiNS_11char_traitsIDiEENS_9allocatorIDiEEEE,__rep_)==0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>()==32);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE: Drop);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE: Copy);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,__rep_)==0);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_3pmr21polymorphic_allocatorIDsEEEE,__alloc_)==24);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>()==24);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE: Drop);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE: Copy);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u12basic_stringIDsNS_11char_traitsIDsEENS_9allocatorIDsEEEE,__rep_)==0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>()==32);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE: Drop);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE: Copy);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,__rep_)==0);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u12basic_stringIcNS_11char_traitsIcEENS_3pmr21polymorphic_allocatorIcEEEE,__alloc_)==24);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDiEEEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPDsEEEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDiEEEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKDsEEEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKcEEEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPcEEEE,
            current
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE>() == 8);
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDiEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE>() == 8);
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDsEE, current)
            == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE>() == 8);
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorIPKDuEE, current)
            == 0
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
    assert!(::core::mem::size_of::<crate::__CcTemplateInstSt16initializer_listIDiE>() == 16);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstSt16initializer_listIDiE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstSt16initializer_listIDiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstSt16initializer_listIDiE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstSt16initializer_listIDiE, __begin_) == 0
    );
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstSt16initializer_listIDiE, __size_) == 8);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstSt16initializer_listIDsE>() == 16);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstSt16initializer_listIDsE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstSt16initializer_listIDsE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstSt16initializer_listIDsE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstSt16initializer_listIDsE, __begin_) == 0
    );
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstSt16initializer_listIDsE, __size_) == 8);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstSt16initializer_listIcE>() == 16);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstSt16initializer_listIcE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstSt16initializer_listIcE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstSt16initializer_listIcE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstSt16initializer_listIcE, __begin_) == 0);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstSt16initializer_listIcE, __size_) == 8);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPDiEE, __i_) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPDsEE, __i_) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDiEE, __i_) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKDsEE, __i_) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKcEE, __i_) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPcEE, __i_) == 0);
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE>()
            == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE>(
        ) == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000000ELl1EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE>()
            == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE>()
            == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1000000000000000ELl1EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1000000000000ELl1EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1000000000ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1000000ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1000ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1000ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1000ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl100ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl100ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl100ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl100ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl10ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl10ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl10ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl10ELl1EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE>()
            == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE>(
        ) == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000000EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE>()
            == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE>()
            == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000000EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000000EEE: Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000000EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000000EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1000EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl100EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl100EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl100EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl100EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl10EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl10EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl10EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl10EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl1ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl1ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl2629746ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl31556952ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl3600ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl3600ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl3600ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl3600ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl604800ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl604800ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl604800ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl604800ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl60ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl60ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl60ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl60ELl1EEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u5ratioILl86400ELl1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u5ratioILl86400ELl1EEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u5ratioILl86400ELl1EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u5ratioILl86400ELl1EEE: Drop);

    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE,
        >() == 4
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE,
        >() == 4
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl2629746ELl1EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE,
        >() == 4
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE,
        >() == 4
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl31556952ELl1EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE,
        >() == 4
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE,
        >() == 4
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl604800ELl1EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE,
        >() == 4
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE,
        >() == 4
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIiNS_5ratioILl86400ELl1EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl3600ELl1EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIlNS_5ratioILl60ELl1EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000000EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000000EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1000EEEEE,
            __rep_
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u6chrono8durationIxNS_5ratioILl1ELl1EEEEE,
            __rep_
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE>()==8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12steady_clockENS0_8durationIxNS_5ratioILl1ELl1000000000EEEEEEE,__d_)==0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE>()==4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE>()==4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIiNS_5ratioILl86400ELl1EEEEEEE,__d_)==0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE>()==8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1000000EEEEEEE,__d_)==0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE>()==8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u6chrono10time_pointINS0_12system_clockENS0_8durationIxNS_5ratioILl1ELl1EEEEEEE,__d_)==0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE>() == 2);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE>() == 2);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIbLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseImLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE>() == 2);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE>() == 2);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE>() == 2);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE>() == 2);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseItLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE: Copy,Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb0EEE, __a_) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDiLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE>() == 2);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE>() == 2);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDsLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIDuLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIaLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIcLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIhLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIiLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIjLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIlLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseImLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE>() == 2);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE>() == 2);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIsLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE>() == 2);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE>() == 2);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseItLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIwLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIxLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u13__atomic_baseIyLb1EEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u6atomicIDuEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u6atomicIDuEE>() == 1);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6atomicIDuEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u6atomicIlEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u6atomicIlEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6atomicIlEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u6atomicImEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u6atomicImEE>() == 8);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6atomicImEE: Copy,Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u6atomicIwEE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u6atomicIwEE>() == 4);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u6atomicIwEE: Copy,Drop);

    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE>() == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE, ptr)
            == 0
    );
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPDimEE, count)
            == 8
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE>() == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE, ptr)
            == 0
    );
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPDsmEE, count)
            == 8
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE>() == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE>() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE: Drop);
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE, ptr)
            == 0
    );
    assert!(
        ::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u19__allocation_resultIPcmEE, count)
            == 8
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDiNS_11char_traitsIDiEEEE,
            __size_
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDsNS_11char_traitsIDsEEEE,
            __size_
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u17basic_string_viewIDuNS_11char_traitsIDuEEEE,
            __size_
        ) == 8
    );
};
