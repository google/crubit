// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;
use repeatedly_failed_template_instantiation::*;

/// Mostly a compile-time test.
///
/// `TakesUninstantiable1`, `TakesUninstantiable2`, `TakesNestedUninstantiable`,
/// `TakesPointerToUninstantiable`, `TakesInnerLater`,
/// `TakesConceptCheckOnFailedBase` and `TakesFailedInImplicitCopy` must all be
/// left without bindings.
/// Previously only the *first* reference to a non-instantiable specialization
/// was rejected, and the later ones produced a `..._rs_api_impl.cc` that did
/// not compile. Getting this crate to build at all is therefore the actual
/// regression check; the assertions below merely pin down that instantiable
/// specializations are unaffected.
#[gtest]
fn test_instantiable_specializations_still_get_bindings() {
    let mut instantiable = __CcTemplateInst20InstantiableTemplateI10IncompleteE::default();
    // SAFETY: the C++ functions ignore the pointer they are handed.
    unsafe {
        TakesInstantiable1(&raw mut instantiable);
        TakesInstantiable2(&raw mut instantiable);
    }
}
