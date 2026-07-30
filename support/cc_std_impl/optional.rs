// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::mem::MaybeUninit;

/// C++ std::optional<T>.
// This is the layout that libc++ uses, which is guaranteed to be stable:
// https://github.com/llvm/llvm-project/blob/main/libc/src/__support/CPP/optional.h#L34-L39
// TODO(b/493911621): Add crubit_annotate::layout_compatible annotation.
#[allow(non_snake_case)]
#[repr(C)]
pub struct optional<T> {
    // Safety invariant: payload is initialized if and only if engaged is true.
    payload: MaybeUninit<T>,
    engaged: bool,
}

impl<T> Drop for optional<T> {
    fn drop(&mut self) {
        if self.engaged {
            // Safety: payload is initialized because engaged is true.
            unsafe { self.payload.assume_init_drop() };
        }
    }
}
