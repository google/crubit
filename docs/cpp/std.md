# Rust bindings for C++ standard library (`std`) types

When generating Rust bindings for a C++ target, bindings to the C++ standard
library types are provided by the `cpp_std` support library (which re-exports
the legacy `cc_std::std::*`). This contains a combination of manually written
types and automatically generated bindings.

You can then use standard library types directly via `cpp_std::{sometype}`,
e.g., `cpp_std::unique_ptr<T>`.

TODO(b/454337872): The following is stub documentation in the absence of rustdoc
support. Instead of linking to API docs, we link to Rust source files.

## `std::string` {#string}

TODO(b/408961701): Bindings for the real `std::string` type.

`std::string` is only supported as a "bridge type", with runtime conversion.
When a `std::string` is passed or returned by value, it is transformed into a
`cpp_std::string_wrapper`.

API: support/cc_std_impl/string.rs

## `std::string_view` and `absl::string_view` {#string_view}

C++'s `std::string_view` type has two analogues in Rust. If no lifetime can be
specified or inferred, or the data may be mutably aliased, then one should use
`cpp_std::raw_string_view`. This is analogous to a `*const [u8]` in Rust.
Otherwise, `cpp_std::string_view<'a>`.

API: support/cc_std_impl/string_view.rs

## `std::unique_ptr<T>` {#unique_ptr}

C++'s `std::unique_ptr<T>` type has two analogues in Rust. If the type has a
virtual destructor, or an overloaded `operator delete`, then it becomes a
`cpp_std::virtual_unique_ptr<T>`. In particular, polymorphic types will be
stored in a `cpp_std::virtual_unique_ptr<T>`.

Otherwise, when the destructor is directly callable and not virtual, it becomes
a `cpp_std::unique_ptr<T>`.

For example, a C++ `std::unique_ptr<int>` is a Rust `cpp_std::unique_ptr<int>`,
but a C++ `std::unique_ptr<ios_base>` would become a Rust
`cpp_std::virtual_unique_ptr<ios_base>`.

Both types implement `Deref<Target = T>` and `DerefMut<Target = T>` (when `T: Unpin`).
While C++ `std::unique_ptr` is only shallow-const (i.e. `const std::unique_ptr<T>&` allows
mutating the underlying `T`), it is conventionally treated as deep-const. In order for
`&unique_ptr<T>` to be usable at all from Rust, we treat it as deep-const. C++ code which
mutates a value of type `T` while Rust has obtained a `&T` via `&unique_ptr<T>` -> `&T`
deref will result in undefined behavior.

API: support/cc_std_impl/unique_ptr.rs

See also: crubit.rs/errors/delete

## `std::shared_ptr<T>` {#shared_ptr}

A C++ `std::shared_ptr<T>` becomes a Rust `cpp_std::shared_ptr<T>`. It implements
`Deref<Target = T>`, allowing `&shared_ptr<T>` to be dereferenced to `&T`.

API: support/cc_std_impl/shared_ptr.rs

## `std::vector<T>` {#vector}

A C++ `std::vector<T>` becomes a Rust `cpp_std::vector<T>`.

API: support/cc_std_impl/vector.rs

## `std::optional<T>` {#optional}

A C++ `std::optional<T>` becomes a Rust `cpp_std::optional<T>`.

API: support/cc_std_impl/optional.rs
