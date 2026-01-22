// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Lazily interpret byte slices as lossily-converted UTF-8 strings without any copies.

use core::cmp::{self, Ordering, PartialEq, PartialOrd};
use core::fmt::{Display, Formatter, Result};
use core::hash::{Hash, Hasher};
use core::str::Utf8Chunks;

/// An [`Iterator`] over `&str` chunks of a lossily-converted UTF-8 byte slice.
///
/// This is created by calling [`Utf8Lossy::chunks`].
pub struct Utf8LossyChunks<'a> {
    chunks: Utf8Chunks<'a>,
    return_replacement_character_next: bool,
}

impl<'a> Iterator for Utf8LossyChunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        const REPLACEMENT_CHARACTER_STR: &str = "\u{FFFD}";

        if self.return_replacement_character_next {
            self.return_replacement_character_next = false;
            Some(REPLACEMENT_CHARACTER_STR)
        } else {
            let chunk = self.chunks.next()?;
            self.return_replacement_character_next = !chunk.invalid().is_empty();
            Some(chunk.valid())
        }
    }
}

/// A zero-copy wrapper around a byte slice that lazily interprets the slice as a lossy UTF-8
/// string.
///
/// This type logically acts like a lossily-converted UTF-8 string. As such, trait implementations
/// like `Display`, `PartialEq`, and `Ord` are implemented based on that logical interpretation,
/// without ever having to realize it in memory.
// TODO(okabayashi): May want to change the internal representation at some point for layout
// stability and FFI purposes.
#[derive(Copy, Clone, Debug, Eq, PartialOrd)]
pub struct Utf8Lossy<'a>(&'a [u8]);

impl<'a> Utf8Lossy<'a> {
    /// Creates a new `Utf8Lossy` from the given byte slice.
    pub fn new(bytes: &'a [u8]) -> Self {
        Utf8Lossy(bytes)
    }

    /// Returns the underlying byte slice.
    pub fn as_bytes(self) -> &'a [u8] {
        self.0
    }

    /// Returns the length of the logical lossily-converted string in bytes.
    pub fn len(self) -> usize {
        self.chunks().map(str::len).sum()
    }

    /// Returns `true` if the logical lossily-converted string is empty.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over chunks of the lossily-converted string.
    pub fn chunks(self) -> Utf8LossyChunks<'a> {
        Utf8LossyChunks { chunks: self.0.utf8_chunks(), return_replacement_character_next: false }
    }
}

impl Display for Utf8Lossy<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.chunks().try_for_each(|chunk| f.write_str(chunk))
    }
}

impl PartialEq for Utf8Lossy<'_> {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}

impl Ord for Utf8Lossy<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_joined_chunks(self.chunks(), other.chunks())
    }
}

// Hash implementation takes inspiration from VecDeque's implementation, which faces a similar
// challenge of allowing multiple equivalent representations.
impl Hash for Utf8Lossy<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Write the length prefix to the hasher.
        vec![(); self.len()].hash(state);

        // Hasher only guarantees equivalence for the exact same set of calls to its methods, so we
        // hash each byte of the UTF-8 lossy representation one at a time to ensure that equivalent
        // Utf8Lossy's call the Hasher methods in the same order.
        for chunk in self.chunks() {
            for byte in chunk.as_bytes() {
                byte.hash(state);
            }
        }
    }
}

/// Lexicographically compares the two logical strings resulting from joining
/// the chunks of each iterator, without actually realizing these logical
/// concatenations in memory.
///
/// # Algorithm
///
/// A brute-force solution would be to flatten both iterators of chunks into
/// iterators of `char`s, and then compare them lexicographically. While this
/// avoids allocating memory to join chunks into `String`s, it can be
/// inefficient because comparison happens character by character.
///
/// This implementation is an optimization that works on `&str` chunks rather
/// than individual characters.
///
/// The algorithm proceeds by consuming chunks from `lhs_chunks` and
/// `rhs_chunks`. At each step, it holds the current chunk from each iterator
/// (`lhs_chunk` and `rhs_chunk`). It compares prefixes of these two chunks up to
/// `l = min(lhs_chunk.len(), rhs_chunk.len())`.
/// - If `lhs_chunk[..l]` and `rhs_chunk[..l]` differ, their lexicographical
///   order determines the result.
/// - If they are equal, this prefix is consumed from both slices. If a slice
///   becomes empty after consuming the prefix, the next non-empty chunk is
///   drawn from its iterator.
/// The loop continues until a difference is found and returned, or one
/// iterator is exhausted.
/// - If `lhs_chunks` is exhausted first, `Ordering::Less` is returned.
/// - If `rhs_chunks` is exhausted first, `Ordering::Greater` is returned.
/// - If both iterators are exhausted simultaneously, `Ordering::Equal` is
///   returned.
fn compare_joined_chunks<'a>(
    lhs_chunks: impl IntoIterator<Item = &'a str>,
    rhs_chunks: impl IntoIterator<Item = &'a str>,
) -> Ordering {
    let mut lhs_iter = lhs_chunks.into_iter().filter(|s| !s.is_empty());
    let mut lhs = lhs_iter.next();

    let mut rhs_iter = rhs_chunks.into_iter().filter(|s| !s.is_empty());
    let mut rhs = rhs_iter.next();

    loop {
        match (&mut lhs, &mut rhs) {
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (None, None) => return Ordering::Equal,
            (Some(lhs_chunk), Some(rhs_chunk)) => {
                let l = cmp::min(lhs_chunk.len(), rhs_chunk.len());

                match Ord::cmp(&lhs_chunk[..l], &rhs_chunk[..l]) {
                    Ordering::Equal => {}
                    ordering => return ordering,
                }

                *lhs_chunk = &lhs_chunk[l..];
                *rhs_chunk = &rhs_chunk[l..];

                if lhs_chunk.is_empty() {
                    lhs = lhs_iter.next();
                }
                if rhs_chunk.is_empty() {
                    rhs = rhs_iter.next();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_compare_joined_chunks_same_length() {
        let lhs = ["hel", "lo w", "orld"];
        let rhs = ["hell", "o ", "wor", "ld"];
        let is_equal = compare_joined_chunks(lhs.iter().copied(), rhs.iter().copied());
        expect_eq!(is_equal, Ordering::Equal);
    }

    #[gtest]
    fn test_compare_joined_chunks_lhs_shorter() {
        let lhs = ["hel", "lo w", "orld"];
        let rhs = ["hell", "o ", "wor", "ld!!!!!!"];
        let left_finished_first = compare_joined_chunks(lhs.iter().copied(), rhs.iter().copied());
        expect_eq!(left_finished_first, Ordering::Less);
    }

    #[gtest]
    fn test_equal_bytes_are_eq() {
        expect_eq!(Utf8Lossy::new(b"hello"), Utf8Lossy::new(b"hello"));
        expect_eq!(Utf8Lossy::new(b""), Utf8Lossy::new(b""));
    }

    #[gtest]
    fn test_equivalent_bytes_are_eq() {
        let invalid_utf8 = b"hello \xF0\x90\x80";
        let lossy_utf8_equivalent = "hello \u{FFFD}".as_bytes();
        expect_eq!(Utf8Lossy::new(invalid_utf8), Utf8Lossy::new(lossy_utf8_equivalent));
    }

    #[gtest]
    fn test_non_equivalent_bytes_are_not_eq() {
        expect_ne!(Utf8Lossy::new(b"foo"), Utf8Lossy::new(b"bar"));
    }

    #[gtest]
    fn test_valid_utf8_ord() {
        expect_eq!(Ord::cmp(&Utf8Lossy::new(b"a"), &Utf8Lossy::new(b"a")), Ordering::Equal);
        expect_eq!(Ord::cmp(&Utf8Lossy::new(b"a"), &Utf8Lossy::new(b"b")), Ordering::Less);
        expect_eq!(Ord::cmp(&Utf8Lossy::new(b"b"), &Utf8Lossy::new(b"a")), Ordering::Greater);
    }

    #[gtest]
    fn test_lossy_utf8_ord() {
        let lhs = b"hello \xF0\x90\x80a";
        let rhs = "hello \u{FFFD}b".as_bytes();
        expect_eq!(Ord::cmp(&Utf8Lossy::new(lhs), &Utf8Lossy::new(rhs)), Ordering::Less);
    }
}
