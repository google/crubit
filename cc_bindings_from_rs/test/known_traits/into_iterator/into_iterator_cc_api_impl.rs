// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// into_iterator_rust_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () =
    assert!(::std::mem::size_of::<::into_iterator_rust_golden::ContainerWithInherentBegin>() == 12);
const _: () =
    assert!(::std::mem::align_of::<::into_iterator_rust_golden::ContainerWithInherentBegin>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_begin(
    __self: &'static ::into_iterator_rust_golden::ContainerWithInherentBegin,
) -> i32 {
    unsafe { ::into_iterator_rust_golden::ContainerWithInherentBegin::begin(__self) }
}
const _: () = assert!(
    ::core::mem::offset_of!(::into_iterator_rust_golden::ContainerWithInherentBegin, data) == 0
);
const _: () =
    assert!(::std::mem::size_of::<::into_iterator_rust_golden::ContainerWithRefIntoIter>() == 8);
const _: () =
    assert!(::std::mem::align_of::<::into_iterator_rust_golden::ContainerWithRefIntoIter>() == 8);
const _: () = assert!(
    ::core::mem::offset_of!(::into_iterator_rust_golden::ContainerWithRefIntoIter, iter) == 0
);
const _: () = assert!(::std::mem::size_of::<::into_iterator_rust_golden::MyContainer>() == 12);
const _: () = assert!(::std::mem::align_of::<::into_iterator_rust_golden::MyContainer>() == 4);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uiter(
    __self: &'static mut ::core::mem::MaybeUninit<::into_iterator_rust_golden::MyContainer>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            <::into_iterator_rust_golden::MyContainer as ::core::iter::IntoIterator>::into_iter(
                __self,
            );
        (__ret_ptr as *mut ::into_iterator_rust_golden::MyContainerIntoIter)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uiter(
    __self: &'static ::into_iterator_rust_golden::MyContainer,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<&'static::into_iterator_rust_golden::MyContainer as::core::iter::IntoIterator>::into_iter(__self);
        (__ret_ptr as *mut ::into_iterator_rust_golden::MyContainerIter<'static>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_into_uiter(
    __self: &'static mut ::into_iterator_rust_golden::MyContainer,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value=<&'static mut::into_iterator_rust_golden::MyContainer as::core::iter::IntoIterator>::into_iter(__self);
        (__ret_ptr as *mut ::into_iterator_rust_golden::MyContainerIterMut<'static>)
            .write(__rs_return_value);
    }
}
const _: () = assert!(::core::mem::offset_of!(::into_iterator_rust_golden::MyContainer, data) == 0);
const _: () =
    assert!(::std::mem::size_of::<::into_iterator_rust_golden::MyContainerIntoIter>() == 24);
const _: () =
    assert!(::std::mem::align_of::<::into_iterator_rust_golden::MyContainerIntoIter>() == 8);
const _: () = assert!(::std::mem::size_of::<::into_iterator_rust_golden::MyContainerIter>() == 16);
const _: () = assert!(::std::mem::align_of::<::into_iterator_rust_golden::MyContainerIter>() == 8);
const _: () =
    assert!(::std::mem::size_of::<::into_iterator_rust_golden::MyContainerIterMut>() == 16);
const _: () =
    assert!(::std::mem::align_of::<::into_iterator_rust_golden::MyContainerIterMut>() == 8);
const _: () = assert!(::std::mem::size_of::<::into_iterator_rust_golden::MyIterator>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_iterator_rust_golden::MyIterator>() == 4);
const _: () = assert!(::core::mem::offset_of!(::into_iterator_rust_golden::MyIterator, value) == 0);
const _: () = assert!(::std::mem::size_of::<::into_iterator_rust_golden::SimpleIntoIter>() == 4);
const _: () = assert!(::std::mem::align_of::<::into_iterator_rust_golden::SimpleIntoIter>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::into_iterator_rust_golden::SimpleIntoIter, val) == 0);
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_ucontainer(
    a: i32,
    b: i32,
    c: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::into_iterator_rust_golden::make_container(a, b, c);
        (__ret_ptr as *mut ::into_iterator_rust_golden::MyContainer).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uinherent_ucontainer(
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::into_iterator_rust_golden::make_inherent_container();
        (__ret_ptr as *mut ::into_iterator_rust_golden::ContainerWithInherentBegin)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uiterator(
    value: i32,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::into_iterator_rust_golden::make_iterator(value);
        (__ret_ptr as *mut ::into_iterator_rust_golden::MyIterator).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_make_uref_ucontainer(
    iter: &'static mut ::into_iterator_rust_golden::MyIterator,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __rs_return_value = ::into_iterator_rust_golden::make_ref_container(iter);
        (__ret_ptr as *mut ::into_iterator_rust_golden::ContainerWithRefIntoIter<'static>)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aContainerWithInherentBegin(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::into_iterator_rust_golden::ContainerWithInherentBegin,
    >,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value=<::into_iterator_rust_golden::ContainerWithInherentBegin as::core::iter::IntoIterator>::into_iter(__self);
        (__ret_ptr as *mut ::into_iterator_rust_golden::SimpleIntoIter).write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aContainerWithRefIntoIter_x0000003c_x00000027a_x0000003e(
    __self: &'static mut ::core::mem::MaybeUninit<
        ::into_iterator_rust_golden::ContainerWithRefIntoIter<'static>,
    >,
) -> &'static mut ::into_iterator_rust_golden::MyIterator {
    unsafe {
        let __self = __self.assume_init_read();
        <::into_iterator_rust_golden::ContainerWithRefIntoIter as::core::iter::IntoIterator>::into_iter(__self)
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
    __self: &'static mut ::core::mem::MaybeUninit<::into_iterator_rust_golden::MyContainer>,
    __ret_ptr: *mut core::ffi::c_void,
) -> () {
    unsafe {
        let __self = __self.assume_init_read();
        let __rs_return_value =
            <::into_iterator_rust_golden::MyContainer as ::core::iter::IntoIterator>::into_iter(
                __self,
            );
        (__ret_ptr as *mut ::into_iterator_rust_golden::MyContainerIntoIter)
            .write(__rs_return_value);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIntoIter(
    __self: &'static mut ::into_iterator_rust_golden::MyContainerIntoIter,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_iterator_rust_golden::MyContainerIntoIter as ::core::iter::Iterator>::next(
                __self,
            );
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIter_x0000003c_x00000027a_x0000003e(
    __self: &'static mut ::into_iterator_rust_golden::MyContainerIter<'static>,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_iterator_rust_golden::MyContainerIter as ::core::iter::Iterator>::next(__self);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<&'static i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIterMut_x0000003c_x00000027a_x0000003e(
    __self: &'static mut ::into_iterator_rust_golden::MyContainerIterMut<'static>,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_iterator_rust_golden::MyContainerIterMut as ::core::iter::Iterator>::next(
                __self,
            );
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<&'static mut i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyIterator(
    __self: &'static mut ::into_iterator_rust_golden::MyIterator,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_iterator_rust_golden::MyIterator as ::core::iter::Iterator>::next(__self);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aSimpleIntoIter(
    __self: &'static mut ::into_iterator_rust_golden::SimpleIntoIter,
    __ret_ptr: *mut core::ffi::c_uchar,
) -> () {
    unsafe {
        let __rs_return_value =
            <::into_iterator_rust_golden::SimpleIntoIter as ::core::iter::Iterator>::next(__self);
        unsafe {
            ::bridge_rust::internal::encode(
                ::bridge_rust::OptionAbi(::bridge_rust::transmute_abi::<i32>()),
                __ret_ptr as *mut core::ffi::c_uchar,
                __rs_return_value,
            );
        }
    }
}
