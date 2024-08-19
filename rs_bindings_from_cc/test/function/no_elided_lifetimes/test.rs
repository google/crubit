// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use no_elided_lifetimes::*;

    #[gtest]
    fn test_store_pointer() {
        let mut boxed_int = Box::new(123);

        // Without (human-, or machine-verified) lifetime annotations, passing a
        // pointer (or reference) across FFI boundary is unsafe. The call to
        // `StorePointer` should *not* be possible without an `unsafe` block.
        //
        // Note that if `StorePointer` function was *not* marked as `unsafe`,
        // then Rust Clippy would warn about the code below having an
        // "unnecessary `unsafe` block". Seeing such Clippy warning would
        // indicate a regression.
        unsafe {
            StorePointer(boxed_int.as_mut());
        }
        assert_eq!(123, ReadStoredPointer());

        *boxed_int.as_mut() = 456;
        assert_eq!(456, ReadStoredPointer());

        // The commented-out `ReadStoredPointer()` would dereference a dangling
        // pointer, resulting in Undefined Behavior (UB). In normal builds, UB
        // might result in `ReadStoredPointer()` returning 0. In ASan builds,
        // the UB would be caught and reported as an error.
        drop(boxed_int);
        //assert_eq!(456, ReadStoredPointer());
    }
}
