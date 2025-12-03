// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DO_NOT_EAGERLY_IMPORT_TEMPLATE_TYPE_ARGS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DO_NOT_EAGERLY_IMPORT_TEMPLATE_TYPE_ARGS_H_

template <typename T>
struct DoesNotUse {};

// `DoesNotUse<DoesNotUse<int>>` does not instantiate the inner template
// parameter, `DoesNotUse<int>`, but it _is_ instantiatable. This test
// shows that we should not import it in its uninstantiated form, otherwise
// ImportedSecond will read this cached import and think that `DoesNotUse<int>`
// is incomplete, which is false.

// Doc comment
void ImportedFirst(DoesNotUse<DoesNotUse<int>>);

// We expect ImportedSecond to fail because we need wrapper mode, _not_ because
// `DoesNotUse<int>` is incomplete.

// Doc comment
void ImportedSecond(DoesNotUse<int>);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_DO_NOT_EAGERLY_IMPORT_TEMPLATE_TYPE_ARGS_H_
