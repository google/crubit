// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub struct GenericStruct<T> {
    pub value: T,
}

pub struct MultiGenericStruct<T, U> {
    pub first: T,
    pub seconds: Vec<U>,
}
