// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use ctor::emplace;
use ctor::CtorNew;
use googletest::gtest;
use static_assertions::assert_not_impl_any;
use thread_safe::crubit::test::ThreadSafeStruct;

#[gtest]
fn test_thread_safe_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<ThreadSafeStruct>();
}

#[gtest]
fn test_thread_safe_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<ThreadSafeStruct>();
}

#[gtest]
fn test_thread_safe_struct_methods_via_shared_self_ref() {
    let s = emplace!(ThreadSafeStruct::ctor_new(()));
    assert_eq!(s.NonConstGet(), 0);
    assert_eq!(s.ConstGet(), 0);
}

#[gtest]
fn test_regular_struct_is_not_send_or_sync() {
    assert_not_impl_any!(thread_safe::crubit::test::RegularStruct: Send, Sync);
}

#[gtest]
fn test_regular_struct_round_trip_via_raw_ptr() {
    use thread_safe::crubit::test::RegularStruct;
    let s = emplace!(RegularStruct::ctor_new(()));
    // The non-thread-safe struct must use raw pointers for method calls.
    let ptr: *mut RegularStruct = s.as_ref().get_ref() as *const _ as *mut _;
    unsafe {
        assert_eq!(RegularStruct::ConstGet(ptr as *const _), 0);
        assert_eq!(RegularStruct::NonConstGet(ptr), 0);
    }
}
