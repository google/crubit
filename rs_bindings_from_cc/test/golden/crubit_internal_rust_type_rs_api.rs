// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

pub mod crubit {
    pub mod rust_type { // error: class `crubit::rust_type::Args` could not be bound
                        //   Class templates are not yet supported

        // error: class `crubit::rust_type::Const` could not be bound
        //   Class templates are not yet supported
    }
}

// namespace crubit::rust_type

// Type bindings for MyI8Struct suppressed due to being mapped to an existing Rust type (i8)

// Note that this is potentially visited, even if the original declaration is
// skipped due to crubit_internal_rust_type.

// Type bindings for MyI8Class suppressed due to being mapped to an existing Rust type (i8)

// Type bindings for MyI8Enum suppressed due to being mapped to an existing Rust type (i8)

// Type bindings for MyI8Alias suppressed due to being mapped to an existing Rust type (i8)

/// Invalid annotations cause bindings to fail to be generated.
/// (It's important not to fall back to the underlying type, since the user
/// intent was to override it.)
/// Uncomment these invalid annotations to observe the build-time errors.
/// TODO: b/402989591 - Use compile-fail UI test to check these outputs.
pub type TooFewArgs = ::ffi_11::c_uchar;

pub type TooManyArgs = ::ffi_11::c_uchar;

pub type NonStringArg = ::ffi_11::c_uchar;

pub type BadSameAbiAttr = ::ffi_11::c_uchar;

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: ExistingRustTypeFieldTypes
pub struct ExistingRustTypeFieldTypes {
    pub my_i8_struct: i8,
    pub my_i8_class: i8,
    pub my_i8_enum: i8,
    pub my_i8_alias: i8,
    pub error: crate::TooFewArgs,
}
impl !Send for ExistingRustTypeFieldTypes {}
impl !Sync for ExistingRustTypeFieldTypes {}
unsafe impl ::cxx::ExternType for ExistingRustTypeFieldTypes {
    type Id = ::cxx::type_id!(":: ExistingRustTypeFieldTypes");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ExistingRustTypeFieldTypes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN26ExistingRustTypeFieldTypesC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// error: class `Ptr` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn AcceptPtrInt(mut ptr: crate::RustPtr<::ffi_11::c_int>) {
    unsafe { crate::detail::__rust_thunk___Z12AcceptPtrInt3PtrIiE(&mut ptr) }
}

// error: class `CppTypeWithTemplateArgs` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn AcceptCppTypeWithTemplateArgs(
    mut cpp_type: crate::RustTypeWithReorderedGenerics<::ffi_11::c_int, f32, true>,
) {
    unsafe {
        crate::detail::__rust_thunk___Z29AcceptCppTypeWithTemplateArgs23CppTypeWithTemplateArgsIifLb1EE(&mut cpp_type)
    }
}

// error: class `ConvertPtrs` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn AcceptReordered(mut x: crate::RustTypeReordered<f32, ::ffi_11::c_int>) {
    unsafe { crate::detail::__rust_thunk___Z15AcceptReordered11ConvertPtrsIfiE(&mut x) }
}

// error: class `WithDefault` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn AcceptWithDefault(mut x: crate::RustTypeWithDefault<f32, ::ffi_11::c_int>) {
    unsafe { crate::detail::__rust_thunk___Z17AcceptWithDefault11WithDefaultIfiE(&mut x) }
}

// error: class `MyContainer` could not be bound
//   Class templates are not yet supported

#[inline(always)]
pub fn AcceptSpecialized(
    mut a: crate::MyRustContainer<::ffi_11::c_int>,
    mut b: crate::MyRustContainerVoid,
) {
    unsafe {
        crate::detail::__rust_thunk___Z17AcceptSpecialized11MyContainerIiES_IvE(&mut a, &mut b)
    }
}

// Type bindings for Ptr<int> suppressed due to being mapped to an existing Rust type (crate::RustPtr<::ffi_11::c_int>)

// Type bindings for CppTypeWithTemplateArgs<int, float, true> suppressed due to being mapped to an existing Rust type (crate::RustTypeWithReorderedGenerics<::ffi_11::c_int,f32,true>)

// Type bindings for ConvertPtrs<float, int> suppressed due to being mapped to an existing Rust type (crate::RustTypeReordered<f32,::ffi_11::c_int>)

// Type bindings for WithDefault<float> suppressed due to being mapped to an existing Rust type (crate::RustTypeWithDefault<f32,::ffi_11::c_int>)

// Type bindings for MyContainer<int> suppressed due to being mapped to an existing Rust type (crate::MyRustContainer<::ffi_11::c_int>)

// Type bindings for MyContainer<void> suppressed due to being mapped to an existing Rust type (crate::MyRustContainerVoid)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN26ExistingRustTypeFieldTypesC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___Z12AcceptPtrInt3PtrIiE(
            ptr: &mut crate::RustPtr<::ffi_11::c_int>,
        );
        pub(crate) unsafe fn __rust_thunk___Z29AcceptCppTypeWithTemplateArgs23CppTypeWithTemplateArgsIifLb1EE(
            cpp_type: &mut crate::RustTypeWithReorderedGenerics<::ffi_11::c_int, f32, true>,
        );
        pub(crate) unsafe fn __rust_thunk___Z15AcceptReordered11ConvertPtrsIfiE(
            x: &mut crate::RustTypeReordered<f32, ::ffi_11::c_int>,
        );
        pub(crate) unsafe fn __rust_thunk___Z17AcceptWithDefault11WithDefaultIfiE(
            x: &mut crate::RustTypeWithDefault<f32, ::ffi_11::c_int>,
        );
        pub(crate) unsafe fn __rust_thunk___Z17AcceptSpecialized11MyContainerIiES_IvE(
            a: &mut crate::MyRustContainer<::ffi_11::c_int>,
            b: &mut crate::MyRustContainerVoid,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<crate::ExistingRustTypeFieldTypes>() == 5);
    assert!(::core::mem::align_of::<crate::ExistingRustTypeFieldTypes>() == 1);
    static_assertions::assert_impl_all!(crate::ExistingRustTypeFieldTypes: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ExistingRustTypeFieldTypes: Drop);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, my_i8_struct) == 0);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, my_i8_class) == 1);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, my_i8_enum) == 2);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, my_i8_alias) == 3);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, error) == 4);
    assert!(::core::mem::size_of::<crate::RustPtr<::ffi_11::c_int>>() == 8);
    assert!(::core::mem::align_of::<crate::RustPtr<::ffi_11::c_int>>() == 8);
    assert!(
        ::core::mem::size_of::<crate::RustTypeWithReorderedGenerics<::ffi_11::c_int, f32, true>>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::RustTypeWithReorderedGenerics<::ffi_11::c_int, f32, true>>()
            == 8
    );
    assert!(::core::mem::size_of::<crate::RustTypeReordered<f32, ::ffi_11::c_int>>() == 1);
    assert!(::core::mem::align_of::<crate::RustTypeReordered<f32, ::ffi_11::c_int>>() == 1);
    assert!(::core::mem::size_of::<crate::RustTypeWithDefault<f32, ::ffi_11::c_int>>() == 1);
    assert!(::core::mem::align_of::<crate::RustTypeWithDefault<f32, ::ffi_11::c_int>>() == 1);
    assert!(::core::mem::size_of::<crate::MyRustContainer<::ffi_11::c_int>>() == 1);
    assert!(::core::mem::align_of::<crate::MyRustContainer<::ffi_11::c_int>>() == 1);
    assert!(::core::mem::size_of::<crate::MyRustContainerVoid>() == 1);
    assert!(::core::mem::align_of::<crate::MyRustContainerVoid>() == 1);
};
