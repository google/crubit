// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

struct Fnv1aHasher {
    hash: u64,
}

impl Fnv1aHasher {
    fn new() -> Self {
        Fnv1aHasher { hash: 0xcbf29ce484222325 }
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash ^= byte as u64;
            self.hash = self.hash.wrapping_mul(0x100000001b3);
        }
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}

pub fn compute_thunk_name(target: &str, file_path: &str, line: usize, col: usize) -> String {
    let mut hasher = Fnv1aHasher::new();
    hasher.write(target.as_bytes());
    hasher.write(file_path.as_bytes());
    hasher.write(&line.to_ne_bytes());
    hasher.write(&col.to_ne_bytes());
    let hash = hasher.finish();

    let filename =
        std::path::Path::new(file_path).file_name().and_then(|s| s.to_str()).unwrap_or("source");
    let escaped_name: String =
        filename.chars().map(|c| if c == '.' || c == '-' { '_' } else { c }).collect();

    format!("__inline_cpp_thunk_{}_{:016x}", escaped_name, hash)
}
