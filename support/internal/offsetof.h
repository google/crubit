// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_SUPPORT_OFFSETOF_H_
#define CRUBIT_SUPPORT_OFFSETOF_H_

#include <cstddef>

namespace crubit::details {

// OffsetOfHelper is very similar to `std::type_identity_t`, except that it
// provides a way to wrap `T` in parens without running into an error message
// like:
//    error: expected a type
//    static_assert(CRUBIT_OFFSET_OF(field, TestStruct) == 0, "");
//                  ^
//    note: expanded from macro 'CRUBIT_OFFSET_OF'
//      offsetof((T), member)
//               ^
// or (when using `std::type_identity_t` with parens around T):
//    error: expected expression
//    static_assert(CRUBIT_OFFSET_OF(offset0, BasicStruct) == 0, "");
//                  ^
//    note: expanded from macro 'CRUBIT_OFFSET_OF'
//      offsetof(std::type_identity_t<(T)>::type, member)
//                                       ^
// or (when using `std::type_identity_t` with different parens placement):
//    error: expected a type
//    static_assert(CRUBIT_OFFSET_OF(field, TestStruct) == 0, "");
//                  ^
//    note: expanded from macro 'CRUBIT_OFFSET_OF'
//      offsetof((crubit::type_identity_t<T>::type), member)
//               ^
//
// The errors are avoided by allowing passing `T` via `void(T)` syntax through
// the single specialization below.
template <typename T>
struct OffsetOfHelper;

template <typename T>
struct OffsetOfHelper<void(T)> {
  using Type = T;
};

}  // namespace crubit::details

// CRUBIT_OFFSET_OF is a wrapper around the standard `offsetof` macro [1] that
// adds support for using a type name (i.e. `...`) that contains commas (e.g.
// `ClassTemplateWithTwoTemplateParameters<int, int>`).
//
// CRUBIT_OFFSET_OF doesn't require wrapping the type name in an extra set of
// parens.  This aspect is achieved by making CRUBIT_OFFSET_OF a variadic macro
// (i.e. accepting 2 *or more* arguments) and by making `...` the last
// parameter (i.e. using a different order of macro parameters than the standard
// `offsetof`).
//
// See the doc comments of OffsetOfHelper above for an explanation why wrapping
// the type name in an extra parens is not sufficient for the standard
// `offsetof` macro.
//
// [1] https://en.cppreference.com/w/cpp/types/offsetof
#define CRUBIT_OFFSET_OF(member, ...) \
  offsetof(::crubit::details::OffsetOfHelper<void(__VA_ARGS__)>::Type, member)

#if defined(__clang__) || defined(__GNUC__)
#define CRUBIT_DO_PRAGMA(x) _Pragma(#x)
#define CRUBIT_WARNING_PUSH(warning) \
  _Pragma("GCC diagnostic push")     \
      CRUBIT_DO_PRAGMA(GCC diagnostic ignored warning)
#define CRUBIT_WARNING_POP _Pragma("GCC diagnostic pop")
#elif defined(_MSC_VER)
#define CRUBIT_WARNING_PUSH(warning) \
  __pragma(warning(push)) __pragma(warning(disable : 4646))
#define CRUBIT_WARNING_POP __pragma(warning(pop))
#else
#define CRUBIT_WARNING_PUSH(warning)
#define CRUBIT_WARNING_POP
#endif

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_SUPPORT_OFFSETOF_H_
