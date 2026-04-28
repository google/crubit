// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DISPLAYABLES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DISPLAYABLES_H_

struct DisplayableStruct {
  template <typename Sink>
  friend void AbslStringify(Sink& sink, const DisplayableStruct& value) {
    sink.Append("DisplayableStruct");
  }
};

enum class DisplayableEnum {
  kKnown = 1,
};
template <typename Sink>
void AbslStringify(Sink& sink, DisplayableEnum value) {
  switch (value) {
    case DisplayableEnum::kKnown:
      sink.Append("Known");
      break;
    default:
      sink.Append("Unknown");
      break;
  }
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DISPLAYABLES_H_
