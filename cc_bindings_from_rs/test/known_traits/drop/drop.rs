// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `drop_test.cc`.

pub mod counters {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CLONE_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static CLONE_FROM_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static DEFAULT_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    pub fn reset_counts() {
        CLONE_COUNTER.store(0, Ordering::Relaxed);
        CLONE_FROM_COUNTER.store(0, Ordering::Relaxed);
        DEFAULT_COUNTER.store(0, Ordering::Relaxed);
        DROP_COUNTER.store(0, Ordering::Relaxed);
    }

    pub fn get_clone_count() -> usize {
        CLONE_COUNTER.load(Ordering::Relaxed)
    }

    pub fn get_clone_from_count() -> usize {
        CLONE_FROM_COUNTER.load(Ordering::Relaxed)
    }

    pub fn get_default_count() -> usize {
        DEFAULT_COUNTER.load(Ordering::Relaxed)
    }

    pub fn get_drop_count() -> usize {
        DROP_COUNTER.load(Ordering::Relaxed)
    }

    pub fn increment_clone_count() {
        CLONE_COUNTER.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_clone_from_count() {
        CLONE_FROM_COUNTER.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_default_count() {
        DEFAULT_COUNTER.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_drop_count() {
        DROP_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}

/// Test for `Drop` support when:
/// * `Drop` is provided by `impl` (rather than requiring "drop glue" for
///   fields).
/// * The type implements the `Default` trait.
/// * The type cannot be copied via `Clone` (although this aspect is not
///   load-bearing in this test.)
pub mod drop_impl_with_default {
    pub struct DropImplWithDefault {
        pub field: i32,
    }

    impl Default for DropImplWithDefault {
        fn default() -> Self {
            super::counters::increment_default_count();
            Self { field: 0 }
        }
    }

    impl Drop for DropImplWithDefault {
        fn drop(&mut self) {
            super::counters::increment_drop_count();
        }
    }

    impl DropImplWithDefault {
        pub fn get_int(&self) -> i32 {
            self.field
        }

        pub fn set_int(&mut self, i: i32) {
            self.field = i;
        }
    }
}

/// Test for `Drop` support when:
/// * There is no `Drop` `impl`, but the type under test requires "drop glue"
///   for its fields (i.e. the field implements `Drop`, but the type containing
///   the field doesn't).
/// * The type implements the `Default` trait.
/// * The type cannot be copied via `Clone` (although this aspect is not
///   load-bearing in this test.)
pub mod drop_glue_with_default {
    use super::drop_impl_with_default::DropImplWithDefault;

    #[derive(Default)]
    pub struct DropGlueWithDefault {
        pub field: DropImplWithDefault,
    }

    impl DropGlueWithDefault {
        pub fn get_int(&self) -> i32 {
            self.field.get_int()
        }

        pub fn set_int(&mut self, i: i32) {
            self.field.set_int(i)
        }
    }
}

/// Test for `Drop` support when:
/// * `Drop` is provided by `impl` (rather than requiring "drop glue" for
///   fields).
/// * The type does *not* implement the `Default` trait.
/// * The type can be copied via `Clone`
pub mod drop_impl_with_clone {
    pub struct DropImplWithClone {
        pub field: i32,
    }

    impl Clone for DropImplWithClone {
        fn clone(&self) -> Self {
            super::counters::increment_clone_count();
            Self { field: self.field }
        }

        /// We implement `clone_from` to avoid depending on the implementation
        /// details of the default implementation (which for example ends up
        /// dropping a temporary value).
        fn clone_from(&mut self, source: &Self) {
            super::counters::increment_clone_from_count();
            self.field = source.field
        }
    }

    impl Drop for DropImplWithClone {
        fn drop(&mut self) {
            super::counters::increment_drop_count();
        }
    }

    impl DropImplWithClone {
        pub fn create_from_int(i: i32) -> Self {
            Self { field: i }
        }

        pub fn get_int(&self) -> i32 {
            self.field
        }

        pub fn set_int(&mut self, i: i32) {
            self.field = i;
        }
    }
}

/// Test for `Drop` support when:
/// * `Drop` is provided by `impl` (rather than requiring "drop glue" for
///   fields).
/// * The type does *not* implement the `Default` trait.
/// * The type cannot be copied via `Clone`
pub mod drop_impl_with_nothing_else {
    pub struct DropImplWithNothingElse {
        pub field: i32,
    }

    impl Drop for DropImplWithNothingElse {
        fn drop(&mut self) {
            super::counters::increment_drop_count();
        }
    }

    impl DropImplWithNothingElse {
        pub fn get_int(&self) -> i32 {
            self.field
        }
    }

    /// FFI bindings for `DropImplWithNothingElse` have no constructors (the
    /// move constructor is explicitly `=delete`d, because the Rust
    /// type doesn't provide any APIs that could
    /// leave the moved-away object in a well-formed, safe-to-destruct
    /// state).  This also means that its currently not possible to return
    /// `DropImplWithNothingElse` by value from Rust (e.g. from factory
    /// methods), because `ReturnValueSlot` uses `std::move` internally.
    ///
    /// To work around the constraints above, we construct the value on Rust
    /// side, and expose it to C++ in a "wrapped" form.  This means that tests
    /// don't need to construct or move `DropImplWithNothingElse` on C++
    /// side.
    pub struct WrappedDropImplWithNothingElse {
        pub field: DropImplWithNothingElse,
    }

    impl Default for WrappedDropImplWithNothingElse {
        fn default() -> Self {
            Self { field: DropImplWithNothingElse { field: 123 } }
        }
    }
}
