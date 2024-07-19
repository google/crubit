# Attributes to fine-tune generated bindings

Crubit recognizes custom attributes to fine-tune generated bindings.

To use a Crubit attribute, you must add the following to your Rust crate:

```rust

#![feature(register_tool)]
#![register_tool(__crubit)]
```

The format of the Crubit tool attributes is `#[__crubit::annotate(attribute_name="attribute_value")]`.

## `cpp_name`

This attribute overrides the name of the generated C++ bindings. For
example, the following Rust code:

```rust
#[__crubit::annotate(cpp_name="Create")]
pub fn new() -> i32 {...}
```

Will generate the following C++ bindings:

```cpp
std::int32_t Create(); // named `Create` instead of `new`.
```

Currently this attribute works on functions only (See b/349070421).

## `cpp_type`

TODO(b/315382130): Rename this attribute to `cpp_type` and add the doc.
