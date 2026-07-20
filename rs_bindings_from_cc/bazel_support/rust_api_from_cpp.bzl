# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""
Generates Rust bindings for a C++ target such as a `cc_library`.

This acts like a `rust_library`, and may be listed in `deps` of a rust rule.
The C++ target must still enable bindings generation via `aspect_hints`.

By default, `bazel build` will compile the bindings.
You can request bindings generation only via `--output_groups=sources`.
"""
