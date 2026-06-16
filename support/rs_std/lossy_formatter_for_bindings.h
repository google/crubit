// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/lossy_formatter_for_bindings.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_LOSSY_FORMATTER_FOR_BINDINGS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_LOSSY_FORMATTER_FOR_BINDINGS_H_

#include <cstddef>
#include <cstdint>

namespace lossy_formatter {
struct LossyFormatter;

extern "C" {
size_t crubit_LossyFormatter_write_bytes(LossyFormatter* self, const char* data,
                                         size_t count);
bool crubit_LossyFormatter_write_byte(LossyFormatter* self, uint8_t data);
size_t crubit_LossyFormatter_write_fill(LossyFormatter* self, size_t count,
                                        uint8_t data);
bool crubit_LossyFormatter_flush(LossyFormatter* self);
}

// A type-punned wrapper around the Rust LossyFormatter type.
//
// Note that this type should never be held by value in C++, it should only be
// held by pointer because it doesn't have a matching layout in C++.
struct LossyFormatter {
  LossyFormatter() = delete;
  LossyFormatter(const LossyFormatter&) = delete;
  LossyFormatter& operator=(const LossyFormatter&) = delete;

  size_t write_bytes(const char* data, size_t count) {
    return crubit_LossyFormatter_write_bytes(this, data, count);
  }
  bool write_byte(uint8_t data) {
    return crubit_LossyFormatter_write_byte(this, data);
  }
  size_t write_fill(size_t count, uint8_t data) {
    return crubit_LossyFormatter_write_fill(this, count, data);
  }
  bool flush() { return crubit_LossyFormatter_flush(this); }
};
}  // namespace lossy_formatter

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_LOSSY_FORMATTER_FOR_BINDINGS_H_
