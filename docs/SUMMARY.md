# Summary

<!-- mdformat-off -->

[Home](./index.md)
[Are We Crubit Yet?](./overview/status.md) <!-- mdformat-on -->

# Types

-   [Overview](./types/index.md)
-   [Primitive Types](./types/primitive.md)
-   [Pointer Types](./types/pointer.md)
-   [Callbacks (`absl::AnyInvocable` and `Fn`/`FnMut`/`FnOnce`)](./types/callbacks.md)
-   [`absl::Status`](./types/absl_status.md)
-   [Protobuf](./types/protobuf.md)

# C++ Bindings for Rust Libraries

-   [Overview](./rust/index.md)
-   [Building](./rust/building.md)
-   [Functions](./rust/functions.md)
-   [Callbacks (`Fn`, `FnMut`, `FnOnce`)](./rust/callbacks.md)
-   [Structs](./rust/structs.md)
-   [Enums](./rust/enums.md)
-   [Cpp Enums](./rust/cpp_enums.md)
-   [Traits](./rust/traits.md)
-   [Type Aliases](./rust/type_aliases.md)
-   [`use` Declarations](./rust/use_declarations.md)
-   [Standard Library Types](./rust/std.md)

--------------------------------------------------------------------------------

-   [Movable Types](./rust/movable_types.md)

# Rust Bindings for C++ Libraries

-   [Overview](./cpp/index.md)
-   [Building](./cpp/building.md)
-   [Cookbook](./cpp/cookbook.md)
-   [Best Practices](./cpp/best_practices.md)
-   [Customizing](./cpp/customizing.md)

--------------------------------------------------------------------------------

-   [Functions](./cpp/functions.md)
-   [Callbacks (`absl::AnyInvocable`)](./cpp/callbacks.md)
-   [Classes and Structs](./cpp/classes_and_structs.md)
-   [Static Data Members](./cpp/static_data_members.md)
-   [References and Lifetimes](./cpp/references_and_lifetimes.md)
-   [Enums](./cpp/enums.md)
-   [Inline C++](./cpp/inline_cpp.md)
-   [Standard Library Types](./cpp/std.md)

# Errors

-   [Overview](./errors/index.md)
-   [Unsupported types](./errors/unsupported_type.md)
-   [Unknown targets](./errors/unknown_target.md)
-   [Nested types](./errors/nested_type.md)
-   [Visibility](./errors/visibility.md)
-   [Unknown attributes](./errors/unknown_attribute.md)
-   [Bridge types as struct fields](./errors/bridge_field.md)
-   [Compound types containing bridge types](./errors/bridge_compound_type.md)
-   [The `Delete` trait](./errors/delete.md)

# Integration notes

-   [Build System Integrations](./overview/build_systems.md)
    -   [Bazel](./overview/bazel.md)
    -   [CMake](./overview/cmake.md)
-   [Cargo build](./overview/cargo_build.md)
    -   [Protobuf in the Cargo build](./overview/cargo_build_protobuf.md)
-   [Limitations](./overview/limits.md)
-   [Required unstable features of Rust](./overview/unstable_features.md)

# Design

-   [High-level design of C++/Rust interop](./design/design.md)
-   [Lifetime Annotations for C++](./design/lifetime_annotations_cpp.md)
-   [Static Analysis for C++ Lifetimes](./design/lifetimes_static_analysis.md)
-   [Struct Layout](./design/struct_layout.md)
-   [Thunks for class template member functions](./design/thunks_for_class_template_member_functions.md)
-   [`Unpin` for C++ Types](./design/unpin.md)
-   [Problem Description: forward-declared types](./design/forward_declarations.md)
