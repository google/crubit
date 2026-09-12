// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Crubit hashing support library.
//!
//! Provides `RapidHasher` and `hash_u64` used to bridge Rust `core::hash::Hash`
//! implementations across FFI to C++ `AbslHashValue` and `std::hash`.

use core::hash::{Hash, Hasher};

// Fixed, public, prime-like mathematical constants chosen for optimal
// bit dispersion and coprimality; these are not cryptographic secrets.
const K0: u64 = 0x2d358dccaa6c78a5;
const K1: u64 = 0x8bb84b93962eacc9;
const K2: u64 = 0x4b33a62ed433d4a3;

/// A self-contained 64-bit hasher implementing the Rapidhash algorithm
/// (https://github.com/Nicoshev/rapidhash).
///
/// This hasher is used across Crubit FFI to compute a 64-bit hash for Rust
/// types implementing `core::hash::Hash`.
///
/// # Design & Guarantees
/// - **Algorithm**: Rapidhash is the modern successor to Wyhash. It uses 64-to-128-bit
///   multiplication and XOR mixing (`rapid_mix`), processing 8 bytes per chunk
///   in little-endian byte order.
/// - **Hash Quality**: Passes all SMHasher and SMHasher3 tests with 0 collisions
///   and full avalanche effect across all 64 output bits. This ensures good bucket
///   distribution even when cast to `size_t` for `std::hash`.
/// - **Scope & Stability**: This is an in-memory non-cryptographic hash intended
///   strictly for in-process hash table lookups (e.g. `AbslHashValue` and `std::hash`).
///   It is NOT intended for cryptographic use or persistent on-disk fingerprinting.
/// - **HashDoS Defense**: When used with Abseil containers (`absl::flat_hash_set`/`map`),
///   `AbslHashValue` combines the 64-bit value into `absl::Hash`'s per-process
///   randomized seed state, protecting against algorithmic complexity (hash flooding) attacks.
#[derive(Clone, Copy)]
pub struct RapidHasher {
    seed: u64,
    total_len: u64,
    buffer: [u8; 8],
    buf_len: usize,
}

impl RapidHasher {
    #[inline(always)]
    pub const fn new() -> Self {
        let seed = Self::rapid_mix(K0 ^ K2, K1);
        Self { seed, total_len: 0, buffer: [0; 8], buf_len: 0 }
    }

    #[inline(always)]
    const fn rapid_mix(a: u64, b: u64) -> u64 {
        let r = (a as u128).wrapping_mul(b as u128);
        ((r >> 64) as u64) ^ (r as u64)
    }

    #[inline(always)]
    fn consume_8_bytes(&mut self, val: u64) {
        self.seed = Self::rapid_mix(val ^ K0, self.seed ^ K1);
    }
}

impl Default for RapidHasher {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for RapidHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        let mut final_seed = self.seed;
        if self.buf_len > 0 {
            let mut last_bytes = [0u8; 8];
            last_bytes[..self.buf_len].copy_from_slice(&self.buffer[..self.buf_len]);
            let val = u64::from_le_bytes(last_bytes);
            final_seed = Self::rapid_mix(val ^ K0, final_seed ^ K1);
        }
        Self::rapid_mix(final_seed ^ K2 ^ self.total_len, K1)
    }

    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        self.total_len = self.total_len.wrapping_add(bytes.len() as u64);

        if self.buf_len > 0 {
            let needed = 8 - self.buf_len;
            if bytes.len() < needed {
                self.buffer[self.buf_len..self.buf_len + bytes.len()].copy_from_slice(bytes);
                self.buf_len += bytes.len();
                return;
            }
            self.buffer[self.buf_len..8].copy_from_slice(&bytes[..needed]);
            let chunk = u64::from_le_bytes(self.buffer);
            self.consume_8_bytes(chunk);
            self.buf_len = 0;
            bytes = &bytes[needed..];
        }

        let (chunks, remainder) = bytes.as_chunks::<8>();
        for &chunk in chunks {
            let val = u64::from_le_bytes(chunk);
            self.consume_8_bytes(val);
        }
        bytes = remainder;

        if !bytes.is_empty() {
            self.buffer[..bytes.len()].copy_from_slice(bytes);
            self.buf_len = bytes.len();
        }
    }

    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        self.write(&[i]);
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_le_bytes());
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        if self.buf_len == 0 {
            self.total_len = self.total_len.wrapping_add(8);
            self.consume_8_bytes(i);
        } else {
            self.write(&i.to_le_bytes());
        }
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline(always)]
    fn write_i8(&mut self, i: i8) {
        self.write_u8(i as u8);
    }

    #[inline(always)]
    fn write_i16(&mut self, i: i16) {
        self.write_u16(i as u16);
    }

    #[inline(always)]
    fn write_i32(&mut self, i: i32) {
        self.write_u32(i as u32);
    }

    #[inline(always)]
    fn write_i64(&mut self, i: i64) {
        self.write_u64(i as u64);
    }

    #[inline(always)]
    fn write_isize(&mut self, i: isize) {
        self.write_u64(i as u64);
    }
}

/// Hashes `value` using `RapidHasher` and returns a 64-bit hash.
#[inline(always)]
pub fn hash_u64<T: Hash + ?Sized>(value: &T) -> u64 {
    let mut hasher = RapidHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::expect_eq;
    use googletest::expect_ne;
    use googletest::gtest;

    #[gtest]
    fn test_rapid_hasher() {
        let mut hasher = RapidHasher::new();
        hasher.write(&[1, 2, 3]);
        let h1 = hasher.finish();

        let mut hasher2 = RapidHasher::new();
        hasher2.write(&[1, 2, 3]);
        let h2 = hasher2.finish();

        let mut hasher3 = RapidHasher::new();
        hasher3.write(&[1, 2, 4]);
        let h3 = hasher3.finish();

        expect_eq!(h1, h2);
        expect_ne!(h1, h3);
    }

    #[gtest]
    fn test_hash_u64() {
        let h1 = hash_u64(&42i32);
        let h2 = hash_u64(&42i32);
        let h3 = hash_u64(&43i32);

        expect_eq!(h1, h2);
        expect_ne!(h1, h3);
    }

    #[gtest]
    fn test_write_u64_matches_write_bytes() {
        let mut hasher1 = RapidHasher::new();
        hasher1.write_u64(0x123456789abcdef0);
        let h1 = hasher1.finish();

        let mut hasher2 = RapidHasher::new();
        hasher2.write(&0x123456789abcdef0u64.to_le_bytes());
        let h2 = hasher2.finish();

        expect_eq!(h1, h2);
    }
}
