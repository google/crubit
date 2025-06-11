// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// pathological shadowed names: shadow important modules that the macros use.
mod std {}
mod forward_declare {}

mod test_is_same_0 {
    type _Expected = ::forward_declare::internal::Symbol<"">;
    fn _is_same(x: _Expected) -> ::forward_declare::symbol!("") {
        x
    }
}

mod test_is_same_1 {
    type _Expected = ::forward_declare::internal::Symbol<"x">;
    fn _is_same(x: _Expected) -> ::forward_declare::symbol!("x") {
        x
    }
}

mod test_is_same_3 {
    type _Expected = ::forward_declare::internal::Symbol<"foo">;
    fn _is_same(x: _Expected) -> ::forward_declare::symbol!("foo") {
        x
    }
}
