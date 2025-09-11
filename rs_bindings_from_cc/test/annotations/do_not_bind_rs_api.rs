// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:do_not_bind
// Features: supported, unsafe_types

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

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=12
        // Error while generating bindings for constructor 'ArgumentToBoundOverload::ArgumentToBoundOverload':
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::ArgumentToBoundOverload
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::ArgumentToBoundOverload

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=12
        // Error while generating bindings for constructor 'ArgumentToBoundOverload::ArgumentToBoundOverload':
        // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::ArgumentToBoundOverload
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::ArgumentToBoundOverload

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=12
        // Error while generating bindings for constructor 'ArgumentToBoundOverload::ArgumentToBoundOverload':
        // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::ArgumentToBoundOverload
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::ArgumentToBoundOverload

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=12
        // Error while generating bindings for function 'ArgumentToBoundOverload::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=12
        // Error while generating bindings for function 'ArgumentToBoundOverload::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

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

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=13
        // Error while generating bindings for constructor 'ArgumentToUnboundOverload::ArgumentToUnboundOverload':
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::ArgumentToUnboundOverload
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::ArgumentToUnboundOverload

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=13
        // Error while generating bindings for constructor 'ArgumentToUnboundOverload::ArgumentToUnboundOverload':
        // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::ArgumentToUnboundOverload
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::ArgumentToUnboundOverload

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=13
        // Error while generating bindings for constructor 'ArgumentToUnboundOverload::ArgumentToUnboundOverload':
        // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::ArgumentToUnboundOverload
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::ArgumentToUnboundOverload

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=13
        // Error while generating bindings for function 'ArgumentToUnboundOverload::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=13
        // Error while generating bindings for function 'ArgumentToUnboundOverload::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=16
        #[inline(always)]
        pub fn DoNotBindFn(mut __param_0: crate::crubit::test::ArgumentToBoundOverload) {
            unsafe {
                crate::detail::__rust_thunk___ZN6crubit4test11DoNotBindFnENS0_23ArgumentToBoundOverloadE(&mut __param_0)
            }
        }

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=24
        // Error while generating bindings for function 'crubit::test::FunctionWithDoNotBindArgument':
        // Parameter #0 is not supported: Unsupported type 'crubit::test::DoNotBindStruct': No generated bindings found for 'DoNotBindStruct'

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=26
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

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=26
        // Error while generating bindings for constructor 'StructWithDoNotBindConstructor::StructWithDoNotBindConstructor':
        // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::StructWithDoNotBindConstructor
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::StructWithDoNotBindConstructor

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=26
        // Error while generating bindings for constructor 'StructWithDoNotBindConstructor::StructWithDoNotBindConstructor':
        // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::StructWithDoNotBindConstructor
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::StructWithDoNotBindConstructor

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=26
        // Error while generating bindings for function 'StructWithDoNotBindConstructor::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=26
        // Error while generating bindings for function 'StructWithDoNotBindConstructor::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=29
        // Error while generating bindings for constructor 'StructWithDoNotBindConstructor::StructWithDoNotBindConstructor':
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::StructWithDoNotBindConstructor
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::StructWithDoNotBindConstructor

        /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=32
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

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=32
        // Error while generating bindings for constructor 'StructWithDoNotBindMethod::StructWithDoNotBindMethod':
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::StructWithDoNotBindMethod
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::StructWithDoNotBindMethod

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=32
        // Error while generating bindings for constructor 'StructWithDoNotBindMethod::StructWithDoNotBindMethod':
        // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::StructWithDoNotBindMethod
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::StructWithDoNotBindMethod

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=32
        // Error while generating bindings for constructor 'StructWithDoNotBindMethod::StructWithDoNotBindMethod':
        // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
        // Expected first constructor parameter to be a mutable reference, got: *mut crate::crubit::test::StructWithDoNotBindMethod
        // Expected first reference parameter `__this` to have a lifetime, found *mut crate::crubit::test::StructWithDoNotBindMethod

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=32
        // Error while generating bindings for function 'StructWithDoNotBindMethod::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        // Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=32
        // Error while generating bindings for function 'StructWithDoNotBindMethod::operator=':
        // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

        impl StructWithDoNotBindMethod {
            /// Generated from: rs_bindings_from_cc/test/annotations/do_not_bind.h;l=34
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
        pub(crate) unsafe fn __rust_thunk___ZN6crubit4test11DoNotBindFnENS0_23ArgumentToBoundOverloadE(
            __param_0: &mut crate::crubit::test::ArgumentToBoundOverload,
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
