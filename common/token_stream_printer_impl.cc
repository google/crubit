// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/token_stream_printer_impl.h"

#include <string>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "common/ffi_types.h"
#include "clang/Format/Format.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/JSON.h"

crubit::FfiU8SliceBox Crubit_ClangFormat(crubit::FfiU8Slice cc_source_text) {
  llvm::StringRef input(crubit::StringViewFromFfiU8Slice(cc_source_text));

  // `llvm::vfs::...::InMemoryFileAdaptor` requires that the buffer it wraps is
  // null terminated and verifies this by looking 1 char past the end of
  // `StringRef` - `assert((!RequiresNullTerminator || BufEnd[0] == 0)...)`.
  CHECK(!input.empty() && input.back() == '\0');
  input = input.drop_back();

  llvm::Expected<std::string> maybe_formatted =
      clang::tooling::applyAllReplacements(
          input,
          clang::format::reformat(
              clang::format::getGoogleStyle(clang::format::FormatStyle::LK_Cpp),
              input, clang::tooling::Range(0, input.size()), "<stdin>"));
  if (llvm::Error error = maybe_formatted.takeError()) {
    LOG(FATAL) << "clang-format failure: " << toString(std::move(error));
  }
  return crubit::AllocFfiU8SliceBox(crubit::MakeFfiU8Slice(*maybe_formatted));
}
