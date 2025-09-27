// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This is the Rust type to use to "own" the C++ *Thing.

use core::ptr::NonNull;

#[repr(transparent)]
pub struct OwnedThing {
    thing: NonNull<crate::Thing>,
}

impl Drop for OwnedThing {
    fn drop(&mut self) {
        unsafe {
            crate::Thing::Close(self.thing.as_mut());
        }
    }
}
