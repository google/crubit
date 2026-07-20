// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


#line 1 "rs_bindings_from_cc/test/golden/types_in_extra_cpp_srcs_extra.cc"
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// TODO(b/497927944): This golden test demonstrates that when a type
// (like MyStruct) is defined inside an extra `cpp_srcs` file instead of a
// normal header, Crubit successfully generates the forward Rust binding
// (types_in_extra_cpp_srcs_rs_api.rs). However, what's wrong with this is
// that any Rust APIs using this type will NOT get bindings generated back
// into C++. This is because Crubit's reverse bindings generator
// (`cc_bindings_from_rs`) does not know what C++ header to `#include` to
// refer to `MyStruct` (as it only exists in a `.cc` file in the AST).
//
// What we would like it to do is to find a way to map this type back to a
// C++ target, or generate a forward declaration, so that reverse bindings can
// be properly generated and used from C++.
struct MyStruct {
  int a;
  int b;
};

int MyStructAdder(MyStruct x) { return x.a + x.b; }

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:types_in_extra_cpp_srcs_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/types_in_extra_cpp_srcs.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct MyStruct) == 8);
static_assert(alignof(struct MyStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(a, struct MyStruct) == 0);
static_assert(CRUBIT_OFFSET_OF(b, struct MyStruct) == 4);

extern "C" void __rust_thunk___ZN8MyStructC1Ev(struct MyStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" int __rust_thunk___Z13MyStructAdder8MyStruct(struct MyStruct* x) {
  return MyStructAdder(std::move(*x));
}

static_assert((int (*)(struct MyStruct)) & ::MyStructAdder);

#pragma clang diagnostic pop
