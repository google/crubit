# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""A list of feature flags that goes out with the release.

(Note: this is only relevant for codebases that separately release Crubit
and its internal runtime libraries, while using the support libraries at
head.)

To release a change to a Rust support library in concert with the release, add a new string to this
list, e.g. `"my_feature"`. In a Rust target that sets `crate_features=RUST_CRATE_FEATURES`, use
`#[cfg(feature="my_feature")]`. In testing at head, and in the next release, that feature will be
active. In the old release, it will not be active. This works because `rust_config.bzl` itself is
part of the release.
"""
RUST_CRATE_FEATURES = []
