// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::slice_ptr::get_raw_parts;
use crate::std::raw_string_view;
use core::ptr;

/// Live type for std::string_view bindings.
///
/// This is a raw_string_view wrapper with an associated lifetime.
///
/// # Invariants
///
/// * The contained raw_string_view is valid for lifetime `'a`.
/// * This does **not** make any guarantees about mutable aliasing.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[doc = "CRUBIT_ANNOTATE: cpp_type=std::string_view"]
#[doc = "CRUBIT_ANNOTATE: include_path=<string_view>"]
#[derive(Debug)]
pub struct string_view<'a> {
    raw: raw_string_view,
    phantom_data: core::marker::PhantomData<&'a ()>,
}

impl<'a> string_view<'a> {
    pub fn as_raw(&self) -> &raw_string_view {
        &self.raw
    }

    pub fn into_raw(self) -> raw_string_view {
        self.raw
    }

    pub fn as_ptr(&self) -> *const raw_string_view {
        &self.raw
    }

    ///  Returns a Rust byte slice referring to the string_view's data.
    ///
    /// # Safety
    ///
    /// The viewed memory must NOT be mutated by any C++ code during `'a`.
    /// The returned `&'a [u8]` requires this immutability. While C++ `std::string_view`
    /// itself provides read-only access, the C++ code owning the viewed data must
    /// not modify it via other aliases for the duration of `'a`.
    pub unsafe fn as_bytes(&self) -> &'a [u8] {
        // SAFETY (internal dereference): The method's SAFETY contract (see above) ensures
        // `self.raw` points to memory that is valid and immutable for lifetime `'a`.
        // `self.raw.as_raw_bytes()` provides a `*const [u8]`, correctly handling
        // empty/null cases for dereferencing to an empty slice. Thus, `&*` is safe.
        &*self.raw.as_raw_bytes()
    }

    pub fn len(&self) -> usize {
        // SAFETY: `&self.raw` is valid according to the `string_view` invariants.
        unsafe { self.raw.len() }
    }

    /// Perform UTF-8 validation and returns a &str view of the underlying C++ string if validation
    /// succeeds. Returns an Utf8Error otherwise.
    ///
    /// # Safety
    ///
    /// The viewed memory must NOT be mutated by any C++ code during lifetime `'a`.
    /// The returned `&'a str` (on `Ok`) requires this immutability. While C++
    /// `std::string_view` itself provides read-only access, the C++ code owning the
    /// viewed data must not modify it via other aliases for the duration of `'a`.
    pub unsafe fn to_str(&self) -> Result<&'a str, core::str::Utf8Error> {
        // SAFETY (internal dereference in map):
        // The method's main SAFETY contract (see above) ensures `self.raw` points to
        // memory that is valid and immutable for `'a`. `self.raw.to_str()` (itself unsafe)
        // attempts UTF-8 conversion, yielding a `*const str` if valid. Dereferencing
        // this pointer via `&*s` is safe given the outer contract and successful UTF-8 check.
        self.raw.to_str().map(|s| &*s)
    }
}

impl<'a> From<&'a [u8]> for string_view<'a> {
    fn from(s: &[u8]) -> Self {
        string_view {
            raw: raw_string_view::from(s as *const [u8]),
            phantom_data: core::marker::PhantomData,
        }
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for string_view<'a> {
    fn from(s: &[u8; N]) -> Self {
        string_view {
            raw: raw_string_view::from(s as *const [u8]),
            phantom_data: core::marker::PhantomData,
        }
    }
}

impl<'a> From<&'a str> for string_view<'a> {
    fn from(s: &str) -> Self {
        string_view {
            raw: raw_string_view::from(s.as_bytes()),
            phantom_data: core::marker::PhantomData,
        }
    }
}

impl<'a> From<&'a core::ffi::CStr> for string_view<'a> {
    fn from(cstr: &core::ffi::CStr) -> Self {
        string_view {
            raw: raw_string_view::from(cstr.to_bytes()),
            phantom_data: core::marker::PhantomData,
        }
    }
}

impl raw_string_view {
    /// Returns an equivalent Rust slice pointer.
    ///
    /// The resulting slice pointer is valid for the lifetime of the pointed-to
    /// object.
    ///
    /// Note: For empty strings, the address of the slice pointer may not be the
    /// same as the address of the raw_string_view. Null pointers are converted
    /// to valid, but dangling, pointers.
    #[inline(always)]
    pub fn as_raw_bytes(self) -> *const [u8] {
        self.into()
    }

    /// Converts a `raw_string_view` containing valid UTF-8 to a `*const str`.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the `raw_string_view` has an invalid pointer.
    pub unsafe fn to_str(&self) -> Result<*const str, core::str::Utf8Error> {
        let bytes: &[u8] = unsafe { &*self.as_raw_bytes() };
        let res: *const str = core::str::from_utf8(bytes)?;
        Ok(res)
    }

    /// Returns the length of the underlying string.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if the `raw_string_view` has an invalid pointer.
    pub unsafe fn len(&self) -> usize {
        // TODO(b/249376862): use size(), which does not have the soundness issue below.
        // let size = unsafe {raw_string_view::size(&sv)};
        //
        // SAFETY: the call to end() requires that the raw_string_view not be dangling,
        // so this is unsound. (If `self` is dangling, then `end()` will perform pointer
        // arithmetic on a dangling pointer, which is implementation defined (and treated on
        // Clang as if it were UB).)
        (unsafe { raw_string_view::end(self) }) as usize
            - unsafe { raw_string_view::begin(self) } as usize
    }

    /// Get string_view with lifetime linked to self.
    ///
    /// # Safety
    ///
    /// The data referred to to by `self` must be valid for at least as long as `'s`.
    /// For example, it is not sound to call `as_live<'static>` on a `string_view`
    /// bound to a temporary, because the `string_view<'static>` would
    /// outlive the temporary.
    pub unsafe fn as_live<'s>(&'s self) -> string_view<'s> {
        string_view { raw: *self, phantom_data: core::marker::PhantomData }
    }

    /// Get a string_view with static lifetime.
    ///
    /// # Safety
    ///
    /// The data referred to to by `self` must be valid for the life of the program.
    /// For example, it is not sound to call `as_static_live()` on a `string_view`
    /// bound to a temporary, because the `string_view<'static>` would
    /// outlive the temporary.
    pub unsafe fn as_static_live(&'static self) -> string_view<'static> {
        self.as_live()
    }
}

/// Equivalent to `as_raw_bytes()`.
impl From<raw_string_view> for *const [u8] {
    fn from(sv: raw_string_view) -> Self {
        // SAFETY: `&sv` is a valid pointer. `data()` does not dereference the
        // raw_string_view.
        let mut data = unsafe { raw_string_view::data(&sv) } as *const u8;
        // SAFETY: this is unsound for the reason explained in raw_string_view::len().
        let size = unsafe { sv.len() };
        // Unlike C++, Rust does not allow for null data pointers in slices.
        if data.is_null() {
            data = ptr::NonNull::dangling().as_ptr();
            debug_assert_eq!(size, 0);
        }
        ptr::slice_from_raw_parts(data, size)
    }
}
impl From<&[u8]> for raw_string_view {
    fn from(s: &[u8]) -> Self {
        raw_string_view::from(s as *const [u8])
    }
}

impl<const N: usize> From<&[u8; N]> for raw_string_view {
    fn from(s: &[u8; N]) -> Self {
        raw_string_view::from(s as *const [u8])
    }
}

impl<const N: usize> From<*const [u8; N]> for raw_string_view {
    fn from(s: *const [u8; N]) -> Self {
        raw_string_view::from(s as *const [u8])
    }
}

impl From<&str> for raw_string_view {
    fn from(s: &str) -> Self {
        raw_string_view::from(s.as_bytes())
    }
}

impl From<&core::ffi::CStr> for raw_string_view {
    fn from(cstr: &core::ffi::CStr) -> Self {
        raw_string_view::from(cstr.to_bytes())
    }
}

impl From<*const [u8]> for raw_string_view {
    fn from(slice: *const [u8]) -> Self {
        // TODO(jeanpierreda): We can't access the constructors at the moment.
        // This little maneuver's gonna cost us 51 years of annoying build breakages
        // later, so really we should try to get the constructors callable.

        // SAFETY: raw_string_view (in Rust) is a `repr(C)` struct entirely composed of
        // arrays of MaybeUninit<u8>, so this would be safe even without the
        // zeroed(). With the zeroed, it's also safe even if we accidentally use
        // the type without further initialization. (In C++, the fields are a
        // pointer and an integer).
        let mut sv = unsafe { <core::mem::MaybeUninit<raw_string_view>>::zeroed().assume_init() };
        // We could also use the (unstable) to_raw_parts, but that feature may change
        // over time. It's also difficult, for idiosyncratic reasons, to pipe in
        // the feature flag to the automatically generated bindings for
        // `raw_string_view` that this file attaches onto, or to inject a dependency
        // on a crate to put this logic into. (The crate this file is a part of is
        // automatically generated by Crubit, and so we would need to tell
        // Crubit to add these to the generated bindings for `std`.) So for now,
        // the most expedient thing, and the thing least likely to break in a
        // future version of Rust, to roll our own version of to_raw_parts, which uses
        // transmute under the hood.
        let (ptr, size) = get_raw_parts::<u8>(slice);
        // SAFETY: there is no immediate UB, because we are converting to an array of
        // MaybeUninit. There is also no later UB when read by C++, where it is
        // interpreted as a pointer: any non-dangling valid Rust pointer is also
        // a valid C++ pointer. The only time a dangling pointer exists is if
        // size == 0, in which case we replace it with null, which is
        // also a valid C++ pointer.
        sv.__data_ = unsafe { core::mem::transmute(if size == 0 { 0 as *const _ } else { ptr }) };
        // SAFETY: there is no immediate UB, because we are converting to an array of
        // MaybeUninit. There is also no later UB when read by C++, where it is
        // interpreted as a `size_t`, which has the same set of object
        // representations as `size : usize`.
        sv.__size_ = unsafe { core::mem::transmute(size) };
        sv
    }
}

impl core::fmt::Debug for raw_string_view {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "<raw_string_view>")
    }
}
