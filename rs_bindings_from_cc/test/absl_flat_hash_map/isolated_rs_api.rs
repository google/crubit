// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/absl_flat_hash_map:isolated

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
        pub type MapWithTwoParams =
            crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE;

        pub type MapWithThreeParams =
            crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE;

        /// Maps of non-layout-compatible bridged types get bindings, but without the
        /// methods that access their elements.
        pub type MapWithBridgedKey=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE;

        pub type MapWithBridgedValue=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE;

        /// Maps of view types. `absl::Span` and `rs_std::StrRef` have Rust types with
        /// lifetimes, so their maps get bindings, but without the methods that access
        /// their elements, because the maps' Rust types don't have lifetime parameters
        /// for their elements yet (b/478868411). `std::string_view` and
        /// `rs_std::SliceRef` are accessed through their lifetime-less layout-compatible
        /// Rust types (`raw_string_view` and `*const [T]`), so their maps get all of the
        /// methods.
        pub type MapWithStringViewKey=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE;

        pub type MapWithStringViewValue=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE;

        pub type MapWithSpanValue=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE;

        pub type MapWithStrRefValue=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE;

        pub type MapWithSliceRefValue=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE;

        /// Functions that return the elements of maps of view types, or references to
        /// them. These are stubs, because only their signatures matter.
        ///
        /// Returning `absl::Span` and `rs_std::SliceRef` by value also instantiates
        /// them, which is required to use them as the value types of maps.
        #[inline(always)]
        pub fn GetAtFive<'lt>(
            map: crate::crubit::test::MapWithStringViewValue,
        ) -> ::cc_std::std::string_view<'lt> {
            let mut map = ::core::mem::MaybeUninit::new(map);
            unsafe {
                let mut __crubit_return =
                    ::core::mem::MaybeUninit::<::cc_std::std::string_view<'lt>>::uninit();
                crate::detail::__rust_thunk___ZN6crubit4test9GetAtFiveEN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS3_11char_traitsIcEEEELi42EEE(&raw mut __crubit_return as*mut::core::ffi::c_void,map.as_mut_ptr());
                __crubit_return.assume_init()
            }
        }

        #[inline(always)]
        pub fn GetRefAtFive<'map>(
            map: &'map crate::crubit::test::MapWithStringViewValue,
        ) -> ::cref::CRef<'map, ::cc_std::std::__u::raw_string_view> {
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test12GetRefAtFiveERKN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS3_11char_traitsIcEEEELi42EEE(map)
            }
        }

        #[inline(always)]
        pub fn GetSpanAtFive<'lt>(
            map: crate::crubit::test::MapWithSpanValue,
        ) -> ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'lt, ::ffi_11::c_int> {
            let mut map = ::core::mem::MaybeUninit::new(map);
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<
                    ::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'lt, ::ffi_11::c_int>,
                >::uninit();
                crate::detail::__rust_thunk___ZN6crubit4test13GetSpanAtFiveEN4absl13flat_hash_mapIiNS1_4SpanIKiEELi42EEE(&raw mut __crubit_return as*mut::core::ffi::c_void,map.as_mut_ptr());
                __crubit_return.assume_init()
            }
        }

        #[inline(always)]
        pub fn GetSliceRefAtFive(
            map: crate::crubit::test::MapWithSliceRefValue,
        ) -> *const [::ffi_11::c_int] {
            let mut map = ::core::mem::MaybeUninit::new(map);
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test17GetSliceRefAtFiveEN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE(map.as_mut_ptr())
            }
        }

        forward_declare::forward_declare!(pub Incomplete = forward_declare::symbol!("crubit :: test :: Incomplete"));

        // error: type alias `crubit::test::MapWithIncompleteKey` could not be bound
        //   Type `absl::flat_hash_map<class crubit::test::Incomplete, int, 42>` uses forward-declared type `crate::crubit::test::Incomplete` as an argument to a layout-compatible generic type. This is not supported. For more on why, see crubit.rs/types#incomplete_types.

        // error: type alias `crubit::test::MapWithIncompleteValue` could not be bound
        //   Type `absl::flat_hash_map<int, class crubit::test::Incomplete, 42>` uses forward-declared type `crate::crubit::test::Incomplete` as an argument to a layout-compatible generic type. This is not supported. For more on why, see crubit.rs/types#incomplete_types.

        /// Not Rust-movable: destructor is user-provided; annotate the type with CRUBIT_UNSAFE_BITWISE_RELOCATABLE if a memcpy move is sound
        #[::ctor::recursively_pinned]
        #[cfi_encoding = "N6crubit4test12NoDestructorE"]
        #[repr(C)]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NoDestructor")]
        pub struct NoDestructor {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
        }
        unsafe impl ::cxx::ExternType for NoDestructor {
            type Id = ::cxx::type_id!("crubit :: test :: NoDestructor");
            type Kind = ::cxx::kind::Opaque;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: NoDestructor"),
            crate::crubit::test::NoDestructor
        );

        // error: constructor `crubit::test::NoDestructor::NoDestructor` could not be bound
        //   `NoDestructor` can't be used by-value because it has a non-public or deleted destructor

        // error: constructor `crubit::test::NoDestructor::NoDestructor` could not be bound
        //   `NoDestructor` can't be used by-value because it has a non-public or deleted destructor

        impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NoDestructor {
            #[inline(always)]
            fn assign<'__this>(
                self: ::core::pin::Pin<&'__this mut Self>,
                __param_0: &'__param_0 Self,
            ) {
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test12NoDestructoraSERKS1_(
                        self, __param_0,
                    );
                }
            }
        }

        // error: type alias `crubit::test::MapWithNoDestructorKey` could not be bound
        //   `crate::crubit::test::NoDestructor` can't be used in a Rust absl::flat_hash_map<K, _> because it has a deleted or non-public destructor

        // error: type alias `crubit::test::MapWithNoDestructorValue` could not be bound
        //   `crate::crubit::test::NoDestructor` can't be used in a Rust absl::flat_hash_map<_, V> because it has a deleted or non-public destructor

        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test8NoDeleteE"]
        #[repr(C)]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NoDelete")]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
        pub struct NoDelete {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        unsafe impl ::cxx::ExternType for NoDelete {
            type Id = ::cxx::type_id!("crubit :: test :: NoDelete");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: NoDelete"),
            crate::crubit::test::NoDelete
        );

        impl Default for NoDelete {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test8NoDeleteC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        // error: type alias `crubit::test::MapWithNoDeleteKey` could not be bound
        //   `crate::crubit::test::NoDelete` can't be used in a Rust absl::flat_hash_map<K, _> because it has a deleted or non-public operator delete

        // error: type alias `crubit::test::MapWithNoDeleteValue` could not be bound
        //   `crate::crubit::test::NoDelete` can't be used in a Rust absl::flat_hash_map<_, V> because it has a deleted or non-public operator delete

        /// `reserve`, `insert`, and `try_insert` construct elements from rvalues, so
        /// they aren't generated for element types that have neither a move constructor
        /// nor a copy constructor.
        ///
        /// Not Rust-movable: type is neither copyable nor movable
        #[::ctor::recursively_pinned]
        #[cfi_encoding = "N6crubit4test12NoMoveNoCopyE"]
        #[repr(C)]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: NoMoveNoCopy")]
        pub struct NoMoveNoCopy {
            __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 1],
        }
        unsafe impl ::cxx::ExternType for NoMoveNoCopy {
            type Id = ::cxx::type_id!("crubit :: test :: NoMoveNoCopy");
            type Kind = ::cxx::kind::Opaque;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: NoMoveNoCopy"),
            crate::crubit::test::NoMoveNoCopy
        );

        impl ::ctor::CtorNew<()> for NoMoveNoCopy {
            type CtorType = ::ctor::Ctor![Self];
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: ()) -> Self::CtorType {
                unsafe {
                    ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                        let () = args;
                        crate::detail::__rust_thunk___ZN6crubit4test12NoMoveNoCopyC1Ev(
                            __crubit_dest as *mut ::core::ffi::c_void,
                        );
                    })
                }
            }
        }

        pub type MapWithNoMoveNoCopyKey=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE;

        pub type MapWithNoMoveNoCopyValue=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE;

        /// An rvalue of a type without a move constructor binds to its copy
        /// constructor, so `reserve`, `insert`, and `try_insert` are generated.
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[cfi_encoding = "N6crubit4test8CopyOnlyE"]
        #[repr(C)]
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=crubit :: test :: CopyOnly")]
        pub struct CopyOnly {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        unsafe impl ::cxx::ExternType for CopyOnly {
            type Id = ::cxx::type_id!("crubit :: test :: CopyOnly");
            type Kind = ::cxx::kind::Trivial;
        }
        forward_declare::unsafe_define!(
            forward_declare::symbol!("crubit :: test :: CopyOnly"),
            crate::crubit::test::CopyOnly
        );

        impl Default for CopyOnly {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test8CopyOnlyC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        pub type MapWithCopyOnlyKey=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE;

        pub type MapWithCopyOnlyValue=crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE;
    }
}

// namespace crubit::test

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE"]
    #[repr(C, align(8))]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < class std :: __wrap_iter < const int *>>"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
    }
    /// Cannot infer `Send` because:
    /// * base class subobjects are not represented as Rust fields
    /// * field `current` is represented as opaque bytes
    impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {}
    /// Cannot infer `Sync` because:
    /// * base class subobjects are not represented as Rust fields
    /// * field `current` is represented as opaque bytes
    impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {}
    forward_declare::unsafe_define!(forward_declare::symbol!("std :: reverse_iterator < class std :: __wrap_iter < const int *>>"),crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE);
    impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        #[must_use]
        #[inline(always)]
        pub fn base<'__this>(
            &'__this self,
        ) -> crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<
                    crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
                >::uninit();
                crate::detail::__rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEE4baseEv(&raw mut __crubit_return as*mut::core::ffi::c_void,self);
                __crubit_return.assume_init()
            }
        }
    }

    impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    impl From<crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>
        for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
    {
        #[inline(always)]
        fn from(
            args: crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        ) -> Self {
            let mut __x = args;
            let mut __x = ::core::mem::MaybeUninit::new(__x);
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(&raw mut tmp as*mut _,__x.as_mut_ptr());
                tmp.assume_init()
            }
        }
    }
    impl
        ::ctor::CtorNew<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        > for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
    {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(
            args: crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        ) -> Self::CtorType {
            <Self as From<
                crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >>::from(args)
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE"]
    #[repr(C, align(8))]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const char *>")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
    }
    /// Cannot infer `Send` because:
    /// * base class subobjects are not represented as Rust fields
    /// * field `current` is represented as opaque bytes
    impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {}
    /// Cannot infer `Sync` because:
    /// * base class subobjects are not represented as Rust fields
    /// * field `current` is represented as opaque bytes
    impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("std :: reverse_iterator < const char *>"),
        crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
    );
    impl __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
        #[must_use]
        #[inline(always)]
        pub fn base<'__this>(&'__this self) -> *const ::ffi_11::c_char {
            unsafe {
                crate::detail::__rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorIPKcE4baseEv(self)
            }
        }
    }

    impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__x`: raw pointer
    impl ::ctor::UnsafeFrom<*const ::ffi_11::c_char>
        for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
    {
        #[inline(always)]
        unsafe fn unsafe_from(args: *const ::ffi_11::c_char) -> Self {
            let mut __x = args;
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
                    &raw mut tmp as *mut _,
                    __x,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::UnsafeCtorNew<*const ::ffi_11::c_char>
        for __CcTemplateInstNSt3__u16reverse_iteratorIPKcEE
    {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        unsafe fn ctor_new(args: *const ::ffi_11::c_char) -> Self::CtorType {
            unsafe { <Self as ::ctor::UnsafeFrom<*const ::ffi_11::c_char>>::unsafe_from(args) }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE"]
    #[repr(C, align(8))]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const int *>")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
    }
    /// Cannot infer `Send` because:
    /// * base class subobjects are not represented as Rust fields
    /// * field `current` is represented as opaque bytes
    impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {}
    /// Cannot infer `Sync` because:
    /// * base class subobjects are not represented as Rust fields
    /// * field `current` is represented as opaque bytes
    impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("std :: reverse_iterator < const int *>"),
        crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE
    );
    impl __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {
        #[must_use]
        #[inline(always)]
        pub fn base<'__this>(&'__this self) -> *const ::ffi_11::c_int {
            unsafe {
                crate::detail::__rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorIPKiE4baseEv(self)
            }
        }
    }

    impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKiEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `__x`: raw pointer
    impl ::ctor::UnsafeFrom<*const ::ffi_11::c_int>
        for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE
    {
        #[inline(always)]
        unsafe fn unsafe_from(args: *const ::ffi_11::c_int) -> Self {
            let mut __x = args;
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorIPKiEC1ES2_(
                    &raw mut tmp as *mut _,
                    __x,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::UnsafeCtorNew<*const ::ffi_11::c_int>
        for __CcTemplateInstNSt3__u16reverse_iteratorIPKiEE
    {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        unsafe fn ctor_new(args: *const ::ffi_11::c_int) -> Self::CtorType {
            unsafe { <Self as ::ctor::UnsafeFrom<*const ::ffi_11::c_int>>::unsafe_from(args) }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE"]
    #[repr(C, align(8))]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < const wchar_t *>"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
    }
    /// Cannot infer `Send` because:
    /// * base class subobjects are not represented as Rust fields
    /// * field `current` is represented as opaque bytes
    impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {}
    /// Cannot infer `Sync` because:
    /// * base class subobjects are not represented as Rust fields
    /// * field `current` is represented as opaque bytes
    impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("std :: reverse_iterator < const wchar_t *>"),
        crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE
    );

    impl Default for __CcTemplateInstNSt3__u16reverse_iteratorIPKwEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPKiEE"]
    #[repr(C, align(8))]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < const int *>")]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
    }
    /// Cannot infer `Send` because:
    /// * field `__i_` is represented as opaque bytes
    impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {}
    /// Cannot infer `Sync` because:
    /// * field `__i_` is represented as opaque bytes
    impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("std :: __wrap_iter < const int *>"),
        crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
    );

    impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__83c617d2__ZNSt3__u11__wrap_iterIPKiEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    impl<'__this> ::core::ops::Add<isize>
        for &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
    {
        type Output = crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        #[inline(always)]
        fn add(self, __n: isize) -> Self::Output {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<
                    crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
                >::uninit();
                crate::detail::__rust_thunk__45d7cb71__ZNKSt3__u11__wrap_iterIPKiEplEl(
                    &raw mut __crubit_return as *mut ::core::ffi::c_void,
                    self,
                    __n,
                );
                __crubit_return.assume_init()
            }
        }
    }

    impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        #[inline(always)]
        fn add_assign<'__this>(&'__this mut self, __n: isize) {
            unsafe {
                crate::detail::__rust_thunk__c15f65fd__ZNSt3__u11__wrap_iterIPKiEpLEl(self, __n);
            }
        }
    }

    impl<'__this> ::core::ops::Sub<isize>
        for &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
    {
        type Output = crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        #[inline(always)]
        fn sub(self, __n: isize) -> Self::Output {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<
                    crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
                >::uninit();
                crate::detail::__rust_thunk__ee08d685__ZNKSt3__u11__wrap_iterIPKiEmiEl(
                    &raw mut __crubit_return as *mut ::core::ffi::c_void,
                    self,
                    __n,
                );
                __crubit_return.assume_init()
            }
        }
    }

    impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        #[inline(always)]
        fn sub_assign<'__this>(&'__this mut self, __n: isize) {
            unsafe {
                crate::detail::__rust_thunk__55a37ce5__ZNSt3__u11__wrap_iterIPKiEmIEl(self, __n);
            }
        }
    }

    impl ::operator::CcIndex<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        type Output<'ctnr> = &'ctnr ::ffi_11::c_int;
        #[inline(always)]
        fn cc_index<'ctnr>(&'ctnr self, __n: isize) -> Self::Output<'ctnr> {
            unsafe {
                crate::detail::__rust_thunk__f7b43e06__ZNKSt3__u11__wrap_iterIPKiEixEl(self, __n)
            }
        }
    }
    impl ::core::ops::Index<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        type Output = ::ffi_11::c_int;
        #[inline(always)]
        fn index(&self, index: isize) -> &Self::Output {
            ::operator::CcIndex::cc_index(self, index)
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE"]
    #[repr(C, align(8))]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=std :: span < const int , 18446744073709551615UL >"
    )]
    pub struct __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
    }
    /// Cannot infer `Send` because:
    /// * fields `__data_`, `__size_` are represented as opaque bytes
    impl !Send for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {}
    /// Cannot infer `Sync` because:
    /// * fields `__data_`, `__size_` are represented as opaque bytes
    impl !Sync for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("std :: span < const int , 18446744073709551615UL >"),
        crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE
    );
    impl __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        #[must_use]
        #[inline(always)]
        pub fn first<'__this>(&'__this self, __count: usize) -> crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>::uninit();
                crate::detail::__rust_thunk__4e54b5b5__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm(
                    &raw mut __crubit_return as *mut ::core::ffi::c_void,
                    self,
                    __count,
                );
                __crubit_return.assume_init()
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn last<'__this>(&'__this self, __count: usize) -> crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>::uninit();
                crate::detail::__rust_thunk__0f590027__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm(
                    &raw mut __crubit_return as *mut ::core::ffi::c_void,
                    self,
                    __count,
                );
                __crubit_return.assume_init()
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn subspan<'__this>(&'__this self, __offset: usize, __count: usize) -> crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>::uninit();
                crate::detail::__rust_thunk__022263c7__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm(
                    &raw mut __crubit_return as *mut ::core::ffi::c_void,
                    self,
                    __offset,
                    __count,
                );
                __crubit_return.assume_init()
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn size<'__this>(&'__this self) -> usize {
            unsafe {
                crate::detail::__rust_thunk__ea271768__ZNKSt3__u4spanIKiLm18446744073709551615EE4sizeEv(self)
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn size_bytes<'__this>(&'__this self) -> usize {
            unsafe {
                crate::detail::__rust_thunk__7475e8e4__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv(self)
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn empty<'__this>(&'__this self) -> bool {
            unsafe {
                crate::detail::__rust_thunk__0e958be1__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv(self)
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn front<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
            unsafe {
                crate::detail::__rust_thunk__e6dd0be6__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv(self)
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn back<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
            unsafe {
                crate::detail::__rust_thunk__4df28266__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv(self)
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn data<'__this>(&'__this self) -> *const ::ffi_11::c_int {
            unsafe {
                crate::detail::__rust_thunk__b265a8aa__ZNKSt3__u4spanIKiLm18446744073709551615EE4dataEv(self)
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn begin<'__this>(
            &'__this self,
        ) -> crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<
                    crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
                >::uninit();
                crate::detail::__rust_thunk__626b734c__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,self);
                __crubit_return.assume_init()
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn end<'__this>(
            &'__this self,
        ) -> crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<
                    crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
                >::uninit();
                crate::detail::__rust_thunk__09c98b89__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv(&raw mut __crubit_return as*mut::core::ffi::c_void,self);
                __crubit_return.assume_init()
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn rbegin<'__this>(&'__this self) -> crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE>::uninit();
                crate::detail::__rust_thunk__af6f680d__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as *mut ::core::ffi::c_void, self);
                __crubit_return.assume_init()
            }
        }
        #[must_use]
        #[inline(always)]
        pub fn rend<'__this>(&'__this self) -> crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
            unsafe {
                let mut __crubit_return = ::core::mem::MaybeUninit::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE>::uninit();
                crate::detail::__rust_thunk__e01cc11e__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(&raw mut __crubit_return as *mut ::core::ffi::c_void, self);
                __crubit_return.assume_init()
            }
        }
    }

    impl Default for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__7fa98692__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    impl ::operator::CcIndex<usize> for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        type Output<'ctnr> = &'ctnr ::ffi_11::c_int;
        #[inline(always)]
        fn cc_index<'ctnr>(&'ctnr self, __idx: usize) -> Self::Output<'ctnr> {
            unsafe {
                crate::detail::__rust_thunk__b2c6c9fa__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm(
                    self, __idx,
                )
            }
        }
    }
    impl ::core::ops::Index<usize> for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        type Output = ::ffi_11::c_int;
        #[inline(always)]
        fn index(&self, index: usize) -> &Self::Output {
            ::operator::CcIndex::cc_index(self, index)
        }
    }

    // error: class `absl::flat_hash_map<class crubit::test::Incomplete, int, 42>` could not be bound
    //   Type `absl::flat_hash_map<class crubit::test::Incomplete, int, 42>` uses forward-declared type `crate::crubit::test::Incomplete` as an argument to a layout-compatible generic type. This is not supported. For more on why, see crubit.rs/types#incomplete_types.

    // error: class `absl::flat_hash_map<class crubit::test::NoDestructor, int, 42>` could not be bound
    //   `crate::crubit::test::NoDestructor` can't be used in a Rust absl::flat_hash_map<K, _> because it has a deleted or non-public destructor

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < class crubit :: test :: NoMoveNoCopy , int , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < class crubit :: test :: NoMoveNoCopy , int , 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE {
        // error: `absl::flat_hash_map` methods could not be generated
        //   the key type `crate::crubit::test::NoMoveNoCopy` can't be constructed from an rvalue because it has no public move or copy constructor
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<crubit::test::NoMoveNoCopy, int>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::NoMoveNoCopy, int>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::NoMoveNoCopy, int>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::NoMoveNoCopy, int>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::NoMoveNoCopy, int>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::NoMoveNoCopy, int>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::NoMoveNoCopy, int>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::NoMoveNoCopy, int>::try_emplace` could not be bound
    //   Function templates are not yet supported

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < struct crubit :: test :: Bridged , int , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < struct crubit :: test :: Bridged , int , 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE {
        // error: `absl::flat_hash_map` methods could not be generated
        //   the key type `crate::Bridged` can't be accessed in place because it is a non-layout-compatible bridged type
        //   See crubit.rs/types.
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<crubit::test::Bridged, int>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::Bridged, int>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::Bridged, int>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::Bridged, int>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::Bridged, int>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::Bridged, int>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::Bridged, int>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::Bridged, int>::try_emplace` could not be bound
    //   Function templates are not yet supported

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < class crubit :: test :: CopyOnly , int , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < class crubit :: test :: CopyOnly , int , 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE {
        #[inline(always)]
        #[must_use]
        pub fn len(&self) -> usize {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::len(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::is_empty(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn capacity(&self) -> usize {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::capacity(self)
        }
        #[inline(always)]
        pub fn clear(self: ::core::pin::Pin<&mut Self>) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::clear(self)
        }
        #[inline(always)]
        pub fn reserve(self: ::core::pin::Pin<&mut Self>, additional: usize) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::reserve(self,additional)
        }
        #[inline(always)]
        #[must_use]
        pub fn contains_key(&self, key: &crate::crubit::test::CopyOnly) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::contains_key(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get<'a>(
            &'a self,
            key: &crate::crubit::test::CopyOnly,
        ) -> ::core::option::Option<&'a ::ffi_11::c_int> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_key_value<'a>(
            &'a self,
            key: &crate::crubit::test::CopyOnly,
        ) -> ::core::option::Option<(&'a crate::crubit::test::CopyOnly, &'a ::ffi_11::c_int)>
        {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_key_value(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_mut<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: &crate::crubit::test::CopyOnly,
        ) -> ::core::option::Option<&'a mut ::ffi_11::c_int> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_mut(self,key)
        }
        #[inline(always)]
        pub fn insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![crate::crubit::test::CopyOnly],
            value: ::ctor::Ctor![::ffi_11::c_int],
        ) -> (&'a crate::crubit::test::CopyOnly, &'a mut ::ffi_11::c_int) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::insert(self,key,value)
        }
        #[inline(always)]
        #[must_use]
        pub fn try_insert<'a>(self: ::core::pin::Pin<&'a mut Self>, key: ::ctor::Ctor![crate::crubit::test::CopyOnly], value: ::ctor::Ctor![::ffi_11::c_int]) -> ::core::result::Result<(&'a crate::crubit::test::CopyOnly, &'a mut ::ffi_11::c_int), ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::OccupiedError<(&'a crate::crubit::test::CopyOnly, &'a mut ::ffi_11::c_int)>> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::try_insert(self, key, value)
        }
        #[inline(always)]
        pub fn remove(
            self: ::core::pin::Pin<&mut Self>,
            key: &crate::crubit::test::CopyOnly,
        ) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::remove(self,key)
        }
    }
    unsafe impl ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map for crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE {
        type Key = crate::crubit::test::CopyOnly;
        type Value = ::ffi_11::c_int;
        type ValueMut<'a>
            = &'a mut ::ffi_11::c_int
        where
            Self: 'a;
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_len(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_capacity(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_clear(self: ::core::pin::Pin<&mut Self>) {
            unsafe { crate::detail::__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_reserve(self: ::core::pin::Pin<&mut Self>, count: usize) {
            unsafe { crate::detail::__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, count) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_key_value(&self, key: &Self::Key, result_key: *mut *const Self::Key, result_value: *mut *const Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_mut(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> *mut Self::Value {
            unsafe { crate::detail::__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_insert(self: ::core::pin::Pin<&mut Self>, key: *mut Self::Key, value: *mut Self::Value, overwrite: bool, result_key: *mut *const Self::Key, result_value: *mut *mut Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, value, overwrite, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_remove(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<crubit::test::CopyOnly, int>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::CopyOnly, int>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::CopyOnly, int>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::CopyOnly, int>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::CopyOnly, int>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::CopyOnly, int>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::CopyOnly, int>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<crubit::test::CopyOnly, int>::try_emplace` could not be bound
    //   Function templates are not yet supported

    // error: class `absl::flat_hash_map<class crubit::test::NoDelete, int, 42>` could not be bound
    //   `crate::crubit::test::NoDelete` can't be used in a Rust absl::flat_hash_map<K, _> because it has a deleted or non-public operator delete

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < class std :: basic_string_view < char , struct std :: char_traits < char >>, int , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE
    {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < class std :: basic_string_view < char , struct std :: char_traits < char >>, int , 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE {
        #[inline(always)]
        #[must_use]
        pub fn len(&self) -> usize {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::len(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::is_empty(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn capacity(&self) -> usize {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::capacity(self)
        }
        #[inline(always)]
        pub fn clear(self: ::core::pin::Pin<&mut Self>) {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::clear(self)
        }
        #[inline(always)]
        pub fn reserve(self: ::core::pin::Pin<&mut Self>, additional: usize) {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::reserve(self, additional)
        }
        #[inline(always)]
        #[must_use]
        pub fn contains_key(&self, key: &::cc_std::std::__u::raw_string_view) -> bool {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::contains_key(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get<'a>(&'a self, key: &::cc_std::std::__u::raw_string_view) -> ::core::option::Option<&'a ::ffi_11::c_int> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_key_value<'a>(&'a self, key: &::cc_std::std::__u::raw_string_view) -> ::core::option::Option<(&'a ::cc_std::std::__u::raw_string_view, &'a ::ffi_11::c_int)> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_key_value(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_mut<'a>(self: ::core::pin::Pin<&'a mut Self>, key: &::cc_std::std::__u::raw_string_view) -> ::core::option::Option<&'a mut ::ffi_11::c_int> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_mut(self, key)
        }
        #[inline(always)]
        pub fn insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![::cc_std::std::__u::raw_string_view],
            value: ::ctor::Ctor![::ffi_11::c_int],
        ) -> (
            &'a ::cc_std::std::__u::raw_string_view,
            &'a mut ::ffi_11::c_int,
        ) {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::insert(self, key, value)
        }
        #[inline(always)]
        #[must_use]
        pub fn try_insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![::cc_std::std::__u::raw_string_view],
            value: ::ctor::Ctor![::ffi_11::c_int],
        ) -> ::core::result::Result<
            (
                &'a ::cc_std::std::__u::raw_string_view,
                &'a mut ::ffi_11::c_int,
            ),
            ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::OccupiedError<(
                &'a ::cc_std::std::__u::raw_string_view,
                &'a mut ::ffi_11::c_int,
            )>,
        > {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::try_insert(self, key, value)
        }
        #[inline(always)]
        pub fn remove(self: ::core::pin::Pin<&mut Self>, key: &::cc_std::std::__u::raw_string_view) -> bool {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::remove(self, key)
        }
    }
    unsafe impl ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map for crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE {
        type Key = ::cc_std::std::__u::raw_string_view;
        type Value = ::ffi_11::c_int;
        type ValueMut<'a>
            = &'a mut ::ffi_11::c_int
        where
            Self: 'a;
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_len(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_capacity(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_clear(self: ::core::pin::Pin<&mut Self>) {
            unsafe { crate::detail::__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_reserve(self: ::core::pin::Pin<&mut Self>, count: usize) {
            unsafe { crate::detail::__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, count) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_key_value(&self, key: &Self::Key, result_key: *mut *const Self::Key, result_value: *mut *const Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_mut(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> *mut Self::Value {
            unsafe { crate::detail::__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_insert(self: ::core::pin::Pin<&mut Self>, key: *mut Self::Key, value: *mut Self::Value, overwrite: bool, result_key: *mut *const Self::Key, result_value: *mut *mut Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, value, overwrite, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_remove(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<std::string_view, int>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<std::string_view, int>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<std::string_view, int>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<std::string_view, int>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<std::string_view, int>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<std::string_view, int>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<std::string_view, int>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<std::string_view, int>::try_emplace` could not be bound
    //   Function templates are not yet supported

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < int , class rs :: StrRef , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < int , class rs :: StrRef , 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE {
        // error: `absl::flat_hash_map` methods could not be generated
        //   the value type `&str` has a lifetime, which is not yet supported
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN2rs6StrRefELi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<int, rs::StrRef>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::StrRef>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::StrRef>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::StrRef>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::StrRef>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::StrRef>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::StrRef>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::StrRef>::try_emplace` could not be bound
    //   Function templates are not yet supported

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < int , class rs :: SliceRef < const int >, 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < int , class rs :: SliceRef < const int >, 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE {
        #[inline(always)]
        #[must_use]
        pub fn len(&self) -> usize {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::len(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::is_empty(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn capacity(&self) -> usize {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::capacity(self)
        }
        #[inline(always)]
        pub fn clear(self: ::core::pin::Pin<&mut Self>) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::clear(self)
        }
        #[inline(always)]
        pub fn reserve(self: ::core::pin::Pin<&mut Self>, additional: usize) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::reserve(self,additional)
        }
        #[inline(always)]
        #[must_use]
        pub fn contains_key(&self, key: &::ffi_11::c_int) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::contains_key(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get<'a>(
            &'a self,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<&'a *const [::ffi_11::c_int]> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_key_value<'a>(
            &'a self,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<(&'a ::ffi_11::c_int, &'a *const [::ffi_11::c_int])> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_key_value(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_mut<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<&'a mut *const [::ffi_11::c_int]> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_mut(self,key)
        }
        #[inline(always)]
        pub fn insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![::ffi_11::c_int],
            value: ::ctor::Ctor![*const [::ffi_11::c_int]],
        ) -> (&'a ::ffi_11::c_int, &'a mut *const [::ffi_11::c_int]) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::insert(self,key,value)
        }
        #[inline(always)]
        #[must_use]
        pub fn try_insert<'a>(self: ::core::pin::Pin<&'a mut Self>, key: ::ctor::Ctor![::ffi_11::c_int], value: ::ctor::Ctor![*const [::ffi_11::c_int]]) -> ::core::result::Result<(&'a ::ffi_11::c_int, &'a mut *const [::ffi_11::c_int]), ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::OccupiedError<(&'a ::ffi_11::c_int, &'a mut *const [::ffi_11::c_int])>> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::try_insert(self, key, value)
        }
        #[inline(always)]
        pub fn remove(self: ::core::pin::Pin<&mut Self>, key: &::ffi_11::c_int) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::remove(self,key)
        }
    }
    unsafe impl ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map for crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE {
        type Key = ::ffi_11::c_int;
        type Value = *const [::ffi_11::c_int];
        type ValueMut<'a>
            = &'a mut *const [::ffi_11::c_int]
        where
            Self: 'a;
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_len(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_capacity(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_clear(self: ::core::pin::Pin<&mut Self>) {
            unsafe { crate::detail::__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_reserve(self: ::core::pin::Pin<&mut Self>, count: usize) {
            unsafe { crate::detail::__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, count) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_key_value(&self, key: &Self::Key, result_key: *mut *const Self::Key, result_value: *mut *const Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_mut(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> *mut Self::Value {
            unsafe { crate::detail::__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_insert(self: ::core::pin::Pin<&mut Self>, key: *mut Self::Key, value: *mut Self::Value, overwrite: bool, result_key: *mut *const Self::Key, result_value: *mut *mut Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, value, overwrite, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_remove(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<int, rs::SliceRef<const int>>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::SliceRef<const int>>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::SliceRef<const int>>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::SliceRef<const int>>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::SliceRef<const int>>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::SliceRef<const int>>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::SliceRef<const int>>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, rs::SliceRef<const int>>::try_emplace` could not be bound
    //   Function templates are not yet supported

    // error: class `absl::flat_hash_map<int, class crubit::test::Incomplete, 42>` could not be bound
    //   Type `absl::flat_hash_map<int, class crubit::test::Incomplete, 42>` uses forward-declared type `crate::crubit::test::Incomplete` as an argument to a layout-compatible generic type. This is not supported. For more on why, see crubit.rs/types#incomplete_types.

    // error: class `absl::flat_hash_map<int, class crubit::test::NoDestructor, 42>` could not be bound
    //   `crate::crubit::test::NoDestructor` can't be used in a Rust absl::flat_hash_map<_, V> because it has a deleted or non-public destructor

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < int , class crubit :: test :: NoMoveNoCopy , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < int , class crubit :: test :: NoMoveNoCopy , 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE {
        // error: `absl::flat_hash_map` methods could not be generated
        //   the value type `crate::crubit::test::NoMoveNoCopy` can't be constructed from an rvalue because it has no public move or copy constructor
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<int, crubit::test::NoMoveNoCopy>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::NoMoveNoCopy>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::NoMoveNoCopy>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::NoMoveNoCopy>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::NoMoveNoCopy>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::NoMoveNoCopy>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::NoMoveNoCopy>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::NoMoveNoCopy>::try_emplace` could not be bound
    //   Function templates are not yet supported

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < int , struct crubit :: test :: Bridged , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < int , struct crubit :: test :: Bridged , 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE {
        // error: `absl::flat_hash_map` methods could not be generated
        //   the value type `crate::Bridged` can't be accessed in place because it is a non-layout-compatible bridged type
        //   See crubit.rs/types.
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<int, crubit::test::Bridged>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::Bridged>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::Bridged>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::Bridged>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::Bridged>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::Bridged>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::Bridged>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::Bridged>::try_emplace` could not be bound
    //   Function templates are not yet supported

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < int , class crubit :: test :: CopyOnly , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < int , class crubit :: test :: CopyOnly , 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE {
        #[inline(always)]
        #[must_use]
        pub fn len(&self) -> usize {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::len(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::is_empty(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn capacity(&self) -> usize {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::capacity(self)
        }
        #[inline(always)]
        pub fn clear(self: ::core::pin::Pin<&mut Self>) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::clear(self)
        }
        #[inline(always)]
        pub fn reserve(self: ::core::pin::Pin<&mut Self>, additional: usize) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::reserve(self,additional)
        }
        #[inline(always)]
        #[must_use]
        pub fn contains_key(&self, key: &::ffi_11::c_int) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::contains_key(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get<'a>(
            &'a self,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<&'a crate::crubit::test::CopyOnly> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_key_value<'a>(
            &'a self,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<(&'a ::ffi_11::c_int, &'a crate::crubit::test::CopyOnly)>
        {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_key_value(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_mut<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<&'a mut crate::crubit::test::CopyOnly> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_mut(self,key)
        }
        #[inline(always)]
        pub fn insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![::ffi_11::c_int],
            value: ::ctor::Ctor![crate::crubit::test::CopyOnly],
        ) -> (&'a ::ffi_11::c_int, &'a mut crate::crubit::test::CopyOnly) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::insert(self,key,value)
        }
        #[inline(always)]
        #[must_use]
        pub fn try_insert<'a>(self: ::core::pin::Pin<&'a mut Self>, key: ::ctor::Ctor![::ffi_11::c_int], value: ::ctor::Ctor![crate::crubit::test::CopyOnly]) -> ::core::result::Result<(&'a ::ffi_11::c_int, &'a mut crate::crubit::test::CopyOnly), ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::OccupiedError<(&'a ::ffi_11::c_int, &'a mut crate::crubit::test::CopyOnly)>> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::try_insert(self, key, value)
        }
        #[inline(always)]
        pub fn remove(self: ::core::pin::Pin<&mut Self>, key: &::ffi_11::c_int) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::remove(self,key)
        }
    }
    unsafe impl ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map for crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE {
        type Key = ::ffi_11::c_int;
        type Value = crate::crubit::test::CopyOnly;
        type ValueMut<'a>
            = &'a mut crate::crubit::test::CopyOnly
        where
            Self: 'a;
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_len(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_capacity(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_clear(self: ::core::pin::Pin<&mut Self>) {
            unsafe { crate::detail::__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_reserve(self: ::core::pin::Pin<&mut Self>, count: usize) {
            unsafe { crate::detail::__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, count) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_key_value(&self, key: &Self::Key, result_key: *mut *const Self::Key, result_value: *mut *const Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_mut(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> *mut Self::Value {
            unsafe { crate::detail::__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_insert(self: ::core::pin::Pin<&mut Self>, key: *mut Self::Key, value: *mut Self::Value, overwrite: bool, result_key: *mut *const Self::Key, result_value: *mut *mut Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, value, overwrite, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_remove(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<int, crubit::test::CopyOnly>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::CopyOnly>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::CopyOnly>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::CopyOnly>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::CopyOnly>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::CopyOnly>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::CopyOnly>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, crubit::test::CopyOnly>::try_emplace` could not be bound
    //   Function templates are not yet supported

    // error: class `absl::flat_hash_map<int, class crubit::test::NoDelete, 42>` could not be bound
    //   `crate::crubit::test::NoDelete` can't be used in a Rust absl::flat_hash_map<_, V> because it has a deleted or non-public operator delete

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < int , class absl :: Span < const int >, 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < int , class absl :: Span < const int >, 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE {
        // error: `absl::flat_hash_map` methods could not be generated
        //   the value type `::_y__y_third_party_y_absl_y_types_x_span::absl::Span<'_,::ffi_11::c_int>` has a lifetime, which is not yet supported
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEC1Ev(&raw mut tmp as*mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<int, absl::Span<const int>>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, absl::Span<const int>>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, absl::Span<const int>>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, absl::Span<const int>>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, absl::Span<const int>>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, absl::Span<const int>>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, absl::Span<const int>>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, absl::Span<const int>>::try_emplace` could not be bound
    //   Function templates are not yet supported

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < int , class std :: basic_string_view < char , struct std :: char_traits < char >>, 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE
    {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("absl :: flat_hash_map < int , class std :: basic_string_view < char , struct std :: char_traits < char >>, 42 >"),crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE);
    impl __CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE {
        #[inline(always)]
        #[must_use]
        pub fn len(&self) -> usize {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::len(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::is_empty(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn capacity(&self) -> usize {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::capacity(self)
        }
        #[inline(always)]
        pub fn clear(self: ::core::pin::Pin<&mut Self>) {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::clear(self)
        }
        #[inline(always)]
        pub fn reserve(self: ::core::pin::Pin<&mut Self>, additional: usize) {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::reserve(self, additional)
        }
        #[inline(always)]
        #[must_use]
        pub fn contains_key(&self, key: &::ffi_11::c_int) -> bool {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::contains_key(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get<'a>(&'a self, key: &::ffi_11::c_int) -> ::core::option::Option<&'a ::cc_std::std::__u::raw_string_view> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_key_value<'a>(&'a self, key: &::ffi_11::c_int) -> ::core::option::Option<(&'a ::ffi_11::c_int, &'a ::cc_std::std::__u::raw_string_view)> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_key_value(self, key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_mut<'a>(self: ::core::pin::Pin<&'a mut Self>, key: &::ffi_11::c_int) -> ::core::option::Option<&'a mut ::cc_std::std::__u::raw_string_view> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_mut(self, key)
        }
        #[inline(always)]
        pub fn insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![::ffi_11::c_int],
            value: ::ctor::Ctor![::cc_std::std::__u::raw_string_view],
        ) -> (
            &'a ::ffi_11::c_int,
            &'a mut ::cc_std::std::__u::raw_string_view,
        ) {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::insert(self, key, value)
        }
        #[inline(always)]
        #[must_use]
        pub fn try_insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![::ffi_11::c_int],
            value: ::ctor::Ctor![::cc_std::std::__u::raw_string_view],
        ) -> ::core::result::Result<
            (
                &'a ::ffi_11::c_int,
                &'a mut ::cc_std::std::__u::raw_string_view,
            ),
            ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::OccupiedError<(
                &'a ::ffi_11::c_int,
                &'a mut ::cc_std::std::__u::raw_string_view,
            )>,
        > {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::try_insert(self, key, value)
        }
        #[inline(always)]
        pub fn remove(self: ::core::pin::Pin<&mut Self>, key: &::ffi_11::c_int) -> bool {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::remove(self, key)
        }
    }
    unsafe impl ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map for crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE {
        type Key = ::ffi_11::c_int;
        type Value = ::cc_std::std::__u::raw_string_view;
        type ValueMut<'a>
            = &'a mut ::cc_std::std::__u::raw_string_view
        where
            Self: 'a;
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_len(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_capacity(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_clear(self: ::core::pin::Pin<&mut Self>) {
            unsafe { crate::detail::__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_reserve(self: ::core::pin::Pin<&mut Self>, count: usize) {
            unsafe { crate::detail::__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, count) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_key_value(&self, key: &Self::Key, result_key: *mut *const Self::Key, result_value: *mut *const Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_mut(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> *mut Self::Value {
            unsafe { crate::detail::__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_insert(self: ::core::pin::Pin<&mut Self>, key: *mut Self::Key, value: *mut Self::Value, overwrite: bool, result_key: *mut *const Self::Key, result_value: *mut *mut Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, value, overwrite, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_remove(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<int, std::string_view>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, std::string_view>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, std::string_view>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, std::string_view>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, std::string_view>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, std::string_view>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, std::string_view>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, std::string_view>::try_emplace` could not be bound
    //   Function templates are not yet supported

    /// An empty implementation of absl::flat_hash_map to test code generation
    /// without the absl dependency.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE"]
    #[repr(C)]
    #[cfg_attr(
        not(doc),
        doc = "CRUBIT_ANNOTATE: cpp_type=absl :: flat_hash_map < int , unsigned long , 42 >"
    )]
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
    pub struct __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("absl :: flat_hash_map < int , unsigned long , 42 >"),
        crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE
    );
    impl __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
        #[inline(always)]
        #[must_use]
        pub fn len(&self) -> usize {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::len(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn is_empty(&self) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::is_empty(self)
        }
        #[inline(always)]
        #[must_use]
        pub fn capacity(&self) -> usize {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::capacity(self)
        }
        #[inline(always)]
        pub fn clear(self: ::core::pin::Pin<&mut Self>) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::clear(self)
        }
        #[inline(always)]
        pub fn reserve(self: ::core::pin::Pin<&mut Self>, additional: usize) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::reserve(self,additional)
        }
        #[inline(always)]
        #[must_use]
        pub fn contains_key(&self, key: &::ffi_11::c_int) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::contains_key(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get<'a>(
            &'a self,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<&'a ::ffi_11::c_ulong> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_key_value<'a>(
            &'a self,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<(&'a ::ffi_11::c_int, &'a ::ffi_11::c_ulong)> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_key_value(self,key)
        }
        #[inline(always)]
        #[must_use]
        pub fn get_mut<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: &::ffi_11::c_int,
        ) -> ::core::option::Option<&'a mut ::ffi_11::c_ulong> {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::get_mut(self,key)
        }
        #[inline(always)]
        pub fn insert<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            key: ::ctor::Ctor![::ffi_11::c_int],
            value: ::ctor::Ctor![::ffi_11::c_ulong],
        ) -> (&'a ::ffi_11::c_int, &'a mut ::ffi_11::c_ulong) {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::insert(self,key,value)
        }
        #[inline(always)]
        #[must_use]
        pub fn try_insert<'a>(self: ::core::pin::Pin<&'a mut Self>, key: ::ctor::Ctor![::ffi_11::c_int], value: ::ctor::Ctor![::ffi_11::c_ulong]) -> ::core::result::Result<(&'a ::ffi_11::c_int, &'a mut ::ffi_11::c_ulong), ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::OccupiedError<(&'a ::ffi_11::c_int, &'a mut ::ffi_11::c_ulong)>> {
            <Self as ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::try_insert(self, key, value)
        }
        #[inline(always)]
        pub fn remove(self: ::core::pin::Pin<&mut Self>, key: &::ffi_11::c_int) -> bool {
            <Self as::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map>::remove(self,key)
        }
    }
    unsafe impl ::_y__y_third_party_y_crubit_y_rs_bindings_from_cc_y_test_y_absl_flat_hash_map_x_flat_hash_map::Map for crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
        type Key = ::ffi_11::c_int;
        type Value = ::ffi_11::c_ulong;
        type ValueMut<'a>
            = &'a mut ::ffi_11::c_ulong
        where
            Self: 'a;
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_len(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_capacity(&self) -> usize {
            unsafe { crate::detail::__crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_clear(self: ::core::pin::Pin<&mut Self>) {
            unsafe { crate::detail::__crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_reserve(self: ::core::pin::Pin<&mut Self>, count: usize) {
            unsafe { crate::detail::__crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, count) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_key_value(&self, key: &Self::Key, result_key: *mut *const Self::Key, result_value: *mut *const Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_get_mut(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> *mut Self::Value {
            unsafe { crate::detail::__crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_insert(self: ::core::pin::Pin<&mut Self>, key: *mut Self::Key, value: *mut Self::Value, overwrite: bool, result_key: *mut *const Self::Key, result_value: *mut *mut Self::Value) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key, value, overwrite, result_key, result_value) }
        }
        #[doc(hidden)]
        #[inline(always)]
        unsafe fn raw_remove(self: ::core::pin::Pin<&mut Self>, key: &Self::Key) -> bool {
            unsafe { crate::detail::__crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(self, key) }
        }
    }

    impl Default for __CcTemplateInstN4absl13flat_hash_mapIimLi42EEE {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIimLi42EEC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    // error: function `absl::flat_hash_map<int, unsigned long>::HarmlessTemplateFunction` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, unsigned long>::contains` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, unsigned long>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, unsigned long>::find` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, unsigned long>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, unsigned long>::end` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, unsigned long>::erase` could not be bound
    //   Function templates are not yet supported

    // error: function `absl::flat_hash_map<int, unsigned long>::try_emplace` could not be bound
    //   Function templates are not yet supported
}
mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test9GetAtFiveEN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS3_11char_traitsIcEEEELi42EEE<
            'lt,
        >(
            __return: *mut ::core::ffi::c_void,
            map: *mut crate::crubit::test::MapWithStringViewValue,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12GetRefAtFiveERKN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS3_11char_traitsIcEEEELi42EEE<
            'map,
        >(
            map: &'map crate::crubit::test::MapWithStringViewValue,
        ) -> ::cref::CRef<'map, ::cc_std::std::__u::raw_string_view>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test13GetSpanAtFiveEN4absl13flat_hash_mapIiNS1_4SpanIKiEELi42EEE<
            'lt,
        >(
            __return: *mut ::core::ffi::c_void,
            map: *mut crate::crubit::test::MapWithSpanValue,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test17GetSliceRefAtFiveEN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE(
            map: *mut crate::crubit::test::MapWithSliceRefValue,
        ) -> *const [::ffi_11::c_int];
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12NoDestructoraSERKS1_<
            '__param_0,
            '__this,
        >(
            __this: ::core::pin::Pin<&'__this mut crate::crubit::test::NoDestructor>,
            __param_0: &'__param_0 crate::crubit::test::NoDestructor,
        ) -> ::core::pin::Pin<&'__this mut crate::crubit::test::NoDestructor>;
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test8NoDeleteC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test12NoMoveNoCopyC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test8CopyOnlyC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        );
        pub(crate) unsafe fn __rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEE4baseEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
        );
        pub(crate) unsafe fn __rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKcEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorIPKcEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const ::ffi_11::c_char,
        );
        pub(crate) unsafe fn __rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorIPKcE4baseEv<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
        ) -> *const ::ffi_11::c_char;
        pub(crate) unsafe fn __rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__fdf38745__ZNSt3__u16reverse_iteratorIPKiEC1ES2_(
            __this: *mut ::core::ffi::c_void,
            __x: *const ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__6ecbc2e1__ZNKSt3__u16reverse_iteratorIPKiE4baseEv<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__780bb1b7__ZNSt3__u16reverse_iteratorIPKwEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__83c617d2__ZNSt3__u11__wrap_iterIPKiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__45d7cb71__ZNKSt3__u11__wrap_iterIPKiEplEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        );
        pub(crate)unsafe fn __rust_thunk__c15f65fd__ZNSt3__u11__wrap_iterIPKiEpLEl<'__this>(__this: &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,__n: isize)->&'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        pub(crate) unsafe fn __rust_thunk__ee08d685__ZNKSt3__u11__wrap_iterIPKiEmiEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        );
        pub(crate)unsafe fn __rust_thunk__55a37ce5__ZNSt3__u11__wrap_iterIPKiEmIEl<'__this>(__this: &'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,__n: isize)->&'__this mut crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        pub(crate) unsafe fn __rust_thunk__f7b43e06__ZNKSt3__u11__wrap_iterIPKiEixEl<'__this>(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__7fa98692__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__4e54b5b5__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__0f590027__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__022263c7__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__ea271768__ZNKSt3__u4spanIKiLm18446744073709551615EE4sizeEv<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__7475e8e4__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__0e958be1__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__b2c6c9fa__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __idx: usize,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__e6dd0be6__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__4df28266__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__b265a8aa__ZNKSt3__u4spanIKiLm18446744073709551615EE4dataEv<
            '__this,
        >(
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__626b734c__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__09c98b89__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__af6f680d__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__e01cc11e__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE>,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE>,
            count: usize,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE,
            key: &crate::crubit::test::CopyOnly,
            result_key: *mut *const crate::crubit::test::CopyOnly,
            result_value: *mut *const ::ffi_11::c_int,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE>,
            key: &crate::crubit::test::CopyOnly,
        ) -> *mut ::ffi_11::c_int;
        pub(crate) unsafe fn __crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE>,
            key: *mut crate::crubit::test::CopyOnly,
            value: *mut ::ffi_11::c_int,
            overwrite: bool,
            result_key: *mut *const crate::crubit::test::CopyOnly,
            result_value: *mut *mut ::ffi_11::c_int,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE>,
            key: &crate::crubit::test::CopyOnly,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE>,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE>,
            count: usize,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE,
            key: &::cc_std::std::__u::raw_string_view,
            result_key: *mut *const ::cc_std::std::__u::raw_string_view,
            result_value: *mut *const ::ffi_11::c_int,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE>,
            key: &::cc_std::std::__u::raw_string_view,
        ) -> *mut ::ffi_11::c_int;
        pub(crate) unsafe fn __crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE>,
            key: *mut ::cc_std::std::__u::raw_string_view,
            value: *mut ::ffi_11::c_int,
            overwrite: bool,
            result_key: *mut *const ::cc_std::std::__u::raw_string_view,
            result_value: *mut *mut ::ffi_11::c_int,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE>,
            key: &::cc_std::std::__u::raw_string_view,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN2rs6StrRefELi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE>,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE>,
            count: usize,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE,
            key: &::ffi_11::c_int,
            result_key: *mut *const ::ffi_11::c_int,
            result_value: *mut *const *const [::ffi_11::c_int],
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE>,
            key: &::ffi_11::c_int,
        ) -> *mut *const [::ffi_11::c_int];
        pub(crate) unsafe fn __crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE>,
            key: *mut ::ffi_11::c_int,
            value: *mut *const [::ffi_11::c_int],
            overwrite: bool,
            result_key: *mut *const ::ffi_11::c_int,
            result_value: *mut *mut *const [::ffi_11::c_int],
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE>,
            key: &::ffi_11::c_int,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE>,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE>,
            count: usize,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE,
            key: &::ffi_11::c_int,
            result_key: *mut *const ::ffi_11::c_int,
            result_value: *mut *const crate::crubit::test::CopyOnly,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE>,
            key: &::ffi_11::c_int,
        ) -> *mut crate::crubit::test::CopyOnly;
        pub(crate) unsafe fn __crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE>,
            key: *mut ::ffi_11::c_int,
            value: *mut crate::crubit::test::CopyOnly,
            overwrite: bool,
            result_key: *mut *const ::ffi_11::c_int,
            result_value: *mut *mut crate::crubit::test::CopyOnly,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE>,
            key: &::ffi_11::c_int,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE>,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE>,
            count: usize,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE,
            key: &::ffi_11::c_int,
            result_key: *mut *const ::ffi_11::c_int,
            result_value: *mut *const ::cc_std::std::__u::raw_string_view,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE>,
            key: &::ffi_11::c_int,
        ) -> *mut ::cc_std::std::__u::raw_string_view;
        pub(crate) unsafe fn __crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE>,
            key: *mut ::ffi_11::c_int,
            value: *mut ::cc_std::std::__u::raw_string_view,
            overwrite: bool,
            result_key: *mut *const ::ffi_11::c_int,
            result_value: *mut *mut ::cc_std::std::__u::raw_string_view,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE>,
            key: &::ffi_11::c_int,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__ea9eadcb__ZN4absl13flat_hash_mapIimLi42EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_len___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_capacity___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE,
        ) -> usize;
        pub(crate) unsafe fn __crubit_flat_hash_map_clear___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_reserve___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>,
            count: usize,
        );
        pub(crate) unsafe fn __crubit_flat_hash_map_get_key_value___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: &crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE,
            key: &::ffi_11::c_int,
            result_key: *mut *const ::ffi_11::c_int,
            result_value: *mut *const ::ffi_11::c_ulong,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_get_mut___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>,
            key: &::ffi_11::c_int,
        ) -> *mut ::ffi_11::c_ulong;
        pub(crate) unsafe fn __crubit_flat_hash_map_insert___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>,
            key: *mut ::ffi_11::c_int,
            value: *mut ::ffi_11::c_ulong,
            overwrite: bool,
            result_key: *mut *const ::ffi_11::c_int,
            result_value: *mut *mut ::ffi_11::c_ulong,
        ) -> bool;
        pub(crate) unsafe fn __crubit_flat_hash_map_remove___CcTemplateInstN4absl13flat_hash_mapIimLi42EEE___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fabsl_5fflat_5fhash_5fmap_3aisolated(
            __this: ::core::pin::Pin<&mut crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE>,
            key: &::ffi_11::c_int,
        ) -> bool;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::NoDestructor>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::NoDestructor>() == 1);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NoDestructor: Copy,Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::NoDelete>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::NoDelete>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::NoDelete: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NoDelete: Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::NoMoveNoCopy>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::NoMoveNoCopy>() == 1);
    static_assertions::assert_not_impl_any!(crate::crubit::test::NoMoveNoCopy: Copy,Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::CopyOnly>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::CopyOnly>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::CopyOnly: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::CopyOnly: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE>()==8);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE>()==8);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,current)==0);
    assert!(
        ::core::mem::size_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKcEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKiEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u16reverse_iteratorIPKwEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::zz_template_instantiations::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __i_
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>()==16);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>()==8);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE: Drop);
    assert!(::core::mem::offset_of!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,__data_)==0);
    assert!(::core::mem::offset_of!(crate::zz_template_instantiations::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,__size_)==8);
    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test12NoMoveNoCopyEiLi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test7BridgedEiLi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIN6crubit4test8CopyOnlyEiLi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapINSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEEiLi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs6StrRefELi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN2rs8SliceRefIKiEELi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test12NoMoveNoCopyELi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test7BridgedELi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiN6crubit4test8CopyOnlyELi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNS_4SpanIKiEELi42EEE: Drop);

    assert!(::core::mem::size_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE>()==1);
    assert!(::core::mem::align_of::<crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE>()==1);
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIiNSt3__u17basic_string_viewIcNS1_11char_traitsIcEEEELi42EEE: Drop);

    assert!(
        ::core::mem::size_of::<
            crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE,
        >() == 1
    );
    assert!(
        ::core::mem::align_of::<
            crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE,
        >() == 1
    );
    static_assertions::assert_impl_all!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::zz_template_instantiations::__CcTemplateInstN4absl13flat_hash_mapIimLi42EEE: Drop);

    assert!(::core::mem::size_of::<*const [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*const [::ffi_11::c_int]>() == 8);
};
