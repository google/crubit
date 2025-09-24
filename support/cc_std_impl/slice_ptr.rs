// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Emulate the (unstable) `to_raw_parts` on slices, to get the pointer and
/// length of the slice, without requiring turning on the unstable feature flag.
// TODO: b/324045078 - Move slice_ptr into its own crate once `cc_std` can
// depend on a Rust crate.
pub mod slice_ptr {
    pub fn get_raw_parts<T>(slice: *const [T]) -> (*const T, usize) {
        #[allow(dead_code)]
        #[repr(C)]
        struct RawSlice<T>(*const T, usize);
        // # Safety:
        // No stable way to do this per se, but the UCG documents this transmute is OK.
        // An RFC is in the works to officially stabilize this. It is expected
        // to be noncontroversial.
        //
        // See:
        //
        // * UCG: https://rust-lang.github.io/unsafe-code-guidelines/layout/pointers.html
        // * Zulip: https://rust-lang.zulipchat.com/#narrow/stream/213817-t-lang/topic/Can.20we.20stabilize.20the.20layout.20of.20.26.5BT.5D.20and.20.26str.3F/near/394683395
        let RawSlice::<T>(ptr, size) = unsafe { core::mem::transmute(slice) };
        (ptr, size)
    }

    pub fn get_raw_parts_mut<T>(slice: *mut [T]) -> (*mut T, usize) {
        // Assumption: The layout isn't affected by whether a pointer is mut or const.
        let (ptr, size) = get_raw_parts::<T>(slice);
        (ptr as *mut _, size)
    }
}
