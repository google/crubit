// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:types_in_extra_cpp_srcs_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
/// TODO(b/497927944): This golden test demonstrates that when a type
/// (like MyStruct) is defined inside an extra `cpp_srcs` file instead of a
/// normal header, Crubit successfully generates the forward Rust binding
/// (types_in_extra_cpp_srcs_rs_api.rs). However, what's wrong with this is
/// that any Rust APIs using this type will NOT get bindings generated back
/// into C++. This is because Crubit's reverse bindings generator
/// (`cc_bindings_from_rs`) does not know what C++ header to `#include` to
/// refer to `MyStruct` (as it only exists in a `.cc` file in the AST).
///
/// What we would like it to do is to find a way to map this type back to a
/// C++ target, or generate a forward declaration, so that reverse bindings can
/// be properly generated and used from C++.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "8MyStruct"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=MyStruct
pub struct MyStruct {
    pub a: ::ffi_11::c_int,
    pub b: ::ffi_11::c_int,
}
impl !Send for MyStruct {}
impl !Sync for MyStruct {}
unsafe impl ::cxx::ExternType for MyStruct {
    type Id = ::cxx::type_id!("MyStruct");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for MyStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8MyStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn MyStructAdder(mut x: crate::MyStruct) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z13MyStructAdder8MyStruct(&mut x) }
}

#[path = "rs_bindings_from_cc/test/golden/types_in_extra_cpp_srcs_extra.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8MyStructC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z13MyStructAdder8MyStruct(
            x: &mut crate::MyStruct,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::MyStruct>() == 8);
    assert!(::core::mem::align_of::<crate::MyStruct>() == 4);
    static_assertions::assert_impl_all!(crate::MyStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MyStruct: Drop);
    assert!(::core::mem::offset_of!(crate::MyStruct, a) == 0);
    assert!(::core::mem::offset_of!(crate::MyStruct, b) == 4);
};
