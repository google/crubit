// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:view_owner

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
        // CRUBIT_VIEW and CRUBIT_OWNER do not yet change the bindings (see
        // annotations.h). This golden records today's behaviour; `view_owner_unsafe`
        // shows the one effect, under the `unsafe_view` feature.

        // The pointers are private, so that a public pointer field does not make the
        // records unsafe on its own.

        /// A view: borrows what `data_` points to.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test4ViewE"]
        #[repr(C, align(8))]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: View")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct View {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) data_: [::core::mem::MaybeUninit<u8>; 8],
        }
        /// Cannot infer `Send` because:
        /// * field `data_` is represented as opaque bytes
        impl !Send for View {}
        /// Cannot infer `Sync` because:
        /// * field `data_` is represented as opaque bytes
        impl !Sync for View {}
        unsafe impl ::cxx::ExternType for View {
            type Id = ::cxx::type_id!("crubit :: test :: View");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: View"),
            crate::crubit::test::View
        );
        impl View {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[inline(always)]
            pub unsafe fn Get(__this: *const Self) -> ::ffi_11::c_int {
                unsafe { self::view::Get(__this) }
            }
        }

        impl Default for View {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test4ViewC1Ev(&raw mut tmp as *mut _);
                    tmp.assume_init()
                }
            }
        }

        pub mod view {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: do_not_bind=")]
            #[inline(always)]
            pub unsafe fn Get(__this: *const crate::crubit::test::View) -> ::ffi_11::c_int {
                unsafe { crate::detail::__rust_thunk___ZNK6crubit4test4View3GetEv(__this) }
            }
        }

        /// An owner: owns what `data_` points to.
        ///
        /// Not Rust-movable: type is neither copyable nor movable
        #[::ctor::recursively_pinned(PinnedDrop)]
        #[cfi_encoding = "N6crubit4test5OwnerE"]
        #[repr(C, align(8))]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Owner")]
        pub struct Owner {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) data_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
        }
        /// Cannot infer `Send` because:
        /// * field `data_` is represented as opaque bytes
        impl !Send for Owner {}
        /// Cannot infer `Sync` because:
        /// * field `data_` is represented as opaque bytes
        impl !Sync for Owner {}
        unsafe impl ::cxx::ExternType for Owner {
            type Id = ::cxx::type_id!("crubit :: test :: Owner");
            type Kind = ::cxx::kind::Opaque;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: Owner"),
            crate::crubit::test::Owner
        );

        impl ::ctor::CtorNew<()> for Owner {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        let () = args;
                        crate::detail::__rust_thunk___ZN6crubit4test5OwnerC1Ev(
                            __crubit_dest as *mut ::core::ffi::c_void,
                        );
                    })
                }
            }
        }

        impl ::ctor::PinnedDrop for Owner {
            #[inline(always)]
            unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
                unsafe { crate::detail::__rust_thunk___ZN6crubit4test5OwnerD1Ev(self) }
            }
        }

        /// Neither annotation, for comparison.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test5PlainE"]
        #[repr(C, align(8))]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Plain")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct Plain {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) data_: [::core::mem::MaybeUninit<u8>; 8],
        }
        /// Cannot infer `Send` because:
        /// * field `data_` is represented as opaque bytes
        impl !Send for Plain {}
        /// Cannot infer `Sync` because:
        /// * field `data_` is represented as opaque bytes
        impl !Sync for Plain {}
        unsafe impl ::cxx::ExternType for Plain {
            type Id = ::cxx::type_id!("crubit :: test :: Plain");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: Plain"),
            crate::crubit::test::Plain
        );
        impl Plain {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[inline(always)]
            pub unsafe fn Get(__this: *const Self) -> ::ffi_11::c_int {
                unsafe { self::plain::Get(__this) }
            }
        }

        impl Default for Plain {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test5PlainC1Ev(&raw mut tmp as *mut _);
                    tmp.assume_init()
                }
            }
        }

        pub mod plain {
            /// # Safety
            ///
            /// The caller must ensure that the following unsafe arguments are not misused by the function:
            /// * `__this`: raw pointer
            #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: do_not_bind=")]
            #[inline(always)]
            pub unsafe fn Get(__this: *const crate::crubit::test::Plain) -> ::ffi_11::c_int {
                unsafe { crate::detail::__rust_thunk___ZNK6crubit4test5Plain3GetEv(__this) }
            }
        }

        #[inline(always)]
        pub fn ReadView(v: crate::crubit::test::View) -> ::ffi_11::c_int {
            let mut v = ::core::mem::MaybeUninit::new(v);
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test8ReadViewENS0_4ViewE(v.as_mut_ptr())
            }
        }

        #[inline(always)]
        pub fn ReadPlain(p: crate::crubit::test::Plain) -> ::ffi_11::c_int {
            let mut p = ::core::mem::MaybeUninit::new(p);
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test9ReadPlainENS0_5PlainE(p.as_mut_ptr())
            }
        }
    }
}

// namespace crubit::test

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test4ViewC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test4View3GetEv(
            __this: *const crate::crubit::test::View,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test5OwnerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test5OwnerD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::crubit::test::Owner>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test5PlainC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK6crubit4test5Plain3GetEv(
            __this: *const crate::crubit::test::Plain,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test8ReadViewENS0_4ViewE(
            v: *mut crate::crubit::test::View,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test9ReadPlainENS0_5PlainE(
            p: *mut crate::crubit::test::Plain,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::View>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::View>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::View: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::View: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::View, data_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::Owner>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::Owner>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::Owner: Drop);
    static_assertions::assert_not_impl_any!(crate::crubit::test::Owner: Copy);
    assert!(::core::mem::offset_of!(crate::crubit::test::Owner, data_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::Plain>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::Plain>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::Plain: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::Plain: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::Plain, data_) == 0);
};
