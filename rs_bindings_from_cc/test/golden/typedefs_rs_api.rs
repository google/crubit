// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:typedefs_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct SomeStruct {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

impl ::ctor::CtorNew<()> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10SomeStructC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

impl<'b> ::ctor::CtorNew<&'b crate::SomeStruct> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b crate::SomeStruct) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10SomeStructC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(&'b crate::SomeStruct,)> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b crate::SomeStruct,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b crate::SomeStruct>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::SomeStruct>> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::SomeStruct>) -> Self::CtorType {
        let __param_0 = args;
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN10SomeStructC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::SomeStruct>,)> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, crate::SomeStruct>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::SomeStruct>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b crate::SomeStruct> for SomeStruct {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b crate::SomeStruct) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, crate::SomeStruct>> for SomeStruct {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSEOS_(self, __param_0);
        }
    }
}

// rs_bindings_from_cc/test/golden/typedefs.h;l=11
// Error while generating bindings for item 'SomeStruct':
// Typedef only used to introduce a name in C. Not importing.

// rs_bindings_from_cc/test/golden/typedefs.h;l=13
// Error while generating bindings for item 'SomeOtherStruct':
// Unsupported type 'struct SomeOtherStruct': No generated bindings found for ''

#[derive(Clone, Copy)]
#[repr(C)]
pub union SomeUnion {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeUnion"), crate::SomeUnion);

impl Default for SomeUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::SomeUnion>> for SomeUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::SomeUnion>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/typedefs.h;l=16
// Error while generating bindings for item 'SomeUnion::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/typedefs.h;l=16
// Error while generating bindings for item 'SomeUnion::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/typedefs.h;l=17
// Error while generating bindings for item 'SomeUnion':
// Typedef only used to introduce a name in C. Not importing.

// rs_bindings_from_cc/test/golden/typedefs.h;l=19
// Error while generating bindings for item 'SomeOtherUnion':
// Unsupported type 'union SomeOtherUnion': No generated bindings found for ''

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEFS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10SomeStructC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SomeStruct>,
            __param_0: &'b crate::SomeStruct,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SomeStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::SomeStruct>,
            __param_0: &'b crate::SomeStruct,
        ) -> ::std::pin::Pin<&'a mut crate::SomeStruct>;
        pub(crate) fn __rust_thunk___ZN10SomeStructaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::SomeStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        ) -> ::std::pin::Pin<&'a mut crate::SomeStruct>;
        pub(crate) fn __rust_thunk___ZN9SomeUnionC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SomeUnion>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::SomeUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeUnion>,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::SomeStruct>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::SomeStruct>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::SomeUnion>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::SomeUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeUnion: Drop);
};
