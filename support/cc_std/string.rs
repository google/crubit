// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate alloc;

use crate::crubit_cc_std_internal::conversion_function_helpers;
use alloc::string::String;
use alloc::vec::Vec;
use core::clone::Clone;
use core::cmp::Eq;
use core::cmp::PartialEq;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr::NonNull;

/// An owned C++ string. The pointer is guaranteed to be a non-null C++
/// allocated pointer to std::string.
///
/// We make sure that conversion between `string` and `std::string` is O(1), but
/// conversion between `string` and other types like `&str` or `String` can be
/// O(n).
// TODO: Make it mutable?.
// TODO: add a depedency on `crubit_annotate` to use that directly rather than doc attributes.
#[doc = "CRUBIT_ANNOTATE: cpp_type=std::string"]
#[doc = "CRUBIT_ANNOTATE: include_path=<string>"]
#[doc = "CRUBIT_ANNOTATE: cpp_to_rust_converter=cpp_string_to_rust_string"]
#[doc = "CRUBIT_ANNOTATE: rust_to_cpp_converter=rust_string_to_cpp_string"]
#[allow(non_snake_case)]
pub struct string {
    owned_cpp_string: NonNull<c_void>,
}

impl string {
    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }

    /// Returns a `*const c_void` pointing to the underlying C++
    /// `std::string` object.
    ///
    /// The caller must ensure that the `string` outlives the pointer.
    ///
    /// This method guarantees that for the purpose of the aliasing
    /// model, this method does not materialize a reference to the
    /// underlying data the string owns, and thus the returned
    /// pointer will remain valid when mixed with other calls to
    /// `as_void_ptr` and `as_mut_void_ptr`.
    pub fn as_void_ptr(&self) -> *const c_void {
        self.owned_cpp_string.as_ptr() as *const _
    }

    /// Returns a `*mut c_void` pointing to the underlying C++
    /// `std::string` object.
    ///
    /// The caller must ensure that the `string` outlives the pointer.
    ///
    /// This method guarantees that for the purpose of the aliasing
    /// model, this method does not materialize a reference to the
    /// underlying data the string owns, and thus the returned
    /// pointer will remain valid when mixed with other calls to
    /// `as_void_ptr` and `as_mut_void_ptr`.
    ///
    /// However, note that writing to the pointer will invalidate references to
    /// the underlying data. For example, the behavior is undefined if the
    /// underlying string is mutated through this pointer during the lifetime
    /// of a slice as returned by `as_slice`.
    pub fn as_mut_void_ptr(&mut self) -> *mut c_void {
        self.owned_cpp_string.as_ptr()
    }
}

impl PartialEq for string {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            // SAFETY: `owned_cpp_string` is guaranteed to be a non-null C++ allocated
            // pointer to std::string.
            conversion_function_helpers::StringEqual(
                self.owned_cpp_string.as_ptr(),
                other.owned_cpp_string.as_ptr(),
            )
        }
    }
}

impl Eq for string {}

impl Default for string {
    fn default() -> Self {
        "".into()
    }
}

impl Clone for string {
    fn clone(&self) -> Self {
        // SAFETY: `owned_cpp_string` is guaranteed to be a non-null C++ allocated
        // pointer to std::string.
        let raw_string = unsafe {
            conversion_function_helpers::StringCopyOwnedPtr(self.owned_cpp_string.as_ptr())
        };
        if let Some(ptr) = NonNull::new(raw_string) {
            Self { owned_cpp_string: ptr }
        } else {
            panic!("Failed to copy string");
        }
    }
}

impl Drop for string {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: `owned_cpp_string` is guaranteed to be a non-null C++ allocated
            // pointer to std::string.
            conversion_function_helpers::StringDelete(self.owned_cpp_string.as_ptr());
        }
    }
}

impl From<String> for string {
    fn from(s: String) -> Self {
        s.as_bytes().into()
    }
}

impl From<&String> for string {
    fn from(s: &String) -> Self {
        s.as_bytes().into()
    }
}

impl From<&Vec<u8>> for string {
    fn from(s: &Vec<u8>) -> Self {
        s.as_slice().into()
    }
}

impl From<&str> for string {
    fn from(s: &str) -> Self {
        s.as_bytes().into()
    }
}

impl From<&[u8]> for string {
    fn from(s: &[u8]) -> Self {
        // SAFETY: Rust slice returns a valid pointer to a buffer of bytes.
        let raw_string = unsafe {
            conversion_function_helpers::StringCreateFromBuffer(s.as_ptr() as _, s.len())
        };
        if let Some(ptr) = NonNull::new(raw_string) {
            Self { owned_cpp_string: ptr }
        } else {
            panic!("Failed to create string");
        }
    }
}

impl Deref for string {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        let ptr = self.owned_cpp_string.as_ptr();
        // SAFETY:
        //
        // * `owned_cpp_string` is guaranteed to be a non-null C++ allocated pointer to
        //   std::string so `ptr` is non-null.
        // * `StringGetData` returns the pointer of the C++ `std::string::data()`, which
        //   is guaranteed to be non-null and point to a continuous memory region. Every
        //   byte in [ptr, ptr + len)) is intialized. (See https://en.cppreference.com/w/cpp/string/basic_string/data)
        // * The data is guaranteed to be not mutated because we don't ever mutate
        //   data() except when accessed via &mut self, which is blocked by Rust borrow
        //   checker.
        // * `len` is guaranteed to be less than `isize::MAX` because C++
        //   implementations guarantee in practice that the object won't go past the end
        //   of the address space.
        unsafe {
            let len = conversion_function_helpers::StringGetSize(ptr);
            core::slice::from_raw_parts(conversion_function_helpers::StringGetData(ptr) as _, len)
        }
    }
}

impl core::convert::AsRef<[u8]> for string {
    fn as_ref(&self) -> &[u8] {
        &*self
    }
}

impl core::fmt::Debug for string {
    // TODO(b/351976622): Make a pretty Debug like std::string(b"\xffhello\xde") or
    // similar.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "cc_std::string({:?})", self.as_slice())
    }
}

impl core::fmt::Display for string {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        String::from_utf8_lossy(self.as_slice()).fmt(f)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_string_to_cpp_string(input: *const c_void, output: *mut c_void) {
    // SAFETY:
    // * `input` is a valid `string`.
    // * `input.owned_cpp_string` is guaranteed to be a non-null C++ allocated
    //   pointer to std::string.
    // * `output` is a valid C++ `std::string`.
    unsafe {
        let input = &*(input as *const string);
        conversion_function_helpers::StringCreateInPlace(output, input.owned_cpp_string.as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpp_string_to_rust_string(input: *mut c_void, output: *mut c_void) {
    // SAFETY: `input` is a valid `std::string` so it can be safely moved.
    let owned_cpp_string = unsafe { conversion_function_helpers::StringMoveOwnedPtr(input) };
    if let Some(ptr) = NonNull::new(owned_cpp_string) {
        let output = &mut *(output as *mut MaybeUninit<string>);
        output.as_mut_ptr().write(string { owned_cpp_string: ptr });
    } else {
        panic!("Failed to create owned string");
    }
}
