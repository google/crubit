// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:string_view
// Features: assume_lifetimes, supported, types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/string_view.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void
__rust_thunk___Z16string_view_sinkNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
    std::__u::string_view* s) {
  string_view_sink(std::move(*s));
}

static_assert((void (*)(std::__u::string_view)) & ::string_view_sink);

extern "C" void
__rust_thunk___Z18string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
    std::__u::string_view* __return, std::__u::string_view* s) {
  new (__return) auto(string_view_return(std::move(*s)));
}

static_assert((std::__u::string_view (*)(std::__u::string_view)) &
              ::string_view_return);

extern "C" void
__rust_thunk___Z28ambiguous_string_view_returnNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(
    std::__u::string_view* __return, std::__u::string_view* a,
    std::__u::string_view* b) {
  new (__return) auto(
      ambiguous_string_view_return(std::move(*a), std::move(*b)));
}

static_assert((std::__u::string_view (*)(std::__u::string_view,
                                         std::__u::string_view)) &
              ::ambiguous_string_view_return);

extern "C" void
__rust_thunk___Z29explicit_lifetime_string_viewNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEE(
    std::__u::string_view* x) {
  explicit_lifetime_string_view(std::move(*x));
}

static_assert((void (*)(std::__u::string_view)) &
              ::explicit_lifetime_string_view);

extern "C" void
__rust_thunk___Z40unambiguous_string_view_return_annotatedNSt3__u17basic_string_viewIcNS_11char_traitsIcEEEES3_(
    std::__u::string_view* __return, std::__u::string_view* x,
    std::__u::string_view* y) {
  new (__return) auto(
      unambiguous_string_view_return_annotated(std::move(*x), std::move(*y)));
}

static_assert((std::__u::string_view (*)(std::__u::string_view,
                                         std::__u::string_view)) &
              ::unambiguous_string_view_return_annotated);

#pragma clang diagnostic pop
