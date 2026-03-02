// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DISPLAY_DISPLAYABLES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DISPLAY_DISPLAYABLES_H_

#include <cstddef>
#include <ostream>

#include "absl/strings/has_ostream_operator.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "support/annotations.h"

struct CRUBIT_MUST_BIND CanAbslStringify {
  absl::string_view value;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const CanAbslStringify& value) {
    sink.Append(value.value);
  }
};

struct CRUBIT_MUST_BIND CanAbslStringifyByFill {
  size_t count = 0;
  char ch = '\0';

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const CanAbslStringifyByFill& value) {
    sink.Append(/*count=*/value.count, /*ch=*/value.ch);
  }
};

struct CRUBIT_MUST_BIND CanAbslStringifyByFormat {
  absl::string_view value;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const CanAbslStringifyByFormat& value) {
    absl::Format(&sink, "%s", value.value);
  }
};

struct CRUBIT_MUST_BIND CanOstream {
  absl::string_view value;

  friend std::ostream& operator<<(std::ostream& out, const CanOstream& value) {
    return out << value.value;
  }
};

struct CRUBIT_MUST_BIND CanAbslStringifyAndOstream {
  absl::string_view stringify;
  absl::string_view ostream;

  template <typename Sink>
  friend void AbslStringify(Sink& sink,
                            const CanAbslStringifyAndOstream& value) {
    sink.Append(value.stringify);
  }

  friend std::ostream& operator<<(std::ostream& out,
                                  const CanAbslStringifyAndOstream& value) {
    return out << value.ostream;
  }
};

enum class CRUBIT_MUST_BIND DisplayableEnum {
  kKnown = 1,
};
template <typename Sink>
void AbslStringify(Sink& sink, DisplayableEnum value) {
  switch (value) {
    case DisplayableEnum::kKnown:
      sink.Append("Known");
      break;
    default:
      absl::Format(&sink, "%d", static_cast<int>(value));
      break;
  }
}

template <typename T>
struct CRUBIT_OVERRIDE_DISPLAY(absl::HasOstreamOperator<T>::value) Templated {
  T value;
};
template <typename Sink, typename T>
void AbslStringify(Sink& sink, const Templated<T>& value) {
  absl::Format(&sink, "%v", absl::FormatStreamed(value.value));
}

struct NotDisplayable {};
struct CRUBIT_MUST_BIND TemplatedStringView : Templated<absl::string_view> {
  explicit TemplatedStringView(absl::string_view v) : Templated{v} {}
};
struct CRUBIT_MUST_BIND TemplatedNotDisplayable : Templated<NotDisplayable> {
  TemplatedNotDisplayable() = default;
};

struct CRUBIT_MUST_BIND CRUBIT_OVERRIDE_DISPLAY(false) DisplayInRust {
  absl::string_view cc_value;
  absl::string_view rust_value;
  template <typename Sink>
  friend void AbslStringify(Sink& sink, const DisplayInRust& value) {
    sink.Append(value.cc_value);
  }
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_DISPLAY_DISPLAYABLES_H_
