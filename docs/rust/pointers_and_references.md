# C++ bindings for Rust pointer and reference types

Rust bindings for lifetime-annotated C++ pointers look as follows:

<!-- The contents of the table below are somewhat based on
`test_format_ty_for_cc_successes` from `cc_bindings_from_rs/bindings.rs` -->

Rust API   | C++ bindings
---------- | ------------
`*const T` | `const T*`
`*mut T`   | `T*`

When used as function parameter types or function return types, Rust references
map into the corresponding C++ types as follows:

<!-- The contents of the table below are somewhat based on
`test_format_ty_for_cc_successes` from `cc_bindings_from_rs/bindings.rs` -->

Rust API    | C++ bindings
----------- | ----------------------------------------------------------------
`&'a T`     | `const std::int32_t & [[clang::annotate_type("lifetime", "a")]]`
`&'a mut T` | `std::int32_t & [[clang::annotate_type("lifetime", "a")]]`
`&str`      | TODO(b/262580415): Not supported yet.
`&mut str`  | TODO(b/262580415): Not supported yet.
`&[T]`      | TODO(b/271016831): Not supported yet.
`&mut[T]`   | TODO(b/271016831): Not supported yet.

TODO(b/286299326): Use shorter `$a` syntax in the generated C++.

TODO(b/279913786): Generate `ABSL_ATTRIBUTE_LIFETIME_BOUND` when appropriate.

TODO(b/286256327): Support Rust references in fields and nested types.
