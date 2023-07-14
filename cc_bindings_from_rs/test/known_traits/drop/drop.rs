// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `drop_test.cc`.

pub mod counters {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DEFAULT_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    pub fn reset_counts() {
        DEFAULT_COUNTER.store(0, Ordering::Relaxed);
        DROP_COUNTER.store(0, Ordering::Relaxed);
    }

    pub fn get_default_count() -> usize {
        DEFAULT_COUNTER.load(Ordering::Relaxed)
    }

    pub fn get_drop_count() -> usize {
        DROP_COUNTER.load(Ordering::Relaxed)
    }

    pub fn increment_default_count() {
        DEFAULT_COUNTER.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_drop_count() {
        DROP_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}

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

/// `DropGlueWithDefault` doesn't directly implement `Drop`, but its field does.
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
