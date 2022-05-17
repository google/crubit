// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:templates_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

use ::std as rust_std;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// rs_bindings_from_cc/test/golden/templates.h;l=8
// Error while generating bindings for item 'MyTemplate':
// Class templates are not supported yet

pub type MyTypeAlias = crate::__CcTemplateInst10MyTemplateIiE;

pub type OtherTypeAliasInSameTarget = crate::__CcTemplateInst10MyTemplateIiE;

// rs_bindings_from_cc/test/golden/templates.h;l=26
// Error while generating bindings for item 'TemplateWithTwoParams':
// Class templates are not supported yet

pub type AliasToTemplateWithTwoParams = crate::__CcTemplateInst21TemplateWithTwoParamsIifE;

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_H_

#[ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateIiE {
    value_: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<int>"),
    crate::__CcTemplateInst10MyTemplateIiE
);

// rs_bindings_from_cc/test/golden/templates.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::MyTemplate<int>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/templates.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::MyTemplate<int>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/templates.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'class MyTemplate<int> &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/templates.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/templates.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::operator=':
// Parameter #0 is not supported: Unsupported type 'class MyTemplate<int> &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    pub fn Create(value: i32) -> crate::__CcTemplateInst10MyTemplateIiE {
        unsafe {
            crate::detail::__rust_thunk___ZN10MyTemplateIiE6CreateEi___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(value)
        }
    }
}

impl __CcTemplateInst10MyTemplateIiE {
    #[inline(always)]
    pub unsafe fn value(__this: *const crate::__CcTemplateInst10MyTemplateIiE) -> *const i32 {
        crate::detail::__rust_thunk___ZNK10MyTemplateIiE5valueEv___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(__this)
    }
}

#[ctor::recursively_pinned]
#[repr(C)]
pub struct __CcTemplateInst21TemplateWithTwoParamsIifE {
    pub value1: i32,
    pub value2: f32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TemplateWithTwoParams<int, float>"),
    crate::__CcTemplateInst21TemplateWithTwoParamsIifE
);

// rs_bindings_from_cc/test/golden/templates.h;l=27
// Error while generating bindings for item 'TemplateWithTwoParams<int, float>::TemplateWithTwoParams<int, float>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/templates.h;l=27
// Error while generating bindings for item 'TemplateWithTwoParams<int, float>::TemplateWithTwoParams<int, float>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// rs_bindings_from_cc/test/golden/templates.h;l=27
// Error while generating bindings for item 'TemplateWithTwoParams<int, float>::TemplateWithTwoParams':
// Parameter #0 is not supported: Unsupported type 'struct TemplateWithTwoParams<int, float> &&': Unsupported type: && without lifetime

// rs_bindings_from_cc/test/golden/templates.h;l=27
// Error while generating bindings for item 'TemplateWithTwoParams<int, float>::operator=':
// Bindings for this kind of operator are not supported

// rs_bindings_from_cc/test/golden/templates.h;l=27
// Error while generating bindings for item 'TemplateWithTwoParams<int, float>::operator=':
// Parameter #0 is not supported: Unsupported type 'struct TemplateWithTwoParams<int, float> &&': Unsupported type: && without lifetime

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10MyTemplateIiE6CreateEi___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
            value: i32,
        ) -> crate::__CcTemplateInst10MyTemplateIiE;
        pub(crate) fn __rust_thunk___ZNK10MyTemplateIiE5valueEv___third_party_crubit_rs_bindings_from_cc_test_golden_templates_cc(
            __this: *const crate::__CcTemplateInst10MyTemplateIiE,
        ) -> *const i32;
    }
}

const _: () = assert!(rust_std::mem::size_of::<Option<&i32>>() == rust_std::mem::size_of::<&i32>());

const _: () = assert!(rust_std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
const _: () = assert!(rust_std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, value_) * 8 == 0
);

const _: () =
    assert!(rust_std::mem::size_of::<crate::__CcTemplateInst21TemplateWithTwoParamsIifE>() == 8);
const _: () =
    assert!(rust_std::mem::align_of::<crate::__CcTemplateInst21TemplateWithTwoParamsIifE>() == 4);
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInst21TemplateWithTwoParamsIifE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_all!(
        crate::__CcTemplateInst21TemplateWithTwoParamsIifE: Drop
    );
};
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
        value1
    ) * 8
        == 0
);
const _: () = assert!(
    memoffset_unstable_const::offset_of!(
        crate::__CcTemplateInst21TemplateWithTwoParamsIifE,
        value2
    ) * 8
        == 32
);
