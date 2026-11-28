// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:view_owner_unsafe

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod crubit {
    pub mod test {
        /// Built with the `unsafe_view` feature: the CRUBIT_VIEW record is unsafe, the
        /// CRUBIT_OWNER record is not.
        ///
        /// # Safety
        ///
        /// To call a function that accepts this type, you must uphold these requirements:
        /// * The C++ type is explicitly annotated as unsafe. Ensure that its safety requirements are upheld.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test10UnsafeViewE"]
        #[repr(C, align(8))]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: UnsafeView")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct UnsafeView {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) data_: [::core::mem::MaybeUninit<u8>; 8],
        }
        /// Cannot infer `Send` because:
        /// * field `data_` is represented as opaque bytes
        impl !Send for UnsafeView {}
        /// Cannot infer `Sync` because:
        /// * field `data_` is represented as opaque bytes
        impl !Sync for UnsafeView {}
        unsafe impl ::cxx::ExternType for UnsafeView {
            type Id = ::cxx::type_id!("crubit :: test :: UnsafeView");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: UnsafeView"),
            crate::crubit::test::UnsafeView
        );
        impl UnsafeView {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[inline(always)]
            pub unsafe fn Get(__this: *const Self) -> ::ffi_11::c_int {
                unsafe { self::unsafe_view::Get(__this) }
            }
        }

        impl Default for UnsafeView {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test10UnsafeViewC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        pub mod unsafe_view {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: do_not_bind=")]
            #[inline(always)]
            pub unsafe fn Get(__this: *const crate::crubit::test::UnsafeView) -> ::ffi_11::c_int {
                unsafe { crate::detail::__rust_thunk___ZNK6crubit4test10UnsafeView3GetEv(__this) }
            }
        }

        /// Not Rust-movable: type is neither copyable nor movable
        #[::ctor::recursively_pinned(PinnedDrop)]
        #[cfi_encoding = "N6crubit4test9SafeOwnerE"]
        #[repr(C, align(8))]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: SafeOwner")]
        pub struct SafeOwner {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) data_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
        }
        /// Cannot infer `Send` because:
        /// * field `data_` is represented as opaque bytes
        impl !Send for SafeOwner {}
        /// Cannot infer `Sync` because:
        /// * field `data_` is represented as opaque bytes
        impl !Sync for SafeOwner {}
        unsafe impl ::cxx::ExternType for SafeOwner {
            type Id = ::cxx::type_id!("crubit :: test :: SafeOwner");
            type Kind = ::cxx::kind::Opaque;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: SafeOwner"),
            crate::crubit::test::SafeOwner
        );

        impl ::ctor::CtorNew<()> for SafeOwner {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        let () = args;
                        crate::detail::__rust_thunk___ZN6crubit4test9SafeOwnerC1Ev(
                            __crubit_dest as *mut ::core::ffi::c_void,
                        );
                    })
                }
            }
        }

        impl ::ctor::PinnedDrop for SafeOwner {
            #[inline(always)]
            unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                unsafe { crate::detail::__rust_thunk___ZN6crubit4test9SafeOwnerD1Ev(self) }
            }
        }

        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `v`: unsafe struct or union
        #[inline(always)]
        pub unsafe fn ReadUnsafeView(v: crate::crubit::test::UnsafeView) -> ::ffi_11::c_int {
            let mut v = ::core::mem::MaybeUninit::new(v);
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test14ReadUnsafeViewENS0_10UnsafeViewE(
                    v.as_mut_ptr(),
                )
            }
        }
    }
}

// namespace crubit::test

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test10UnsafeViewC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test10UnsafeView3GetEv(
            __this: *const crate::crubit::test::UnsafeView,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test9SafeOwnerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test9SafeOwnerD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::crubit::test::SafeOwner>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test14ReadUnsafeViewENS0_10UnsafeViewE(
            v: *mut crate::crubit::test::UnsafeView,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::UnsafeView>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::UnsafeView>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::UnsafeView: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::UnsafeView: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::UnsafeView, data_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::SafeOwner>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::SafeOwner>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::SafeOwner: Drop);
    static_assertions::assert_not_impl_any!(crate::crubit::test::SafeOwner: Copy);
    assert!(::core::mem::offset_of!(crate::crubit::test::SafeOwner, data_) == 0);
};
