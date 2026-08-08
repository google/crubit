# Rust bindings for C++ standard library (`std`) types

When generating Rust bindings for a C++ target, bindings to the C++ standard
library types are used from the `cc_std` crate. This contains a combination of
manually written types and automatically generated bindings.

TODO(b/454337872): The following is stub documentation in the absence of rustdoc
support. Instead of linking to API docs, we link to Rust source files.

## `std::string` {#string}

TODO(b/408961701): Bindings for the real `std::string` type.

`std::string` is only supported as a "bridge type", with runtime conversion.
When a `std::string` is passed or returned by value, it is transformed into a
`cc_std::std::string_wrapper`.

API: support/cc_std_impl/string.rs

## `std::string_view` and `absl::string_view` {#string_view}

C++'s `string_view` type has two analogues in Rust. If no lifetime can be
specified or inferred, or the data may be mutably aliased, then one should use
`cc_std::std::raw_string_view`. This is analogous to a `*const [u8]` in Rust.
Otherwise, `cc_std::std::string_view<'a>`.

API: support/cc_std_impl/string_view.rs

## `std::unique_ptr<T>` {#unique_ptr}

C++'s `unique_ptr<T>` type has two analogues in Rust. If the type has a virtual
destructor, or an overloaded `operator delete`, then it becomes a
`cc_std::std::virtual_unique_ptr<T>`. In particular, polymorphic types will be
stored in a `virtual_unique_ptr<T>`.

Otherwise, when the destructor is directly callable and not virtual, it becomes
a `cc_std::std::unique_ptr<T>`.

For example, a C++ `unique_ptr<int>` is a Rust `unique_ptr<int>`, but a C++
`unique_ptr<ios_base>` would become a Rust `virtual_unique_ptr<ios_base>`.

API: support/cc_std_impl/unique_ptr.rs

See also: crubit.rs/errors/delete

## `std::shared_ptr<T>` {#shared_ptr}

A C++ `shared_ptr<T>` becomes a Rust `cc_std::std::shared_ptr<T>`.

API: support/cc_std_impl/shared_ptr.rs

## `std::vector<T>` {#vector}

A C++ `vector<T>` becomes a Rust `cc_std::std::vector<T>`.

API: support/cc_std_impl/vector.rs

## `std::optional<T>` {#optional}

A C++ `optional<T>` becomes a Rust `cc_std::std::optional<T>`.

API: support/cc_std_impl/optional.rs
