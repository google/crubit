// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FMT_FORMATTABLES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FMT_FORMATTABLES_H_

#include <cstddef>
#include <ostream>

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

struct CRUBIT_MUST_BIND CRUBIT_TRAIT_DERIVE("Debug") DerivesDebug {
  absl::string_view display;
  absl::string_view only_debug;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const DerivesDebug& value) {
    sink.Append(value.display);
  }
};

enum class CRUBIT_MUST_BIND FormattableEnum {
  kKnown = 1,
};
template <typename Sink>
void AbslStringify(Sink& sink, FormattableEnum value) {
  switch (value) {
    case FormattableEnum::kKnown:
      sink.Append("Known");
      break;
    default:
      absl::Format(&sink, "%d", static_cast<int>(value));
      break;
  }
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FMT_FORMATTABLES_H_
