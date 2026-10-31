// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:boxes_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
extern crate alloc;

pub mod test {
    #[inline(always)]
    pub fn MakeBox(v: ::ffi_11::c_int) -> ::alloc::boxed::Box<::ffi_11::c_int> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::alloc::boxed::Box<::ffi_11::c_int>>::uninit();
            crate::detail::__rust_thunk___ZN4test7MakeBoxEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                v,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn ReadBox<'b>(b: &'b ::alloc::boxed::Box<::ffi_11::c_int>) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN4test7ReadBoxERKN2rs3BoxIiEE(b) }
    }

    #[inline(always)]
    pub fn MutateBox<'b>(b: &'b mut ::alloc::boxed::Box<::ffi_11::c_int>) {
        unsafe { crate::detail::__rust_thunk___ZN4test9MutateBoxERN2rs3BoxIiEE(b) }
    }

    #[inline(always)]
    pub fn ConsumeBox(b: ::alloc::boxed::Box<::ffi_11::c_int>) -> ::ffi_11::c_int {
        let mut b = ::core::mem::MaybeUninit::new(b);
        unsafe { crate::detail::__rust_thunk___ZN4test10ConsumeBoxEN2rs3BoxIiEE(b.as_mut_ptr()) }
    }

    #[inline(always)]
    pub fn RoundtripBox(
        b: ::alloc::boxed::Box<::ffi_11::c_int>,
    ) -> ::alloc::boxed::Box<::ffi_11::c_int> {
        let mut b = ::core::mem::MaybeUninit::new(b);
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::alloc::boxed::Box<::ffi_11::c_int>>::uninit();
            crate::detail::__rust_thunk___ZN4test12RoundtripBoxEN2rs3BoxIiEE(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                b.as_mut_ptr(),
            );
            __crubit_return.assume_init()
        }
    }

    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `b`: raw pointer
    #[inline(always)]
    pub unsafe fn ReadBoxPtr(b: *const ::alloc::boxed::Box<::ffi_11::c_int>) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN4test10ReadBoxPtrEPKN2rs3BoxIiEE(b) }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N4test5PointE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=:: test :: Point
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Point {
        pub x: ::ffi_11::c_int,
        pub y: ::ffi_11::c_int,
    }
    impl !Send for Point {}
    impl !Sync for Point {}
    unsafe impl ::cxx::ExternType for Point {
        type Id = ::cxx::type_id!(":: test :: Point");
        type Kind = ::cxx::kind::Trivial;
    }
    impl ::core::fmt::Debug for Point {
        fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            formatter.debug_struct("Point").field("x", &self.x).field("y", &self.y).finish()
        }
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!(":: test :: Point"),
        crate::test::Point
    );

    impl Default for Point {
        #[inline(always)]
        fn default() -> Self {
            unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
        }
    }

    #[inline(always)]
    pub fn MakeBoxedPoint() -> ::alloc::boxed::Box<crate::test::Point> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::alloc::boxed::Box<crate::test::Point>>::uninit();
            crate::detail::__rust_thunk___ZN4test14MakeBoxedPointEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn PointX(b: ::alloc::boxed::Box<crate::test::Point>) -> ::ffi_11::c_int {
        let mut b = ::core::mem::MaybeUninit::new(b);
        unsafe {
            crate::detail::__rust_thunk___ZN4test6PointXEN2rs3BoxINS_5PointEEE(b.as_mut_ptr())
        }
    }

    #[cfi_encoding = "N4test6HolderE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=:: test :: Holder
    pub struct Holder {
        pub b: ::core::mem::ManuallyDrop<::alloc::boxed::Box<::ffi_11::c_int>>,
    }
    impl !Send for Holder {}
    impl !Sync for Holder {}
    unsafe impl ::cxx::ExternType for Holder {
        type Id = ::cxx::type_id!(":: test :: Holder");
        type Kind = ::cxx::kind::Trivial;
    }
    impl ::core::fmt::Debug for Holder {
        fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            formatter.debug_struct("Holder").finish_non_exhaustive()
        }
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!(":: test :: Holder"),
        crate::test::Holder
    );

    impl Drop for Holder {
        #[inline(always)]
        fn drop<'__this>(&'__this mut self) {
            unsafe { crate::detail::__rust_thunk___ZN4test6HolderD1Ev(self) }
        }
    }

    #[inline(always)]
    pub fn MakeHolder(v: ::ffi_11::c_int) -> crate::test::Holder {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::test::Holder>::uninit();
            crate::detail::__rust_thunk___ZN4test10MakeHolderEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                v,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn HolderValue<'h>(h: &'h crate::test::Holder) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN4test11HolderValueERKNS_6HolderE(h) }
    }

    #[inline(always)]
    pub fn SetHolderValue<'h>(h: &'h mut crate::test::Holder, v: ::ffi_11::c_int) {
        unsafe { crate::detail::__rust_thunk___ZN4test14SetHolderValueERNS_6HolderEi(h, v) }
    }

    #[cfi_encoding = "N4test5OuterE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=:: test :: Outer
    pub struct Outer {
        pub h: crate::test::Holder,
        pub tag: ::ffi_11::c_int,
    }
    impl !Send for Outer {}
    impl !Sync for Outer {}
    unsafe impl ::cxx::ExternType for Outer {
        type Id = ::cxx::type_id!(":: test :: Outer");
        type Kind = ::cxx::kind::Trivial;
    }
    impl ::core::fmt::Debug for Outer {
        fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            formatter.debug_struct("Outer").field("h", &self.h).field("tag", &self.tag).finish()
        }
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!(":: test :: Outer"),
        crate::test::Outer
    );

    #[inline(always)]
    pub fn MakeOuter(v: ::ffi_11::c_int) -> crate::test::Outer {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::test::Outer>::uninit();
            crate::detail::__rust_thunk___ZN4test9MakeOuterEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                v,
            );
            __crubit_return.assume_init()
        }
    }

    /// # Safety
    ///
    /// To call a function that accepts this type, you must uphold these requirements:
    /// * Document why the following public unsafe fields of this type cannot be misused by callee:
    ///   * `boxes`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'rs::Box<int>[2]': C-style arrays of non-primitive, non-enum types are not yet supported (b/575922365)
    #[cfi_encoding = "N4test11ArrayHolderE"]
    #[repr(C, align(8))]
    ///CRUBIT_ANNOTATE: cpp_type=:: test :: ArrayHolder
    pub struct ArrayHolder {
        /// Reason for representing this field as a blob of bytes:
        /// Unsupported type 'rs::Box<int>[2]': C-style arrays of non-primitive, non-enum types are not yet supported (b/575922365)
        pub(crate) boxes: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 16],
    }
    impl !Send for ArrayHolder {}
    impl !Sync for ArrayHolder {}
    unsafe impl ::cxx::ExternType for ArrayHolder {
        type Id = ::cxx::type_id!(":: test :: ArrayHolder");
        type Kind = ::cxx::kind::Trivial;
    }
    impl ::core::fmt::Debug for ArrayHolder {
        fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            formatter.debug_struct("ArrayHolder").finish_non_exhaustive()
        }
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!(":: test :: ArrayHolder"),
        crate::test::ArrayHolder
    );

    impl Drop for ArrayHolder {
        #[inline(always)]
        fn drop<'__this>(&'__this mut self) {
            unsafe { crate::detail::__rust_thunk___ZN4test11ArrayHolderD1Ev(self) }
        }
    }

    /// A type that is not Rust-movable (`!Unpin` in Rust) because it has a
    /// user-defined move constructor and destructor.
    ///
    /// Not Rust-movable: destructor or move constructor is user-provided
    #[::ctor::recursively_pinned(PinnedDrop)]
    #[cfi_encoding = "N4test10NonTrivialE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=:: test :: NonTrivial
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct NonTrivial {
        __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
        pub val: ::ffi_11::c_int,
    }
    impl !Send for NonTrivial {}
    impl !Sync for NonTrivial {}
    unsafe impl ::cxx::ExternType for NonTrivial {
        type Id = ::cxx::type_id!(":: test :: NonTrivial");
        type Kind = ::cxx::kind::Opaque;
    }
    impl ::core::fmt::Debug for NonTrivial {
        fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            formatter.debug_struct("NonTrivial").field("val", &self.val).finish()
        }
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!(":: test :: NonTrivial"),
        crate::test::NonTrivial
    );

    impl ::ctor::CtorNew<::ffi_11::c_int> for NonTrivial {
        type CtorType = ::ctor::Ctor![Self];
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
            unsafe {
                ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                    let mut v = args;
                    crate::detail::__rust_thunk___ZN4test10NonTrivialC1Ei(
                        __crubit_dest as *mut ::core::ffi::c_void,
                        v,
                    );
                })
            }
        }
    }
    impl ::ctor::CtorNew<(::ffi_11::c_int,)> for NonTrivial {
        type CtorType = ::ctor::Ctor![Self];
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
        }
    }

    impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for NonTrivial {
        type CtorType = ::ctor::Ctor![Self];
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
            unsafe {
                ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                    let mut other = args;
                    crate::detail::__rust_thunk___ZN4test10NonTrivialC1EOS0_(
                        __crubit_dest as *mut ::core::ffi::c_void,
                        other,
                    );
                })
            }
        }
    }
    impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for NonTrivial {
        type CtorType = ::ctor::Ctor![Self];
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
        }
    }

    impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for NonTrivial {
        #[inline(always)]
        fn assign<'__this>(
            self: ::core::pin::Pin<&'__this mut Self>,
            other: ::ctor::RvalueReference<'_, Self>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN4test10NonTrivialaSEOS0_(self, other);
            }
        }
    }

    impl ::ctor::PinnedDrop for NonTrivial {
        #[inline(always)]
        unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
            unsafe { crate::detail::__rust_thunk___ZN4test10NonTrivialD1Ev(self) }
        }
    }

    #[cfi_encoding = "N4test9PinHolderE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=:: test :: PinHolder
    pub struct PinHolder {
        pub p: ::core::mem::ManuallyDrop<
            ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
        >,
    }
    impl !Send for PinHolder {}
    impl !Sync for PinHolder {}
    unsafe impl ::cxx::ExternType for PinHolder {
        type Id = ::cxx::type_id!(":: test :: PinHolder");
        type Kind = ::cxx::kind::Trivial;
    }
    impl ::core::fmt::Debug for PinHolder {
        fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            formatter.debug_struct("PinHolder").finish_non_exhaustive()
        }
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!(":: test :: PinHolder"),
        crate::test::PinHolder
    );

    impl Drop for PinHolder {
        #[inline(always)]
        fn drop<'__this>(&'__this mut self) {
            unsafe { crate::detail::__rust_thunk___ZN4test9PinHolderD1Ev(self) }
        }
    }

    #[diagnostic::on_unimplemented(
        message = "binding generation for function failed\n`Holder` cannot be passed by value: it is Rust-movable but has no C++ copy or move constructor (for example because it contains an `rs::Box`), so the C++ thunk cannot relocate it. Pass it by reference or pointer instead. See crubit.rs/types/box#fields"
    )]
    pub trait BindingFailedFor_ZN4test18TakesHolderByValueENS_6HolderE {}
    #[inline(always)]
    pub fn TakesHolderByValue(h: crate::test::Holder)
    where
        for<'error> &'error (): BindingFailedFor_ZN4test18TakesHolderByValueENS_6HolderE,
    {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
        )
    }

    #[inline(always)]
    pub fn MakePinBoxNonTrivial(
        val: ::ffi_11::c_int,
    ) -> ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>> {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
            >::uninit();
            crate::detail::__rust_thunk___ZN4test20MakePinBoxNonTrivialEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                val,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn SharePinBoxNonTrivial<'b>(
        b: &'b ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
    ) -> ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>> {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
            >::uninit();
            crate::detail::__rust_thunk___ZN4test21SharePinBoxNonTrivialERKN2rs6PinBoxINS_10NonTrivialEEE(&raw mut __crubit_return as*mut::core::ffi::c_void,b);
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn NonTrivialUseCount<'b>(
        b: &'b ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
    ) -> ::ffi_11::c_long {
        unsafe {
            crate::detail::__rust_thunk___ZN4test18NonTrivialUseCountERKN2rs6PinBoxINS_10NonTrivialEEE(b)
        }
    }

    #[inline(always)]
    pub fn ConsumePinBox(b: ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>) {
        let mut b = ::core::mem::MaybeUninit::new(b);
        unsafe {
            crate::detail::__rust_thunk___ZN4test13ConsumePinBoxEN2rs6PinBoxINS_10NonTrivialEEE(
                b.as_mut_ptr(),
            )
        }
    }

    #[inline(always)]
    pub fn ReadPinBoxNonTrivial<'b>(
        b: &'b ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
    ) -> ::ffi_11::c_int {
        unsafe {
            crate::detail::__rust_thunk___ZN4test20ReadPinBoxNonTrivialERKN2rs6PinBoxINS_10NonTrivialEEE(b)
        }
    }

    #[inline(always)]
    pub fn MakePinBoxUnpin() -> ::core::pin::Pin<::alloc::boxed::Box<::ffi_11::c_int>> {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                ::core::pin::Pin<::alloc::boxed::Box<::ffi_11::c_int>>,
            >::uninit();
            crate::detail::__rust_thunk___ZN4test15MakePinBoxUnpinEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn MakeSliceBox(n: ::ffi_11::c_ulong) -> ::alloc::boxed::Box<[::ffi_11::c_int]> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::alloc::boxed::Box<[::ffi_11::c_int]>>::uninit();
            crate::detail::__rust_thunk___ZN4test12MakeSliceBoxEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                n,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn SumSliceBox<'b>(b: &'b ::alloc::boxed::Box<[::ffi_11::c_int]>) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN4test11SumSliceBoxERKN2rs8SliceBoxIiEE(b) }
    }

    #[inline(always)]
    pub fn ConsumeSliceBox(b: ::alloc::boxed::Box<[::ffi_11::c_int]>) -> ::ffi_11::c_ulong {
        let mut b = ::core::mem::MaybeUninit::new(b);
        unsafe {
            crate::detail::__rust_thunk___ZN4test15ConsumeSliceBoxEN2rs8SliceBoxIiEE(b.as_mut_ptr())
        }
    }

    #[inline(always)]
    pub fn MakeStrBox() -> ::alloc::boxed::Box<str> {
        unsafe {
            let mut __crubit_return =
                ::core::mem::MaybeUninit::<::alloc::boxed::Box<str>>::uninit();
            crate::detail::__rust_thunk___ZN4test10MakeStrBoxEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn StrBoxLen<'b>(b: &'b ::alloc::boxed::Box<str>) -> ::ffi_11::c_ulong {
        unsafe { crate::detail::__rust_thunk___ZN4test9StrBoxLenERKN2rs6StrBoxE(b) }
    }

    #[inline(always)]
    pub fn ConsumeStrBox(b: ::alloc::boxed::Box<str>) -> ::ffi_11::c_ulong {
        let mut b = ::core::mem::MaybeUninit::new(b);
        unsafe { crate::detail::__rust_thunk___ZN4test13ConsumeStrBoxEN2rs6StrBoxE(b.as_mut_ptr()) }
    }

    // error: function `test::Rejected` could not be bound
    //   Cannot use an error type `rs :: Box < test :: NonTrivial >` by value:
    //     `rs::Box<T>` cannot be used with `T` = `crate::test::NonTrivial` because `T` is not Rust-movable (`!Unpin`): Rust could move the pointee out of the box, which would relocate a C++ object whose move constructor is non-trivial or deleted. Use `rs::PinBox<T>` (`Pin<Box<T>>`) instead. See crubit.rs/types/box#pinbox.

    // error: function `test::TakesRejected` could not be bound
    //   Unsupported parameter type `rs::Box<test::NonTrivial> b`:
    //     `rs::Box<T>` cannot be used with `T` = `crate::test::NonTrivial` because `T` is not Rust-movable (`!Unpin`): Rust could move the pointee out of the box, which would relocate a C++ object whose move constructor is non-trivial or deleted. Use `rs::PinBox<T>` (`Pin<Box<T>>`) instead. See crubit.rs/types/box#pinbox.

    // error: function `test::RejectedSlice` could not be bound
    //   Cannot use an error type `rs :: SliceBox < test :: NonTrivial >` by value:
    //     `rs::SliceBox<T>` cannot be used with `T` = `crate::test::NonTrivial` because `T` is not Rust-movable (`!Unpin`): Rust could move the pointee out of the box, which would relocate a C++ object whose move constructor is non-trivial or deleted. See crubit.rs/types/box#pinbox.
}

// namespace test

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstSt16initializer_listIiE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: initializer_list < int >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstSt16initializer_listIiE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __begin_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstSt16initializer_listIiE {}
impl !Sync for __CcTemplateInstSt16initializer_listIiE {}
impl ::core::fmt::Debug for __CcTemplateInstSt16initializer_listIiE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("__CcTemplateInstSt16initializer_listIiE").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: std :: initializer_list < int >"),
    crate::__CcTemplateInstSt16initializer_listIiE
);
impl __CcTemplateInstSt16initializer_listIiE {
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::cc_template_inst_st16initializer_list_ii_e::data(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe { self::cc_template_inst_st16initializer_list_ii_e::size(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe { self::cc_template_inst_st16initializer_list_ii_e::empty(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::cc_template_inst_st16initializer_list_ii_e::begin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::cc_template_inst_st16initializer_list_ii_e::end(self) }
    }
}

impl Default for __CcTemplateInstSt16initializer_listIiE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__d3f8dea6__ZNSt16initializer_listIiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_st16initializer_list_ii_e {
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstSt16initializer_listIiE,
    ) -> *const ::ffi_11::c_int {
        unsafe { (&raw const *__this).cast::<*const ::ffi_11::c_int>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstSt16initializer_listIiE,
    ) -> usize {
        unsafe { (&raw const *__this).byte_add(8).cast::<usize>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstSt16initializer_listIiE,
    ) -> bool {
        unsafe { crate::detail::__rust_thunk__c77c867f__ZNKSt16initializer_listIiE5emptyEv(__this) }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstSt16initializer_listIiE,
    ) -> *const ::ffi_11::c_int {
        unsafe { (&raw const *__this).cast::<*const ::ffi_11::c_int>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstSt16initializer_listIiE,
    ) -> *const ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk__2f3b1219__ZNKSt16initializer_listIiE3endEv(__this) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: reverse_iterator < class std :: __wrap_iter < const struct test :: NonTrivial *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
{
}
impl !Sync
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
{
}
impl ::core::fmt::Debug
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
{
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: std :: reverse_iterator < class std :: __wrap_iter < const struct test :: NonTrivial *>>"),crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE);
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipkn4test10_non_trivial_eeeee::base(self)
        }
    }
}

impl Default
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
{
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE) -> Self {
        let mut __x = args;
        let mut __x = ::core::mem::MaybeUninit::new(__x);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEC1ES6_(&raw mut tmp as*mut _,__x.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
    ) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE>>::from(
            args,
        )
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipkn4test10_non_trivial_eeeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
        unsafe {
            (&raw const *__this)
                .cast::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE>()
                .read()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: reverse_iterator < class std :: __wrap_iter < const int *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {}
impl ::core::fmt::Debug for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE")
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        ":: std :: reverse_iterator < class std :: __wrap_iter < const int *>>"
    ),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_ki_eeee::base(self)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE) -> Self {
        let mut __x = args;
        let mut __x = ::core::mem::MaybeUninit::new(__x);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(&raw mut tmp as*mut _,__x.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_ki_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            (&raw const *__this).cast::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>().read()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: reverse_iterator < class std :: __wrap_iter < struct test :: NonTrivial *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE {}
impl ::core::fmt::Debug
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
{
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        ":: std :: reverse_iterator < class std :: __wrap_iter < struct test :: NonTrivial *>>"
    ),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipn4test10_non_trivial_eeeee::base(self)
        }
    }
}

impl Default
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
{
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE) -> Self {
        let mut __x = args;
        let mut __x = ::core::mem::MaybeUninit::new(__x);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEC1ES5_(&raw mut tmp as*mut _,__x.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(
        args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
    ) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE>>::from(
            args,
        )
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ipn4test10_non_trivial_eeeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
        unsafe {
            (&raw const *__this)
                .cast::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE>()
                .read()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: reverse_iterator < class std :: __wrap_iter < int *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {}
impl ::core::fmt::Debug for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE")
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: std :: reverse_iterator < class std :: __wrap_iter < int *>>"),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_i_pi_eeee::base(self)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE) -> Self {
        let mut __x = args;
        let mut __x = ::core::mem::MaybeUninit::new(__x);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(&raw mut tmp as*mut _,__x.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_i_pi_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            (&raw const *__this).cast::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>().read()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: __wrap_iter < const struct test :: NonTrivial *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {}
impl ::core::fmt::Debug for __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE")
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: std :: __wrap_iter < const struct test :: NonTrivial *>"),
    crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE
);

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Add<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            >::uninit();
            crate::detail::__rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEplEl(&raw mut __crubit_return as*mut::core::ffi::c_void,self,__n);
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize>
    for __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE
{
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEpLEl(
                self, __n,
            );
        }
    }
}

impl<'__this> ::core::ops::Sub<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            >::uninit();
            crate::detail::__rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEmiEl(&raw mut __crubit_return as*mut::core::ffi::c_void,self,__n);
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize>
    for __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE
{
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEmIEl(
                self, __n,
            );
        }
    }
}

impl ::operator::CcIndex<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
    type Output<'ctnr> = &'ctnr crate::test::NonTrivial;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __n: isize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__3dcfade5__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEixEl(
                self, __n,
            )
        }
    }
}
impl ::core::ops::Index<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
    type Output = crate::test::NonTrivial;
    #[inline(always)]
    fn index(&self, index: isize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPKiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: __wrap_iter < const int *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {}
impl ::core::fmt::Debug for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("__CcTemplateInstNSt3__u11__wrap_iterIPKiEE").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: std :: __wrap_iter < const int *>"),
    crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
);

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPKiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Add<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPKiEplEl(
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
            crate::detail::__rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPKiEpLEl(self, __n);
        }
    }
}

impl<'__this> ::core::ops::Sub<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPKiEmiEl(
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
            crate::detail::__rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPKiEmIEl(self, __n);
        }
    }
}

impl ::operator::CcIndex<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    type Output<'ctnr> = &'ctnr ::ffi_11::c_int;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __n: isize) -> Self::Output<'ctnr> {
        unsafe { crate::detail::__rust_thunk__3dcfade5__ZNKSt3__u11__wrap_iterIPKiEixEl(self, __n) }
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
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: __wrap_iter < struct test :: NonTrivial *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {}
impl ::core::fmt::Debug for __CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE")
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: std :: __wrap_iter < struct test :: NonTrivial *>"),
    crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE
);

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Add<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            >::uninit();
            crate::detail::__rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPN4test10NonTrivialEEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEpLEl(
                self, __n,
            );
        }
    }
}

impl<'__this> ::core::ops::Sub<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            >::uninit();
            crate::detail::__rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPN4test10NonTrivialEEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEmIEl(
                self, __n,
            );
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: __wrap_iter < int *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {}
impl ::core::fmt::Debug for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.debug_struct("__CcTemplateInstNSt3__u11__wrap_iterIPiEE").finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: std :: __wrap_iter < int *>"),
    crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE
);

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Add<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPiEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPiEpLEl(self, __n);
        }
    }
}

impl<'__this> ::core::ops::Sub<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPiEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPiEmIEl(self, __n);
        }
    }
}

// error: item `rs::Box<test::NonTrivial>` could not be bound
//   `rs::Box<T>` cannot be used with `T` = `crate::test::NonTrivial` because `T` is not Rust-movable (`!Unpin`): Rust could move the pointee out of the box, which would relocate a C++ object whose move constructor is non-trivial or deleted. Use `rs::PinBox<T>` (`Pin<Box<T>>`) instead. See crubit.rs/types/box#pinbox.

// Type bindings for rs::Box<test::Point> suppressed due to being mapped to an existing Rust type (::alloc::boxed::Box<crate::test::Point>)

// Type bindings for rs::Box<int> suppressed due to being mapped to an existing Rust type (::alloc::boxed::Box<::ffi_11::c_int>)

// Type bindings for rs::PinBox<test::NonTrivial> suppressed due to being mapped to an existing Rust type (::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>)

// Type bindings for rs::PinBox<int> suppressed due to being mapped to an existing Rust type (::core::pin::Pin<::alloc::boxed::Box<::ffi_11::c_int>>)

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: span < const struct test :: NonTrivial , 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {}
impl !Sync for __CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {}
impl ::core::fmt::Debug
    for __CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE
{
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct(
                "__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE",
            )
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        ":: std :: span < const struct test :: NonTrivial , 18446744073709551615UL >"
    ),
    crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE
);
impl __CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
    #[must_use]
    #[inline(always)]
    pub fn first<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::first(self,__count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn last<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::last(self,__count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn subspan<'__this>(
        &'__this self,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::subspan(self,__offset,__count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::size(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size_bytes<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::size_bytes(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::empty(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CRef<'__this, crate::test::NonTrivial> {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::front(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CRef<'__this, crate::test::NonTrivial> {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::back(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *const crate::test::NonTrivial {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::data(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::begin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::end(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
    {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::rbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
    {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee::rend(self)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a2602314__ZNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl ::operator::CcIndex<usize>
    for __CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE
{
    type Output<'ctnr> = &'ctnr crate::test::NonTrivial;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __idx: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__863bfd93__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEixEm(self,__idx)
        }
    }
}
impl ::core::ops::Index<usize>
    for __CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE
{
    type Output = crate::test::NonTrivial;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

pub mod cc_template_inst_n_st3_u4span_ikn4test10_non_trivial_e_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn first<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__6ddb9499__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5firstEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn last<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__70f2e79a__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4lastEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn subspan<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__0b402a5e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> usize {
        unsafe { (&raw const *__this).byte_add(8).cast::<usize>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__db8aea99__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__adeb3748__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5emptyEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> ::cref::CRef<'__this, crate::test::NonTrivial> {
        unsafe {
            crate::detail::__rust_thunk__0eff2f4e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5frontEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> ::cref::CRef<'__this, crate::test::NonTrivial> {
        unsafe {
            crate::detail::__rust_thunk__0b709465__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4backEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> *const crate::test::NonTrivial {
        unsafe { (&raw const *__this).cast::<*const crate::test::NonTrivial>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            >::uninit();
            crate::detail::__rust_thunk__c3b9de9e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5beginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            >::uninit();
            crate::detail::__rust_thunk__c0096ae4__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE3endEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
    {
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE>::uninit();
            crate::detail::__rust_thunk__30ed2d1f__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE
    {
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE>::uninit();
            crate::detail::__rust_thunk__bd3e0276__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: span < const int , 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {}
impl !Sync for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {}
impl ::core::fmt::Debug for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE")
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: std :: span < const int , 18446744073709551615UL >"),
    crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE
);
impl __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    #[must_use]
    #[inline(always)]
    pub fn first<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::first(
                self, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn last<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::last(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn subspan<'__this>(
        &'__this self,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::subspan(
                self, __offset, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::size(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn size_bytes<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::size_bytes(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::empty(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::front(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::back(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::data(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::begin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::end(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::rbegin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::rend(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a2602314__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
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
            crate::detail::__rust_thunk__863bfd93__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm(
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

pub mod cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn first<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__6ddb9499__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn last<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__70f2e79a__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn subspan<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__0b402a5e__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> usize {
        unsafe { (&raw const *__this).byte_add(8).cast::<usize>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__db8aea99__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__adeb3748__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__0eff2f4e__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__0b709465__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> *const ::ffi_11::c_int {
        unsafe { (&raw const *__this).cast::<*const ::ffi_11::c_int>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__c3b9de9e__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__c0096ae4__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__30ed2d1f__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__bd3e0276__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: span < struct test :: NonTrivial , 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {}
impl !Sync for __CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {}
impl ::core::fmt::Debug
    for __CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE
{
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct(
                "__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE",
            )
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(
        ":: std :: span < struct test :: NonTrivial , 18446744073709551615UL >"
    ),
    crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE
);
impl __CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
    #[must_use]
    #[inline(always)]
    pub fn first<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::first(self,__count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn last<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::last(self,__count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn subspan<'__this>(
        &'__this self,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::subspan(self,__offset,__count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::size(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size_bytes<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::size_bytes(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::empty(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CMut<'__this, crate::test::NonTrivial> {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::front(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CMut<'__this, crate::test::NonTrivial> {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::back(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *mut crate::test::NonTrivial {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::data(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::begin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::end(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
    {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::rbegin(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
    {
        unsafe {
            self::cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee::rend(self)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a2602314__ZNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u4span_in4test10_non_trivial_e_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn first<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__6ddb9499__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5firstEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn last<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__70f2e79a__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4lastEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn subspan<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__0b402a5e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> usize {
        unsafe { (&raw const *__this).byte_add(8).cast::<usize>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__db8aea99__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__adeb3748__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5emptyEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, crate::test::NonTrivial> {
        unsafe {
            crate::detail::__rust_thunk__0eff2f4e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5frontEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, crate::test::NonTrivial> {
        unsafe {
            crate::detail::__rust_thunk__0b709465__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4backEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> *mut crate::test::NonTrivial {
        unsafe { (&raw const *__this).cast::<*mut crate::test::NonTrivial>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            >::uninit();
            crate::detail::__rust_thunk__c3b9de9e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5beginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            >::uninit();
            crate::detail::__rust_thunk__c0096ae4__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE3endEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
    {
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE>::uninit();
            crate::detail::__rust_thunk__30ed2d1f__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE
    {
        unsafe {
            let mut __crubit_return=::core::mem::MaybeUninit::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE>::uninit();
            crate::detail::__rust_thunk__bd3e0276__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4rendEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=:: std :: span < int , 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {}
impl !Sync for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {}
impl ::core::fmt::Debug for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter
            .debug_struct("__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE")
            .finish_non_exhaustive()
    }
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: std :: span < int , 18446744073709551615UL >"),
    crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE
);
impl __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    #[must_use]
    #[inline(always)]
    pub fn first<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::first(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn last<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::last(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn subspan<'__this>(
        &'__this self,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::subspan(
                self, __offset, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::size(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn size_bytes<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::size_bytes(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::empty(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::front(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::back(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *mut ::ffi_11::c_int {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::data(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::begin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::end(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::rbegin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::rend(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__a2602314__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn first<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__6ddb9499__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn last<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__70f2e79a__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn subspan<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__0b402a5e__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> usize {
        unsafe { (&raw const *__this).byte_add(8).cast::<usize>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__db8aea99__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__adeb3748__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__0eff2f4e__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__0b709465__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> *mut ::ffi_11::c_int {
        unsafe { (&raw const *__this).cast::<*mut ::ffi_11::c_int>().read() }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__c3b9de9e__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__c0096ae4__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__30ed2d1f__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__bd3e0276__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

// Type bindings for rs::SliceRef<const test::NonTrivial> suppressed due to being mapped to an existing Rust type (*const[crate::test::NonTrivial])

// Type bindings for rs::SliceRef<const int> suppressed due to being mapped to an existing Rust type (*const[::ffi_11::c_int])

// Type bindings for rs::SliceRef<test::NonTrivial> suppressed due to being mapped to an existing Rust type (*mut[crate::test::NonTrivial])

// Type bindings for rs::SliceRef<int> suppressed due to being mapped to an existing Rust type (*mut[::ffi_11::c_int])

// error: item `rs::SliceBox<test::NonTrivial>` could not be bound
//   `rs::SliceBox<T>` cannot be used with `T` = `crate::test::NonTrivial` because `T` is not Rust-movable (`!Unpin`): Rust could move the pointee out of the box, which would relocate a C++ object whose move constructor is non-trivial or deleted. See crubit.rs/types/box#pinbox.

// Type bindings for rs::SliceBox<int> suppressed due to being mapped to an existing Rust type (::alloc::boxed::Box<[::ffi_11::c_int]>)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN4test7MakeBoxEi(
            __return: *mut ::core::ffi::c_void,
            v: ::ffi_11::c_int,
        );
        #[link_name = "_ZN4test7ReadBoxERKN2rs3BoxIiEE"]
        pub(crate) unsafe fn __rust_thunk___ZN4test7ReadBoxERKN2rs3BoxIiEE<'b>(
            b: &'b ::alloc::boxed::Box<::ffi_11::c_int>,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZN4test9MutateBoxERN2rs3BoxIiEE"]
        pub(crate) unsafe fn __rust_thunk___ZN4test9MutateBoxERN2rs3BoxIiEE<'b>(
            b: &'b mut ::alloc::boxed::Box<::ffi_11::c_int>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test10ConsumeBoxEN2rs3BoxIiEE(
            b: *mut ::alloc::boxed::Box<::ffi_11::c_int>,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN4test12RoundtripBoxEN2rs3BoxIiEE(
            __return: *mut ::core::ffi::c_void,
            b: *mut ::alloc::boxed::Box<::ffi_11::c_int>,
        );
        #[link_name = "_ZN4test10ReadBoxPtrEPKN2rs3BoxIiEE"]
        pub(crate) unsafe fn __rust_thunk___ZN4test10ReadBoxPtrEPKN2rs3BoxIiEE(
            b: *const ::alloc::boxed::Box<::ffi_11::c_int>,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN4test14MakeBoxedPointEv(
            __return: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test6PointXEN2rs3BoxINS_5PointEEE(
            b: *mut ::alloc::boxed::Box<crate::test::Point>,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN4test6HolderD1Ev<'__this>(
            __this: &'__this mut crate::test::Holder,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test10MakeHolderEi(
            __return: *mut ::core::ffi::c_void,
            v: ::ffi_11::c_int,
        );
        #[link_name = "_ZN4test11HolderValueERKNS_6HolderE"]
        pub(crate) unsafe fn __rust_thunk___ZN4test11HolderValueERKNS_6HolderE<'h>(
            h: &'h crate::test::Holder,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZN4test14SetHolderValueERNS_6HolderEi"]
        pub(crate) unsafe fn __rust_thunk___ZN4test14SetHolderValueERNS_6HolderEi<'h>(
            h: &'h mut crate::test::Holder,
            v: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test9MakeOuterEi(
            __return: *mut ::core::ffi::c_void,
            v: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test11ArrayHolderD1Ev<'__this>(
            __this: &'__this mut crate::test::ArrayHolder,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test10NonTrivialC1Ei(
            __this: *mut ::core::ffi::c_void,
            v: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test10NonTrivialC1EOS0_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            other: ::ctor::RvalueReference<'__unelided, crate::test::NonTrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test10NonTrivialaSEOS0_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::test::NonTrivial>,
            other: ::ctor::RvalueReference<'_, crate::test::NonTrivial>,
        ) -> ::core::pin::Pin<&'__this mut crate::test::NonTrivial>;
        pub(crate) unsafe fn __rust_thunk___ZN4test10NonTrivialD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::test::NonTrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test9PinHolderD1Ev<'__this>(
            __this: &'__this mut crate::test::PinHolder,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test20MakePinBoxNonTrivialEi(
            __return: *mut ::core::ffi::c_void,
            val: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test21SharePinBoxNonTrivialERKN2rs6PinBoxINS_10NonTrivialEEE<
            'b,
        >(
            __return: *mut ::core::ffi::c_void,
            b: &'b ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
        );
        #[link_name = "_ZN4test18NonTrivialUseCountERKN2rs6PinBoxINS_10NonTrivialEEE"]
        pub(crate) unsafe fn __rust_thunk___ZN4test18NonTrivialUseCountERKN2rs6PinBoxINS_10NonTrivialEEE<
            'b,
        >(
            b: &'b ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
        ) -> ::ffi_11::c_long;
        pub(crate) unsafe fn __rust_thunk___ZN4test13ConsumePinBoxEN2rs6PinBoxINS_10NonTrivialEEE(
            b: *mut ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
        );
        #[link_name = "_ZN4test20ReadPinBoxNonTrivialERKN2rs6PinBoxINS_10NonTrivialEEE"]
        pub(crate) unsafe fn __rust_thunk___ZN4test20ReadPinBoxNonTrivialERKN2rs6PinBoxINS_10NonTrivialEEE<
            'b,
        >(
            b: &'b ::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN4test15MakePinBoxUnpinEv(
            __return: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN4test12MakeSliceBoxEm(
            __return: *mut ::core::ffi::c_void,
            n: ::ffi_11::c_ulong,
        );
        #[link_name = "_ZN4test11SumSliceBoxERKN2rs8SliceBoxIiEE"]
        pub(crate) unsafe fn __rust_thunk___ZN4test11SumSliceBoxERKN2rs8SliceBoxIiEE<'b>(
            b: &'b ::alloc::boxed::Box<[::ffi_11::c_int]>,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN4test15ConsumeSliceBoxEN2rs8SliceBoxIiEE(
            b: *mut ::alloc::boxed::Box<[::ffi_11::c_int]>,
        ) -> ::ffi_11::c_ulong;
        pub(crate) unsafe fn __rust_thunk___ZN4test10MakeStrBoxEv(
            __return: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN4test9StrBoxLenERKN2rs6StrBoxE"]
        pub(crate) unsafe fn __rust_thunk___ZN4test9StrBoxLenERKN2rs6StrBoxE<'b>(
            b: &'b ::alloc::boxed::Box<str>,
        ) -> ::ffi_11::c_ulong;
        pub(crate) unsafe fn __rust_thunk___ZN4test13ConsumeStrBoxEN2rs6StrBoxE(
            b: *mut ::alloc::boxed::Box<str>,
        ) -> ::ffi_11::c_ulong;
        pub(crate) unsafe fn __rust_thunk__d3f8dea6__ZNSt16initializer_listIiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__c77c867f__ZNKSt16initializer_listIiE5emptyEv<'__this>(
            __this: &'__this crate::__CcTemplateInstSt16initializer_listIiE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__2f3b1219__ZNKSt16initializer_listIiE3endEv<'__this>(
            __this: &'__this crate::__CcTemplateInstSt16initializer_listIiE,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEC1ES6_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
        );
        pub(crate) unsafe fn __rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        );
        pub(crate) unsafe fn __rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEC1ES5_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
        );
        pub(crate) unsafe fn __rust_thunk__34c90dec__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__466217a1__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
        );
        pub(crate) unsafe fn __rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEplEl<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEpLEl<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE;
        pub(crate) unsafe fn __rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEmiEl<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPKN4test10NonTrivialEEmIEl<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE;
        pub(crate) unsafe fn __rust_thunk__3dcfade5__ZNKSt3__u11__wrap_iterIPKN4test10NonTrivialEEixEl<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            __n: isize,
        ) -> &'__this crate::test::NonTrivial;
        pub(crate) unsafe fn __rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPKiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPKiEplEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPKiEpLEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        pub(crate) unsafe fn __rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPKiEmiEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPKiEmIEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        pub(crate) unsafe fn __rust_thunk__3dcfade5__ZNKSt3__u11__wrap_iterIPKiEixEl<'__this>(
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPN4test10NonTrivialEEplEl<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEpLEl<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE;
        pub(crate) unsafe fn __rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPN4test10NonTrivialEEmiEl<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPN4test10NonTrivialEEmIEl<
            '__this,
        >(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE;
        pub(crate) unsafe fn __rust_thunk__27ad6cd0__ZNSt3__u11__wrap_iterIPiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__d9068c5b__ZNKSt3__u11__wrap_iterIPiEplEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ed7b06d0__ZNSt3__u11__wrap_iterIPiEpLEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
        pub(crate) unsafe fn __rust_thunk__58f1a91b__ZNKSt3__u11__wrap_iterIPiEmiEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__ebbbe4fe__ZNSt3__u11__wrap_iterIPiEmIEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
        pub(crate) unsafe fn __rust_thunk__a2602314__ZNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__6ddb9499__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__70f2e79a__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__0b402a5e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__db8aea99__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__adeb3748__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__863bfd93__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEixEm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            __idx: usize,
        ) -> &'__this crate::test::NonTrivial;
        pub(crate) unsafe fn __rust_thunk__0eff2f4e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, crate::test::NonTrivial>;
        pub(crate) unsafe fn __rust_thunk__0b709465__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, crate::test::NonTrivial>;
        pub(crate) unsafe fn __rust_thunk__c3b9de9e__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__c0096ae4__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__30ed2d1f__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__bd3e0276__ZNKSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__a2602314__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__6ddb9499__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__70f2e79a__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__0b402a5e__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__db8aea99__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__adeb3748__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__863bfd93__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __idx: usize,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__0eff2f4e__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__0b709465__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__c3b9de9e__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__c0096ae4__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__30ed2d1f__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__bd3e0276__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__a2602314__ZNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__6ddb9499__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__70f2e79a__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__0b402a5e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__db8aea99__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__adeb3748__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__0eff2f4e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, crate::test::NonTrivial>;
        pub(crate) unsafe fn __rust_thunk__0b709465__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, crate::test::NonTrivial>;
        pub(crate) unsafe fn __rust_thunk__c3b9de9e__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__c0096ae4__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__30ed2d1f__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__bd3e0276__ZNKSt3__u4spanIN4test10NonTrivialELm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__a2602314__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__6ddb9499__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__70f2e79a__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__0b402a5e__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__db8aea99__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__adeb3748__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__0eff2f4e__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__0b709465__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__c3b9de9e__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__c0096ae4__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__30ed2d1f__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__bd3e0276__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::test::Point>() == 8);
    assert!(::core::mem::align_of::<crate::test::Point>() == 4);
    static_assertions::assert_impl_all!(crate::test::Point: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test::Point: Drop);
    assert!(::core::mem::offset_of!(crate::test::Point, x) == 0);
    assert!(::core::mem::offset_of!(crate::test::Point, y) == 4);
    assert!(::core::mem::size_of::<crate::test::Holder>() == 8);
    assert!(::core::mem::align_of::<crate::test::Holder>() == 8);
    static_assertions::assert_impl_all!(crate::test::Holder: Drop);
    static_assertions::assert_not_impl_any!(crate::test::Holder: Copy);
    assert!(::core::mem::offset_of!(crate::test::Holder, b) == 0);
    assert!(::core::mem::size_of::<crate::test::Outer>() == 16);
    assert!(::core::mem::align_of::<crate::test::Outer>() == 8);
    static_assertions::assert_not_impl_any!(crate::test::Outer: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::test::Outer, h) == 0);
    assert!(::core::mem::offset_of!(crate::test::Outer, tag) == 8);
    assert!(::core::mem::size_of::<crate::test::ArrayHolder>() == 16);
    assert!(::core::mem::align_of::<crate::test::ArrayHolder>() == 8);
    static_assertions::assert_impl_all!(crate::test::ArrayHolder: Drop);
    static_assertions::assert_not_impl_any!(crate::test::ArrayHolder: Copy);
    assert!(::core::mem::offset_of!(crate::test::ArrayHolder, boxes) == 0);
    assert!(::core::mem::size_of::<crate::test::NonTrivial>() == 4);
    assert!(::core::mem::align_of::<crate::test::NonTrivial>() == 4);
    static_assertions::assert_impl_all!(crate::test::NonTrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::test::NonTrivial: Copy);
    assert!(::core::mem::offset_of!(crate::test::NonTrivial, val) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
    assert!(::core::mem::size_of::<crate::test::PinHolder>() == 8);
    assert!(::core::mem::align_of::<crate::test::PinHolder>() == 8);
    static_assertions::assert_impl_all!(crate::test::PinHolder: Drop);
    static_assertions::assert_not_impl_any!(crate::test::PinHolder: Copy);
    assert!(::core::mem::offset_of!(crate::test::PinHolder, p) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstSt16initializer_listIiE>() == 16);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstSt16initializer_listIiE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstSt16initializer_listIiE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstSt16initializer_listIiE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstSt16initializer_listIiE, __begin_) == 0);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstSt16initializer_listIiE, __size_) == 8);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE>()==8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKN4test10NonTrivialEEEEE,current)==0);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            current
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE>()==8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE>()==8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPN4test10NonTrivialEEEEE,current)==0);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE>(
        ) == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u11__wrap_iterIPKN4test10NonTrivialEEE,
            __i_
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE, __i_) == 0);
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE>(
        ) == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u11__wrap_iterIPN4test10NonTrivialEEE,
            __i_
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE, __i_) == 0);
    assert!(::core::mem::size_of::<::alloc::boxed::Box<crate::test::Point>>() == 8);
    assert!(::core::mem::align_of::<::alloc::boxed::Box<crate::test::Point>>() == 8);
    assert!(::core::mem::size_of::<::alloc::boxed::Box<::ffi_11::c_int>>() == 8);
    assert!(::core::mem::align_of::<::alloc::boxed::Box<::ffi_11::c_int>>() == 8);
    assert!(
        ::core::mem::size_of::<::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>>()
            == 8
    );
    assert!(
        ::core::mem::align_of::<::core::pin::Pin<::alloc::boxed::Box<crate::test::NonTrivial>>>()
            == 8
    );
    assert!(::core::mem::size_of::<::core::pin::Pin<::alloc::boxed::Box<::ffi_11::c_int>>>() == 8);
    assert!(::core::mem::align_of::<::core::pin::Pin<::alloc::boxed::Box<::ffi_11::c_int>>>() == 8);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIKN4test10NonTrivialELm18446744073709551615EEE,
            __size_
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __size_
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIN4test10NonTrivialELm18446744073709551615EEE,
            __size_
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __size_
        ) == 8
    );
    assert!(::core::mem::size_of::<*const [crate::test::NonTrivial]>() == 16);
    assert!(::core::mem::align_of::<*const [crate::test::NonTrivial]>() == 8);
    assert!(::core::mem::size_of::<*const [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*const [::ffi_11::c_int]>() == 8);
    assert!(::core::mem::size_of::<*mut [crate::test::NonTrivial]>() == 16);
    assert!(::core::mem::align_of::<*mut [crate::test::NonTrivial]>() == 8);
    assert!(::core::mem::size_of::<*mut [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*mut [::ffi_11::c_int]>() == 8);
    assert!(::core::mem::size_of::<::alloc::boxed::Box<[::ffi_11::c_int]>>() == 16);
    assert!(::core::mem::align_of::<::alloc::boxed::Box<[::ffi_11::c_int]>>() == 8);
};
