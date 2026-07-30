// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use debuggables::{
    has_anonymous_union, rust_keywords, Abstract, AliasedDebuggable, Debuggable, Enum, Exhaustive,
    HasAnonymousUnion, HasNonDebuggable, OptOut, RustKeywords, Union,
};
use existing_rust_debuggable::ExistingRustDebuggable;
use googletest::{expect_eq, gtest};
use static_assertions::assert_not_impl_any;
use std::fmt::Debug;

#[gtest]
fn test_exhaustive() {
    expect_eq!(
        format!(
            "{:?}",
            Exhaustive {
                primitive: 42,
                pointer: std::ptr::null(),
                function: None,
                an_enum: Enum::kOne,
                named_union: Union { i: 42 },
                debuggable: Debuggable::default(),
                aliased_debuggable: AliasedDebuggable::default(),
                rust_debuggable: ExistingRustDebuggable { value: 'a' },
            }
        ),
        "Exhaustive { primitive: 42, pointer: 0x0, function: None, an_enum: Enum(1), named_union: Union { .. }, debuggable: Debuggable, aliased_debuggable: Debuggable, rust_debuggable: ExistingRustDebuggable { value: 'a' } }"
    );
}

#[gtest]
fn test_rust_keywords() {
    expect_eq!(
        format!(
            "{:?}",
            RustKeywords {
                r#type: rust_keywords::Type::kFiber,
                r#use: rust_keywords::Use::kToSleep,
                r#yield: 3.14,
            }
        ),
        "RustKeywords { type: Type(1), use: Use(2), yield: 3.14 }"
    );
}

#[gtest]
fn test_has_anonymous_union() {
    let mut u = HasAnonymousUnion::default();
    u.tag = has_anonymous_union::Tag::kUninit;
    expect_eq!(format!("{u:?}"), "HasAnonymousUnion { tag: Tag(0), .. }");
}

#[gtest]
fn test_has_non_debuggable() {
    expect_eq!(
        format!(
            "{:?}",
            HasNonDebuggable {
                debuggable: Debuggable::default(),
                non_debuggable: OptOut::default(),
            }
        ),
        "HasNonDebuggable { debuggable: Debuggable, .. }"
    );
}

#[gtest]
fn test_abstract() {
    assert_not_impl_any!(Abstract: Debug);
}

#[gtest]
fn test_abstract_opt_in() {
    expect_eq!(
        format!("{:?}", unsafe { &*debuggables::abstract_opt_in_instance() }),
        "AbstractOptIn"
    );
}

#[gtest]
fn test_opt_out() {
    assert_not_impl_any!(OptOut: Debug);
}
