// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
extern crate fd_reexporter;

use std::os::fd::AsRawFd;
use std::os::fd::BorrowedFd;

/// Both functions take the identical Rust type, spelled two different ways. They must bind to a
/// single C++ type named after `std`, not after the shorter re-export in `fd_reexporter`.
pub fn raw_fd_via_std(fd: BorrowedFd<'_>) -> i32 {
    fd.as_raw_fd()
}

pub fn raw_fd_via_reexport(fd: fd_reexporter::fd::BorrowedFd<'_>) -> i32 {
    fd.as_raw_fd()
}
