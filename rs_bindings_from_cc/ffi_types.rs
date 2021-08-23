// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::boxed::Box;
use std::panic::catch_unwind;
use std::process;
use std::slice;

/// Frees `FfiU8SliceBox` allocated by Rust.
#[no_mangle]
pub unsafe extern "C" fn FreeFfiU8SliceBox(sb: FfiU8SliceBox) {
    catch_unwind(|| {
        let _ = sb.into_boxed_slice();
    })
    .unwrap_or_else(|_| process::abort())
}

#[repr(C)]
pub struct FfiU8Slice {
    ptr: *const u8,
    size: usize,
}

impl FfiU8Slice {
    /// Borrows data pointed to by this `FfiU8Slice` as a slice.
    pub fn as_slice(&self) -> &[u8] {
        // Safety:
        // Instances of `FfiU8Slice` are only created by FFI functions, which are unsafe themselves
        // so it's their responsibility to maintain safety.
        unsafe { slice::from_raw_parts(self.ptr, self.size) }
    }
}

#[repr(C)]
pub struct FfiU8SliceBox {
    ptr: *const u8,
    size: usize,
}

impl FfiU8SliceBox {
    pub fn from_boxed_slice(bytes: Box<[u8]>) -> FfiU8SliceBox {
        let slice = Box::leak(bytes);
        FfiU8SliceBox { ptr: slice.as_mut_ptr(), size: slice.len() }
    }

    /// Consumes self and returns boxed slice.
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        // Safety:
        // Instances of `FfiU8SliceBox` are either created by `from_boxed_slice`, which is safe,
        // or by FFI functions, which are unsafe themselves so it's their responsibility to maintain
        // safety.
        unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr as *mut u8, self.size)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_into_ffi_u8_slice_box() {
        let slice = Box::<[u8]>::from(*b"Hello World!");
        let ffi_slice = FfiU8SliceBox::from_boxed_slice(slice.clone());
        assert_eq!(ffi_slice.into_boxed_slice(), slice);
    }
}
