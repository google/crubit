// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::MutexGuard;

/// 1. Implements both Send and Sync.
pub struct SendAndSync(pub i32);

/// 2. Implements Send but NOT Sync (via Cell).
pub struct SendButNotSync(pub i32, pub PhantomData<Cell<()>>);

/// 3. Does NOT implement Send, but implements Sync (via MutexGuard).
pub struct SyncButNotSend(pub i32, pub PhantomData<MutexGuard<'static, ()>>);

/// 4. Implements neither Send nor Sync (via raw pointer).
pub struct NeitherSendNorSync(pub i32, pub PhantomData<*const ()>);
