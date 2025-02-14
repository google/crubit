// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::boxed::Box;
use std::panic::catch_unwind;
use std::process;
use std::slice;

/// Returns an `FfiU8SliceBox` containing a copy of the data in `ffi_u8_slice`.
/// The returned `FfiU8SliceBox` must be freed by calling `FreeFfiU8SliceBox()`.
///
/// # Safety
///
/// Expectations:
///    * function expects that param `input` is a FfiU8Slice for a valid array
///      of bytes with the given size.
///    * function expects that param `input` doesn't change during the call, but
///      it is okay if it changes afterwards (because the data from `input` will
///      be copied/boxed into heap).
///
/// Ownership:
///    * function doesn't take ownership of (in other words it borrows) the
///      param `input`
///    * function passes ownership of the returned value to the caller
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocFfiU8SliceBox(input: FfiU8Slice) -> FfiU8SliceBox {
    FfiU8SliceBox::from_boxed_slice(Box::<[u8]>::from(input.as_slice()))
}

/// Frees `FfiU8SliceBox` allocated by Rust.
///
/// # Safety
///
/// Expectations:
///    * function expects that param `sb` is a valid FfiU8SliceBox that has been
///      allocated earlier by AllocFfiU8SliceBox.
///    * function expects that there are no remaining references to
///      FfiU8SliceBox
///
/// Ownership:
///    * function takes ownership of the param `sb` and frees its memory.
#[unsafe(no_mangle)]
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
    /// Returns an FfiU8Slice pointing to the data of `slice`.
    pub fn from_slice(slice: &[u8]) -> FfiU8Slice {
        FfiU8Slice { ptr: slice.as_ptr(), size: slice.len() }
    }

    /// Borrows data pointed to by this `FfiU8Slice` as a slice.
    pub fn as_slice(&self) -> &[u8] {
        // Safety:
        // Instances of `FfiU8Slice` are only created by FFI functions, which are unsafe
        // themselves so it's their responsibility to maintain safety.
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
        // Instances of `FfiU8SliceBox` are either created by `from_boxed_slice`, which
        // is safe, or by FFI functions, which are unsafe themselves so it's
        // their responsibility to maintain safety.
        unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr as *mut u8, self.size)) }
    }
}

/// The environment that the bindings are generated for. This is used to
/// determine what kinds of non mandatory (but potentially useful) information is
/// generated.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Environment {
    /// The bindings are generated for a golden test.
    /// Source location doc comments and the features list are disabled to reduce
    /// noise.
    GoldenTest,
    /// The bindings are generated for production.
    /// Source location doc comments and the features list are enabled.
    Production,
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_from_into_ffi_u8_slice_box() {
        let slice = Box::<[u8]>::from(*b"Hello World!");
        let ffi_slice = FfiU8SliceBox::from_boxed_slice(slice.clone());
        assert_eq!(ffi_slice.into_boxed_slice(), slice);
    }
}
