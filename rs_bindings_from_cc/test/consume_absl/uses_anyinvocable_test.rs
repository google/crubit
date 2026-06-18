// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_eq, expect_that, gtest};
use std::sync::{Arc, Mutex};
use uses_anyinvocable::*;

#[gtest]
fn test_call_void_void() {
    let called = Arc::new(Mutex::new(false));
    let called_clone = Arc::clone(&called);
    CallVoidVoid(Box::new(move || -> () {
        assert_eq!(Arc::strong_count(&called_clone), 2);
        *called_clone.lock().unwrap() = true;
    }));
    assert_eq!(Arc::strong_count(&called), 1);
    expect_that!(*called.lock().unwrap(), eq(true));
}

#[gtest]
fn test_call_with_any_invocable_param() {
    let res = CallWithAnyInvocableParam(Box::new(|g: Box<dyn FnOnce() + Send + Sync>| {
        g();
    }));
    expect_that!(res, eq(true));
}

#[gtest]
fn test_return_int_void() {
    let f: Box<dyn Fn(i32) -> i32 + Send + Sync> = ReturnIntMapper();
    expect_that!(f(41), eq(42));
}

#[gtest]
fn test_call_int_int() {
    expect_that!(CallIntInt(Box::new(|x: i32| -> i32 { x + 1 }), 41), eq(42));
}

#[gtest]
fn test_return_optional_int_mapper() {
    let f: Box<dyn Fn(Option<i32>) -> Option<i32> + Send + Sync> = ReturnOptionalIntMapper();
    expect_eq!(f(Some(41)), Some(42));
    expect_eq!(f(None), None);
}

#[gtest]
fn test_call_optional_int_mapper() {
    let f = |x: Option<i32>| -> Option<i32> { x.map(|x| x + 1) };
    expect_eq!(CallOptionalIntMapper(Box::new(f), Some(41)), Some(42));
    expect_eq!(CallOptionalIntMapper(Box::new(f), None), None);
}

#[gtest]
fn test_return_pointer_mapper() {
    let mut val = 42;
    let ptr = &mut val as *mut i32;
    let f: Box<dyn Fn(*mut i32) -> *mut i32 + Send + Sync> = ReturnPointerMapper();
    let res = f(ptr);
    expect_eq!(res, ptr);
}

#[gtest]
fn test_call_pointer_mapper() {
    let mut val = 42;
    let ptr = &mut val as *mut i32;
    unsafe {
        let res = CallPointerMapper(Box::new(|x: *mut i32| -> *mut i32 { x }), ptr);
        expect_eq!(*res, *ptr);
    }
}

#[gtest]
fn test_return_reference_mapper() {
    let mut val = 42;
    let f: Box<dyn Fn(*mut i32) -> *mut i32 + Send + Sync> = ReturnReferenceMapper();
    let res = f(&mut val);
    expect_eq!(unsafe { *res }, 42);
}

#[gtest]
fn test_call_reference_mapper() {
    let mut val = 42;
    unsafe {
        let res: *mut i32 =
            CallReferenceMapper(Box::new(|x: *mut i32| -> *mut i32 { x }), &mut val);
        expect_eq!(*res, 42);
    }
}

extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

#[gtest]
fn test_return_func_ptr_mapper() {
    let f: Box<
        dyn Fn(Option<extern "C" fn(i32) -> i32>) -> Option<extern "C" fn(i32) -> i32>
            + Send
            + Sync,
    > = ReturnFuncPtrMapper();
    let mapped_func = f(Some(add_one));
    expect_eq!(mapped_func.unwrap()(41), 42);
}

#[gtest]
fn test_call_func_ptr_mapper() {
    let mapped_func: Option<extern "C" fn(i32) -> i32> = CallFuncPtrMapper(
        Box::new(|x: Option<extern "C" fn(i32) -> i32>| -> Option<extern "C" fn(i32) -> i32> { x }),
        Some(add_one),
    );
    expect_eq!(mapped_func.unwrap()(41), 42);
}

#[gtest]
fn test_return_enum_mapper() {
    let f: Box<dyn Fn(MyEnum) -> MyEnum + Send + Sync> = ReturnEnumMapper();
    expect_eq!(f(MyEnum::kZero), MyEnum::kOne);
    expect_eq!(f(MyEnum::kOne), MyEnum::kZero);
}

#[gtest]
fn test_call_enum_mapper() {
    let f = |x| match x {
        MyEnum::kZero => MyEnum::kOne,
        MyEnum::kOne => MyEnum::kZero,
        _ => panic!("Unexpected enum value"),
    };
    expect_eq!(CallEnumMapper(Box::new(f), MyEnum::kZero), MyEnum::kOne);
    expect_eq!(CallEnumMapper(Box::new(f), MyEnum::kOne), MyEnum::kZero);
}

#[gtest]
fn test_return_type_alias_mapper() {
    let f = ReturnTypeAliasMapper();
    expect_eq!(f(41), 42);
}

#[gtest]
fn test_call_type_alias_mapper() {
    let res = CallTypeAliasMapper(Box::new(|x: i32| -> i32 { x + 1 }), 41);
    expect_eq!(res, 42);
}

#[gtest]
fn test_return_record_mapper() {
    let f: Box<dyn Fn(MyStruct) -> MyStruct + Send + Sync> = ReturnRecordMapper();
    let s = MyStruct { value: 41 };
    expect_eq!(f(s).value, 42);
}

#[gtest]
fn test_call_record_mapper() {
    let s = MyStruct { value: 41 };
    let res: MyStruct = CallRecordMapper(
        Box::new(|x: MyStruct| -> MyStruct { MyStruct { value: x.value + 1 } }),
        s,
    );
    expect_eq!(res.value, 42);
}

#[gtest]
fn test_return_rvalue_ref_consumer() {
    let f: Box<dyn Fn(*mut i32) -> i32 + Send + Sync> = ReturnRvalueRefConsumer();
    let mut val = 41;
    expect_eq!(f(&mut val), 42);
}

#[gtest]
fn test_call_rvalue_ref_consumer() {
    let mut val = 41;
    let res: i32 = unsafe {
        CallRvalueRefConsumer(
            Box::new(|x: *mut i32| -> i32 {
                *x += 1;
                *x
            }),
            &mut val,
        )
    };
    expect_eq!(res, 42);
}

#[gtest]
fn test_return_incomplete_pointer_mapper() {
    let f: Box<dyn Fn(*mut Incomplete) -> *mut Incomplete + Send + Sync> =
        ReturnIncompletePointerMapper();
    let ptr = std::ptr::null_mut::<Incomplete>();
    expect_eq!(f(ptr), ptr);
}

#[gtest]
fn test_call_incomplete_pointer_mapper() {
    let ptr = std::ptr::null_mut::<Incomplete>();
    let res: *mut Incomplete = unsafe {
        CallIncompletePointerMapper(Box::new(|x: *mut Incomplete| -> *mut Incomplete { x }), ptr)
    };
    expect_eq!(res, ptr);
}

#[gtest]
fn test_return_record_rvalue_ref_consumer() {
    let f: Box<dyn Fn(*mut MyStruct) -> i32 + Send + Sync> = ReturnRecordRvalueRefConsumer();
    let mut s = MyStruct { value: 41 };
    expect_eq!(f(&mut s), 42);
}

#[gtest]
fn test_call_record_rvalue_ref_consumer() {
    let mut s = MyStruct { value: 41 };
    let res: i32 = unsafe {
        CallRecordRvalueRefConsumer(
            // TODO(b/489098131): Currently, AnyInvocable maps rvalue references to raw pointers.
            // But we should be using ctor::RvalueReference.
            Box::new(|x: *mut MyStruct| -> i32 {
                (*x).value += 1;
                (*x).value
            }),
            &mut s,
        )
    };
    expect_eq!(res, 42);
}
