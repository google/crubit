// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use crate::incomplete_utf8::IncompleteUtf8;
use std::fmt;
use std::fmt::{Debug, Formatter, Write};
use std::iter;
use std::slice;

/// Wraps a [`Formatter<'formatter>`] for the duration of `'scope` to lossily write UTF-8.
///
/// Writes bytes instead of [`str`]. Buffers up to 3 bytes of an incomplete UTF-8 sequence.
#[crubit_annotate::must_bind]
pub struct LossyFormatter<'scope, 'formatter> {
    writer: &'scope mut Formatter<'formatter>,
    incomplete: IncompleteUtf8,
}

impl<'scope, 'formatter> From<&'scope mut Formatter<'formatter>>
    for LossyFormatter<'scope, 'formatter>
{
    fn from(writer: &'scope mut Formatter<'formatter>) -> Self {
        Self { writer, incomplete: IncompleteUtf8::default() }
    }
}

impl<'scope, 'formatter> LossyFormatter<'scope, 'formatter> {
    pub fn new(writer: &'scope mut Formatter<'formatter>) -> Self {
        writer.into()
    }

    /// Lossily writes bytes of UTF-8 to the underlying [`Formatter`].
    ///
    /// Upon a complete UTF-8 sequence, simply calls [`write_str`]. Upon a complete but invalid
    /// sequence, writes [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which looks like this: �
    ///
    /// Returns the number of bytes successfully progressed: whether written to the underlying
    /// formatter or buffered as incomplete UTF-8.
    ///
    /// # Safety
    /// `data` must point to `count` bytes. `data` may be null only if `count` is 0.
    ///
    /// [U+FFFD]: char::REPLACEMENT_CHARACTER
    /// [`write_str`]: Formatter::write_str
    #[crubit_annotate::must_bind]
    #[must_use]
    pub unsafe fn write_bytes(&mut self, data: *const ffi_11::c_char, count: usize) -> usize {
        let data = if count > 0 {
            // SAFETY: caller guarantees that `data` points to `count` bytes.
            unsafe { slice::from_raw_parts(data as *const u8, count) }
        } else {
            &[]
        };
        self.write_slice(data)
    }

    fn write_slice(&mut self, mut data: &[u8]) -> usize {
        let mut progressed = 0;
        while !self.incomplete.is_empty() {
            let Some((first, rest)) = data.split_first() else {
                break;
            };
            if !self.write_byte(*first) {
                return progressed;
            }
            progressed += 1;
            data = rest;
        }

        let mut write_chunks = || -> fmt::Result {
            let mut chunks = data.utf8_chunks().peekable();
            while let Some(chunk) = chunks.next() {
                self.writer.write_str(chunk.valid())?;
                progressed += chunk.valid().len();
                let Err(e) = str::from_utf8(chunk.invalid()) else {
                    assert!(
                        chunk.invalid().is_empty(),
                        "Invalid part of {chunk:?} should be empty"
                    );
                    continue;
                };
                let has_next_chunk = chunks.peek().is_some();
                let has_unexpected_byte = e.error_len().is_some();
                if has_next_chunk || has_unexpected_byte {
                    self.writer.write_char(char::REPLACEMENT_CHARACTER)?;
                } else {
                    self.incomplete
                        .copy_from_slice(chunk.invalid())
                        .expect("Err from str::from_utf8 should have error_len after 3 bytes");
                }
                progressed += chunk.invalid().len();
            }
            Ok(())
        };
        let _ = write_chunks();
        progressed
    }

    /// Lossily writes UTF-8 to the underlying [`Formatter`].
    ///
    /// Upon a complete UTF-8 sequence, simply calls [`write_char`]. Upon a complete but invalid
    /// sequence, writes [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which looks like this: �
    ///
    /// Returns false if a write error occurred; otherwise, returns true.
    ///
    /// [U+FFFD]: char::REPLACEMENT_CHARACTER
    /// [`write_char`]: Write::write_char
    #[crubit_annotate::must_bind]
    #[must_use]
    pub fn write_byte(&mut self, data: u8) -> bool {
        let mut write_two_chars = |a, b| -> fmt::Result {
            self.writer.write_char(a)?;
            self.writer.write_char(b)?;
            Ok(())
        };
        use incomplete_utf8::PushState::*;
        match self.incomplete.push(data) {
            Incomplete => true,
            Valid(c) => self.writer.write_char(c).is_ok(),
            Invalid => self.writer.write_char(char::REPLACEMENT_CHARACTER).is_ok(),
            InvalidThenValid(c) => write_two_chars(char::REPLACEMENT_CHARACTER, c).is_ok(),
            InvalidThenInvalid => {
                write_two_chars(char::REPLACEMENT_CHARACTER, char::REPLACEMENT_CHARACTER).is_ok()
            }
        }
    }

    /// Lossily writes UTF-8 to the underlying [`Formatter`]. Equivalent to calling
    /// [`Self::write_byte`] `count` times.
    ///
    /// Returns the number of bytes successfully progressed: whether written to the underlying
    /// formatter or buffered as incomplete UTF-8.
    #[crubit_annotate::must_bind]
    #[must_use]
    pub fn write_fill(&mut self, count: usize, data: u8) -> usize {
        if data.is_ascii() {
            // Fast path for ASCII characters.
            if !self.incomplete.is_empty() {
                // An incomplete UTF-8 sequence never has ASCII bytes.
                self.incomplete.clear();
                if self.writer.write_char(char::REPLACEMENT_CHARACTER).is_err() {
                    return 0;
                }
            }
            let data = char::from_u32(data as u32).expect("ASCII character should be valid char");
            return iter::repeat_n(/*element=*/ data, /*count=*/ count)
                .take_while(|c| self.writer.write_char(*c).is_ok())
                .count();
        }
        iter::repeat_n(/*element=*/ data, /*count=*/ count)
            .take_while(|byte| self.write_byte(*byte))
            .count()
    }

    /// Discards any incomplete data.
    ///
    /// If there is no incomplete data, does nothing. Otherwise, writes
    /// [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which looks like this: �
    ///
    /// Returns false if a write error occurred; otherwise, returns true.
    ///
    /// [U+FFFD]: char::REPLACEMENT_CHARACTER
    #[crubit_annotate::must_bind]
    #[must_use]
    pub fn flush(&mut self) -> bool {
        if self.incomplete.is_empty() {
            return true;
        }
        self.incomplete.clear();
        self.writer.write_char(char::REPLACEMENT_CHARACTER).is_ok()
    }
}

impl<'scope, 'formatter> Debug for LossyFormatter<'scope, 'formatter> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LossyFormatter")
            .field("incomplete", &self.incomplete)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest;
    use googletest::prelude::*;
    use std::fmt::Display;
    use std::io::Write;

    fn display_with_lossy_formatter<F: Fn(&mut LossyFormatter) -> googletest::Result<()>>(
        body: F,
    ) -> impl Display {
        struct Impl<F> {
            body: F,
        }
        impl<F: Fn(&mut LossyFormatter) -> googletest::Result<()>> Display for Impl<F> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let mut f = LossyFormatter::new(f);
                let test_result = (self.body)(&mut f);
                let fmt_result = match test_result {
                    Ok(()) => Ok(()),
                    Err(_) => Err(fmt::Error),
                };
                test_result.and_log_failure_with_message(|| format!("{f:?}"));
                fmt_result
            }
        }
        Impl { body }
    }

    #[gtest]
    fn write_slice_ok() {
        let bytes = [
            b'h', b'i', b' ', // Valid ASCII.
            240, 159, 146, 150, // Valid UTF-8.
            240, 159, 146, b' ', // Invalid UTF-8 due to unexpected ASCII.
            240, 159, b' ', // Invalid UTF-8 due to unexpected ASCII.
            240, b' ', // Invalid UTF-8 due to unexpected ASCII.
            255,  // Invalid UTF-8 byte.
            b' ', // Valid ASCII.
            240, 255,  // Invalid UTF-8 due to unexpected invalid byte.
            b' ', // Valid ASCII.
            240, 159, 255,  // Invalid UTF-8 due to unexpected invalid byte.
            b' ', // Valid ASCII.
            240, 159, 146, 255, // Invalid UTF-8 due to unexpected invalid byte.
            b' ', b'b', b'y', b'e', b' ', // Valid ASCII.
            255,  // Invalid UTF-8 byte.
            240, 159, 146, // Incomplete UTF-8 without flush.
        ];
        expect_eq!(
            display_with_lossy_formatter(|f| verify_eq!(f.write_slice(&bytes), bytes.len()))
                .to_string(),
            "hi 💖� � � � �� �� �� bye �"
        );
    }

    #[gtest]
    fn write_slice_incomplete_then_complete_ok() {
        expect_eq!(
            display_with_lossy_formatter(|f| {
                verify_eq!(f.write_slice(&[240, 159, 146]), 3)?;
                verify_eq!(f.write_slice(&[150, b'!']), 2)?;
                Ok(())
            })
            .to_string(),
            "💖!"
        );
    }

    #[gtest]
    fn write_slice_valid_eof() -> googletest::Result<()> {
        write!(
            &mut [0u8; 3][..],
            "{}",
            display_with_lossy_formatter(|f| verify_eq!(f.write_slice(&[255, 1]), 1))
        )?;
        Ok(())
    }

    #[gtest]
    fn write_slice_invalid_eof() -> googletest::Result<()> {
        write!(
            &mut [0u8; 3][..],
            "{}",
            display_with_lossy_formatter(|f| verify_eq!(f.write_slice(&[1, 2, 3, 255]), 3))
        )?;
        Ok(())
    }

    #[gtest]
    fn write_byte_ok() {
        expect_eq!(
            display_with_lossy_formatter(|f| [
                b'h', b'i', b' ', // Valid ASCII.
                240, 159, 146, 150, // Valid UTF-8.
                240, 159, 146, b' ', // Invalid UTF-8 due to unexpected ASCII.
                240, 159, b' ', // Invalid UTF-8 due to unexpected ASCII.
                240, b' ', // Invalid UTF-8 due to unexpected ASCII.
                255,  // Invalid UTF-8 byte.
                b' ', // Valid ASCII.
                240, 255,  // Invalid UTF-8 due to unexpected invalid byte.
                b' ', // Valid ASCII.
                240, 159, 255,  // Invalid UTF-8 due to unexpected invalid byte.
                b' ', // Valid ASCII.
                240, 159, 146, 255, // Invalid UTF-8 due to unexpected invalid byte.
                b' ', b'b', b'y', b'e', b' ', // Valid ASCII.
                255,  // Invalid UTF-8 byte.
                240, 159, 146, // Incomplete UTF-8 without flush.
            ]
            .into_iter()
            .map(|b| verify_true!(f.write_byte(b)))
            .collect())
            .to_string(),
            "hi 💖� � � � �� �� �� bye �"
        );
    }

    #[gtest]
    fn write_byte_valid_eof() -> googletest::Result<()> {
        write!(
            &mut [] as &mut [u8],
            "{}",
            display_with_lossy_formatter(|f| verify_false!(f.write_byte(b'a')))
        )?;
        Ok(())
    }

    #[gtest]
    fn write_byte_invalid_eof() -> googletest::Result<()> {
        write!(
            &mut [] as &mut [u8],
            "{}",
            display_with_lossy_formatter(|f| verify_false!(f.write_byte(255)))
        )?;
        Ok(())
    }

    #[gtest]
    fn write_byte_invalid_then_valid_eof() -> googletest::Result<()> {
        write!(
            &mut [] as &mut [u8],
            "{}",
            display_with_lossy_formatter(|f| {
                verify_true!(f.write_byte(240))?;
                verify_false!(f.write_byte(b'a'))?;
                Ok(())
            })
        )?;
        Ok(())
    }

    #[gtest]
    fn write_byte_invalid_then_valid_writes_replacement_then_eof() -> googletest::Result<()> {
        let mut bytes = [0u8; 3];
        write!(
            &mut bytes[..],
            "{}",
            display_with_lossy_formatter(|f| {
                verify_true!(f.write_byte(240))?;
                verify_false!(f.write_byte(b'a'))?;
                Ok(())
            })
        )?;
        verify_eq!(bytes, "�".as_bytes())?;
        Ok(())
    }

    #[gtest]
    fn write_byte_invalid_then_invalid_eof() -> googletest::Result<()> {
        write!(
            &mut [] as &mut [u8],
            "{}",
            display_with_lossy_formatter(|f| {
                verify_true!(f.write_byte(240))?;
                verify_false!(f.write_byte(255))?;
                Ok(())
            })
        )?;
        Ok(())
    }

    #[gtest]
    fn write_byte_invalid_then_invalid_writes_replacement_then_eof() -> googletest::Result<()> {
        let mut bytes = [0u8; 3];

        write!(
            &mut bytes[..],
            "{}",
            display_with_lossy_formatter(|f| {
                verify_true!(f.write_byte(240))?;
                verify_false!(f.write_byte(255))?;
                Ok(())
            })
        )?;
        verify_eq!(bytes, "�".as_bytes())?;
        Ok(())
    }

    #[gtest]
    fn write_fill_ascii_ok() {
        expect_eq!(
            display_with_lossy_formatter(|f| verify_eq!(
                f.write_fill(/*count=*/ 4, /*data=*/ b'a'),
                4
            ))
            .to_string(),
            "aaaa"
        );
    }

    #[gtest]
    fn write_fill_invalid_ok() {
        expect_eq!(
            display_with_lossy_formatter(|f|
                // Two invalid sequences, then one incomplete sequence without flush.
                verify_eq!(f.write_fill(/*count=*/ 3, /*data=*/ 240), 3))
            .to_string(),
            "��"
        );
    }

    #[gtest]
    fn write_incomplete_then_fill_ascii_ok() {
        expect_eq!(
            display_with_lossy_formatter(|f| {
                // Incomplete sequence.
                verify_true!(f.write_byte(240))?;
                // Complete one invalid sequence, then 4 valid ASCII characters.
                verify_eq!(f.write_fill(/*count=*/ 4, /*data=*/ b'a'), 4)?;
                Ok(())
            })
            .to_string(),
            "�aaaa"
        );
    }

    #[gtest]
    fn write_fill_ascii_eof() -> googletest::Result<()> {
        write!(
            &mut [0u8; 3][..],
            "{}",
            display_with_lossy_formatter(|f| verify_eq!(
                f.write_fill(/*count=*/ 4, /*data=*/ b'a'),
                3
            ))
        )?;
        Ok(())
    }

    #[gtest]
    fn write_fill_invalid_eof() -> googletest::Result<()> {
        write!(
            &mut [0u8; 6][..],
            "{}",
            display_with_lossy_formatter(|f| verify_eq!(
                f.write_fill(/*count=*/ 3, /*data=*/ 255),
                2
            ))
        )?;
        Ok(())
    }

    #[gtest]
    fn write_complete_then_flush_ok() {
        expect_eq!(
            display_with_lossy_formatter(|f| {
                verify_eq!(f.write_slice(&[240, 159, 146, 150]), 4)?;
                verify_true!(f.flush())
            })
            .to_string(),
            "💖"
        );
    }

    #[gtest]
    fn write_incomplete_then_flush_ok() {
        expect_eq!(
            display_with_lossy_formatter(|f| {
                verify_eq!(f.write_slice(&[240, 159, 146]), 3)?;
                verify_true!(f.flush())
            })
            .to_string(),
            "�"
        );
    }

    #[gtest]
    fn write_incomplete_then_flush_eof() -> googletest::Result<()> {
        write!(
            &mut [] as &mut [u8],
            "{}",
            display_with_lossy_formatter(|f| {
                verify_eq!(f.write_slice(&[240, 159, 146]), 3)?;
                verify_false!(f.flush())
            })
        )?;
        Ok(())
    }
}

mod incomplete_utf8 {
    #[derive(Debug, Default)]
    pub struct IncompleteUtf8 {
        bytes: [u8; 3],
        len: Len,
    }

    pub enum PushState {
        /// Pushing one byte results in an incomplete sequence.
        Incomplete,
        /// Pushing one byte results in a valid sequence.
        ///
        /// The buffer is now empty.
        Valid(char),
        /// Pushing one byte results in one completed invalid sequence.
        ///
        /// The buffer is either empty or has an incomplete sequence.
        Invalid,
        /// Pushing one byte results in one completed invalid sequence and one valid byte.
        ///
        /// The buffer is now empty.
        InvalidThenValid(char),
        /// Pushing one byte results in two invalid sequences: the first completed, the second
        /// the invalid byte.
        ///
        /// The buffer is now empty.
        InvalidThenInvalid,
    }

    impl IncompleteUtf8 {
        pub fn is_empty(&self) -> bool {
            self.len == Len::Zero
        }

        /// Pushes a byte onto the end of the buffer.
        ///
        /// Upon a complete sequence, clears the buffer.
        pub fn push(&mut self, data: u8) -> PushState {
            use Len::*;
            use PushState::*;
            let next_bytes = match (self.len, self.bytes) {
                (Zero, [_, _, _]) => [data, 0, 0, 0],
                (One, [a, _, _]) => [a, data, 0, 0],
                (Two, [a, b, _]) => [a, b, data, 0],
                (Three, [a, b, c]) => [a, b, c, data],
            };
            let mut chunks = next_bytes[..=(self.len as usize)].utf8_chunks();
            let first_chunk = chunks.next().expect("non-empty bytes should have first chunk");
            let Err(first_error) = str::from_utf8(first_chunk.invalid()) else {
                assert!(
                    chunks.next().is_none(),
                    "{self:?} then {data:?} should have only valid chunk"
                );
                self.clear();
                return Valid(
                    first_chunk
                        .valid()
                        .chars()
                        .next()
                        .expect("valid non-empty UTF-8 should have first char"),
                );
            };
            let Some(second_chunk) = chunks.next() else {
                let first_unexpected = first_error.error_len().is_some();
                if first_unexpected {
                    self.clear();
                    return Invalid;
                }
                self.copy_from_slice(first_chunk.invalid())
                    .expect("Err from str::from_utf8 should have error_len after 3 bytes");
                return Incomplete;
            };
            assert!(chunks.next().is_none(), "{self:?} then {data:?} should have only 2 chunks");
            let Err(second_error) = str::from_utf8(second_chunk.invalid()) else {
                self.clear();
                return InvalidThenValid(
                    second_chunk
                        .valid()
                        .chars()
                        .next()
                        .expect("valid non-empty UTF-8 should have first char"),
                );
            };
            let second_unexpected = second_error.error_len().is_some();
            if second_unexpected {
                self.clear();
                return InvalidThenInvalid;
            }
            self.copy_from_slice(second_chunk.invalid())
                .expect("Err from str::from_utf8 should have error_len after 3 bytes");
            Invalid
        }

        pub fn copy_from_slice(&mut self, data: &[u8]) -> Result<(), LenOutOfRangeError> {
            self.len = data.len().try_into()?;
            self.bytes[..data.len()].copy_from_slice(data);
            Ok(())
        }

        pub fn clear(&mut self) {
            self.len = Len::Zero;
        }
    }

    #[derive(Debug)]
    pub struct LenOutOfRangeError;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    #[repr(u8)]
    enum Len {
        #[default]
        Zero,
        One,
        Two,
        Three,
    }

    impl TryFrom<usize> for Len {
        type Error = LenOutOfRangeError;

        fn try_from(value: usize) -> Result<Self, Self::Error> {
            use Len::*;
            match value {
                0 => Ok(Zero),
                1 => Ok(One),
                2 => Ok(Two),
                3 => Ok(Three),
                _ => Err(LenOutOfRangeError),
            }
        }
    }
}
