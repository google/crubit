// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// The definitions below model assorted definitions in Abseil's logging library.
struct string;

// `something` is chosen to indicate that we don't want to depend on the details
// of which particular namespace the real implementations reside in. They need
// only be somewhere in `absl`.
namespace absl::something {
template <typename T>
const T &GetReferenceableValue(const T &);

template <typename T1, typename T2>
string *Check_NEImpl(const T1 &, const T2 &, const char *);

class LogMessageFatal {
 public:
  LogMessageFatal();
  [[noreturn]] ~LogMessageFatal();
  LogMessageFatal &InternalStream();
};
}  // namespace absl::something

#define CHECK_OP(name, op, a, b)                                  \
  while (string *result = ::absl::something::name##Impl(          \
             ::absl::something::GetReferenceableValue(a),         \
             ::absl::something::GetReferenceableValue(b), "msg")) \
  ::absl::something::LogMessageFatal().InternalStream()

#define CHECK_NE(a, b) CHECK_OP(Check_NE, !=, (a), (b))
