# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""
Generates Rust bindings for a C++ target such as a `cc_library`.

This acts like a `rust_library`, and may be listed in `deps` of a rust rule.

The C++ target must enable bindings generation by adding the automatically
generated `<name>.hint` target to its `aspect_hints` attribute.

Example:
```python
cc_library(
    name = "foo",
    aspect_hints = [":foo_rust.hint"],
)

rust_api_from_cpp(
    name = "foo_rust",
    cpp_target = ":foo",
)
```

By default, `bazel build` will compile the bindings.
You can request bindings generation only via `--output_groups=sources`.
"""
