// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:template_inst_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// This library reproduces the issue in b/458678348.
#[inline(always)]
pub(crate) fn GetMyTemplate() -> crate::__CcTemplateInst10MyTemplateIiE {
    unsafe {
        let mut __return =
            ::core::mem::MaybeUninit::<crate::__CcTemplateInst10MyTemplateIiE>::uninit();
        crate::detail::__rust_thunk___Z13GetMyTemplatev(
            &raw mut __return as *mut ::core::ffi::c_void,
        );
        __return.assume_init()
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyTemplate < int >
pub(crate) struct __CcTemplateInst10MyTemplateIiE {
    pub field: ::core::ffi::c_int,
}
impl !Send for __CcTemplateInst10MyTemplateIiE {}
impl !Sync for __CcTemplateInst10MyTemplateIiE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate < int >"),
    crate::__CcTemplateInst10MyTemplateIiE
);

// Error while generating bindings for constructor 'MyTemplate<int>::MyTemplate<int>':
// Can't generate bindings for MyTemplate<int>::MyTemplate<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:template_inst_cc needs [//features:experimental] for MyTemplate<int>::MyTemplate<int> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for constructor 'MyTemplate<int>::MyTemplate<int>':
// Can't generate bindings for MyTemplate<int>::MyTemplate<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:template_inst_cc needs [//features:experimental] for MyTemplate<int>::MyTemplate<int> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for constructor 'MyTemplate<int>::MyTemplate<int>':
// Can't generate bindings for MyTemplate<int>::MyTemplate<int>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:template_inst_cc needs [//features:experimental] for MyTemplate<int>::MyTemplate<int> (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for function 'MyTemplate<int>::operator=':
// Can't generate bindings for MyTemplate<int>::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:template_inst_cc needs [//features:experimental] for MyTemplate<int>::operator= (b/248542210: template instantiation of member function cannot reliably get bindings)

// Error while generating bindings for function 'MyTemplate<int>::operator=':
// Can't generate bindings for MyTemplate<int>::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:template_inst_cc needs [//features:experimental] for MyTemplate<int>::operator= (b/248542210: template instantiation of member function cannot reliably get bindings)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z13GetMyTemplatev(__return: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, field) == 0);
};
