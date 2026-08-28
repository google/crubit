// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:do_not_eagerly_import_template_type_args_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/do_not_eagerly_import_template_type_args.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z13ImportedFirst10DoesNotUseIS_IiEE(
    struct DoesNotUse<DoesNotUse<int>>* __param_0) {
  ImportedFirst(std::move(*__param_0));
}

static_assert((void (*)(struct DoesNotUse<DoesNotUse<int>>)) & ::ImportedFirst);

extern "C" void __rust_thunk___Z14ImportedSecond10DoesNotUseIiE(
    struct DoesNotUse<int>* __param_0) {
  ImportedSecond(std::move(*__param_0));
}

static_assert((void (*)(struct DoesNotUse<int>)) & ::ImportedSecond);

static_assert(sizeof(struct DoesNotUse<DoesNotUse<int>>) == 1);
static_assert(alignof(struct DoesNotUse<DoesNotUse<int>>) == 1);

extern "C" void __rust_thunk__2e59fe08__ZN10DoesNotUseIS_IiEEC1Ev(
    struct DoesNotUse<DoesNotUse<int>>* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct DoesNotUse<int>) == 1);
static_assert(alignof(struct DoesNotUse<int>) == 1);

extern "C" void __rust_thunk__2e59fe08__ZN10DoesNotUseIiEC1Ev(
    struct DoesNotUse<int>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
