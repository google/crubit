// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:polymorphic_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std as rust_std;
use memoffset_unstable_const::offset_of;
use static_assertions::{assert_impl_all, assert_not_impl_all};

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[repr(C)]
pub struct PolymorphicClass {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: rust_std::mem::MaybeUninit<u8>,
}

impl !Unpin for PolymorphicClass {}

// rs_bindings_from_cc/test/golden/polymorphic.h;l=10
// Error while generating bindings for item 'PolymorphicClass::PolymorphicClass':
// Only single-parameter constructors for T: !Unpin are supported for now

impl<'b> ctor::CtorNew<&'b PolymorphicClass> for PolymorphicClass {
    type CtorType = impl ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(__param_0: &'b PolymorphicClass) -> Self::CtorType {
        ctor::FnCtor::new(
            move |dest: rust_std::pin::Pin<&mut rust_std::mem::MaybeUninit<Self>>| unsafe {
                crate::detail::__rust_thunk___ZN16PolymorphicClassC1ERKS_(
                    rust_std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            },
        )
    }
}

// rs_bindings_from_cc/test/golden/polymorphic.h;l=10
// Error while generating bindings for item 'PolymorphicClass::operator=':
// Bindings for this kind of operator are not supported

impl Drop for PolymorphicClass {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN16PolymorphicClassD1Ev(self) }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_POLYMORPHIC_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN16PolymorphicClassC1ERKS_<'a, 'b>(
            __this: &'a mut rust_std::mem::MaybeUninit<PolymorphicClass>,
            __param_0: &'b PolymorphicClass,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicClassD1Ev<'a>(__this: *mut PolymorphicClass);
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<PolymorphicClass>() == 8usize);
const _: () = assert!(rust_std::mem::align_of::<PolymorphicClass>() == 8usize);
const _: () = {
    assert_not_impl_all!(PolymorphicClass: Copy);
};
const _: () = {
    assert_impl_all!(PolymorphicClass: Drop);
};
