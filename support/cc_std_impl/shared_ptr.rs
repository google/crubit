// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::crubit_cc_std_internal::std_allocator;

#[allow(non_snake_case)]
#[repr(C)]
pub struct shared_ptr_const<T: Sized> {
    ptr: *const T,
    cntrl: *mut core::ffi::c_void,
}

impl<T: Sized> shared_ptr_const<T> {
    pub unsafe fn from_raw_parts(ptr: *const T, cntrl: *mut core::ffi::c_void) -> Self {
        Self { ptr, cntrl }
    }

    pub fn get(&self) -> *const T {
        self.ptr
    }

    pub fn as_ref(&self) -> Option<&T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: Sized> Clone for shared_ptr_const<T> {
    fn clone(&self) -> Self {
        if !self.cntrl.is_null() {
            unsafe {
                std_allocator::shared_ptr_add_shared(self.cntrl);
            }
        }
        Self { ptr: self.ptr, cntrl: self.cntrl }
    }
}

impl<T: Sized> Drop for shared_ptr_const<T> {
    fn drop(&mut self) {
        if !self.cntrl.is_null() {
            unsafe {
                std_allocator::shared_ptr_release_shared(self.cntrl);
            }
        }
    }
}
