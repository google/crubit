// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_COMMON_TOKEN_STREAM_PRINTER_IMPL_H_
#define THIRD_PARTY_CRUBIT_COMMON_TOKEN_STREAM_PRINTER_IMPL_H_

#include "common/ffi_types.h"

// Uses `clang::format::reformat` to prettify/reformat `cc_source_text`.
extern "C" crubit::FfiU8SliceBox Crubit_ClangFormat(
    crubit::FfiU8Slice cc_source_text);

#endif  // THIRD_PARTY_CRUBIT_COMMON_TOKEN_STREAM_PRINTER_IMPL_H_
