// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsupported_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(stable_features)]
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TrivialCustomType {
    pub i: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TrivialCustomType"),
    crate::TrivialCustomType
);

impl Default for TrivialCustomType {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17TrivialCustomTypeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for TrivialCustomType {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17TrivialCustomTypeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for TrivialCustomType {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN17TrivialCustomTypeaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TrivialCustomType {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN17TrivialCustomTypeaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for item 'TrivialCustomType::operator||':
// Bindings for this kind of operator (operator || with 2 parameter(s)) are not supported

// Error while generating bindings for item 'TrivialCustomType::operator int':
// Function name is not supported: Unsupported name: operator int

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct NontrivialCustomType {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub i: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialCustomType"),
    crate::NontrivialCustomType
);

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialCustomType {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN20NontrivialCustomTypeC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialCustomType {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

// Error while generating bindings for item 'NontrivialCustomType::operator||':
// Bindings for this kind of operator (operator || with 2 parameter(s)) are not supported

// Error while generating bindings for item 'PackedLayout':
// Records with packed layout are not supported

// Error while generating bindings for item 'MultipleReasons':
// Parameter #0 is not supported: Unsupported type 'volatile int *': Unsupported `volatile` qualifier: volatile int
//
// Return type is not supported: Unsupported type 'volatile int *': Unsupported `volatile` qualifier: volatile int

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContainingStruct {
    /// Doc comment for an unsupported field.
    ///
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'struct ContainingStruct::NestedStruct': No generated bindings found for 'NestedStruct'
    pub(crate) nested_struct: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ContainingStruct"),
    crate::ContainingStruct
);

impl Default for ContainingStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for ContainingStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for ContainingStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for ContainingStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for item 'ContainingStruct::NestedStruct':
// Nested classes are not supported yet

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN17TrivialCustomTypeC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TrivialCustomType>,
        );
        pub(crate) fn __rust_thunk___ZN17TrivialCustomTypeC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::TrivialCustomType>,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialCustomType>,
        );
        pub(crate) fn __rust_thunk___ZN17TrivialCustomTypeaSERKS_<'a, 'b>(
            __this: &'a mut crate::TrivialCustomType,
            __param_0: &'b crate::TrivialCustomType,
        ) -> &'a mut crate::TrivialCustomType;
        pub(crate) fn __rust_thunk___ZN17TrivialCustomTypeaSEOS_<'a, 'b>(
            __this: &'a mut crate::TrivialCustomType,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialCustomType>,
        ) -> &'a mut crate::TrivialCustomType;
        #[link_name = "_ZN20NontrivialCustomTypeC1EOS_"]
        pub(crate) fn __rust_thunk___ZN20NontrivialCustomTypeC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialCustomType>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialCustomType>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ContainingStruct>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ContainingStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::ContainingStruct>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::ContainingStruct,
            __param_0: &'b crate::ContainingStruct,
        ) -> &'a mut crate::ContainingStruct;
        pub(crate) fn __rust_thunk___ZN16ContainingStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::ContainingStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::ContainingStruct>,
        ) -> &'a mut crate::ContainingStruct;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::TrivialCustomType>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::TrivialCustomType>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::TrivialCustomType: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TrivialCustomType: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TrivialCustomType: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::TrivialCustomType, i) == 0);

const _: () = assert!(::core::mem::size_of::<crate::NontrivialCustomType>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::NontrivialCustomType>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialCustomType: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialCustomType: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::NontrivialCustomType, i) == 0);

const _: () = assert!(::core::mem::size_of::<crate::ContainingStruct>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::ContainingStruct>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::ContainingStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ContainingStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ContainingStruct: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ContainingStruct, nested_struct) == 0);
