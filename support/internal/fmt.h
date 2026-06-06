// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_FMT_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_FMT_H_

#include <cstddef>
#include <ios>
#include <ostream>
#include <streambuf>
#include <string_view>
#include <type_traits>

#if defined(__clang__)
#define CRUBIT_LIFETIME_BOUND [[clang::lifetimebound]]
#else
#define CRUBIT_LIFETIME_BOUND
#endif

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
[[nodiscard]] bool Fmt(const T& value, LossyFormatter& formatter);
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
  explicit AbslSink(LossyFormatter* formatter CRUBIT_LIFETIME_BOUND)
      : formatter_(formatter) {}
  AbslSink(const AbslSink&) = delete;
  AbslSink& operator=(const AbslSink&) = delete;

  bool ok() const { return formatter_ != nullptr; }

  void Append(size_t count, char ch) {
    if (formatter_ != nullptr &&
        formatter_->write_fill(/*count=*/count, /*data=*/ch) < count) {
      formatter_ = nullptr;
    }
  }

  void Append(std::string_view v) {
    if (formatter_ != nullptr &&
        formatter_->write_bytes(v.data(), v.size()) < v.size()) {
      formatter_ = nullptr;
    }
  }

  friend void AbslFormatFlush(AbslSink* sink, std::string_view v) {
    sink->Append(v);
  }

 private:
  LossyFormatter* formatter_;
};
template <typename LossyFormatter>
explicit AbslSink(LossyFormatter*) -> AbslSink<LossyFormatter>;

// Implements a `std::streambuf` that writes to a Rust formatter.
template <typename LossyFormatter>
class Streambuf : public std::streambuf {
 public:
  explicit Streambuf(LossyFormatter* formatter CRUBIT_LIFETIME_BOUND)
      : formatter_(formatter) {}
  Streambuf(const Streambuf&) = delete;
  Streambuf& operator=(const Streambuf&) = delete;

 private:
  using int_type = std::streambuf::int_type;
  using traits_type = std::streambuf::traits_type;

  std::streamsize xsputn(const char* s, std::streamsize count) override {
    return formatter_->write_bytes(s, count);
  }

  int_type overflow(int_type ch) override {
    if (traits_type::eq_int_type(ch, traits_type::eof())) {
      return formatter_->flush() ? traits_type::not_eof(ch)
                                 : traits_type::eof();
    }
    return formatter_->write_byte(ch) ? ch : traits_type::eof();
  }

  int sync() override { return formatter_->flush() ? 0 : -1; }

  LossyFormatter* formatter_;
};
template <typename LossyFormatter>
explicit Streambuf(LossyFormatter*) -> Streambuf<LossyFormatter>;

// Concepts to detect formatting support.
template <typename T, typename LossyFormatter>
concept HasAbslStringify =
    requires(AbslSink<LossyFormatter>& sink, const T& value) {
      AbslStringify(sink, value);
    };

template <typename T>
concept HasOstreamOperator =
    requires(std::ostream& os, const T& value) { os << value; };

}  // namespace fmt_detail

template <typename T, typename LossyFormatter>
[[nodiscard]] bool Fmt(const T& value, LossyFormatter& formatter) {
  static_assert(
      std::is_same_v<LossyFormatter, lossy_formatter::LossyFormatter>);
  if constexpr (fmt_detail::HasAbslStringify<T, LossyFormatter>) {
    fmt_detail::AbslSink sink(&formatter);
    AbslStringify(sink, value);
    return sink.ok() && formatter.flush();
  } else if constexpr (fmt_detail::HasOstreamOperator<T>) {
    fmt_detail::Streambuf buf(&formatter);
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
