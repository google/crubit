// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// error: class `DoesNotUse` could not be bound
//   Class templates are not yet supported

// `DoesNotUse<DoesNotUse<int>>` does not instantiate the inner template
// parameter, `DoesNotUse<int>`, but it _is_ instantiatable. This test
// shows that we should not import it in its uninstantiated form, otherwise
// ImportedSecond will read this cached import and think that `DoesNotUse<int>`
// is incomplete, which is false.

/// Doc comment
#[inline(always)]
pub fn ImportedFirst(mut __param_0: crate::__CcTemplateInst10DoesNotUseIS_IiEE) {
    unsafe { crate::detail::__rust_thunk___Z13ImportedFirst10DoesNotUseIS_IiEE(&mut __param_0) }
}

// We expect ImportedSecond to fail because we need wrapper mode, _not_ because
// `DoesNotUse<int>` is incomplete.

/// Doc comment
#[inline(always)]
pub fn ImportedSecond(mut __param_0: crate::__CcTemplateInst10DoesNotUseIiE) {
    unsafe { crate::detail::__rust_thunk___Z14ImportedSecond10DoesNotUseIiE(&mut __param_0) }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10DoesNotUseIS_IiEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DoesNotUse < DoesNotUse < int >>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst10DoesNotUseIS_IiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10DoesNotUseIS_IiEE {}
impl !Sync for __CcTemplateInst10DoesNotUseIS_IiEE {}

impl Default for __CcTemplateInst10DoesNotUseIS_IiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__2e59fe08__ZN10DoesNotUseIS_IiEEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInst10DoesNotUseIiE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DoesNotUse < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInst10DoesNotUseIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInst10DoesNotUseIiE {}
impl !Sync for __CcTemplateInst10DoesNotUseIiE {}

impl Default for __CcTemplateInst10DoesNotUseIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__2e59fe08__ZN10DoesNotUseIiEC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z13ImportedFirst10DoesNotUseIS_IiEE(
            __param_0: &mut crate::__CcTemplateInst10DoesNotUseIS_IiEE,
        );
        pub(crate) unsafe fn __rust_thunk___Z14ImportedSecond10DoesNotUseIiE(
            __param_0: &mut crate::__CcTemplateInst10DoesNotUseIiE,
        );
        pub(crate) unsafe fn __rust_thunk__2e59fe08__ZN10DoesNotUseIS_IiEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__2e59fe08__ZN10DoesNotUseIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::__CcTemplateInst10DoesNotUseIS_IiEE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10DoesNotUseIS_IiEE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10DoesNotUseIS_IiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10DoesNotUseIS_IiEE: Drop);

    assert!(::core::mem::size_of::<crate::__CcTemplateInst10DoesNotUseIiE>() == 1);
    assert!(::core::mem::align_of::<crate::__CcTemplateInst10DoesNotUseIiE>() == 1);
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10DoesNotUseIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10DoesNotUseIiE: Drop);
};
