// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:typedef_incomplete_types_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
forward_declare::forward_declare!(pub IncompleteExternC = forward_declare::symbol!("IncompleteExternC"));

forward_declare::forward_declare!(pub Incomplete = forward_declare::symbol!("Incomplete"));

/// # Safety
///
/// To call a function that accepts this type, you must uphold these requirements:
/// * Document why the following public unsafe fields of this type cannot be misused by callee:
///   * `incomplete_extern_c`: raw pointer
///   * `incomplete`: raw pointer
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "30HasPointerToIncompleteTypedefs"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=HasPointerToIncompleteTypedefs
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct HasPointerToIncompleteTypedefs {
    pub incomplete_extern_c: *mut crate::IncompleteExternC,
    pub incomplete: *mut crate::Incomplete,
}
impl !Send for HasPointerToIncompleteTypedefs {}
impl !Sync for HasPointerToIncompleteTypedefs {}
unsafe impl ::cxx::ExternType for HasPointerToIncompleteTypedefs {
    type Id = ::cxx::type_id!("HasPointerToIncompleteTypedefs");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("HasPointerToIncompleteTypedefs"),
    crate::HasPointerToIncompleteTypedefs
);

impl Default for HasPointerToIncompleteTypedefs {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30HasPointerToIncompleteTypedefsC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN30HasPointerToIncompleteTypedefsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::HasPointerToIncompleteTypedefs>() == 16);
    assert!(::core::mem::align_of::<crate::HasPointerToIncompleteTypedefs>() == 8);
    static_assertions::assert_impl_all!(crate::HasPointerToIncompleteTypedefs: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::HasPointerToIncompleteTypedefs: Drop);
    assert!(
        ::core::mem::offset_of!(crate::HasPointerToIncompleteTypedefs, incomplete_extern_c) == 0
    );
    assert!(::core::mem::offset_of!(crate::HasPointerToIncompleteTypedefs, incomplete) == 8);
};
