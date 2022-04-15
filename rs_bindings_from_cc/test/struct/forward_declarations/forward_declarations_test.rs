// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use ctor::CtorNew as _;
use forward_declare::{forward_declare, symbol, IncompleteCast};
use std::pin::Pin;

// Rust user forward declarations.
forward_declare!(pub IncompleteUnpinStruct = symbol!("UnpinStruct"));
forward_declare!(pub IncompleteNonunpinStruct = symbol!("NonunpinStruct"));

#[test]
fn test_unpin_struct() {
    let mut s = definition::UnpinStruct::default();
    let incomplete_s: &mut IncompleteUnpinStruct = (&mut s).incomplete_cast();
    definition::WriteCompleteUnpinStruct(incomplete_s.incomplete_cast(), 42);
    assert_eq!(definition::ReadCompleteUnpinStruct(incomplete_s.incomplete_cast()), 42);
}

#[test]
fn test_nonunpin_struct() {
    ctor::emplace! {
      let mut s = definition::NonunpinStruct::ctor_new(());
    }
    let mut incomplete_s: Pin<&mut IncompleteNonunpinStruct> = s.as_mut().incomplete_cast();
    definition::WriteCompleteNonunpinStruct(incomplete_s.as_mut().incomplete_cast(), 42);
    assert_eq!(definition::ReadCompleteNonunpinStruct((&*incomplete_s).incomplete_cast()), 42);
}
