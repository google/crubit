// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Type invariant: must only contain a valid, immortal pointer to an `UnsupportedType`.
#[derive(Copy, Clone)]
pub struct WrapperType(*mut core::ffi::c_void);

pub fn get_global() -> WrapperType {
    WrapperType(crate::GetGlobalUnsupportedType() as *mut _)
}

impl WrapperType {
    pub fn set(self, value: i32) {
        // SAFETY: never null, always valid for writes
        unsafe {
            crate::SetValue(self.0 as *mut _, value);
        }
    }

    pub fn get(self) -> i32 {
        // SAFETY: never null, always valid for reads
        unsafe { crate::GetValue(self.0 as *const _ as *const _) }
    }
}
