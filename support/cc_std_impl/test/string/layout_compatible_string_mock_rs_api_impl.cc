// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //support/cc_std_impl/test/string:layout_compatible_string_mock_cc

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "support/cc_std_impl/test/string/layout_compatible_string_mock.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void
__rust_thunk___ZN4test9RoundTripENSt3__u12basic_stringIcNS0_11char_traitsIcEENS0_9allocatorIcEEEE(
    class std::basic_string<char, std::char_traits<char>, std::allocator<char>>*
        __return,
    class std::basic_string<char, std::char_traits<char>, std::allocator<char>>*
        s) {
  new (__return) auto(test::RoundTrip(std::move(*s)));
}

static_assert((class std::basic_string<char, std::char_traits<char>,
                                       std::allocator<char>> (*)(
                  class std::basic_string<char, std::char_traits<char>,
                                          std::allocator<char>>)) &
              ::test::RoundTrip);

static_assert(sizeof(struct test::StringStruct) == 32);
static_assert(alignof(struct test::StringStruct) == 1);
static_assert(CRUBIT_OFFSET_OF(s, struct test::StringStruct) == 0);

extern "C" void __rust_thunk___ZN4test12StringStructC1Ev(
    struct test::StringStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN4test12StringStructC1ERKS0_(
    struct test::StringStruct* __this,
    struct test::StringStruct const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN4test12StringStructC1EOS0_(
    struct test::StringStruct* __this, struct test::StringStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct test::StringStruct*
__rust_thunk___ZN4test12StringStructaSERKS0_(
    struct test::StringStruct* __this,
    struct test::StringStruct const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" struct test::StringStruct*
__rust_thunk___ZN4test12StringStructaSEOS0_(
    struct test::StringStruct* __this, struct test::StringStruct* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

#pragma clang diagnostic pop
