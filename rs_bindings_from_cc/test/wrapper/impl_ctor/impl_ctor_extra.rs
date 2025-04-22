// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate alloc;
use ::ctor::Emplace;
use alloc::boxed::Box;
use core::pin::Pin;

/// Wraps `Create()` with a `Box`-based API.
pub fn create() -> Pin<Box<crate::Nontrivial>> {
    Box::emplace(crate::Create())
}

// TODO(b/411467353): don't support move constructors yet, so can't do this:
// use ::ctor::{CtorNew, mov};
// /// Wraps `Read()` with a `Box`-based API.
// pub fn read(x: Pin<Box<crate::Nontrivial>>) -> core::ffi::c_int {
//   // The ctor API makes everything more explicit than C++ or Rust:
//   // 1. we must explicitly reborrow the pointee, using .as_mut()
//   // 2. we must cast to an rvalue reference, using `mov!()`
//   // 3. we must call the constructor, using `CtorNew`.
//   //
//   // (If we were to materialize the temporary, we'd need a call to
//   // `emplace!()` or Box::emplace, too!)
//   crate::Read(crate::Nontrivial::ctor_new(mov!(x.as_mut())))
// }
