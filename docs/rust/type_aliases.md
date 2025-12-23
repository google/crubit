# C++ bindings for Rust type aliases.

A rust Rust type aliases, such as `pub type X = ...;`, is mapped to the equivalent
C++ type alias, such as `using X = ...;`.

**Limitations:**

*   The type must be a supported type.
*   The alias must not be generic: aliases with generic parameters, such as `pub
    type X<T> = ...`, are not supported.

## Example

Given the following Rust crate:

```
{{ #include ../../examples/rust/type_alias/example.rs }}
```
<!--  content:\bpub\ type\b -->


Crubit will generate the following bindings:

```
{{ #include ../../examples/rust/type_alias/example_generated.h }}
```
<!--  content:\busing\b -->

