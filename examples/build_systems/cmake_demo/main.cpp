// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include <iostream>
#include <string>

#include "crubit/rust_blake3_scanner.h"
// Crubit automatically includes bindings to the Rust standard library crates,
// generated from the standard library included in your Rust toolchain.
// They can be found at `crubit/{std, core, alloc}.h`.
#include "crubit/std.h"
#include "support/rs_std/option.h"
#include "support/rs_std/result.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/vec.h"

int main(int argc, char* argv[]) {
  // Validate CLI arguments
  if (argc < 2) {
    std::cerr << "Usage: " << argv[0] << " <directory_path>" << std::endl;
    return 1;
  }

  // Crubit includes C++ equivalents for Rust builtin types for use in bindings.
  // `rs_std::StrRef` is equivalent to Rust's `&str`.
  using ::rs::std::string::String;
  using ::rs_std::StrRef;
  std::optional<StrRef> dir_path = StrRef::FromUtf8(argv[1]);
  if (!dir_path.has_value()) {
    std::cerr << "The filename \"" << argv[1] << "\" contains invalid utf8"
              << std::endl;
    return 1;
  }

  // Walk directory hashing each file in the directory individually.
  using ::rust_blake3_scanner::DirIterator;
  using ::rust_blake3_scanner::Error;
  using ::rust_blake3_scanner::DirEntry;
  // new is a normal identifier in Rust, but a keyword in C++. Crubit will
  // escape C++ keywords that appear in bindings by appending an underscore.
  rs_std::Result<DirIterator, Error> walk_dir_res =
      DirIterator::new_(*dir_path);
  if (!walk_dir_res.has_value()) {
    std::cerr << "Error: Provided path is not a valid directory: "
              << walk_dir_res.err().message
              << std::endl;
    return 1;
  }
  DirIterator& walk_dir = *walk_dir_res;

  for (rs_std::Result<DirEntry, Error>&& entry : walk_dir) {
    if (!entry.has_value()) {
      std::cerr << "Error: Failed to read directory entry: "
                << entry.err() << std::endl;
      continue;
    }
    const String& path = entry->path;
    const rs_std::Vec<std::uint8_t>& archive = entry->contents;
    blake3::Hash hash = rust_blake3_scanner::hash_archive(archive);
    std::cout << path << " - "
              << hash << std::endl;
  }

  // Hash and print the full archive of the directory.
  rs_std::Result<rs_std::Vec<std::uint8_t>, Error> archive_res = walk_dir.take_archive();
  if (!archive_res.has_value()) {
    std::cerr << "Error: Failed to package archive: "
              << archive_res.err()
              << std::endl;
  }
  std::cout << "Hashing Archive of full directory..." << std::endl;
  blake3::Hash hash = rust_blake3_scanner::hash_archive(*archive_res);
  std::cout << "Blake3 Hash of Archive: " << hash
            << std::endl;

  return 0;
}
