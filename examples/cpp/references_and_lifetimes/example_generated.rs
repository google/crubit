// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/references_and_lifetimes:example_lib

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod example {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N7example7CounterE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=example :: Counter
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Counter {
        pub value: ::ffi_11::c_int,
    }
    impl !Send for Counter {}
    impl !Sync for Counter {}
    unsafe impl ::cxx::ExternType for Counter {
        type Id = ::cxx::type_id!("example :: Counter");
        type Kind = ::cxx::kind::Trivial;
    }
    impl Counter {
        #[inline(always)]
        pub fn Get<'__this>(&'__this self) -> ::ffi_11::c_int {
            unsafe { self::counter::Get(self) }
        }
        #[inline(always)]
        pub fn Increment<'__this>(&'__this mut self) {
            unsafe { self::counter::Increment(self) }
        }
        #[inline(always)]
        pub fn GetRef<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
            unsafe { self::counter::GetRef(self) }
        }
        #[inline(always)]
        pub fn GetMutRef<'__this>(&'__this mut self) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
            unsafe { self::counter::GetMutRef(self) }
        }
    }

    impl Default for Counter {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN7example7CounterC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod counter {
        #[inline(always)]
        pub(crate) fn Get<'__this>(__this: &'__this crate::example::Counter) -> ::ffi_11::c_int {
            unsafe { crate::detail::__rust_thunk___ZNK7example7Counter3GetEv(__this) }
        }
        #[inline(always)]
        pub(crate) fn Increment<'__this>(__this: &'__this mut crate::example::Counter) {
            unsafe { crate::detail::__rust_thunk___ZN7example7Counter9IncrementEv(__this) }
        }
        #[inline(always)]
        pub(crate) fn GetRef<'__this>(
            __this: &'__this crate::example::Counter,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
            unsafe { crate::detail::__rust_thunk___ZNK7example7Counter6GetRefEv(__this) }
        }
        #[inline(always)]
        pub(crate) fn GetMutRef<'__this>(
            __this: &'__this mut crate::example::Counter,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
            unsafe { crate::detail::__rust_thunk___ZN7example7Counter9GetMutRefEv(__this) }
        }
    }

    #[inline(always)]
    pub fn AddOne<'x>(x: &'x mut ::ffi_11::c_int) {
        unsafe { crate::detail::__rust_thunk___ZN7example6AddOneERi(x) }
    }

    /// Lifetime elision doesn't apply: there are two input references and no `this`.
    #[inline(always)]
    pub fn Smaller<'x, 'y>(
        x: &'x ::ffi_11::c_int,
        y: &'y ::ffi_11::c_int,
    ) -> *const ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZN7example7SmallerERKiS1_(x, y) }
    }

    /// Explicit lifetime annotations: The result may refer to `x` or to `y`.
    #[inline(always)]
    pub fn SmallerWithLifetimes<'a>(
        x: &'a ::ffi_11::c_int,
        y: &'a ::ffi_11::c_int,
    ) -> ::cref::CRef<'a, ::ffi_11::c_int> {
        unsafe { crate::detail::__rust_thunk___ZN7example20SmallerWithLifetimesERKiS1_(x, y) }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N7example6NumberE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=example :: Number
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Number {
        pub value: ::ffi_11::c_int,
    }
    impl !Send for Number {}
    impl !Sync for Number {}
    unsafe impl ::cxx::ExternType for Number {
        type Id = ::cxx::type_id!("example :: Number");
        type Kind = ::cxx::kind::Trivial;
    }
    impl Number {
        /// The result may refer to `this` or to `other`, but lifetime elision ties the
        /// result only to `this`.
        #[inline(always)]
        pub fn GetLarger<'__this, 'other>(
            &'__this self,
            other: &'other ::ffi_11::c_int,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
            unsafe { self::number::GetLarger(self, other) }
        }
        /// The result may refer to `this` or to `other`.
        #[inline(always)]
        pub fn GetLargerWithLifetimes<'a>(
            &'a self,
            other: &'a ::ffi_11::c_int,
        ) -> ::cref::CRef<'a, ::ffi_11::c_int> {
            unsafe { self::number::GetLargerWithLifetimes(self, other) }
        }
    }

    impl Default for Number {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN7example6NumberC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod number {
        /// The result may refer to `this` or to `other`, but lifetime elision ties the
        /// result only to `this`.
        #[inline(always)]
        pub(crate) fn GetLarger<'__this, 'other>(
            __this: &'__this crate::example::Number,
            other: &'other ::ffi_11::c_int,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
            unsafe { crate::detail::__rust_thunk___ZNK7example6Number9GetLargerERKi(__this, other) }
        }
        /// The result may refer to `this` or to `other`.
        #[inline(always)]
        pub(crate) fn GetLargerWithLifetimes<'a>(
            __this: &'a crate::example::Number,
            other: &'a ::ffi_11::c_int,
        ) -> ::cref::CRef<'a, ::ffi_11::c_int> {
            unsafe {
                crate::detail::__rust_thunk___ZNK7example6Number22GetLargerWithLifetimesERKi(
                    __this, other,
                )
            }
        }
    }
}

// namespace example

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN7example7CounterC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK7example7Counter3GetEv<'__this>(
            __this: &'__this crate::example::Counter,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN7example7Counter9IncrementEv<'__this>(
            __this: &'__this mut crate::example::Counter,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK7example7Counter6GetRefEv<'__this>(
            __this: &'__this crate::example::Counter,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk___ZN7example7Counter9GetMutRefEv<'__this>(
            __this: &'__this mut crate::example::Counter,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk___ZN7example6AddOneERi<'x>(x: &'x mut ::ffi_11::c_int);
        pub(crate) unsafe fn __rust_thunk___ZN7example7SmallerERKiS1_<'x, 'y>(
            x: &'x ::ffi_11::c_int,
            y: &'y ::ffi_11::c_int,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN7example20SmallerWithLifetimesERKiS1_<'a>(
            x: &'a ::ffi_11::c_int,
            y: &'a ::ffi_11::c_int,
        ) -> ::cref::CRef<'a, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk___ZN7example6NumberC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZNK7example6Number9GetLargerERKi<'__this, 'other>(
            __this: &'__this crate::example::Number,
            other: &'other ::ffi_11::c_int,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk___ZNK7example6Number22GetLargerWithLifetimesERKi<'a>(
            __this: &'a crate::example::Number,
            other: &'a ::ffi_11::c_int,
        ) -> ::cref::CRef<'a, ::ffi_11::c_int>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::example::Counter>() == 4);
    assert!(::core::mem::align_of::<crate::example::Counter>() == 4);
    static_assertions::assert_impl_all!(crate::example::Counter: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::example::Counter: Drop);
    assert!(::core::mem::offset_of!(crate::example::Counter, value) == 0);
    assert!(::core::mem::size_of::<crate::example::Number>() == 4);
    assert!(::core::mem::align_of::<crate::example::Number>() == 4);
    static_assertions::assert_impl_all!(crate::example::Number: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::example::Number: Drop);
    assert!(::core::mem::offset_of!(crate::example::Number, value) == 0);
};
