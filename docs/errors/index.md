<!-- <internal link> -->

# Errors

When Crubit can't generate bindings for an item, it usually leaves a comment in
the generated bindings explaining why, often with a link to one of the pages
below. To find the comment, look at the generated bindings for [C++
libraries](../cpp/index.md#examine) or [Rust
libraries](../rust/index.md#examine).

*   [Unsupported types](unsupported_type.md): a type that Crubit can't bind,
    which also prevents bindings for every function that uses it.
*   [Nested types](nested_type.md): C++ types nested inside other types, when
    the snake_case module Crubit generates for them collides with another name.
*   [Visibility](visibility.md): types whose bindings are `pub(crate)` and so
    aren't available outside their own library.
*   [Unknown attributes](unknown_attribute.md): C++ or Rust attributes that
    Crubit doesn't understand, which cause it to skip the item.
*   [Bridge types as struct fields](bridge_field.md): struct or union fields
    whose type is a bridge type.
*   [Compound types containing bridge types](bridge_compound_type.md): bridge
    types behind a pointer or inside a type like `std::vector`.
*   [The `Delete` trait](delete.md): using `virtual_unique_ptr<T>` with a type
    that has neither a virtual destructor nor a custom `operator delete`.
