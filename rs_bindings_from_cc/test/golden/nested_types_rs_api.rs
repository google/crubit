// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nested_types_cc

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
///CRUBIT_ANNOTATE: cpp_type=Foo
pub struct Foo {
    pub foo: ::core::ffi::c_int,
}
impl !Send for Foo {}
impl !Sync for Foo {}
forward_declare::unsafe_define!(forward_declare::symbol!("Foo"), crate::Foo);

impl Default for Foo {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for Foo {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3FooC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Foo {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for Foo {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN3FooaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Foo {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN3FooaSEOS_(self, __param_0);
        }
    }
}

pub mod foo {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=Foo :: Bar
    pub struct Bar {
        pub bar: ::core::ffi::c_int,
    }
    impl !Send for Bar {}
    impl !Sync for Bar {}
    forward_declare::unsafe_define!(forward_declare::symbol!("Foo :: Bar"), crate::foo::Bar);

    impl Default for Bar {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BarC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl From<::ctor::RvalueReference<'_, Self>> for Bar {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BarC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Bar {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    impl ::ctor::UnpinAssign<&Self> for Bar {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BaraSERKS0_(self, __param_0);
            }
        }
    }

    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Bar {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN3Foo3BaraSEOS0_(self, __param_0);
            }
        }
    }

    pub mod bar {
        #[allow(unused_imports)]
        use super::*;

        #[derive(Clone, Copy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=Foo :: Bar :: Baz
        pub struct Baz {
            pub baz: ::core::ffi::c_int,
        }
        impl !Send for Baz {}
        impl !Sync for Baz {}
        forward_declare::unsafe_define!(
            forward_declare::symbol!("Foo :: Bar :: Baz"),
            crate::foo::bar::Baz
        );

        impl Default for Baz {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazC1Ev(
                        &raw mut tmp as *mut ::core::ffi::c_void,
                    );
                    tmp.assume_init()
                }
            }
        }

        impl From<::ctor::RvalueReference<'_, Self>> for Baz {
            #[inline(always)]
            fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazC1EOS1_(
                        &raw mut tmp as *mut ::core::ffi::c_void,
                        __param_0,
                    );
                    tmp.assume_init()
                }
            }
        }
        impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Baz {
            type CtorType = Self;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
                <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
            }
        }

        impl ::ctor::UnpinAssign<&Self> for Baz {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: &Self) {
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazaSERKS1_(self, __param_0);
                }
            }
        }

        impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Baz {
            #[inline(always)]
            fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
                unsafe {
                    crate::detail::__rust_thunk___ZN3Foo3Bar3BazaSEOS1_(self, __param_0);
                }
            }
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN3FooC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3FooC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Foo>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3FooaSERKS_<'__return_lifetime>(
            __this: &mut crate::Foo,
            __param_0: &crate::Foo,
        ) -> &'__return_lifetime mut crate::Foo;
        pub(crate) unsafe fn __rust_thunk___ZN3FooaSEOS_<'__return_lifetime>(
            __this: &mut crate::Foo,
            __param_0: ::ctor::RvalueReference<'_, crate::Foo>,
        ) -> &'__return_lifetime mut crate::Foo;
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BarC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::Bar>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BaraSERKS0_<'__return_lifetime>(
            __this: &mut crate::foo::Bar,
            __param_0: &crate::foo::Bar,
        ) -> &'__return_lifetime mut crate::foo::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3BaraSEOS0_<'__return_lifetime>(
            __this: &mut crate::foo::Bar,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::Bar>,
        ) -> &'__return_lifetime mut crate::foo::Bar;
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazC1EOS1_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::bar::Baz>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazaSERKS1_<'__return_lifetime>(
            __this: &mut crate::foo::bar::Baz,
            __param_0: &crate::foo::bar::Baz,
        ) -> &'__return_lifetime mut crate::foo::bar::Baz;
        pub(crate) unsafe fn __rust_thunk___ZN3Foo3Bar3BazaSEOS1_<'__return_lifetime>(
            __this: &mut crate::foo::bar::Baz,
            __param_0: ::ctor::RvalueReference<'_, crate::foo::bar::Baz>,
        ) -> &'__return_lifetime mut crate::foo::bar::Baz;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Foo>() == 4);
    assert!(::core::mem::align_of::<crate::Foo>() == 4);
    static_assertions::assert_impl_all!(crate::Foo: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Foo: Drop);
    assert!(::core::mem::offset_of!(crate::Foo, foo) == 0);
    assert!(::core::mem::size_of::<crate::foo::Bar>() == 4);
    assert!(::core::mem::align_of::<crate::foo::Bar>() == 4);
    static_assertions::assert_impl_all!(crate::foo::Bar: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::Bar: Drop);
    assert!(::core::mem::offset_of!(crate::foo::Bar, bar) == 0);
    assert!(::core::mem::size_of::<crate::foo::bar::Baz>() == 4);
    assert!(::core::mem::align_of::<crate::foo::bar::Baz>() == 4);
    static_assertions::assert_impl_all!(crate::foo::bar::Baz: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::foo::bar::Baz: Drop);
    assert!(::core::mem::offset_of!(crate::foo::bar::Baz, baz) == 0);
};
