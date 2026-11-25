// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:callables_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
extern crate alloc;

#[inline(always)]
pub fn invoke_once(
    f: ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z11invoke_onceN6rs_std11DynCallableIFvvOEEE(::crubit_support::bridge::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::FnOnce()+::core::marker::Send+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::FnOnce()+::core::marker::Send+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn invoke(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z6invokeN6rs_std11DynCallableIFvvEEE(::crubit_support::bridge::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn invoke_const(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
    >,
) {
    unsafe {
        crate::detail::__rust_thunk___Z12invoke_constN6rs_std11DynCallableIKFvvEEE(::crubit_support::bridge::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn()+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(||{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn()+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8)
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
        crate::detail::__rust_thunk___Z7map_intN6rs_std11DynCallableIKFiiEEEi(::crubit_support::bridge::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: ::ffi_11::c_int|->::ffi_11::c_int{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,arg)
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
        ::crubit_support::bridge::unstable_return!(@crate::RustBridgedAbi,crate::RustBridgedAbi,|__crubit_return_abi_buffer|{ crate::detail::__rust_thunk___Z11map_bridgedN6rs_std11DynCallableIKF7BridgedS1_EEES1_(__crubit_return_abi_buffer,::crubit_support::bridge::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::RustBridged)->crate::RustBridged+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::RustBridged|->crate::RustBridged{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::RustBridged)->crate::RustBridged+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,::crubit_support::bridge::unstable_encode!(@crate::RustBridgedAbi,crate::RustBridgedAbi,arg).as_ptr()as*const u8); })
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "13ABICompatible"]
#[repr(C)]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=ABICompatible")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct ABICompatible {
    pub x: ::ffi_11::c_int,
}
unsafe impl ::cxx::ExternType for ABICompatible {
    type Id = ::cxx::type_id!("ABICompatible");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("ABICompatible"), crate::ABICompatible);

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
    arg: crate::ABICompatible,
) -> crate::ABICompatible {
    let mut arg = ::core::mem::MaybeUninit::new(arg);
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ABICompatible>::uninit();
        crate::detail::__rust_thunk___Z18map_abi_compatibleN6rs_std11DynCallableIKF13ABICompatibleS1_EEES1_(&raw mut __crubit_return as*mut::core::ffi::c_void,::crubit_support::bridge::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::ABICompatible)->crate::ABICompatible+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::ABICompatible|->crate::ABICompatible{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::ABICompatible)->crate::ABICompatible+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,arg.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "16LayoutCompatible"]
#[repr(C, align(4))]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=LayoutCompatible")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct LayoutCompatible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) private_: [::core::mem::MaybeUninit<u8>; 4],
}
/// Cannot infer `Send` because:
/// * field `private_` is represented as opaque bytes
impl !Send for LayoutCompatible {}
/// Cannot infer `Sync` because:
/// * field `private_` is represented as opaque bytes
impl !Sync for LayoutCompatible {}
unsafe impl ::cxx::ExternType for LayoutCompatible {
    type Id = ::cxx::type_id!("LayoutCompatible");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("LayoutCompatible"),
    crate::LayoutCompatible
);
impl LayoutCompatible {
    #[inline(always)]
    pub fn Create(x: ::ffi_11::c_int) -> crate::LayoutCompatible {
        unsafe { self::layout_compatible::Create(x) }
    }
    #[inline(always)]
    pub fn get<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe { self::layout_compatible::get(self) }
    }
}

pub mod layout_compatible {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: do_not_bind=")]
    #[inline(always)]
    pub fn Create(x: ::ffi_11::c_int) -> crate::LayoutCompatible {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::LayoutCompatible>::uninit();
            crate::detail::__rust_thunk___ZN16LayoutCompatible6CreateEi(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                x,
            );
            __crubit_return.assume_init()
        }
    }
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: do_not_bind=")]
    #[inline(always)]
    pub fn get<'__this>(__this: &'__this crate::LayoutCompatible) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK16LayoutCompatible3getEv(__this) }
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
    arg: crate::LayoutCompatible,
) -> crate::LayoutCompatible {
    let mut arg = ::core::mem::MaybeUninit::new(arg);
    unsafe {
        let mut __crubit_return = ::core::mem::MaybeUninit::<crate::LayoutCompatible>::uninit();
        crate::detail::__rust_thunk___Z21map_layout_compatibleN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEES1_(&raw mut __crubit_return as*mut::core::ffi::c_void,::crubit_support::bridge::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(crate::LayoutCompatible)->crate::LayoutCompatible+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: crate::LayoutCompatible|->crate::LayoutCompatible{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(crate::LayoutCompatible)->crate::LayoutCompatible+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,arg.as_mut_ptr());
        __crubit_return.assume_init()
    }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `f`: Callable takes unsafe parameters: param 1 is of unsafe type raw pointer
#[inline(always)]
pub unsafe fn callable_taking_reference(
    f: ::alloc::boxed::Box<
        dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
            + ::core::marker::Send
            + ::core::marker::Sync
            + 'static,
    >,
    arg: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z25callable_taking_referenceN6rs_std11DynCallableIFvRiEEEi(::crubit_support::bridge::unstable_encode!(@::dyn_callable_rs::DynCallableAbi::<dyn::core::ops::Fn(*mut::ffi_11::c_int)+::core::marker::Send+::core::marker::Sync+'static>::new(::alloc::boxed::Box::new(|_: *mut::ffi_11::c_int|{ ::core::panic!("moved-from value") }),),::dyn_callable_rs::DynCallableAbi<dyn::core::ops::Fn(*mut::ffi_11::c_int)+::core::marker::Send+::core::marker::Sync+'static>,f).as_ptr()as*const u8,arg)
    }
}

#[inline(always)]
pub fn invoke_fn_ref(f: &(dyn ::core::ops::Fn() + '_)) {
    unsafe {
        crate::detail::__rust_thunk___Z13invoke_fn_refN2rs5FnRefIFvvEEE(::crubit_support::bridge::unstable_encode!(@::crubit_support::bridge::FunctionRefAbi::<'_,dyn::core::ops::Fn()+'_>::new(),::crubit_support::bridge::FunctionRefAbi<'static,dyn::core::ops::Fn()+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn invoke_fn_ref_const(f: &(dyn ::core::ops::Fn() + '_)) {
    unsafe {
        crate::detail::__rust_thunk___Z19invoke_fn_ref_constN2rs5FnRefIKFvvEEE(::crubit_support::bridge::unstable_encode!(@::crubit_support::bridge::FunctionRefAbi::<'_,dyn::core::ops::Fn()+'_>::new(),::crubit_support::bridge::FunctionRefAbi<'static,dyn::core::ops::Fn()+'static>,f).as_ptr()as*const u8)
    }
}

#[inline(always)]
pub fn map_int_fn_ref(
    f: &(dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int + '_),
    arg: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z14map_int_fn_refN2rs5FnRefIKFiiEEEi(::crubit_support::bridge::unstable_encode!(@::crubit_support::bridge::FunctionRefAbi::<'_,dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+'_>::new(),::crubit_support::bridge::FunctionRefAbi<'static,dyn::core::ops::Fn(::ffi_11::c_int)->::ffi_11::c_int+'static>,f).as_ptr()as*const u8,arg)
    }
}

#[inline(always)]
pub fn find_if_fn_ref<'v>(
    v: &'v ::ffi_11::c_int,
    pred: &(dyn ::core::ops::Fn(::ffi_11::c_int) -> bool + '_),
) -> ::cref::CRef<'v, ::ffi_11::c_int> {
    unsafe {
        crate::detail::__rust_thunk___Z14find_if_fn_refRKiN2rs5FnRefIFbiEEE(v,::crubit_support::bridge::unstable_encode!(@::crubit_support::bridge::FunctionRefAbi::<'_,dyn::core::ops::Fn(::ffi_11::c_int)->bool+'_>::new(),::crubit_support::bridge::FunctionRefAbi<'static,dyn::core::ops::Fn(::ffi_11::c_int)->bool+'static>,pred).as_ptr()as*const u8)
    }
}

// error: function `return_fn_ref` could not be bound
//   Return type is not supported: borrowed callable type `rs::FnRef<void ()>` cannot be used as a return type

// error: function `return_fn_ref_from_dyn` could not be bound
//   Return type is not supported: return type `rs_std :: DynCallable < class rs :: FnRef < void () > () const >` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

// error: function `fn_ref_returning_fn_ref` could not be bound
//   Return type is not supported: return type `rs :: FnRef < class rs :: FnRef < void () > () >` contains borrowed callable type `rs::FnRef<class rs::FnRef<void ()> ()>`, which cannot be used in a return type

// error: function `accept_nested_fn_ref` could not be bound
//   Unsupported parameter type `rs::FnRef<void (class rs::FnRef<void ()>)> f`:
//     borrowed callable type cannot be used inside another callable

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "15StructWithFnRef"]
#[repr(C, align(8))]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=StructWithFnRef")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct StructWithFnRef {
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/bridge_field: '&(dyn::core::ops::Fn()+'_)' is not layout-compatible between Rust and C++.
    pub(crate) f: [::core::mem::MaybeUninit<u8>; 16],
}
/// Cannot infer `Send` because:
/// * field `f` is represented as opaque bytes
impl !Send for StructWithFnRef {}
/// Cannot infer `Sync` because:
/// * field `f` is represented as opaque bytes
impl !Sync for StructWithFnRef {}
unsafe impl ::cxx::ExternType for StructWithFnRef {
    type Id = ::cxx::type_id!("StructWithFnRef");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithFnRef"),
    crate::StructWithFnRef
);

// error: function `StructWithFnRef::operator=` could not be bound
//   Return type is not supported: return type `StructWithFnRef&` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

// error: function `StructWithFnRef::operator=` could not be bound
//   Return type is not supported: return type `StructWithFnRef&` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

// error: function `make_struct_with_fn_ref` could not be bound
//   Return type is not supported: return type `StructWithFnRef` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "21NestedStructWithFnRef"]
#[repr(C)]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=NestedStructWithFnRef")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct NestedStructWithFnRef {
    pub inner: crate::StructWithFnRef,
}
unsafe impl ::cxx::ExternType for NestedStructWithFnRef {
    type Id = ::cxx::type_id!("NestedStructWithFnRef");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NestedStructWithFnRef"),
    crate::NestedStructWithFnRef
);

// error: function `NestedStructWithFnRef::operator=` could not be bound
//   Return type is not supported: return type `NestedStructWithFnRef&` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

// error: function `NestedStructWithFnRef::operator=` could not be bound
//   Return type is not supported: return type `NestedStructWithFnRef&` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

// error: function `make_nested_struct_with_fn_ref` could not be bound
//   Return type is not supported: return type `NestedStructWithFnRef` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "26DerivedFromStructWithFnRef"]
#[repr(C, align(8))]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=DerivedFromStructWithFnRef")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct DerivedFromStructWithFnRef {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 16],
}
/// Cannot infer `Send` because:
/// * base class subobjects are not represented as Rust fields
impl !Send for DerivedFromStructWithFnRef {}
/// Cannot infer `Sync` because:
/// * base class subobjects are not represented as Rust fields
impl !Sync for DerivedFromStructWithFnRef {}
unsafe impl ::cxx::ExternType for DerivedFromStructWithFnRef {
    type Id = ::cxx::type_id!("DerivedFromStructWithFnRef");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("DerivedFromStructWithFnRef"),
    crate::DerivedFromStructWithFnRef
);

// error: function `DerivedFromStructWithFnRef::operator=` could not be bound
//   Return type is not supported: return type `DerivedFromStructWithFnRef&` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

// error: function `DerivedFromStructWithFnRef::operator=` could not be bound
//   Return type is not supported: return type `DerivedFromStructWithFnRef&` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

// error: function `make_derived_struct_with_fn_ref` could not be bound
//   Return type is not supported: return type `DerivedFromStructWithFnRef` contains borrowed callable type `rs::FnRef<void ()>`, which cannot be used in a return type

/// Class template instantiations that aren't named by a C++ type alias.
pub mod zz_template_instantiations {
    // error: class `absl::internal_any_invocable::CoreImpl<false, struct ABICompatible, struct ABICompatible>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //rs_bindings_from_cc/test/golden:callables_cc
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported

    // error: class `absl::internal_any_invocable::CoreImpl<false, class LayoutCompatible, class LayoutCompatible>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //rs_bindings_from_cc/test/golden:callables_cc
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported

    // error: class `absl::internal_any_invocable::CoreImpl<false, struct Bridged, struct Bridged>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //rs_bindings_from_cc/test/golden:callables_cc
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported

    // error: class `absl::internal_any_invocable::CoreImpl<false, class rs::FnRef<void ()>>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //rs_bindings_from_cc/test/golden:callables_cc
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported

    // error: class `absl::internal_any_invocable::CoreImpl<false, int, int>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //rs_bindings_from_cc/test/golden:callables_cc
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported

    // error: class `absl::internal_any_invocable::CoreImpl<false, void>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //rs_bindings_from_cc/test/golden:callables_cc
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported

    // error: class `absl::internal_any_invocable::CoreImpl<false, void, int &>` could not be bound
    //   crubit.rs/errors/unknown_target: the type is defined in
    //     third_party/absl/functional/internal/any_invocable.h
    //   which is not a public header of any Crubit-enabled target that
    //     //rs_bindings_from_cc/test/golden:callables_cc
    //   depends on directly. Either enable Crubit on the library that provides the header,
    //   add that library as a direct dependency (and `#include` the header directly),
    //   or move the type to a public header.
    //   template instantiation is not yet supported

    // error: class `absl::AnyInvocable<class rs::FnRef<void ()> () const>` could not be bound
    //   borrowed callable type cannot be used inside another callable

    // error: class `rs::DynCallable<class rs::FnRef<void ()> () const>` could not be bound
    //   borrowed callable type cannot be used inside another callable

    // error: class `rs_std::DynCallable<class rs::FnRef<void ()> () const>` could not be bound
    //   borrowed callable type cannot be used inside another callable

    // error: class `rs::FnRef<class rs::FnRef<void ()> ()>` could not be bound
    //   borrowed callable type cannot be used inside another callable

    // error: class `rs::FnRef<void (class rs::FnRef<void ()>)>` could not be bound
    //   borrowed callable type cannot be used inside another callable
}
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
            arg: *mut crate::ABICompatible,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16LayoutCompatible6CreateEi(
            __return: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK16LayoutCompatible3getEv<'__this>(
            __this: &'__this crate::LayoutCompatible,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z21map_layout_compatibleN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEES1_(
            __return: *mut ::core::ffi::c_void,
            f: *const ::core::ffi::c_uchar,
            arg: *mut crate::LayoutCompatible,
        );
        pub(crate) unsafe fn __rust_thunk___Z25callable_taking_referenceN6rs_std11DynCallableIFvRiEEEi(
            f: *const ::core::ffi::c_uchar,
            arg: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z13invoke_fn_refN2rs5FnRefIFvvEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z19invoke_fn_ref_constN2rs5FnRefIKFvvEEE(
            f: *const ::core::ffi::c_uchar,
        );
        pub(crate) unsafe fn __rust_thunk___Z14map_int_fn_refN2rs5FnRefIKFiiEEEi(
            f: *const ::core::ffi::c_uchar,
            arg: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z14find_if_fn_refRKiN2rs5FnRefIFbiEEE<'v>(
            v: &'v ::ffi_11::c_int,
            pred: *const ::core::ffi::c_uchar,
        ) -> ::cref::CRef<'v, ::ffi_11::c_int>;
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::ffi_11::c_int,
    ) {
        (unsafe { &*f })(param_0);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: *mut ::ffi_11::c_int,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &*f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
            f: *mut ::any_invocable::RawAnyInvocable,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
    ) {
        (unsafe {
            ::core::ptr::replace(
                f,
                ::alloc::boxed::Box::new(|| {
                    ::core::unreachable!("Called FnOnce after it was moved");
                }),
            )
        })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
        to: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
            f: *mut ::any_invocable::RawAnyInvocable,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::ABICompatible,
        out: *mut crate::ABICompatible,
    ) {
        let param_0 = unsafe { ::core::ptr::read(param_0) };
        unsafe {
            out.write((unsafe { &*f })(param_0));
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: *mut crate::ABICompatible,
            out: *mut crate::ABICompatible,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::LayoutCompatible,
        out: *mut crate::LayoutCompatible,
    ) {
        let param_0 = unsafe { ::core::ptr::read(param_0) };
        unsafe {
            out.write((unsafe { &*f })(param_0));
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: *mut crate::LayoutCompatible,
            out: *mut crate::LayoutCompatible,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::core::ffi::c_uchar,
        bridge_buffer: *mut ::core::ffi::c_uchar,
    ) {
        let param_0 =
            unsafe { ::crubit_support::bridge::internal::decode(crate::RustBridgedAbi, param_0) };
        unsafe {
            ::crubit_support::bridge::internal::encode(
                crate::RustBridgedAbi,
                bridge_buffer,
                (unsafe { &*f })(param_0),
            )
        };
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: *const ::core::ffi::c_uchar,
            out: *mut ::core::ffi::c_uchar,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
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
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
            f: *mut ::any_invocable::RawAnyInvocable,
            param_0: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN4absl12AnyInvocableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &*f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN4absl12AnyInvocableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    unsafe extern "C" {
        pub(crate) unsafe fn __crubit_invoke_any_invocable___CcTemplateInstN4absl12AnyInvocableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
            f: *mut ::any_invocable::RawAnyInvocable,
        );
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs11DynCallableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::ffi_11::c_int,
    ) {
        (unsafe { &*f })(param_0);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN2rs11DynCallableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &*f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN2rs11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
    ) {
        (unsafe {
            ::core::ptr::replace(
                f,
                ::alloc::boxed::Box::new(|| {
                    ::core::unreachable!("Called FnOnce after it was moved");
                }),
            )
        })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN2rs11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
        to: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::ABICompatible,
        out: *mut crate::ABICompatible,
    ) {
        let param_0 = unsafe { ::core::ptr::read(param_0) };
        unsafe {
            out.write((unsafe { &*f })(param_0));
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN2rs11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::LayoutCompatible,
        out: *mut crate::LayoutCompatible,
    ) {
        let param_0 = unsafe { ::core::ptr::read(param_0) };
        unsafe {
            out.write((unsafe { &*f })(param_0));
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN2rs11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::core::ffi::c_uchar,
        bridge_buffer: *mut ::core::ffi::c_uchar,
    ) {
        let param_0 =
            unsafe { ::crubit_support::bridge::internal::decode(crate::RustBridgedAbi, param_0) };
        unsafe {
            ::crubit_support::bridge::internal::encode(
                crate::RustBridgedAbi,
                bridge_buffer,
                (unsafe { &*f })(param_0),
            )
        };
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN2rs11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
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
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN2rs11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &*f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN2rs11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::ffi_11::c_int,
    ) {
        (unsafe { &*f })(param_0);
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvRiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(*mut ::ffi_11::c_int)
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &*f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
    ) {
        (unsafe {
            ::core::ptr::replace(
                f,
                ::alloc::boxed::Box::new(|| {
                    ::core::unreachable!("Called FnOnce after it was moved");
                }),
            )
        })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIFvvOEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
        to: *mut ::alloc::boxed::Box<dyn ::core::ops::FnOnce() + ::core::marker::Send + 'static>,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::ABICompatible,
        out: *mut crate::ABICompatible,
    ) {
        let param_0 = unsafe { ::core::ptr::read(param_0) };
        unsafe {
            out.write((unsafe { &*f })(param_0));
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF13ABICompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::ABICompatible) -> crate::ABICompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut crate::LayoutCompatible,
        out: *mut crate::LayoutCompatible,
    ) {
        let param_0 = unsafe { ::core::ptr::read(param_0) };
        unsafe {
            out.write((unsafe { &*f })(param_0));
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF16LayoutCompatibleS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::LayoutCompatible) -> crate::LayoutCompatible
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        param_0: *mut ::core::ffi::c_uchar,
        bridge_buffer: *mut ::core::ffi::c_uchar,
    ) {
        let param_0 =
            unsafe { ::crubit_support::bridge::internal::decode(crate::RustBridgedAbi, param_0) };
        unsafe {
            ::crubit_support::bridge::internal::encode(
                crate::RustBridgedAbi,
                bridge_buffer,
                (unsafe { &*f })(param_0),
            )
        };
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKF7BridgedS1_EEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(crate::RustBridged) -> crate::RustBridged
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
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
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int
                + ::core::marker::Send
                + ::core::marker::Sync
                + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN6rs_std11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        (unsafe { &*f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_manager___CcTemplateInstN6rs_std11DynCallableIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        operation: ::dyn_callable_rs::FunctionToCall,
        from: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
        to: *mut ::alloc::boxed::Box<
            dyn ::core::ops::Fn() + ::core::marker::Send + ::core::marker::Sync + 'static,
        >,
    ) {
        unsafe {
            ::dyn_callable_rs::manager(operation, from, to);
        }
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs5FnRefIFbiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *const ::core::ffi::c_void,
        param_0: ::ffi_11::c_int,
    ) -> bool {
        let f = unsafe {
            ::core::ptr::read_unaligned(
                f.cast::<*const (dyn ::core::ops::Fn(::ffi_11::c_int) -> bool + '_)>(),
            )
        };
        (unsafe { &*f })(param_0)
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs5FnRefIFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *const ::core::ffi::c_void,
    ) {
        let f =
            unsafe { ::core::ptr::read_unaligned(f.cast::<*const (dyn ::core::ops::Fn() + '_)>()) };
        (unsafe { &*f })();
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs5FnRefIKFiiEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *const ::core::ffi::c_void,
        param_0: ::ffi_11::c_int,
    ) -> ::ffi_11::c_int {
        let f = unsafe {
            ::core::ptr::read_unaligned(
                f.cast::<*const (dyn ::core::ops::Fn(::ffi_11::c_int) -> ::ffi_11::c_int + '_)>(),
            )
        };
        (unsafe { &*f })(param_0)
    }
    #[unsafe(no_mangle)]
    unsafe extern "C" fn __crubit_invoker___CcTemplateInstN2rs5FnRefIKFvvEEE__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3acallables_5fcc(
        f: *const ::core::ffi::c_void,
    ) {
        let f =
            unsafe { ::core::ptr::read_unaligned(f.cast::<*const (dyn ::core::ops::Fn() + '_)>()) };
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
    assert!(::core::mem::size_of::<crate::StructWithFnRef>() == 16);
    assert!(::core::mem::align_of::<crate::StructWithFnRef>() == 8);
    static_assertions::assert_impl_all!(crate::StructWithFnRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithFnRef: Drop);
    assert!(::core::mem::offset_of!(crate::StructWithFnRef, f) == 0);
    assert!(::core::mem::size_of::<crate::NestedStructWithFnRef>() == 16);
    assert!(::core::mem::align_of::<crate::NestedStructWithFnRef>() == 8);
    static_assertions::assert_impl_all!(crate::NestedStructWithFnRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NestedStructWithFnRef: Drop);
    assert!(::core::mem::offset_of!(crate::NestedStructWithFnRef, inner) == 0);
    assert!(::core::mem::size_of::<crate::DerivedFromStructWithFnRef>() == 16);
    assert!(::core::mem::align_of::<crate::DerivedFromStructWithFnRef>() == 8);
    static_assertions::assert_impl_all!(crate::DerivedFromStructWithFnRef: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DerivedFromStructWithFnRef: Drop);
};
