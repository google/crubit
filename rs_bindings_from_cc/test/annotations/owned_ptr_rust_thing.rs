// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

impl Drop for crate::OwnedThing {
    fn drop(&mut self) {
        unsafe {
            crate::RawThing::Close(self.0.as_mut());
        }
    }
}
