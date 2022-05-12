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

use ::std as rust_std;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
pub struct NontrivialCustomType {
    pub i: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialCustomType"),
    crate::NontrivialCustomType
);

impl !Unpin for NontrivialCustomType {}

impl<'b> ctor::CtorNew<ctor::RvalueReference<'b, crate::NontrivialCustomType>>
    for NontrivialCustomType
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ctor::RvalueReference<'b, crate::NontrivialCustomType>) -> Self::CtorType {
        let __param_0 = args;
        ctor::FnCtor::new(
            move |dest: crate::rust_std::pin::Pin<&mut crate::rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN20NontrivialCustomTypeC1EOS_(
                    crate::rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}
impl<'b> ctor::CtorNew<(ctor::RvalueReference<'b, crate::NontrivialCustomType>,)>
    for NontrivialCustomType
{
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (ctor::RvalueReference<'b, crate::NontrivialCustomType>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ctor::CtorNew<ctor::RvalueReference<'b, crate::NontrivialCustomType>>>::ctor_new(
            arg,
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
    __non_field_data: [crate::rust_std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ContainingStruct"),
    crate::ContainingStruct
);

impl Default for ContainingStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16ContainingStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<ctor::RvalueReference<'b, crate::ContainingStruct>> for ContainingStruct {
    #[inline(always)]
    fn from(__param_0: ctor::RvalueReference<'b, crate::ContainingStruct>) -> Self {
        let mut tmp = crate::rust_std::mem::MaybeUninit::<Self>::zeroed();
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

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN20NontrivialCustomTypeC1EOS_"]
        pub(crate) fn __rust_thunk___ZN20NontrivialCustomTypeC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::NontrivialCustomType>,
            __param_0: ctor::RvalueReference<'b, crate::NontrivialCustomType>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1Ev<'a>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::ContainingStruct>,
        );
        pub(crate) fn __rust_thunk___ZN16ContainingStructC1EOS_<'a, 'b>(
            __this: &'a mut crate::rust_std::mem::MaybeUninit<crate::ContainingStruct>,
            __param_0: ctor::RvalueReference<'b, crate::ContainingStruct>,
        );
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::NontrivialCustomType>() == 4usize);
const _: () = assert!(rust_std::mem::align_of::<crate::NontrivialCustomType>() == 4usize);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NontrivialCustomType: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::NontrivialCustomType: Drop);
};
const _: () =
    assert!(memoffset_unstable_const::offset_of!(crate::NontrivialCustomType, i) * 8 == 0usize);

const _: () = assert!(rust_std::mem::size_of::<crate::ContainingStruct>() == 1usize);
const _: () = assert!(rust_std::mem::align_of::<crate::ContainingStruct>() == 1usize);
const _: () = {
    static_assertions::assert_impl_all!(crate::ContainingStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ContainingStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::ContainingStruct: Drop);
};
