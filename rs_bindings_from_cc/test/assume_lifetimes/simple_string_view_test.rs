// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::prelude::*;

#[gtest]
fn my_test() {
    let sv: simple_string_view::SV<'_> = simple_string_view::SV::default();
    expect_eq!(format!("{}", sv), "");
    let _sv_id = simple_string_view::sv_ident(sv);

    let sv_raw = simple_string_view::sv_make_raw();
    let _sv_raw_id = simple_string_view::sv_ident(sv_raw);

    let sv_alias = simple_string_view::SVA::default();
    let _sv_alias_id = simple_string_view::sva_lb(sv_alias);
}
