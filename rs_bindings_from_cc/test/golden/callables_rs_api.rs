// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

extern crate alloc;

#[inline(always)]
pub fn invoke_once(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z11invoke_onceN6rs_std11DynCallableIFvvOEEE(::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::FnOnce()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn invoke(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::FnMut() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z6invokeN6rs_std11DynCallableIFvvEEE(::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::FnMut()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::FnMut()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn invoke_const(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z12invoke_constN6rs_std11DynCallableIKFvvEEE(::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn map_int(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    arg: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z7map_intN6rs_std11DynCallableIKFiiEEEi(::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,arg)
    }
}

#[inline(always)]
pub fn map_bridged(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    arg: crate::RustBridged,
) -> crate::RustBridged {
    unsafe {
        ::bridge_rust::unstable_return!(@crate::RustBridgedAbi,crate::RustBridgedAbi,|__return_abi_buffer|{ crate::detail::__rust_thunk___Z11map_bridgedN6rs_std11DynCallableIKF7BridgedS1_EEES1_(__return_abi_buffer,::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::RustBridged)->crate::RustBridged+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::RustBridged|->crate::RustBridged{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::RustBridged)->crate::RustBridged+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,::bridge_rust::unstable_encode!(@crate::RustBridgedAbi,crate::RustBridgedAbi,arg).as_ptr()as*const u8); })
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ABICompatible
pub struct ABICompatible {
    pub x: ::ffi_11::c_int,
}
impl !Send for ABICompatible {}
impl !Sync for ABICompatible {}
unsafe impl ::cxx::ExternType for ABICompatible {
    type Id = ::cxx::type_id!("ABICompatible");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ABICompatible {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ABICompatibleC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn map_abi_compatible(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    mut arg: crate::ABICompatible,
) -> crate::ABICompatible {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::ABICompatible>::uninit();
        crate::detail::__rust_thunk___Z18map_abi_compatibleN6rs_std11DynCallableIKF13ABICompatibleS1_EEES1_(&raw mut __return as*mut::core::ffi::c_void,::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::ABICompatible)->crate::ABICompatible+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::ABICompatible|->crate::ABICompatible{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::ABICompatible)->crate::ABICompatible+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,&mut arg);
        __return.assume_init()
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=LayoutCompatible
pub struct LayoutCompatible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) private_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for LayoutCompatible {}
impl !Sync for LayoutCompatible {}
unsafe impl ::cxx::ExternType for LayoutCompatible {
    type Id = ::cxx::type_id!("LayoutCompatible");
    type Kind = ::cxx::kind::Trivial;
}
impl LayoutCompatible {
    #[inline(always)]
    pub fn Create(x: ::ffi_11::c_int) -> crate::LayoutCompatible {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN16LayoutCompatible6CreateEi(
                &raw mut __return as *mut ::core::ffi::c_void,
                x,
            );
            __return.assume_init()
        }
    }
    #[inline(always)]
    pub unsafe fn get(__this: *const Self) -> ::ffi_11::c_int {
        crate::detail::__rust_thunk___ZNK16LayoutCompatible3getEv(__this)
    }
}

#[inline(always)]
pub fn map_layout_compatible(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    mut arg: crate::LayoutCompatible,
) -> crate::LayoutCompatible {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::LayoutCompatible>::uninit();
        crate::detail::__rust_thunk___Z21map_layout_compatibleN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEES1_(&raw mut __return as*mut::core::ffi::c_void,::bridge_rust::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::LayoutCompatible)->crate::LayoutCompatible+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::LayoutCompatible|->crate::LayoutCompatible{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::LayoutCompatible)->crate::LayoutCompatible+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,&mut arg);
        __return.assume_init()
    }
}

// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:callables_cc needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z11invoke_onceN6rs_std11DynCallableIFvvOEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z6invokeN6rs_std11DynCallableIFvvEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z12invoke_constN6rs_std11DynCallableIKFvvEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z7map_intN6rs_std11DynCallableIKFiiEEEi(
            f: *const ::core::ffi::c_uchar,
            arg: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z11map_bridgedN6rs_std11DynCallableIKF7BridgedS1_EEES1_(
            __return_abi_buffer: *mut ::core::ffi::c_uchar,
            f: *const ::core::ffi::c_uchar,
            arg: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13ABICompatibleC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z18map_abi_compatibleN6rs_std11DynCallableIKF13ABICompatibleS1_EEES1_(
            __return: *mut ::core::ffi::c_void,
            f: *const ::core::ffi::c_uchar,
            arg: &mut crate::ABICompatible,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16LayoutCompatible6CreateEi(
            __return: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK16LayoutCompatible3getEv(
            __this: *const crate::LayoutCompatible,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z21map_layout_compatibleN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEES1_(
            __return: *mut ::core::ffi::c_void,
            f: *const ::core::ffi::c_uchar,
            arg: &mut crate::LayoutCompatible,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __CcTemplateInstN6rs_std11DynCallableIFvvEEE(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnMut() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &mut *f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __CcTemplateInstN6rs_std11DynCallableIFvvOEEE(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { ::core::ptr::read(f) })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::ABICompatible,
        out: *mut crate::ABICompatible,
    ) {
        let param_0 = ::core::ptr::read(param_0);
        match (unsafe { &*f })(param_0) {
            result => unsafe {
                ::core::ptr::write(out, result);
            },
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::LayoutCompatible,
        out: *mut crate::LayoutCompatible,
    ) {
        let param_0 = ::core::ptr::read(param_0);
        match (unsafe { &*f })(param_0) {
            result => unsafe {
                ::core::ptr::write(out, result);
            },
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::core::ffi::c_uchar,
        bridge_buffer: *mut ::core::ffi::c_uchar,
    ) {
        let param_0 = ::bridge_rust::internal::decode(crate::RustBridgedAbi, param_0);
        ::bridge_rust::internal::encode(
            crate::RustBridgedAbi,
            bridge_buffer,
            (unsafe { &*f })(param_0),
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __CcTemplateInstN6rs_std11DynCallableIKFiiEEE(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: ::ffi_11::c_int,
    ) -> ::ffi_11::c_int {
        (unsafe { &*f })(param_0)
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __CcTemplateInstN6rs_std11DynCallableIKFvvEEE(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &*f })();
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ABICompatible>() == 4);
    assert!(::core::mem::align_of::<crate::ABICompatible>() == 4);
    static_assertions::assert_impl_all!(crate::ABICompatible: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ABICompatible: Drop);
    assert!(::core::mem::offset_of!(crate::ABICompatible, x) == 0);
    assert!(::core::mem::size_of::<crate::LayoutCompatible>() == 4);
    assert!(::core::mem::align_of::<crate::LayoutCompatible>() == 4);
    static_assertions::assert_impl_all!(crate::LayoutCompatible: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::LayoutCompatible: Drop);
    assert!(::core::mem::offset_of!(crate::LayoutCompatible, private_) == 0);
};
