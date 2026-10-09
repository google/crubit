// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bridged_template_to_self_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod ns {
    // error: class `ns::MyBox` could not be bound
    //   Class templates are not yet supported

    /// `Member` mentions `MyBox<Instruction>` before `Instruction` is defined.
    /// Importing that specialization imports `Instruction`, whose methods in turn
    /// mention `MyBox<Instruction>` again, while it is still being imported.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N2ns6MemberE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=ns :: Member
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Member {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Member {}
    impl !Sync for Member {}
    unsafe impl ::cxx::ExternType for Member {
        type Id = ::cxx::type_id!("ns :: Member");
        type Kind = ::cxx::kind::Trivial;
    }

    impl ::ctor::UnsafeFrom<crate::MyBox<crate::ns::Instruction>> for Member {
        #[inline(always)]
        unsafe fn unsafe_from(args: crate::MyBox<crate::ns::Instruction>) -> Self {
            let mut instruction = args;
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns6MemberC1ENS_5MyBoxINS_11InstructionEEE(&raw mut tmp as*mut _,::crubit_support::bridge::unstable_encode!(@crate::MyBoxAbi(::crubit_support::bridge::transmute_abi::<crate::ns::Instruction>()),crate::MyBoxAbi<::crubit_support::bridge::TransmuteAbi<crate::ns::Instruction>>,instruction).as_ptr()as*const u8);
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::UnsafeCtorNew<crate::MyBox<crate::ns::Instruction>> for Member {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        unsafe fn ctor_new(args: crate::MyBox<crate::ns::Instruction>) -> Self::CtorType {
            unsafe {
                <Self as ::ctor::UnsafeFrom<crate::MyBox<crate::ns::Instruction>>>::unsafe_from(
                    args,
                )
            }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N2ns11InstructionE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=ns :: Instruction
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Instruction {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Instruction {}
    impl !Sync for Instruction {}
    unsafe impl ::cxx::ExternType for Instruction {
        type Id = ::cxx::type_id!("ns :: Instruction");
        type Kind = ::cxx::kind::Trivial;
    }
    impl Instruction {
        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `parent`: raw pointer
        #[inline(always)]
        pub unsafe fn Create(
            parent: *mut crate::ns::Region,
        ) -> crate::MyBox<crate::ns::Instruction> {
            unsafe { self::instruction::Create(parent) }
        }
    }

    impl Default for Instruction {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns11InstructionC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod instruction {
        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `parent`: raw pointer
        #[inline(always)]
        pub(crate) unsafe fn Create(
            parent: *mut crate::ns::Region,
        ) -> crate::MyBox<crate::ns::Instruction> {
            unsafe {
                ::crubit_support::bridge::unstable_return!(@crate::MyBoxAbi(::crubit_support::bridge::transmute_abi::<crate::ns::Instruction>()),crate::MyBoxAbi<::crubit_support::bridge::TransmuteAbi<crate::ns::Instruction>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___ZN2ns11Instruction6CreateEPNS_6RegionE(__crubit_return_abi_buffer,parent); })
            }
        }
    }

    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N2ns6RegionE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=ns :: Region
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Region {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Region {}
    impl !Sync for Region {}
    unsafe impl ::cxx::ExternType for Region {
        type Id = ::cxx::type_id!("ns :: Region");
        type Kind = ::cxx::kind::Trivial;
    }
    impl Region {
        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `instruction`: unsafe bridge type
        #[inline(always)]
        pub unsafe fn Append<'__this>(
            &'__this mut self,
            instruction: crate::MyBox<crate::ns::Instruction>,
        ) -> *mut crate::ns::Instruction {
            unsafe { self::region::Append(self, instruction) }
        }
        #[inline(always)]
        pub fn Pop<'__this>(&'__this mut self) -> crate::MyBox<crate::ns::Instruction> {
            unsafe { self::region::Pop(self) }
        }
    }

    impl Default for Region {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns6RegionC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod region {
        /// # Safety
        ///
        /// The caller must ensure that the following unsafe arguments are not misused by the function:
        /// * `instruction`: unsafe bridge type
        #[inline(always)]
        pub(crate) unsafe fn Append<'__this>(
            __this: &'__this mut crate::ns::Region,
            instruction: crate::MyBox<crate::ns::Instruction>,
        ) -> *mut crate::ns::Instruction {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns6Region6AppendENS_5MyBoxINS_11InstructionEEE(__this,::crubit_support::bridge::unstable_encode!(@crate::MyBoxAbi(::crubit_support::bridge::transmute_abi::<crate::ns::Instruction>()),crate::MyBoxAbi<::crubit_support::bridge::TransmuteAbi<crate::ns::Instruction>>,instruction).as_ptr()as*const u8)
            }
        }
        #[inline(always)]
        pub(crate) fn Pop<'__this>(
            __this: &'__this mut crate::ns::Region,
        ) -> crate::MyBox<crate::ns::Instruction> {
            unsafe {
                ::crubit_support::bridge::unstable_return!(@crate::MyBoxAbi(::crubit_support::bridge::transmute_abi::<crate::ns::Instruction>()),crate::MyBoxAbi<::crubit_support::bridge::TransmuteAbi<crate::ns::Instruction>>,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___ZN2ns6Region3PopEv(__crubit_return_abi_buffer,__this); })
            }
        }
    }
}

// namespace ns

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN2ns6MemberC1ENS_5MyBoxINS_11InstructionEEE(
            __this: *mut ::core::ffi::c_void,
            instruction: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2ns11InstructionC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2ns11Instruction6CreateEPNS_6RegionE(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            parent: *mut crate::ns::Region,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2ns6RegionC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN2ns6Region6AppendENS_5MyBoxINS_11InstructionEEE<
            '__this,
        >(
            __this: &'__this mut crate::ns::Region,
            instruction: *const ::core::ffi::c_uchar,
        ) -> *mut crate::ns::Instruction;
        pub(crate) unsafe fn __rust_thunk___ZN2ns6Region3PopEv<'__this>(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            __this: &'__this mut crate::ns::Region,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ns::Member>() == 1);
    assert!(::core::mem::align_of::<crate::ns::Member>() == 1);
    static_assertions::assert_impl_all!(crate::ns::Member: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ns::Member: Drop);

    assert!(::core::mem::size_of::<crate::ns::Instruction>() == 1);
    assert!(::core::mem::align_of::<crate::ns::Instruction>() == 1);
    static_assertions::assert_impl_all!(crate::ns::Instruction: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ns::Instruction: Drop);

    assert!(::core::mem::size_of::<crate::ns::Region>() == 1);
    assert!(::core::mem::align_of::<crate::ns::Region>() == 1);
    static_assertions::assert_impl_all!(crate::ns::Region: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ns::Region: Drop);
};
