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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Plain")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Empty")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ContainsPlain")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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

        /// Send + Sync: non-aggregate with no bases (0-byte `__non_field_data` only
        /// prevents direct initialization and holds no C++ data).
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test17NonAggregatePlainE"]
        #[repr(C)]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NonAggregatePlain")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct NonAggregatePlain {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            pub x_: ::ffi_11::c_int,
        }
        unsafe impl ::cxx::ExternType for NonAggregatePlain {
            type Id = ::cxx::type_id!("crubit :: test :: NonAggregatePlain");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: NonAggregatePlain"),
            crate::crubit::test::NonAggregatePlain
        );

        impl Default for NonAggregatePlain {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test17NonAggregatePlainC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Send + Sync: bitfields and unnamed zero-width bitfields hold only integers /
        /// alignment directives.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test21WithZeroWidthBitfieldE"]
        #[repr(C, align(4))]
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: WithZeroWidthBitfield"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct WithZeroWidthBitfield {
            /// * `a` : 4 bits.
            /// * `b` : 4 bits.
            __bitfields0: [::core::mem::MaybeUninit<u8>; 5],
        }
        unsafe impl ::cxx::ExternType for WithZeroWidthBitfield {
            type Id = ::cxx::type_id!("crubit :: test :: WithZeroWidthBitfield");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: WithZeroWidthBitfield"),
            crate::crubit::test::WithZeroWidthBitfield
        );

        impl Default for WithZeroWidthBitfield {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test21WithZeroWidthBitfieldC1Ev(
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: WithPointer")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ContainsWithPointer"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: WithPrivateField")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct WithPrivateField {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            pub public_field_: ::ffi_11::c_int,
            /// Reason for representing this field as a blob of bytes:
            /// Types of non-public C++ fields can be elided away
            pub(crate) private_field_: [::core::mem::MaybeUninit<u8>; 4],
        }
        /// Cannot infer `Send` because:
        /// * field `private_field_` is represented as opaque bytes
        impl !Send for WithPrivateField {}
        /// Cannot infer `Sync` because:
        /// * field `private_field_` is represented as opaque bytes
        impl !Sync for WithPrivateField {}
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
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: WithUnsupportedField"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct WithUnsupportedField {
            /// Reason for representing this field as a blob of bytes:
            /// Unsupported `volatile` qualifier: volatile int
            pub(crate) unsupported: [::core::mem::MaybeUninit<u8>; 4],
        }
        /// Cannot infer `Send` because:
        /// * field `unsupported` is represented as opaque bytes
        impl !Send for WithUnsupportedField {}
        /// Cannot infer `Sync` because:
        /// * field `unsupported` is represented as opaque bytes
        impl !Sync for WithUnsupportedField {}
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Base")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: Derived")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct Derived {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
            pub derived_field: ::ffi_11::c_int,
        }
        /// Cannot infer `Send` because:
        /// * base class subobjects are not represented as Rust fields
        impl !Send for Derived {}
        /// Cannot infer `Sync` because:
        /// * base class subobjects are not represented as Rust fields
        impl !Sync for Derived {}
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NotSend")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NotSync")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NotSendNotSync")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ExplicitSendSync")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
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

        /// An empty tag type which is explicitly neither Send nor Sync.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test15ThreadAffineTagE"]
        #[repr(C)]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ThreadAffineTag")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct ThreadAffineTag {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for ThreadAffineTag {}
        impl !Sync for ThreadAffineTag {}
        unsafe impl ::cxx::ExternType for ThreadAffineTag {
            type Id = ::cxx::type_id!("crubit :: test :: ThreadAffineTag");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: ThreadAffineTag"),
            crate::crubit::test::ThreadAffineTag
        );

        impl Default for ThreadAffineTag {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test15ThreadAffineTagC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Neither: empty base class subobjects are not emitted as Rust fields.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test26DerivedFromThreadAffineTagE"]
        #[repr(C, align(4))]
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: DerivedFromThreadAffineTag"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct DerivedFromThreadAffineTag {
            pub x: ::ffi_11::c_int,
        }
        /// Cannot infer `Send` because:
        /// * base class subobjects are not represented as Rust fields
        impl !Send for DerivedFromThreadAffineTag {}
        /// Cannot infer `Sync` because:
        /// * base class subobjects are not represented as Rust fields
        impl !Sync for DerivedFromThreadAffineTag {}
        unsafe impl ::cxx::ExternType for DerivedFromThreadAffineTag {
            type Id = ::cxx::type_id!("crubit :: test :: DerivedFromThreadAffineTag");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: DerivedFromThreadAffineTag"),
            crate::crubit::test::DerivedFromThreadAffineTag
        );

        impl Default for DerivedFromThreadAffineTag {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26DerivedFromThreadAffineTagC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Neither: private empty base class subobjects are not emitted as Rust fields.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test9EmptyBaseE"]
        #[repr(C)]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: EmptyBase")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct EmptyBase {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        unsafe impl ::cxx::ExternType for EmptyBase {
            type Id = ::cxx::type_id!("crubit :: test :: EmptyBase");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: EmptyBase"),
            crate::crubit::test::EmptyBase
        );

        impl Default for EmptyBase {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test9EmptyBaseC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test23DerivedFromPrivateEmptyE"]
        #[repr(C, align(4))]
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: DerivedFromPrivateEmpty"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct DerivedFromPrivateEmpty {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
            pub x_: ::ffi_11::c_int,
        }
        /// Cannot infer `Send` because:
        /// * base class subobjects are not represented as Rust fields
        impl !Send for DerivedFromPrivateEmpty {}
        /// Cannot infer `Sync` because:
        /// * base class subobjects are not represented as Rust fields
        impl !Sync for DerivedFromPrivateEmpty {}
        unsafe impl ::cxx::ExternType for DerivedFromPrivateEmpty {
            type Id = ::cxx::type_id!("crubit :: test :: DerivedFromPrivateEmpty");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: DerivedFromPrivateEmpty"),
            crate::crubit::test::DerivedFromPrivateEmpty
        );

        impl Default for DerivedFromPrivateEmpty {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test23DerivedFromPrivateEmptyC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // Neither: zero-sized `[[no_unique_address]]` members are not emitted as Rust
        // fields.

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test30NoUniqueAddressThreadAffineTagE"]
        #[repr(C, align(4))]
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NoUniqueAddressThreadAffineTag"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct NoUniqueAddressThreadAffineTag {
            pub x: ::ffi_11::c_int,
        }
        /// Cannot infer `Send` because:
        /// * zero-sized field `tag` is not represented as a Rust field
        impl !Send for NoUniqueAddressThreadAffineTag {}
        /// Cannot infer `Sync` because:
        /// * zero-sized field `tag` is not represented as a Rust field
        impl !Sync for NoUniqueAddressThreadAffineTag {}
        unsafe impl ::cxx::ExternType for NoUniqueAddressThreadAffineTag {
            type Id = ::cxx::type_id!("crubit :: test :: NoUniqueAddressThreadAffineTag");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: NoUniqueAddressThreadAffineTag"),
            crate::crubit::test::NoUniqueAddressThreadAffineTag
        );

        impl Default for NoUniqueAddressThreadAffineTag {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test30NoUniqueAddressThreadAffineTagC1Ev(&raw mut tmp as*mut _);
                    tmp.assume_init()
                }
            }
        }

        // Neither: zero-length array members are zero-sized and not emitted as Rust
        // fields.

        /// # Safety
        ///
        /// To call a function that accepts this type, you must uphold these requirements:
        /// * Document why the following public unsafe fields of this type cannot be misused by callee:
        ///   * `items`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'int *[0]': Zero-length C arrays are not supported
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test26WithZeroLengthPointerArrayE"]
        #[repr(C, align(8))]
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: WithZeroLengthPointerArray"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct WithZeroLengthPointerArray {
            pub x: f64,
        }
        /// Cannot infer `Send` because:
        /// * zero-sized field `items` is not represented as a Rust field
        impl !Send for WithZeroLengthPointerArray {}
        /// Cannot infer `Sync` because:
        /// * zero-sized field `items` is not represented as a Rust field
        impl !Sync for WithZeroLengthPointerArray {}
        unsafe impl ::cxx::ExternType for WithZeroLengthPointerArray {
            type Id = ::cxx::type_id!("crubit :: test :: WithZeroLengthPointerArray");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: WithZeroLengthPointerArray"),
            crate::crubit::test::WithZeroLengthPointerArray
        );

        impl Default for WithZeroLengthPointerArray {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test26WithZeroLengthPointerArrayC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Neither: contains a member which is neither.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test21MemberThreadAffineTagE"]
        #[repr(C)]
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: MemberThreadAffineTag"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct MemberThreadAffineTag {
            pub tag: crate::crubit::test::ThreadAffineTag,
            pub x: ::ffi_11::c_int,
        }
        unsafe impl ::cxx::ExternType for MemberThreadAffineTag {
            type Id = ::cxx::type_id!("crubit :: test :: MemberThreadAffineTag");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: MemberThreadAffineTag"),
            crate::crubit::test::MemberThreadAffineTag
        );

        impl Default for MemberThreadAffineTag {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test21MemberThreadAffineTagC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // error: class `crubit::test::ThreadAffineTemplate` could not be bound
        //   Class templates are not yet supported

        /// Neither: annotation on a class template applies to its instantiations.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "__CcTemplateInstN6crubit4test20ThreadAffineTemplateIiEE"]
        #[repr(C)]
        #[cfg_attr(
            not(doc),
            doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ThreadAffineTemplate < int >"
        )]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct ThreadAffineTemplateInt {
            pub x: ::ffi_11::c_int,
        }
        impl !Send for ThreadAffineTemplateInt {}
        impl !Sync for ThreadAffineTemplateInt {}
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: ThreadAffineTemplate < int >"),
            crate::crubit::test::ThreadAffineTemplateInt
        );

        impl Default for ThreadAffineTemplateInt {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk__b813127c__ZN6crubit4test20ThreadAffineTemplateIiEC1Ev(&raw mut tmp as*mut _);
                    tmp.assume_init()
                }
            }
        }
    }
}

// namespace crubit::test

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    #[doc(hidden)]
    pub type __CcTemplateInstN6crubit4test20ThreadAffineTemplateIiEE =
        crate::crubit::test::ThreadAffineTemplateInt;
}
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
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test17NonAggregatePlainC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test21WithZeroWidthBitfieldC1Ev(
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
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test15ThreadAffineTagC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26DerivedFromThreadAffineTagC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test9EmptyBaseC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test23DerivedFromPrivateEmptyC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test30NoUniqueAddressThreadAffineTagC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test26WithZeroLengthPointerArrayC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test21MemberThreadAffineTagC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__b813127c__ZN6crubit4test20ThreadAffineTemplateIiEC1Ev(
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
    assert!(::core::mem::size_of::<crate::crubit::test::NonAggregatePlain>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::NonAggregatePlain>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::NonAggregatePlain: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NonAggregatePlain: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::NonAggregatePlain, x_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::WithZeroWidthBitfield>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::WithZeroWidthBitfield>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::WithZeroWidthBitfield: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::WithZeroWidthBitfield: Drop);

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
    assert!(::core::mem::size_of::<crate::crubit::test::ThreadAffineTag>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::ThreadAffineTag>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::ThreadAffineTag: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ThreadAffineTag: Drop,Send,Sync);

    assert!(::core::mem::size_of::<crate::crubit::test::DerivedFromThreadAffineTag>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::DerivedFromThreadAffineTag>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::DerivedFromThreadAffineTag: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::DerivedFromThreadAffineTag: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::DerivedFromThreadAffineTag, x) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::EmptyBase>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::EmptyBase>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::EmptyBase: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::EmptyBase: Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::DerivedFromPrivateEmpty>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::DerivedFromPrivateEmpty>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::DerivedFromPrivateEmpty: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::DerivedFromPrivateEmpty: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::DerivedFromPrivateEmpty, x_) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::NoUniqueAddressThreadAffineTag>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::NoUniqueAddressThreadAffineTag>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::NoUniqueAddressThreadAffineTag: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NoUniqueAddressThreadAffineTag: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::NoUniqueAddressThreadAffineTag, x) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::WithZeroLengthPointerArray>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::WithZeroLengthPointerArray>() == 8);
    static_assertions::assert_impl_all!(crate::crubit::test::WithZeroLengthPointerArray: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::WithZeroLengthPointerArray: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::WithZeroLengthPointerArray, x) == 0);
    assert!(::core::mem::size_of::<crate::crubit::test::MemberThreadAffineTag>() == 8);
    assert!(::core::mem::align_of::<crate::crubit::test::MemberThreadAffineTag>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::MemberThreadAffineTag: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::MemberThreadAffineTag: Drop);
    assert!(::core::mem::offset_of!(crate::crubit::test::MemberThreadAffineTag, tag) == 0);
    assert!(::core::mem::offset_of!(crate::crubit::test::MemberThreadAffineTag, x) == 4);
    assert!(::core::mem::size_of::<crate::crubit::test::ThreadAffineTemplateInt>() == 4);
    assert!(::core::mem::align_of::<crate::crubit::test::ThreadAffineTemplateInt>() == 4);
    static_assertions::assert_impl_all!(crate::crubit::test::ThreadAffineTemplateInt: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ThreadAffineTemplateInt: Drop,Send,Sync);
    assert!(::core::mem::offset_of!(crate::crubit::test::ThreadAffineTemplateInt, x) == 0);
};
