// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_keywords() {
    use keywords::{kKeywordsAlign, kKeywordsSize, Keywords};
    expect_that!(core::mem::size_of::<Keywords>(), eq(unsafe { kKeywordsSize }));
    expect_that!(core::mem::align_of::<Keywords>(), eq(unsafe { kKeywordsAlign }));

    let kw = Keywords { r#type: 1, r#fn: 2, r#match: 3, __field_3: 4, __field_4: 5, __field_5: 6 };
    expect_that!(kw.r#type, eq(1));
    expect_that!(kw.r#fn, eq(2));
    expect_that!(kw.r#match, eq(3));
    expect_that!(kw.__field_3, eq(4));
    expect_that!(kw.__field_4, eq(5));
    expect_that!(kw.__field_5, eq(6));
}
