// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:typedefs_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeStruct
pub struct SomeStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeStruct {}
impl !Sync for SomeStruct {}
unsafe impl ::cxx::ExternType for SomeStruct {
    type Id = ::cxx::type_id!("SomeStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
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

pub mod some_struct {
    #[allow(unused_imports)]
    use super::*;

    pub type nested_type = ::core::ffi::c_int;
}

// Error while generating bindings for type alias 'SomeStruct':
// Typedef only used to introduce a name in C. Not importing.

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeOtherStruct
pub struct SomeOtherStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeOtherStruct {}
impl !Sync for SomeOtherStruct {}
unsafe impl ::cxx::ExternType for SomeOtherStruct {
    type Id = ::cxx::type_id!("SomeOtherStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("SomeOtherStruct"),
    crate::SomeOtherStruct
);

impl Default for SomeOtherStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15SomeOtherStructC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeOtherStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15SomeOtherStructC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for SomeOtherStruct {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeOtherStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15SomeOtherStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeOtherStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN15SomeOtherStructaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeUnion
pub union SomeUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeUnion {}
impl !Sync for SomeUnion {}
unsafe impl ::cxx::ExternType for SomeUnion {
    type Id = ::cxx::type_id!("SomeUnion");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeUnion"), crate::SomeUnion);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN9SomeUnionC1Ev {}
impl<'error> Default for SomeUnion
where
    &'error (): BindingFailedFor_ZN9SomeUnionC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for function 'SomeUnion::SomeUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN9SomeUnionC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for SomeUnion
where
    &'error (): BindingFailedFor_ZN9SomeUnionC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for type alias 'SomeUnion':
// Typedef only used to introduce a name in C. Not importing.

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeOtherUnion
pub union SomeOtherUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeOtherUnion {}
impl !Sync for SomeOtherUnion {}
unsafe impl ::cxx::ExternType for SomeOtherUnion {
    type Id = ::cxx::type_id!("SomeOtherUnion");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeOtherUnion"), crate::SomeOtherUnion);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN14SomeOtherUnionC1Ev {}
impl<'error> Default for SomeOtherUnion
where
    &'error (): BindingFailedFor_ZN14SomeOtherUnionC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for function 'SomeOtherUnion::SomeOtherUnion':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN14SomeOtherUnionC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for SomeOtherUnion
where
    &'error (): BindingFailedFor_ZN14SomeOtherUnionC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeOtherUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN14SomeOtherUnionaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeOtherUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN14SomeOtherUnionaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn FunctionUsingNestedType() -> crate::some_struct::nested_type {
    unsafe { crate::detail::__rust_thunk___Z23FunctionUsingNestedTypev() }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN10SomeStructC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
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
        pub(crate) unsafe fn __rust_thunk___ZN15SomeOtherStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15SomeOtherStructC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeOtherStruct>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15SomeOtherStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeOtherStruct,
            __param_0: &'b crate::SomeOtherStruct,
        ) -> &'a mut crate::SomeOtherStruct;
        pub(crate) unsafe fn __rust_thunk___ZN15SomeOtherStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeOtherStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeOtherStruct>,
        ) -> &'a mut crate::SomeOtherStruct;
        pub(crate) unsafe fn __rust_thunk___ZN9SomeUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeUnion,
            __param_0: &'b crate::SomeUnion,
        ) -> &'a mut crate::SomeUnion;
        pub(crate) unsafe fn __rust_thunk___ZN9SomeUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeUnion>,
        ) -> &'a mut crate::SomeUnion;
        pub(crate) unsafe fn __rust_thunk___ZN14SomeOtherUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeOtherUnion,
            __param_0: &'b crate::SomeOtherUnion,
        ) -> &'a mut crate::SomeOtherUnion;
        pub(crate) unsafe fn __rust_thunk___ZN14SomeOtherUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeOtherUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeOtherUnion>,
        ) -> &'a mut crate::SomeOtherUnion;
        #[link_name = "_Z23FunctionUsingNestedTypev"]
        pub(crate) unsafe fn __rust_thunk___Z23FunctionUsingNestedTypev(
        ) -> crate::some_struct::nested_type;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeStruct>() == 1);
    assert!(::core::mem::align_of::<crate::SomeStruct>() == 1);
    static_assertions::assert_impl_all!(crate::SomeStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);

    assert!(::core::mem::size_of::<crate::SomeOtherStruct>() == 1);
    assert!(::core::mem::align_of::<crate::SomeOtherStruct>() == 1);
    static_assertions::assert_impl_all!(crate::SomeOtherStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeOtherStruct: Drop);

    assert!(::core::mem::size_of::<crate::SomeUnion>() == 1);
    assert!(::core::mem::align_of::<crate::SomeUnion>() == 1);
    static_assertions::assert_impl_all!(crate::SomeUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeUnion: Drop);

    assert!(::core::mem::size_of::<crate::SomeOtherUnion>() == 1);
    assert!(::core::mem::align_of::<crate::SomeOtherUnion>() == 1);
    static_assertions::assert_impl_all!(crate::SomeOtherUnion: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeOtherUnion: Drop);
};
