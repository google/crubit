// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:unsupported_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
pub struct NontrivialCustomType {
    pub i: i32,
}

impl !Unpin for NontrivialCustomType {}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, NontrivialCustomType>> for NontrivialCustomType {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(__param_0: ctor::RvalueReference<'b, NontrivialCustomType>) -> Self::CtorType {
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN20NontrivialCustomTypeC1EOS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=16
// Error while generating bindings for item 'UnsupportedParamType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as parameter #0

// rs_bindings_from_cc/test/golden/unsupported.h;l=17
// Error while generating bindings for item 'UnsupportedReturnType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a return type

// rs_bindings_from_cc/test/golden/unsupported.h;l=19
// Error while generating bindings for item 'MultipleReasons':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a return type
//
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as parameter #0
//
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as parameter #2

// rs_bindings_from_cc/test/golden/unsupported.h;l=22
// Error while generating bindings for item 'ns':
// Namespaces are not supported yet

// namespace ns

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContainingStruct {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: rust_std::mem::MaybeUninit<u8>,
}

impl Default for ContainingStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, ContainingStruct>> for ContainingStruct {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, ContainingStruct>) -> Self {
        let mut tmp = rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=30
// Error while generating bindings for item 'ContainingStruct::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=30
// Error while generating bindings for item 'ContainingStruct::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=31
// Error while generating bindings for item 'ContainingStruct::NestedStruct':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/unsupported.h;l=32
// Error while generating bindings for item 'ContainingStruct::NestedStruct::NonStaticMemberFunction':
// Couldn't import the parent

// rs_bindings_from_cc/test/golden/unsupported.h;l=33
// Error while generating bindings for item 'ContainingStruct::NestedStruct::StaticMemberFunction':
// Couldn't import the parent

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN20NontrivialCustomTypeC1EOS_"]
        pub(crate) fn __rust_thunk___ZN20NontrivialCustomTypeC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<NontrivialCustomType>,
            __param_0: ctor::RvalueReference<'b, NontrivialCustomType>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1Ev<'a>(
            __this: &'a mut rust_std::mem::MaybeUninit<ContainingStruct>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1EOS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<ContainingStruct>,
            __param_0: ctor::RvalueReference<'b, ContainingStruct>,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<NontrivialCustomType>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<NontrivialCustomType>() == 4usize);
const _: () = {
    assert_not_impl_all!(NontrivialCustomType: Copy);
};
const _: () = {
    assert_not_impl_all!(NontrivialCustomType: Drop);
};
const _: () = assert!(offset_of!(NontrivialCustomType, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<ContainingStruct>() == 1usize);
const _: () = assert!(rust_std::mem::align_of::<ContainingStruct>() == 1usize);
const _: () = {
    assert_impl_all!(ContainingStruct: Clone);
};
const _: () = {
    assert_impl_all!(ContainingStruct: Copy);
};
const _: () = {
    assert_not_impl_all!(ContainingStruct: Drop);
};
