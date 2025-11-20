// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:do_not_bind
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod crubit {
    pub mod test {
        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=12
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ArgumentToBoundOverload
        pub struct ArgumentToBoundOverload {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for ArgumentToBoundOverload {}
        impl !Sync for ArgumentToBoundOverload {}
        unsafe impl ::cxx::ExternType for ArgumentToBoundOverload {
            type Id = ::cxx::type_id!("crubit :: test :: ArgumentToBoundOverload");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=12
        impl Default for ArgumentToBoundOverload {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test23ArgumentToBoundOverloadC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=13
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: ArgumentToUnboundOverload
        pub struct ArgumentToUnboundOverload {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for ArgumentToUnboundOverload {}
        impl !Sync for ArgumentToUnboundOverload {}
        unsafe impl ::cxx::ExternType for ArgumentToUnboundOverload {
            type Id = ::cxx::type_id!("crubit :: test :: ArgumentToUnboundOverload");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=13
        impl Default for ArgumentToUnboundOverload {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test25ArgumentToUnboundOverloadC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=16
        #[inline(always)]
        pub fn DoNotBindFn(mut __param_0: crate::crubit::test::ArgumentToBoundOverload) {
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test11DoNotBindFnENS0_23ArgumentToBoundOverloadE(&mut __param_0)
            }
        }

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=26
        // Error while generating bindings for function 'crubit::test::FunctionWithDoNotBindArgument':
        // Parameter #0 is not supported: Unsupported type 'crubit::test::DoNotBindStruct': No generated bindings found for 'DoNotBindStruct'

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=28
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: StructWithDoNotBindConstructor
        pub struct StructWithDoNotBindConstructor {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for StructWithDoNotBindConstructor {}
        impl !Sync for StructWithDoNotBindConstructor {}
        unsafe impl ::cxx::ExternType for StructWithDoNotBindConstructor {
            type Id = ::cxx::type_id!("crubit :: test :: StructWithDoNotBindConstructor");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=31
        impl From<crate::crubit::test::ArgumentToBoundOverload> for StructWithDoNotBindConstructor {
            #[inline(always)]
            fn from(mut __param_0: crate::crubit::test::ArgumentToBoundOverload) -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test30StructWithDoNotBindConstructorC1ENS0_23ArgumentToBoundOverloadE(&raw mut tmp as*mut _,&mut __param_0);
                    tmp.assume_init()
                }
            }
        }
        impl ::ctor::CtorNew<crate::crubit::test::ArgumentToBoundOverload>
            for StructWithDoNotBindConstructor
        {
            type CtorType = Self;
            type Error = ::ctor::Infallible;
            #[inline(always)]
            fn ctor_new(args: crate::crubit::test::ArgumentToBoundOverload) -> Self::CtorType {
                <Self as From<crate::crubit::test::ArgumentToBoundOverload>>::from(args)
            }
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=34
        #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
        #[repr(C)]
        ///CRUBIT_ANNOTATE: cpp_type=crubit :: test :: StructWithDoNotBindMethod
        pub struct StructWithDoNotBindMethod {
            __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
        }
        impl !Send for StructWithDoNotBindMethod {}
        impl !Sync for StructWithDoNotBindMethod {}
        unsafe impl ::cxx::ExternType for StructWithDoNotBindMethod {
            type Id = ::cxx::type_id!("crubit :: test :: StructWithDoNotBindMethod");
            type Kind = ::cxx::kind::Trivial;
        }

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=34
        impl Default for StructWithDoNotBindMethod {
            #[inline(always)]
            fn default() -> Self {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    crate::detail::__rust_thunk___ZN6crubit4test25StructWithDoNotBindMethodC1Ev(
                        &raw mut tmp as *mut _,
                    );
                    tmp.assume_init()
                }
            }
        }

        impl StructWithDoNotBindMethod {
            /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=36
            #[inline(always)]
            pub unsafe fn DoNotBindMethod(
                __this: *mut Self,
                mut __param_0: crate::crubit::test::ArgumentToBoundOverload,
            ) {
                crate::detail::__rust_thunk___ZN6crubit4test25StructWithDoNotBindMethod15DoNotBindMethodENS0_23ArgumentToBoundOverloadE(__this,&mut __param_0)
            }
        }
    }
}

// namespace crubit::test

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/annotations:do_not_bind needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/annotations:do_not_bind needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test23ArgumentToBoundOverloadC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test25ArgumentToUnboundOverloadC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test11DoNotBindFnENS0_23ArgumentToBoundOverloadE(
            __param_0: &mut crate::crubit::test::ArgumentToBoundOverload,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test30StructWithDoNotBindConstructorC1ENS0_23ArgumentToBoundOverloadE(
            __this: *mut ::core::ffi::c_void,
            __param_0: &mut crate::crubit::test::ArgumentToBoundOverload,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test25StructWithDoNotBindMethodC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test25StructWithDoNotBindMethod15DoNotBindMethodENS0_23ArgumentToBoundOverloadE(
            __this: *mut crate::crubit::test::StructWithDoNotBindMethod,
            __param_0: &mut crate::crubit::test::ArgumentToBoundOverload,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::crubit::test::ArgumentToBoundOverload>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::ArgumentToBoundOverload>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::ArgumentToBoundOverload: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ArgumentToBoundOverload: Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::ArgumentToUnboundOverload>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::ArgumentToUnboundOverload>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::ArgumentToUnboundOverload: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::ArgumentToUnboundOverload: Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::StructWithDoNotBindConstructor>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::StructWithDoNotBindConstructor>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::StructWithDoNotBindConstructor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::StructWithDoNotBindConstructor: Drop);

    assert!(::core::mem::size_of::<crate::crubit::test::StructWithDoNotBindMethod>() == 1);
    assert!(::core::mem::align_of::<crate::crubit::test::StructWithDoNotBindMethod>() == 1);
    static_assertions::assert_impl_all!(crate::crubit::test::StructWithDoNotBindMethod: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::crubit::test::StructWithDoNotBindMethod: Drop);
};
