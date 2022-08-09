// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:unsupported_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
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
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17TrivialCustomTypeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::TrivialCustomType>> for TrivialCustomType {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::TrivialCustomType>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN17TrivialCustomTypeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=10
// Error while generating bindings for item 'TrivialCustomType::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/unsupported.h;l=10
// Error while generating bindings for item 'TrivialCustomType::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/unsupported.h;l=13
// Error while generating bindings for item 'TrivialCustomType::operator||':
// Bindings for this kind of operator (operator ||) are not supported

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct NontrivialCustomType {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    pub i: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialCustomType"),
    crate::NontrivialCustomType
);

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, crate::NontrivialCustomType>>
    for NontrivialCustomType
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, crate::NontrivialCustomType>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::NontrivialCustomType>,
                >| {
                    crate::detail::__rust_thunk___ZN20NontrivialCustomTypeC1EOS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, crate::NontrivialCustomType>,)>
    for NontrivialCustomType
{
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(
        args: (::ctor::RvalueReference<'b, crate::NontrivialCustomType>,),
    ) -> Self::CtorType {
        let (arg,) = args;
        <Self as::ctor::CtorNew<::ctor::RvalueReference<'b,crate::NontrivialCustomType>>>::ctor_new(arg)
    }
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=22
// Error while generating bindings for item 'NontrivialCustomType::operator||':
// Bindings for this kind of operator (operator ||) are not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=33
// Error while generating bindings for item 'MultipleReasons':
// Parameter #0 is not supported: Unsupported type 'volatile int *': Unsupported `volatile` qualifier: volatile int
//
// Return type is not supported: Unsupported type 'volatile int *': Unsupported `volatile` qualifier: volatile int

// rs_bindings_from_cc/test/golden/unsupported.h;l=35
// Error while generating bindings for item 'ns':
// Namespaces are not supported yet

// namespace ns

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContainingStruct {
    /// Doc comment for an unsupported field.
    ///
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'struct ContainingStruct::NestedStruct': No generated bindings found for 'NestedStruct'
    pub(crate) nested_struct: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ContainingStruct"),
    crate::ContainingStruct
);

impl Default for ContainingStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::ContainingStruct>> for ContainingStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::ContainingStruct>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=43
// Error while generating bindings for item 'ContainingStruct::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/unsupported.h;l=43
// Error while generating bindings for item 'ContainingStruct::operator=':
// operator= for Unpin types is not yet supported.

// rs_bindings_from_cc/test/golden/unsupported.h;l=44
// Error while generating bindings for item 'ContainingStruct::NestedStruct':
// Nested classes are not supported yet

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN17TrivialCustomTypeC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TrivialCustomType>,
        );
        pub(crate) fn __rust_thunk___ZN17TrivialCustomTypeC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::TrivialCustomType>,
            __param_0: ::ctor::RvalueReference<'b, crate::TrivialCustomType>,
        );
        #[link_name = "_ZN20NontrivialCustomTypeC1EOS_"]
        pub(crate) fn __rust_thunk___ZN20NontrivialCustomTypeC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialCustomType>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialCustomType>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ContainingStruct>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ContainingStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::ContainingStruct>,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::TrivialCustomType>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::TrivialCustomType>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::TrivialCustomType: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TrivialCustomType: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TrivialCustomType: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::TrivialCustomType, i) == 0);

const _: () = assert!(::std::mem::size_of::<crate::NontrivialCustomType>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialCustomType>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialCustomType: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialCustomType: Drop);
};
const _: () = assert!(memoffset_unstable_const::offset_of!(crate::NontrivialCustomType, i) == 0);

const _: () = assert!(::std::mem::size_of::<crate::ContainingStruct>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::ContainingStruct>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::ContainingStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ContainingStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ContainingStruct: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::ContainingStruct, nested_struct) == 0);
