// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_FMT_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_FMT_H_

#include <cstddef>
#include <ios>
#include <ostream>
#include <streambuf>
#include <type_traits>

#include "absl/base/attributes.h"
#include "absl/base/nullability.h"
#include "absl/strings/has_absl_stringify.h"
#include "absl/strings/has_ostream_operator.h"
#include "absl/strings/string_view.h"

ABSL_POINTERS_DEFAULT_NONNULL

namespace crubit {
// Formats to Rust via C++. Returns false if the underlying Rust formatter
// failed; otherwise, returns true.
//
// Uses AbslStringify if available; otherwise, falls back to `std::ostream`
// `operator<<`. Fails to compile if neither is available.
//
// `LossyFormatter` must be `lossy_formatter::LossyFormatter`. It's not included
// to avoid a dependency cycle between `LossyFormatter`'s own bindings and
// the bindings support library.
template <typename T, typename LossyFormatter>
[[nodiscard]] bool Fmt(const T& value, LossyFormatter& writer);
}  // namespace crubit

namespace lossy_formatter {
// Forward declaration to avoid a dependency cycle.
class LossyFormatter;
}  // namespace lossy_formatter

namespace crubit {
namespace fmt_detail {

// Implements an AbslStringify sink that writes to a Rust formatter.
template <typename LossyFormatter>
class AbslSink {
 public:
  explicit AbslSink(LossyFormatter* writer ABSL_ATTRIBUTE_LIFETIME_BOUND)
      : writer_(writer) {}
  AbslSink(const AbslSink&) = delete;
  AbslSink& operator=(const AbslSink&) = delete;

  bool ok() const { return writer_ != nullptr; }

  void Append(size_t count, char ch) {
    if (writer_ != nullptr &&
        writer_->write_fill(/*count=*/count, /*data=*/ch) < count) {
      writer_ = nullptr;
    }
  }

  void Append(absl::string_view v) {
    if (writer_ != nullptr &&
        writer_->write_bytes(v.data(), v.size()) < v.size()) {
      writer_ = nullptr;
    }
  }

  friend void AbslFormatFlush(AbslSink* sink, absl::string_view v) {
    sink->Append(v);
  }

 private:
  LossyFormatter* absl_nullable writer_;
};
template <typename LossyFormatter>
explicit AbslSink(LossyFormatter*) -> AbslSink<LossyFormatter>;

// Implements a `std::streambuf` that writes to a Rust formatter.
template <typename LossyFormatter>
class Streambuf : public std::streambuf {
 public:
  explicit Streambuf(LossyFormatter* writer ABSL_ATTRIBUTE_LIFETIME_BOUND)
      : writer_(writer) {}
  Streambuf(const Streambuf&) = delete;
  Streambuf& operator=(const Streambuf&) = delete;

 private:
  using int_type = std::streambuf::int_type;
  using traits_type = std::streambuf::traits_type;

  std::streamsize xsputn(const char* absl_nullable s,
                         std::streamsize count) override {
    return writer_->write_bytes(s, count);
  }

  int_type overflow(int_type ch) override {
    if (traits_type::eq_int_type(ch, traits_type::eof())) {
      return writer_->flush() ? traits_type::not_eof(ch) : traits_type::eof();
    }
    return writer_->write_byte(ch) ? ch : traits_type::eof();
  }

  int sync() override { return writer_->flush() ? 0 : -1; }

  LossyFormatter* writer_;
};
template <typename LossyFormatter>
explicit Streambuf(LossyFormatter*) -> Streambuf<LossyFormatter>;

}  // namespace fmt_detail

template <typename T, typename LossyFormatter>
[[nodiscard]] bool Fmt(const T& value, LossyFormatter& writer) {
  static_assert(
      std::is_same_v<LossyFormatter, lossy_formatter::LossyFormatter>);
  if constexpr (absl::HasAbslStringify<T>::value) {
    fmt_detail::AbslSink sink(&writer);
    AbslStringify(sink, value);
    return sink.ok() && writer.flush();
  } else if constexpr (absl::HasOstreamOperator<T>::value) {
    fmt_detail::Streambuf buf(&writer);
    std::ostream os(&buf);
    os << value;
    os.flush();
    return os.good();
  } else {
    static_assert(
        false,
        "Expected T to either have AbslStringify or have ostream operator<<");
    return false;
  }
}

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_FMT_H_
