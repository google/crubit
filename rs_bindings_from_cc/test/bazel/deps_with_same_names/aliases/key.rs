// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate std;

// aliased_rust_foo is the rust_foo:foo (rust_library)
// aliased_rust_foo_sub is the deps/foo:foo (rust_library)
// aliased_foo is the cc_foo bindings (aliased from cc_library target)
// aliased_bar is the cc_bar bindings (aliased from rust_api_from_cpp target)

pub fn key_func() -> i32 {
    aliased_rust_foo::rust_foo_func()
        + aliased_rust_foo_sub::rust_foo_sub_func()
        + aliased_foo::cc_foo_func()
        + aliased_bar::cc_bar_func()
}
