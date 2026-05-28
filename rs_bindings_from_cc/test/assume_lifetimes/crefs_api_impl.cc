// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:crefs
// Features: assume_lifetimes, assume_this_lifetimes, callables, check_default_initialized, experimental, layout_compat_tuple, leading_colons_for_cpp_type, supported, template_instantiation, types, unsafe_view, wrapper

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/crefs.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((int& (*)(int&)) & ::id_cmut);

static_assert((int const& (*)(int const&)) & ::id_cref);

#pragma clang diagnostic pop
