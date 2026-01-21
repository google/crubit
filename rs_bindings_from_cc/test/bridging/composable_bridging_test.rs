// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use composable_bridging_lib::*;
use googletest::prelude::*;

#[gtest]
fn test_vec3() {
    expect_eq!(MakeVec3(1.0, 2.0, 3.0), Vec3 { x: 1.0, y: 2.0, z: 3.0 });
}

#[gtest]
fn test_pair() {
    expect_eq!(MakePair(1, 2.0, true), ((1, 2.0), true));
}

#[gtest]
fn test_optional() {
    expect_eq!(MakeOptionalVec3(1.0, 2.0, 3.0, true), Some(Vec3 { x: 1.0, y: 2.0, z: 3.0 }));
    expect_eq!(MakeOptionalVec3(1.0, 2.0, 3.0, false), None);
}

#[gtest]
fn test_map_multiply() {
    expect_eq!(
        MapMultiply(Some(Vec3 { x: 1.0, y: 2.0, z: 3.0 }), 2.0),
        Some(Vec3 { x: 2.0, y: 4.0, z: 6.0 })
    );
    expect_eq!(MapMultiply(None, 2.0), None);
}

#[gtest]
fn test_stuff() {
    expect_eq!(MakeStuff(), (None, Some((3.14, Vec3 { x: 1.0, y: 2.0, z: 3.0 }))));
}

#[gtest]
fn test_vec3_of_structs() {
    expect_eq!(
        MakeVec3OfStructs(Stuff { i: 1, f: 2.0 }, Stuff { i: 3, f: 4.0 }, Stuff { i: 5, f: 6.0 }),
        Vec3 { x: Stuff { i: 1, f: 2.0 }, y: Stuff { i: 3, f: 4.0 }, z: Stuff { i: 5, f: 6.0 } }
    );
}

#[gtest]
fn test_greeting() {
    expect_eq!(ReturnProperGreeting(), cc_std::std::string::from("Hello, world!"));
    expect_true!(IsProperGreeting(cc_std::std::string::from("Hello, world!")));
    expect_false!(IsProperGreeting(cc_std::std::string::from("Hi, world!")));
}

#[gtest]
fn test_properly_greet_stuff() {
    expect_eq!(
        ProperlyGreetStuff(Stuff { i: 1, f: 2.0 }),
        (cc_std::std::string::from("Hello, world!"), Stuff { i: 1, f: 2.0 })
    );
}

#[gtest]
fn test_string_view_by_value() {
    fn live(value: cc_std::std::raw_string_view) -> &'static [u8] {
        unsafe { &*value.as_raw_bytes() }
    }
    expect_eq!(live(StringViewByValue("Hello".into())), b"Hello");
}

#[gtest]
fn test_return_optional_string_view() {
    fn live(value: Option<cc_std::std::raw_string_view>) -> Option<&'static [u8]> {
        value.map(|sv| unsafe { &*sv.as_raw_bytes() })
    }
    expect_eq!(live(ReturnOptionalStringView(true, "Hello".into())), live(Some("Hello".into())));
    expect_eq!(live(ReturnOptionalStringView(false, "Hello".into())), live(None));
}

#[gtest]
fn test_return_slice_ref_string_view() {
    let value: &[cc_std::std::raw_string_view] =
        &[cc_std::std::raw_string_view::from("return slice ref string view test")];

    // TODO(b/440573418): Eventually this should take an `Alias<[cc_std::std::raw_string_view]>`,
    let result: *const [cc_std::std::raw_string_view] =
        unsafe { ReturnSliceRefStringView(value as *const [_]) };

    let result: &[cc_std::std::raw_string_view] = unsafe { &*result };

    assert_that!(result.len(), eq(1));

    let value: &[u8] = unsafe { &*value[0].as_raw_bytes() };
    let result: &[u8] = unsafe { &*result[0].as_raw_bytes() };
    expect_that!(result, eq(value))
}

#[gtest]
fn test_status_of_pointer_is_bridged() {
    let result = AcceptsVoidPtrAndReturnsStatusErrorIfNull(core::ptr::null_mut());
    expect_that!(
        result,
        status_rs_matchers::status_is(status::absl::StatusErrorCode::InvalidArgument)
    );

    let mut thing = true;
    let void_ptr: *mut core::ffi::c_void = &raw mut thing as *mut _;
    let result = AcceptsVoidPtrAndReturnsStatusErrorIfNull(void_ptr);
    expect_that!(result, ok(eq(&void_ptr)),);
}

#[gtest]
fn test_status_of_slice_ref_is_bridged_as_slice_ptr() {
    let empty_slice: &[core::ffi::c_int] = &[];
    let result = AcceptsSliceAndReturnsStatusErrorIfEmpty(empty_slice as *const _);
    expect_that!(
        result,
        status_rs_matchers::status_is(status::absl::StatusErrorCode::InvalidArgument)
    );

    let non_empty_slice: &[core::ffi::c_int] = &[1, 2, 3];
    let result = AcceptsSliceAndReturnsStatusErrorIfEmpty(non_empty_slice as *const _);
    expect_that!(result, ok(eq(&(non_empty_slice as *const _))));
}

#[gtest]
fn test_optional_my_struct() {
    let x = ReturnOptionalMyStruct();
    assert_eq!(x.unwrap().x, 42);
}

#[gtest]
fn test_composable_bridge_with_enum_inside() {
    assert_eq!(ValidateMyEnum(MyEnum::kFoo), Some(MyEnum::kFoo));
    assert_eq!(ValidateMyEnum(MyEnum::kBar), Some(MyEnum::kBar));
    assert_eq!(ValidateMyEnum(MyEnum::from(42)), None);
}
