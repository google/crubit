// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn foo(x: foo::Foo) -> Option<i32> {
    x.foo
}

pub fn bar(x: bar::Bar) -> Option<i32> {
    x.bar
}

pub fn foo_opt() -> Option<foo::SomeStruct> {
    foo::foo_opt()
}

pub fn bar_opt() -> Option<bar::BarStruct> {
    bar::bar_opt()
}
