# Attributes to fine-tune generated bindings

Crubit offers attributes to allow users to customize the functionality of the
generated bindings, such as renaming a type or function or converting Rust
values into a particular C++ type.

To use these attributes, add `//support:crubit_annotate` to
your `rust_library`'s `proc_macro_deps` (not `deps`, since `crubit_annotate` is
a procedural macro crate). Specific documentation for the attributes can be seen
in the docs for that library.
