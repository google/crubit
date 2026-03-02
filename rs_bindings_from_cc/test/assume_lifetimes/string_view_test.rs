// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::raw_string_view;
use cc_std::std::string_view as ccsv;
use googletest::prelude::*;

#[gtest]
fn string_view_sink_test() {
    let sv: ccsv = "hello".into();
    // move occurs because `sv` has type `cc_std::std::string_view<'_>`, which does not implement the `Copy` trait
    // (seems like it should?)
    string_view::string_view_sink(sv);

    let sv2: ccsv = "world".into();
    let sv3 = string_view::string_view_return(sv2);
    string_view::string_view_sink(sv3);

    let sv4: ccsv = "gnirts".into();
    string_view::explicit_lifetime_string_view(sv4);

    let raw_sv: raw_string_view = string_view::ambiguous_string_view_return("a".into(), "b".into());
    let ok_sv: ccsv = string_view::unambiguous_string_view_return_annotated("a".into(), "b".into());
}
