// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/bazel/header_in_multiple_targets:lib

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod shared_ns {
    /// Ordinary `//` comments are used as doc comments, even though precompiled
    /// header modules don't contain them.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N9shared_ns4InfoE"]
    #[repr(C, align(4))]
    ///CRUBIT_ANNOTATE: cpp_type=shared_ns :: Info
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Info {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
    }
    impl !Send for Info {}
    impl !Sync for Info {}
    unsafe impl ::cxx::ExternType for Info {
        type Id = ::cxx::type_id!("shared_ns :: Info");
        type Kind = ::cxx::kind::Trivial;
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("shared_ns :: Info"),
        crate::shared_ns::Info
    );
    impl Info {
        /// Returns the value.
        #[inline(always)]
        pub fn value<'__this>(&'__this self) -> ::ffi_11::c_int {
            unsafe { self::info::value(self) }
        }
    }

    impl Default for Info {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN9shared_ns4InfoC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod info {
        /// Returns the value.
        #[inline(always)]
        pub(crate) fn value<'__this>(__this: &'__this crate::shared_ns::Info) -> ::ffi_11::c_int {
            unsafe { crate::detail::__rust_thunk___ZNK9shared_ns4Info5valueEv(__this) }
        }
    }

    // A free comment, which isn't attached to any declaration.

    // Clang doesn't merge the declarations below (which have no linkage, or
    // internal linkage) when it sees them both in a precompiled header module, and
    // in a textual inclusion of the same header.

    /// An alias of a class template specialization whose arguments are declared in
    /// this header.
    pub type Callback = crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE;

    pub type IntAlias = ::ffi_11::c_int;

    pub type InfoTypedef = crate::shared_ns::Info;

    pub const kSharedConstant: ::ffi_11::c_int = ::ffi_11::new_c_int(3);

    pub const kAnonymousEnumerator: ::ffi_11::c_uint = ::ffi_11::new_c_uint(5);

    /// With header modules, this isn't bound in `:lib`: like overloads, the two
    /// copies of this function make taking its address ambiguous in the generated
    /// C++ code. This checks that the generated C++ code still compiles.
    #[inline(always)]
    pub fn StaticSharedFunction() -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN9shared_nsL20StaticSharedFunctionEv() }
    }

    /// Clang does merge the declarations below (which have external linkage).
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
    #[cfi_encoding = "N9shared_ns5ColorE"]
    ///CRUBIT_ANNOTATE: cpp_type=shared_ns :: Color
    pub struct Color(::ffi_11::c_int);
    impl Color {
        pub const kRed: Color = Color(::ffi_11::new_c_int(0));
        pub const kGreen: Color = Color(::ffi_11::new_c_int(1));
    }
    impl From<::ffi_11::c_int> for Color {
        fn from(value: ::ffi_11::c_int) -> Color {
            Color(value)
        }
    }
    impl From<Color> for ::ffi_11::c_int {
        fn from(value: Color) -> ::ffi_11::c_int {
            value.0
        }
    }

    #[inline(always)]
    pub fn SharedFunction() -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN9shared_ns14SharedFunctionEv() }
    }
}

// namespace shared_ns

pub mod other_ns {
    pub type Info = crate::shared_ns::Info;
}

// namespace other_ns

// Note: this header deliberately includes `shared.h` only indirectly, through
// `user_of_shared.h`, which belongs to another target. When header modules are
// used, that makes the declarations of `shared.h` reachable from the
// precompiled module of `:shared`, in addition to `shared.h` itself being
// parsed as a header of `:lib`.

// IWYU pragma: keep

pub mod lib_ns {
    #[inline(always)]
    pub fn LibFunction(x: ::ffi_11::c_int) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN6lib_ns11LibFunctionEi(x) }
    }
}

// namespace lib_ns

/// Not Rust-movable: member `__f_` is not bitwise-relocatable: destructor or move constructor is user-provided
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: function < int (const class shared_ns :: Info &) >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __f_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 32],
}
impl !Send for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE {}
impl !Sync for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: function < int (const class shared_ns :: Info &) >"),
    crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE
);
impl __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE {
    #[inline(always)]
    pub fn swap<'__f, '__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __f: ::core::pin::Pin<&'__f mut Self>,
    ) {
        unsafe {
            self::cc_template_inst_n_st3_u8function_i_fi_rkn9shared_ns4_info_eeee::swap(self, __f)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn target_type<'__this>(&'__this self) -> ::cref::CRef<'__this, ::cc_std::std::type_info> {
        unsafe {
            self::cc_template_inst_n_st3_u8function_i_fi_rkn9shared_ns4_info_eeee::target_type(self)
        }
    }
}

impl ::ctor::CtorNew<()> for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let () = args;
                crate::detail::__rust_thunk__f12ce4cd__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEC1Ev(__crubit_dest as*mut::core::ffi::c_void);
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self>
    for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE
{
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let mut __param_0 = args;
                crate::detail::__rust_thunk__e98edf78__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEC1ERKS6_(__crubit_dest as*mut::core::ffi::c_void,__param_0);
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)>
    for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE
{
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>
    for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE
{
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let mut __param_0 = args;
                crate::detail::__rust_thunk__62bbc79a__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEC1EOS6_(__crubit_dest as*mut::core::ffi::c_void,__param_0);
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE
{
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl<'__f> ::ctor::Assign<&'__f Self>
    for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE
{
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __f: &'__f Self) {
        unsafe {
            crate::detail::__rust_thunk__0a536d7f__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEaSERKS6_(self,__f);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>>
    for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE
{
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __f: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk__4e5bdb92__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEaSEOS6_(
                self, __f,
            );
        }
    }
}

impl ::ctor::PinnedDrop for __CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe {
            crate::detail::__rust_thunk__fbe0e9d2__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEED1Ev(
                self,
            )
        }
    }
}

pub mod cc_template_inst_n_st3_u8function_i_fi_rkn9shared_ns4_info_eeee {
    #[inline(always)]
    pub(crate) fn swap<'__f, '__this>(
        __this: ::core::pin::Pin<
            &'__this mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
        >,
        __f: ::core::pin::Pin<
            &'__f mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
        >,
    ) {
        unsafe {
            crate::detail::__rust_thunk__44069562__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEE4swapERS6_(__this,__f)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn target_type<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
    ) -> ::cref::CRef<'__this, ::cc_std::std::type_info> {
        unsafe {
            crate::detail::__rust_thunk__2aa8490e__ZNKSt3__u8functionIFiRKN9shared_ns4InfoEEE11target_typeEv(__this)
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9shared_ns4InfoC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZNK9shared_ns4Info5valueEv<'__this>(
            __this: &'__this crate::shared_ns::Info,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN9shared_nsL20StaticSharedFunctionEv(
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN9shared_ns14SharedFunctionEv() -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6lib_ns11LibFunctionEi(
            x: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__f12ce4cd__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__e98edf78__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEC1ERKS6_<
            '__param_0,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__62bbc79a__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEC1EOS6_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<
                '__unelided,
                crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
            >,
        );
        pub(crate) unsafe fn __rust_thunk__0a536d7f__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEaSERKS6_<
            '__f,
            '__this,
        >(
            __this: ::core::pin::Pin<
                &'__this mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
            >,
            __f: &'__f crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
        ) -> ::core::pin::Pin<
            &'__this mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
        >;
        pub(crate) unsafe fn __rust_thunk__4e5bdb92__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEEaSEOS6_<
            '__this,
        >(
            __this: ::core::pin::Pin<
                &'__this mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
            >,
            __f: ::ctor::RvalueReference<
                '_,
                crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
            >,
        ) -> ::core::pin::Pin<
            &'__this mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
        >;
        pub(crate) unsafe fn __rust_thunk__fbe0e9d2__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEED1Ev<
            '__this,
        >(
            __this: ::core::pin::Pin<
                &'__this mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
            >,
        );
        pub(crate) unsafe fn __rust_thunk__44069562__ZNSt3__u8functionIFiRKN9shared_ns4InfoEEE4swapERS6_<
            '__f,
            '__this,
        >(
            __this: ::core::pin::Pin<
                &'__this mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
            >,
            __f: ::core::pin::Pin<
                &'__f mut crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
            >,
        );
        pub(crate) unsafe fn __rust_thunk__2aa8490e__ZNKSt3__u8functionIFiRKN9shared_ns4InfoEEE11target_typeEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
        ) -> ::cref::CRef<'__this, ::cc_std::std::type_info>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::shared_ns::Info>() == 4);
    assert!(::core::mem::align_of::<crate::shared_ns::Info>() == 4);
    static_assertions::assert_impl_all!(crate::shared_ns::Info: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::shared_ns::Info: Drop);
    assert!(::core::mem::offset_of!(crate::shared_ns::Info, value_) == 0);
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE>()
            == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE: Drop);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE: Copy);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u8functionIFiRKN9shared_ns4InfoEEEE,
            __f_
        ) == 0
    );
};
