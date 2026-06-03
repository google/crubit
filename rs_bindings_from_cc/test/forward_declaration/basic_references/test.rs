// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cref::{CMut, CRef};
use googletest::prelude::*;

#[gtest]
fn test_builds() {
    use forward_declaration::*;
    let astatic: CMut<'static, A> = fwd_source();
    // TODO(b/519307513): fwd_ident and fwd_ident_const should take CMut/CRefs directly.
    let _astatic_out: CMut<'static, A> = fwd_ident(unsafe { CMut::unique(astatic) });
    let _astatic_const_out: CRef<'static, A> =
        fwd_ident_const(core::pin::Pin::get_ref(unsafe { CMut::unchanging(astatic) }));
}
