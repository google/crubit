// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// send_sync_types_golden

#![allow(unused_unsafe, deprecated, non_snake_case, unreachable_code)]
#![allow(improper_ctypes_definitions)]
#![deny(warnings)]

extern crate alloc;
extern crate core;
const _: () = assert!(::std::mem::size_of::<::send_sync_types_golden::NeitherSendNorSync>() == 4);
const _: () = assert!(::std::mem::align_of::<::send_sync_types_golden::NeitherSendNorSync>() == 4);
const _: () =
    assert!(::core::mem::offset_of!(::send_sync_types_golden::NeitherSendNorSync, 0) == 0);
const _: () =
    assert!(::core::mem::offset_of!(::send_sync_types_golden::NeitherSendNorSync, 1) == 4);
const _: () = assert!(::std::mem::size_of::<::send_sync_types_golden::SendAndSync>() == 4);
const _: () = assert!(::std::mem::align_of::<::send_sync_types_golden::SendAndSync>() == 4);
const _: () = assert!(::core::mem::offset_of!(::send_sync_types_golden::SendAndSync, 0) == 0);
const _: () = assert!(::std::mem::size_of::<::send_sync_types_golden::SendButNotSync>() == 4);
const _: () = assert!(::std::mem::align_of::<::send_sync_types_golden::SendButNotSync>() == 4);
const _: () = assert!(::core::mem::offset_of!(::send_sync_types_golden::SendButNotSync, 0) == 0);
const _: () = assert!(::core::mem::offset_of!(::send_sync_types_golden::SendButNotSync, 1) == 4);
const _: () = assert!(::std::mem::size_of::<::send_sync_types_golden::SyncButNotSend>() == 4);
const _: () = assert!(::std::mem::align_of::<::send_sync_types_golden::SyncButNotSend>() == 4);
const _: () = assert!(::core::mem::offset_of!(::send_sync_types_golden::SyncButNotSend, 0) == 0);
const _: () = assert!(::core::mem::offset_of!(::send_sync_types_golden::SyncButNotSend, 1) == 4);
