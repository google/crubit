// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::pin::Pin;
use cref::{CMut, CRef};
use example_lib::example::{AddOne, Counter, Number, Smaller, SmallerWithLifetimes};

fn main() {
    // C++ methods take `&self` or `&mut self`.
    let mut counter = Counter { value: 1 };
    counter.Increment();
    assert_eq!(counter.Get(), 2);

    // C++ references returned by C++ functions become `CRef` or `CMut`.
    // SAFETY: Nothing mutates `counter.value` while `value` is in use.
    let value: Pin<&i32> = unsafe { CRef::unchanging(counter.GetRef()) };
    assert_eq!(*value, 2);

    // SAFETY: Nothing else reads or writes `counter.value` while `value` is in use.
    let value: &mut i32 = unsafe { CMut::unpin_unique(counter.GetMutRef()) };
    *value = 3;
    assert_eq!(counter.Get(), 3);

    // C++ reference parameters become Rust references.
    let mut x = 1;
    AddOne(&mut x);
    assert_eq!(x, 2);

    // If lifetime elision doesn't apply, then the result is a raw pointer.
    let (x, y) = (1, 2);
    let smaller: *const i32 = Smaller(&x, &y);
    // SAFETY: `x` and `y` are still alive.
    assert_eq!(unsafe { *smaller }, 1);

    // Explicit lifetime annotations tie the result to both `x` and `y`.
    let smaller = SmallerWithLifetimes(&x, &y);
    assert_eq!(CRef::as_ptr(smaller), &raw const x);

    let number = Number { value: 1 };
    let other = 2;
    let larger = number.GetLargerWithLifetimes(&other);
    assert_eq!(CRef::as_ptr(larger), &raw const other);
}
