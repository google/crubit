// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! # One to one FFI.
//!
//! In `ffi_11`, if a type is distinct in C/C++, it is distinct in Rust. This
//! relationship varies from platform to platform.
//!
//! For example, `char` and `signed char` are distinct types in C++. This means
//! that in `ffi_11`, there is a distinct `ffi_11::c_char` type which is not the
//! same as `ffi::c_schar`, even if `char` is signed. This is unlike the
//! `std::ffi` module, which would instead define both `c_char` and `c_schar`
//! as aliases to `i8`.
//!
//! As another example, on some platforms, `int64_t` is `long`, while on other
//! platforms, it is `long long`. Exactly one of `ffi_11::c_long` or
//! `ffi_11::c_longlong` will be `i64`, depending on the platform.
//!
//! ## Guarantees and backwards compatibility
//!
//! `ffi_11` offers the following guarantees:
//!
//! * Every unique C/C++ type is given a unique Rust type: the Rust->C/C++ type
//!   mapping is one-to-one.
//! * A given type `c_<X>` is the same type as a builtin `iN` or `uN` type if,
//!   and only if, the corresponding C++ type is the same type as the standard
//!   library `(u)intN_t` on this platform.
//!
//!   For example, `c_int` is `i32` if `int32_t` is a type alias to `int`.
//!   Otherwise, `c_int` will be a different type. (Either a newtype, or a
//!   non-`i32` primitive 32 bit integer type.)
//!
//! There are no type identity guarantees other than the above. For example,
//! `long long` may be a `i64`,`isize`, or a newtype, depending on the platform.
//!
//! There is also no guarantee that every Rust primitive type has a C++
//! equivalent. The 1:1 relationship only applies to the types in the `ffi_11`
//! module. To get the equivalent high-fidelity interop in C++, you would need
//! an equivalent `<ffi_11.h>` header, defined in a similar fashion and using
//! newtypes to define C++ types that correspond to Rust primitives that
//! otherwise have no C++ equivalent. (For example, the typical Windows ABI
//! only has one 64 bit integer type, while Rust has two.)
//!
//! ## Supported Operations
//!
//! The following operations are supported:
//!
//! * `From`: any `ffi_11` type can be converted to or from a builtin or
//!   `ffi_11` type if the conversion is lossless. For example, `c_int` can
//!   always be converted to `c_long`, but not to `c_ulong`. And `i32` can
//!   always be converted to `c_int`, but `i64` can only on some platforms.
//!
//! * Separately from the above, `c_char` can be converted to and from both
//!   `i8` and `u8` using `From` and `Into`. It is considered an
//!   ambiguously-signed type for portability.
//!
//! ## Supported platforms
//!
//! For now, the only supported platforms are:
//!
//! * LP64: Any LP64 platform which uses the smallest suitable fundamental type
//!   for `intN_t`. For example, Linux on x86_64 or Aarch64. But not OpenBSD.
//! * LLP64: 64-bit Windows.
//!
//! We will add support over time to other commonly used platforms.
//!
//! TODO(b/333759161): get and test a compatibility matrix, including
//! (currently untested) Windows.
//!
//! ## Unfinished Work
//!
//! This module is still embryonic, and is missing the following:
//!
//! * Support for `long long` on Linux. This depends on a decision about what
//!   the type should be. For example, it could be `isize`, or a newtype.
//!
//! * `TryFrom` impls for lossy conversions.
//!
//! * Any/all other operations (e.g. arithmetic) one might want to support on newtypes.
//!
//! ## References
//!
//! * Discussion: ["Rust / C++ interop, and type collisions" in #t-lang/interop](https://rust-lang.zulipchat.com/#narrow/channel/427678-t-lang.2Finterop/topic/Rust.20.2F.20C.2B.2B.20interop.2C.20and.20type.20collisions)

#![no_std]
#![allow(nonstandard_style)]
extern crate core;
mod newtype;

use newtype::{new_integer, wrapped_to_wrapped};

pub use core::ffi::c_void;

// ===============================
// The classic C fundamental types
// ===============================
//
// Implementation note: Clang picks the _smallest_ available fundamental type to
// be intN_t, with the exception of int16_t on e.g. AVR (int, instead of short),
// and int64_t on e.g. OpenBSD (long long, instead of long). This makes it
// relatively straightforward to maintain guarantee #2 of mapping the
// fundamental types backwards to the correct fixed-size type. We just need to
// know the sizes and whether or not we're on one of the unusual platforms.

pub type c_float = f32;
pub type c_double = f64;

// TODO(jeanpierreda): If you use cpp_type="char", Crubit will try to escape the type name, currently.
// To work around this, we can use decltype('a')) or similar.
//
// Implementation note: we can use `u8` instead of `i8` on the assumption that signedness doesn't
// change the ABI. The actual platform sign is hidden from users, though they can convert to
// `core::ffi::c_char` if they need it.
new_integer! {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=decltype(char(0))")]
    pub struct c_char(u8);
}

impl c_char {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
}

mod private {
    use super::*;

    pub trait Sealed {}
    impl Sealed for c_char {}
    impl Sealed for core::ffi::c_char {}
    impl Sealed for core::ffi::CStr {}
    impl Sealed for *const c_char {}
    impl Sealed for *const core::ffi::c_char {}
}

use private::Sealed;

/// Mirrors behavior of `as_ptr` on various APIs in `std::ffi` but producing their `ffi_11` pointer
/// types, rather than the `std::ffi` pointer types. Implementations of this trait document
/// conversions that are well behaved for the given type.
pub trait AsFfi11Ptr: Sealed {
    type Ptr;
    fn as_ffi_11_ptr(&self) -> Self::Ptr;
}

/// Extension of `CStr` to provide conversion methods for `ffi_11` pointer types.
pub trait CStrExt: Sealed {
    /// # Safety
    ///
    /// * The memory pointed to by `ptr` must contain a valid nul terminator at the
    ///   end of the string.
    ///
    /// * `ptr` must be [valid] for reads of bytes up to and including the nul terminator.
    ///   This means in particular:
    ///
    ///     * The entire memory range of this `CStr` must be contained within a single allocation!
    ///     * `ptr` must be non-null even for a zero-length cstr.
    ///
    /// * The memory referenced by the returned `CStr` must not be mutated for
    ///   the duration of lifetime `'a`.
    ///
    /// * The nul terminator must be within `isize::MAX` from `ptr`
    unsafe fn from_ffi_11_ptr<'a>(ptr: *const c_char) -> &'a Self;
}

impl AsFfi11Ptr for core::ffi::CStr {
    type Ptr = *const c_char;
    fn as_ffi_11_ptr(&self) -> Self::Ptr {
        self.as_ptr().cast()
    }
}

impl CStrExt for core::ffi::CStr {
    unsafe fn from_ffi_11_ptr<'a>(ptr: *const c_char) -> &'a Self {
        unsafe { core::ffi::CStr::from_ptr(ptr.cast()) }
    }
}

/// Casts between `ffi_11` pointers and their `std::ffi` equivalents. Equivalent to a `pointer::cast`
/// call, but the trait implementation documents that this is an intended and well-behaved cast.
pub trait CastFfi11: Sealed {
    type Target;
    fn cast_ffi_11(self) -> Self::Target;
}
impl CastFfi11 for *const c_char {
    type Target = *const core::ffi::c_char;
    fn cast_ffi_11(self) -> Self::Target {
        self.cast()
    }
}

impl CastFfi11 for *const core::ffi::c_char {
    type Target = *const c_char;
    fn cast_ffi_11(self) -> Self::Target {
        self.cast()
    }
}

// Unlike the other new_integer! types, char converts to/from any type with the same bit width.

impl From<c_char> for i8 {
    fn from(c: c_char) -> i8 {
        c.0 as i8
    }
}

impl From<i8> for c_char {
    fn from(c: i8) -> c_char {
        c_char(c as u8)
    }
}

// Attributes on the aliases so that, opportunistically, we use e.g. `signed
// char` instead of `std::int8_t`, though they are the same thing. It's a
// spelling convenience, and optional for all aliases.

// TODO(jeanpierreda): These crubit annotate calls are currently no-ops.
//
// This doesn't result in _incorrect_ bindings -- actually, it would break, the same as `char`,
// if it worked. But the results are less readable than if we directly used the correct
// type: `signed char` instead of `std::int8_t`, `unsigned char` instead of `std::uint8_t`, etc.

#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=signed char")]
pub type c_schar = i8;
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned char")]
pub type c_uchar = u8;

#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=short")]
pub type c_short = i16;
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned short")]
pub type c_ushort = u16;

#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=int")]
pub type c_int = i32;
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned int")]
pub type c_uint = u32;

/// LP64 with long int64_t.
#[cfg(all(target_pointer_width = "64", not(windows), not(target_os = "openbsd")))]
mod long_integers {
    use super::*;
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=long")]
    pub type c_long = i64;
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned long")]
    pub type c_ulong = u64;

    new_integer! {
      #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=long long")]
      pub struct c_longlong(i64);
      #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned long long")]
      pub struct c_ulonglong(u64);
    }
}

// TODO(b/333759161): This is the mirror image of the above.
//
// /// LP64 with long long int64_t
// ///
// /// TODO(b/333759161): List out the full list of LP64 platforms which use long
// /// long here.
// #[cfg(all(target_pointer_width = "64", any(target_os = "openbsd")))]
// mod long_integers {
//     pub type c_long = isize;
//     pub type c_ulong = usize;

//     pub type c_longlong = i64;
//     pub type c_ulonglong = u64;
// }

/// LLP64 (Windows)
#[cfg(all(target_pointer_width = "64", windows))]
mod long_integers {
    use super::*;
    new_integer! {
      #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=long")]
      pub struct c_long(i32);
      #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned long")]
      pub struct c_ulong(u32);
    }

    wrapped_to_wrapped! {
        impl From<c_char> for c_long;
        impl From<c_char> for c_ulong;

        impl From<c_ulong> for c_char32_t;
    }

    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=long long")]
    pub type c_longlong = i64;
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned long long")]
    pub type c_ulonglong = u64;
}

#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=decltype(long(0))")]
pub type c_long = long_integers::c_long;
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned long")]
pub type c_ulong = long_integers::c_ulong;

#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=long long")]
pub type c_longlong = long_integers::c_longlong;

#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=unsigned long long")]
pub type c_ulonglong = long_integers::c_ulonglong;

// ====================================
// Newtypes for other fundamental types
// ====================================

// NOTE: We could also force inclusion of `stddef.h` and use `nullptr_t`.
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=decltype(nullptr)")]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct c_nullptr_t(*mut c_void);

impl Default for c_nullptr_t {
    fn default() -> Self {
        Self(core::ptr::null_mut())
    }
}

// SAFETY: nullptr_t can only have value 0 / is not mutable.
unsafe impl Send for c_nullptr_t {}
unsafe impl Sync for c_nullptr_t {}

// The C++ charN_t types
new_integer! {
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=decltype(char8_t(0))")]
    pub struct c_char8_t(u8);
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=decltype(char16_t(0))")]
    pub struct c_char16_t(u16);
    #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=decltype(char32_t(0))")]
    pub struct c_char32_t(u32);
}

wrapped_to_wrapped! {
    impl From<c_char> for c_char8_t;
    impl From<c_char> for c_char16_t;
    impl From<c_char> for c_char32_t;

    impl From<c_char8_t> for c_char16_t;
    impl From<c_char8_t> for c_char32_t;
    impl From<c_char16_t> for c_char32_t;
}

#[cfg(not(windows))]
mod wchar_type {
    use super::*;
    new_integer! {
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=decltype(wchar_t(0))")]
        pub struct c_wchar_t(u32);
    }

    wrapped_to_wrapped! {
      impl From<c_char> for c_wchar_t;
      impl From<c_char8_t> for c_wchar_t;
      impl From<c_char16_t> for c_wchar_t;
      impl From<c_char32_t> for c_wchar_t;

      impl From<c_wchar_t> for c_char32_t;
    }
}

#[cfg(windows)]
mod wchar_type {
    use super::*;
    new_integer! {
        #[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=decltype(wchar_t(0))")]
        pub struct c_wchar_t(u16);
    }

    wrapped_to_wrapped! {
      impl From<c_char> for c_wchar_t;
      impl From<c_char8_t> for c_wchar_t;
      impl From<c_char16_t> for c_wchar_t;

      impl From<c_wchar_t> for c_char16_t;
      impl From<c_wchar_t> for c_char32_t;
    }
}

#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=wchar_t")]
pub type c_wchar_t = wchar_type::c_wchar_t;
