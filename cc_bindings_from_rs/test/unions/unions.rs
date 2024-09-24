// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `unions_test.cc`.

/// Test for a `#[repr(C)] union
pub mod repr_c {
    #[repr(C)]
    pub union U {
        pub x: u32,
        pub y: u32,
    }

    pub fn create() -> U {
        U { x: 0 }
    }
}

/// Test for a union with default rust representation
pub mod repr_rust {
    pub union U {
        pub x: u32,
        pub y: u32,
    }

    pub fn create() -> U {
        U { x: 0 }
    }

    impl U {
        pub fn set_x(&mut self, x: u32) {
            self.x = x;
        }
        pub unsafe fn get_x(&self) -> u32 {
            unsafe { self.x }
        }
        pub fn set_y(&mut self, y: u32) {
            self.y = y;
        }
        pub unsafe fn get_y(&self) -> u32 {
            unsafe { self.y }
        }
    }
}

/// Test for a `#[repr(C, packed)]` union
pub mod repr_c_packed {
    #[repr(C, packed)]
    pub union U {
        pub x: u32,
        pub y: u32,
    }

    pub fn create() -> U {
        U { x: 0 }
    }
}

/// Test for a union with a `#[repr(packed)]` union
pub mod repr_rust_packed {
    #[repr(packed)]
    pub union U {
        pub x: u32,
        pub y: u32,
    }

    pub fn create() -> U {
        U { x: 0 }
    }
}

/// Test for a `#[repr(C)]` union with a `Clone` implementation
pub mod repr_c_clone {
    #[repr(C)]
    pub union U {
        pub x: u32,
    }

    impl Clone for U {
        fn clone(&self) -> U {
            U { x: unsafe { self.x } }
        }
    }

    pub fn create() -> U {
        U { x: 0 }
    }
}

/// Test for a union with a `Clone` implementation
pub mod repr_rust_clone {
    pub union U {
        pub x: u32,
    }

    impl Clone for U {
        fn clone(&self) -> U {
            U { x: unsafe { self.x } }
        }
    }

    pub fn create() -> U {
        U { x: 0 }
    }

    impl U {
        pub fn set_x(&mut self, x: u32) {
            self.x = x;
        }
        pub unsafe fn get_x(&self) -> u32 {
            unsafe { self.x }
        }
    }
}

/// Test for a `#[repr(C)]` union with a `Drop` implementation
pub mod repr_c_drop {

    #[repr(C)]
    pub union U {
        pub x: *mut i32,
    }

    impl Default for U {
        fn default() -> U {
            U { x: std::ptr::null_mut() }
        }
    }

    impl Drop for U {
        fn drop(&mut self) {
            unsafe { *self.x += 1 }
        }
    }
}

/// Test for a union with a `Drop` implementation
pub mod repr_rust_drop {
    pub union U {
        pub x: *mut i32,
    }

    impl Default for U {
        fn default() -> U {
            U { x: std::ptr::null_mut() }
        }
    }

    impl Drop for U {
        fn drop(&mut self) {
            unsafe { *self.x += 1 }
        }
    }

    impl U {
        pub fn set_x(&mut self, x: *mut i32) {
            self.x = x;
        }
        pub unsafe fn get_x(&self) -> *mut i32 {
            unsafe { self.x }
        }
    }
}
