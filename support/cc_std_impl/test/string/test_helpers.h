// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_CPP_STD_STRING_CPP_STD_STRING_TEST_LIB_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_CPP_STD_STRING_CPP_STD_STRING_TEST_LIB_H_

#include <string>
#include <vector>

namespace cpp_std_string_test {

inline std::string RoundTrip(std::string s) { return s; }

inline std::string RoundTripRef(const std::string& s) { return s; }
inline const std::string& ReturnRef(const std::string& s) { return s; }
inline void MutateRef(std::string& s) { s += " mutated"; }
inline void CopyRef(const std::string& src, std::string& dest) { dest = src; }

inline std::string RoundTripPtr(const std::string* s) { return *s; }
inline const std::string* ReturnPtr(const std::string* s) { return s; }
inline void MutatePtr(std::string* s) { *s += " mutated"; }

inline std::string FirstElement(const std::vector<std::string>& v) {
  if (v.empty()) return "";
  return v[0];
}
inline const std::string& FirstElementRef(const std::vector<std::string>& v) {
  return v[0];
}
inline std::vector<std::string> MakeVector(const std::string& s) {
  return {s, s + "_2"};
}

struct StringStruct {
  std::string s;
};

inline std::string GetStringFromStruct(const StringStruct& str) {
  return str.s;
}
inline const std::string& GetStringRefFromStruct(const StringStruct& str) {
  return str.s;
}

inline StringStruct MakeStringStruct(const std::string& s) {
  return StringStruct{s};
}

}  // namespace cpp_std_string_test

// Placeholder comment to force rebuild 33
#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_IMPL_TEST_CPP_STD_STRING_CPP_STD_STRING_TEST_LIB_H_
