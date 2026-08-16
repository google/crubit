// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_STRING_LAYOUT_COMPATIBLE_STRING_MOCK_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_STRING_LAYOUT_COMPATIBLE_STRING_MOCK_H_

// Mock std templates to avoid toolchain headers.
namespace std {
inline namespace __u {

template <class CharT>
class char_traits {};

template <class T>
class allocator {};

template <class CharT, class Traits = char_traits<CharT>,
          class Allocator = allocator<CharT>>
class basic_string {
 public:
  // Mock layout to match new_string (32 bytes).
  char representation[32];
  ~basic_string() {}
};

using string = basic_string<char>;

}  // namespace __u
}  // namespace std

namespace test {
std::string RoundTrip(std::string s);

struct StringStruct {
  std::string s;
};
}  // namespace test

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_STRING_LAYOUT_COMPATIBLE_STRING_MOCK_H_
