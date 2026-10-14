// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/send_sync_inference:send_sync_inference

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod crubit {
    pub mod test {
        /// Send + Sync: only plain data.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test5PlainE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Plain
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct Plain {
            pub x: ::ffi_11::c_int,
            pub y: f32,
        }
        unsafe impl ::cxx::ExternType for Plain {
            type Id = ::cxx::type_id!("crubit :: test :: Plain");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: Plain"),
            crate::crubit::test::Plain
        );

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

        /// Send + Sync: an empty class holds no data.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test5EmptyE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Empty
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct Empty {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        unsafe impl ::cxx::ExternType for Empty {
            type Id = ::cxx::type_id!("crubit :: test :: Empty");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: Empty"),
            crate::crubit::test::Empty
        );

        impl Default for Empty {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test5EmptyC1Ev(&raw mut tmp as *mut _);
                    tmp.assume_init()
                }
            }
        }

        /// Send + Sync: contains a Send + Sync record.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test13ContainsPlainE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ContainsPlain
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct ContainsPlain {
            pub plain: crate::crubit::test::Plain,
        }
        unsafe impl ::cxx::ExternType for ContainsPlain {
            type Id = ::cxx::type_id!("crubit :: test :: ContainsPlain");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: ContainsPlain"),
            crate::crubit::test::ContainsPlain
        );

        impl Default for ContainsPlain {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test13ContainsPlainC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Neither: raw pointers are neither Send nor Sync.
        ///
        /// # Safety
        ///
        /// To call a function that accepts this type, you must uphold these requirements:
        /// * Document why the following public unsafe fields of this type cannot be misused by callee:
        ///   * `p`: raw pointer
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test11WithPointerE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: WithPointer
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct WithPointer {
            pub p: *mut ::ffi_11::c_int,
        }
        unsafe impl ::cxx::ExternType for WithPointer {
            type Id = ::cxx::type_id!("crubit :: test :: WithPointer");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: WithPointer"),
            crate::crubit::test::WithPointer
        );

        impl Default for WithPointer {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test11WithPointerC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Neither: contains a record which is neither.
        ///
        /// # Safety
        ///
        /// To call a function that accepts this type, you must uphold these requirements:
        /// * Document why the following public unsafe fields of this type cannot be misused by callee:
        ///   * `with_pointer`: unsafe struct or union
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test19ContainsWithPointerE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ContainsWithPointer
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct ContainsWithPointer {
            pub with_pointer: crate::crubit::test::WithPointer,
        }
        unsafe impl ::cxx::ExternType for ContainsWithPointer {
            type Id = ::cxx::type_id!("crubit :: test :: ContainsWithPointer");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: ContainsWithPointer"),
            crate::crubit::test::ContainsWithPointer
        );

        impl Default for ContainsWithPointer {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test19ContainsWithPointerC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Neither: private fields are opaque bytes.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test16WithPrivateFieldE"]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: WithPrivateField
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct WithPrivateField {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            pub public_field_: ::ffi_11::c_int,
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) private_field_: [::core::mem::MaybeUninit<u8>; 4],
            __do_not_auto_impl_send_sync1: ::core::marker::PhantomData<*mut ()>,
        }
        unsafe impl ::cxx::ExternType for WithPrivateField {
            type Id = ::cxx::type_id!("crubit :: test :: WithPrivateField");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: WithPrivateField"),
            crate::crubit::test::WithPrivateField
        );

        impl Default for WithPrivateField {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test16WithPrivateFieldC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Neither: fields of unsupported types are opaque bytes.
        ///
        /// # Safety
        ///
        /// To call a function that accepts this type, you must uphold these requirements:
        /// * Document why the following public unsafe fields of this type cannot be misused by callee:
        ///   * `unsupported`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported `volatile` qualifier: volatile int
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test20WithUnsupportedFieldE"]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: WithUnsupportedField
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct WithUnsupportedField {
            /// Reason for representing this field as a blob of bytes:
            /// Unsupported `volatile` qualifier: volatile int
            pub(crate) unsupported: [::core::mem::MaybeUninit<u8>; 4],
            __do_not_auto_impl_send_sync0: ::core::marker::PhantomData<*mut ()>,
        }
        unsafe impl ::cxx::ExternType for WithUnsupportedField {
            type Id = ::cxx::type_id!("crubit :: test :: WithUnsupportedField");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: WithUnsupportedField"),
            crate::crubit::test::WithUnsupportedField
        );

        impl Default for WithUnsupportedField {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test20WithUnsupportedFieldC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test4BaseE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Base
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct Base {
            pub base_field: ::ffi_11::c_int,
        }
        unsafe impl ::cxx::ExternType for Base {
            type Id = ::cxx::type_id!("crubit :: test :: Base");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: Base"),
            crate::crubit::test::Base
        );

        impl Default for Base {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test4BaseC1Ev(&raw mut tmp as *mut _);
                    tmp.assume_init()
                }
            }
        }

        /// Neither: base class subobjects are opaque bytes.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test7DerivedE"]
        #[repr(C, align(4))]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Derived
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct Derived {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
            __non_field_data_do_not_auto_impl_send_sync: ::core::marker::PhantomData<*mut ()>,
            pub derived_field: ::ffi_11::c_int,
        }
        unsafe impl ::cxx::ExternType for Derived {
            type Id = ::cxx::type_id!("crubit :: test :: Derived");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: Derived"),
            crate::crubit::test::Derived
        );

        impl Default for Derived {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test7DerivedC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Sync, but explicitly not Send.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test7NotSendE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NotSend
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct NotSend {
            pub x: ::ffi_11::c_int,
        }
        impl !Send for NotSend {}
        unsafe impl ::cxx::ExternType for NotSend {
            type Id = ::cxx::type_id!("crubit :: test :: NotSend");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: NotSend"),
            crate::crubit::test::NotSend
        );

        impl Default for NotSend {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test7NotSendC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Send, but explicitly not Sync.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test7NotSyncE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NotSync
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct NotSync {
            pub x: ::ffi_11::c_int,
        }
        impl !Sync for NotSync {}
        unsafe impl ::cxx::ExternType for NotSync {
            type Id = ::cxx::type_id!("crubit :: test :: NotSync");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: NotSync"),
            crate::crubit::test::NotSync
        );

        impl Default for NotSync {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test7NotSyncC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Neither, explicitly.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test14NotSendNotSyncE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NotSendNotSync
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct NotSendNotSync {
            pub x: ::ffi_11::c_int,
        }
        impl !Send for NotSendNotSync {}
        impl !Sync for NotSendNotSync {}
        unsafe impl ::cxx::ExternType for NotSendNotSync {
            type Id = ::cxx::type_id!("crubit :: test :: NotSendNotSync");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: NotSendNotSync"),
            crate::crubit::test::NotSendNotSync
        );

        impl Default for NotSendNotSync {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test14NotSendNotSyncC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Send + Sync, explicitly, despite the pointer.
        ///
        /// # Safety
        ///
        /// To call a function that accepts this type, you must uphold these requirements:
        /// * Document why the following public unsafe fields of this type cannot be misused by callee:
        ///   * `p`: raw pointer
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test16ExplicitSendSyncE"]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ExplicitSendSync
        ///CRUBIT_ANNOTATE: cpp_move_constructible=
        pub struct ExplicitSendSync {
            pub p: *mut ::ffi_11::c_int,
        }
        unsafe impl Send for ExplicitSendSync {}
        unsafe impl Sync for ExplicitSendSync {}
        unsafe impl ::cxx::ExternType for ExplicitSendSync {
            type Id = ::cxx::type_id!("crubit :: test :: ExplicitSendSync");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: ExplicitSendSync"),
            crate::crubit::test::ExplicitSendSync
        );

        impl Default for ExplicitSendSync {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test16ExplicitSendSyncC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }
    }
}

// namespace crubit::test

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test5PlainC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test5EmptyC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13ContainsPlainC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test11WithPointerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test19ContainsWithPointerC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16WithPrivateFieldC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test20WithUnsupportedFieldC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test4BaseC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test7DerivedC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test7NotSendC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test7NotSyncC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test14NotSendNotSyncC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test16ExplicitSendSyncC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::Plain>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::Plain>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::Plain: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::Plain: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::Plain, x) == 0);
    assert!(::core::mem::offset_of!(crate::crubit::test::Plain, y) == 4);
    assert!(::core::mem::size_of::<crate::crubit::test::Empty>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::Empty>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::Empty: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::Empty: Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::ContainsPlain>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainsPlain>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainsPlain: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainsPlain: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainsPlain, plain) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::WithPointer>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::WithPointer>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::WithPointer: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::WithPointer: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::WithPointer, p) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ContainsWithPointer>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::ContainsWithPointer>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::ContainsWithPointer: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ContainsWithPointer: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ContainsWithPointer, with_pointer) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::WithPrivateField>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::WithPrivateField>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::WithPrivateField: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::WithPrivateField: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::WithPrivateField, public_field_) == 0);
    assert!(::core::mem::offset_of!(crate::crubit::test::WithPrivateField, private_field_) == 4);
    assert!(::core::mem::size_of::<crate::crubit::test::WithUnsupportedField>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::WithUnsupportedField>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::WithUnsupportedField: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::WithUnsupportedField: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::WithUnsupportedField, unsupported) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::Base>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::Base>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::Base: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::Base: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::Base, base_field) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::Derived>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::Derived>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::Derived: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::Derived: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::Derived, derived_field) == 4);
    assert!(::core::mem::size_of::<crate::crubit::test::NotSend>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::NotSend>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::NotSend: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NotSend: Drop,Send);
    assert!(::core::mem::offset_of!(crate::crubit::test::NotSend, x) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::NotSync>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::NotSync>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::NotSync: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NotSync: Drop,Sync);
    assert!(::core::mem::offset_of!(crate::crubit::test::NotSync, x) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::NotSendNotSync>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::NotSendNotSync>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::NotSendNotSync: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NotSendNotSync: Drop,Send,Sync);
    assert!(::core::mem::offset_of!(crate::crubit::test::NotSendNotSync, x) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::ExplicitSendSync>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::ExplicitSendSync>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::ExplicitSendSync: Copy,Clone,Send,Sync);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ExplicitSendSync: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::ExplicitSendSync, p) == 0);
};
