// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Mirrors how `rustix` exposes `rustix::fd::BorrowedFd`: one path segment shorter than std's own
/// `std::os::fd::BorrowedFd`, so it competes for the canonical C++ name.
pub mod fd {
    pub use std::os::fd::BorrowedFd;
}
