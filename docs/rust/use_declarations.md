# C++ bindings for Rust `use` declarations

WARNING: This page describes an unreleased feature of Crubit.

Crubit supports `use` declarations for functions and types, mapping them to
equivalent `using` declarations in C++.

**Limitations:**

*   The `use` declaration must refer to a function or type.
    *   If it refers to a function, it must not rename the function.
*   The `use` declaration must import exactly one entity per name. For example,
    `pub use m::x;` is supported if `x` refers to a function, or to a type, but
    not if it refers to *both* a function *and* a type.

## Example

Given the following Rust crate:

```live-snippet
cs/file:examples/rust/use_declaration/example.rs content:\bpub\ use\b
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/rust/use_declaration/example_generated.h content:\busing\b
```
