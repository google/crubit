// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn make_and_use_thing_with_owned_ptr() {
    let thing: owned_ptr::OwnedThing = owned_ptr_user::MakeOwnedThing(42);

    unsafe {
        assert_eq!(owned_ptr_user::ThingToValue(thing), 42);
    }
}

#[gtest]
fn make_and_drop_thing_with_owned_ptr() {
    let _thing: owned_ptr::OwnedThing = owned_ptr_user::MakeOwnedThing(42);
}

#[gtest]
fn make_and_use_thing_with_raw_ptr() {
    let thing: *mut owned_ptr::RawThing = owned_ptr_user::MakeThing(42);

    unsafe {
        assert_eq!(owned_ptr_user::GetThingValue(thing), 42);

        // Since we did not transfer ownersship, we need to close it.
        owned_ptr::RawThing::Close(thing);
    }
}
