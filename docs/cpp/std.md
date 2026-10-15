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

C++'s `std::string` has two representations in Rust:

*   **By value (`cpp_std::string_wrapper`)**: By default, when a `std::string`
    is passed or returned by value, it is bridged into a
    `cpp_std::string_wrapper` (an owned, Rust-movable wrapper that implements
    `Clone` and converts to/from `&str`, `String`, `&[u8]`, and `Vec<u8>`).
*   **By reference, pointer, struct field, or container element
    (`cpp_std::string`)**: In compound positions (such as `const std::string&`,
    `std::string&`, `const std::string*`, struct fields, or
    `std::vector<std::string>`), or when the `layout_compat_string` Crubit
    feature is enabled, `std::string` is bound as the layout-compatible
    `cpp_std::string`.

### Copying and converting `cpp_std::string` {#copying_string}

Because C++ `std::string` is not trivially relocatable across all platforms (for
example, due to Short String Optimization self-pointers), `cpp_std::string` is
`!Unpin` (see crubit.rs/types/non_rust_movable/intro_short) and cannot implement
`Clone` (which returns `Self` by value).

Depending on what you need:

*   **Borrow or copy into a Rust `String`, `&str`, or `Vec<u8>`**:
    *   `s.to_str()` returns `Result<&str, std::str::Utf8Error>`.
    *   `s.to_string()` returns `Result<String, std::str::Utf8Error>`.
    *   `s.display().to_string()` converts to a `String`, replacing invalid
        UTF-8 sequences with the Unicode replacement character.
    *   `s.as_slice()` (or `&*s` via `Deref<Target = [u8]>`) returns `&[u8]`,
        and `s.to_vec()` returns `Vec<u8>`.
*   **Copy-construct a new `cpp_std::string` in place** using `ctor`:

    ```rust
    use ctor::{copy, emplace, CtorNew};

    // Given `s: &cpp_std::string`:
    let copy1 = emplace!(copy(s));
    let copy2 = emplace!(cpp_std::string::ctor_new(s));

    // Given `s: Pin<&mut cpp_std::string>`, reborrow as `&cpp_std::string` with `&*s`:
    let copy3 = emplace!(copy(&*s));
    let copy4 = emplace!(cpp_std::string::ctor_new(&*s));
    ```

*   **Pass to a C++ function taking bridged `std::string` by value**: convert to
    `cpp_std::string_wrapper` with
    `cpp_std::string_wrapper::from(s.as_slice())`.

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

Both types implement `Deref<Target = T>` and `DerefMut<Target = T>` (when `T:
Unpin`). While C++ `std::unique_ptr` is only shallow-const (i.e. `const
std::unique_ptr<T>&` allows mutating the underlying `T`), it is conventionally
treated as deep-const. In order for `&unique_ptr<T>` to be usable at all from
Rust, we treat it as deep-const. C++ code which mutates a value of type `T`
while Rust has obtained a `&T` via `&unique_ptr<T>` -> `&T` deref will result in
undefined behavior.

API: support/cc_std_impl/unique_ptr.rs

See also: crubit.rs/errors/delete

## `std::shared_ptr<T>` {#shared_ptr}

A C++ `std::shared_ptr<T>` becomes a Rust `cpp_std::shared_ptr<T>`. It
implements `Deref<Target = T>`, allowing `&shared_ptr<T>` to be dereferenced to
`&T`.

API: support/cc_std_impl/shared_ptr.rs

## `std::vector<T>` {#vector}

A C++ `std::vector<T>` becomes a Rust `cpp_std::vector<T>`.

API: support/cc_std_impl/vector.rs

## `std::optional<T>` {#optional}

`std::optional<T>` in C++ is bound as `cpp_std::optional<T>` or
`cpp_std::trivial_optional<T>` in Rust. `trivial_optional` is used when `T:
Copy`, which allows `optional` of trivially copyable types to also be trivially
copyable (see the `trivial_optional` docs in the API below). Convert to and from
a Rust `Option<T>` with `From`/`Into`, e.g. `Option::from(opt)` or
`Some(x).into()`.

API: support/cc_std_impl/optional.rs
