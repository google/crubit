// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Controls whether `generate_function` generates a static method, an instance method,
/// or a free function.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StaticMethodMode {
    /// Used when generating methods for `rs_std::impl` - these should be static
    /// methods even though the trait function has a `self` parameter.
    ForceStaticMethod,

    /// Used in other cases - whether to use a `static` method or not depends
    /// on whether the function has a `self` parameter or not.
    Infer,
}
