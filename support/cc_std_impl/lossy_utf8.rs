// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod lossy_utf8 {
    /// Helper struct for safely printing strings that may contain non-Unicode data.
    ///
    /// This struct implements the Display trait. The implementation replaces invalid UTF-8 with the
    /// Unicode replacement character (�), and is potentially lossy.
    pub struct LossyUtf8Display<'a>(pub &'a [u8]);

    impl<'a> core::fmt::Display for LossyUtf8Display<'a> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            use core::fmt::Write;
            for chunk in self.0.utf8_chunks() {
                f.write_str(chunk.valid())?;
                if !chunk.invalid().is_empty() {
                    f.write_char(core::char::REPLACEMENT_CHARACTER)?;
                }
            }
            Ok(())
        }
    }
}
