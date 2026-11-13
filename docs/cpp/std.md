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

When passed or returned directly by value, `std::string` is bridged to
`cpp_std::string_wrapper` unless the
`//features:layout_compat_string` feature flag is enabled
(b/408961701). References (`const std::string&`, `std::string&`), struct fields,
and layout-compatible generic containers (such as `std::optional<std::string>`
and `absl::StatusOr<std::string>`) use the layout-compatible, `!Unpin`
`cpp_std::string` type.

Both `cpp_std::string` and `cpp_std::string_wrapper` convert to
`cpp_std::string_view` via `From<&string>` and `From<&string_wrapper>`. In the
reverse direction, `string_wrapper` implements `From<string_view>`, and `string`
implements `ctor::CtorNew<string_view>` (and can be cloned in place via
`ctor::emplace!(ctor::copy(s))`).

API: support/cc_std_impl/string.rs

## `std::string_view` and `absl::string_view` {#string_view}

C++'s `std::string_view` type has two analogues in Rust. If no lifetime can be
specified or inferred, or the data may be mutably aliased, then one should use
`cpp_std::raw_string_view`. This is analogous to a `*const [u8]` in Rust.
Otherwise, `cpp_std::string_view<'a>`.

API: support/cc_std_impl/string_view.rs

## `std::unique_ptr<T>` {#unique_ptr}

C++'s `std::unique_ptr<T>` type has two non-null analogues in Rust. If the type
has a virtual destructor, or an overloaded `operator delete`, then it becomes a
`cpp_std::virtual_unique_ptr<T>`. In particular, polymorphic types will be
stored in a `cpp_std::virtual_unique_ptr<T>`.

Otherwise, when the destructor is directly callable and not virtual, it becomes
a `cpp_std::unique_ptr<T>`.

When a `std::unique_ptr<T>` is known to be non-null (for example, annotated with
`absl_nonnull` or `_Nonnull`, or under `ABSL_POINTERS_DEFAULT_NONNULL`), Crubit
maps it directly to `cpp_std::unique_ptr<T>` or
`cpp_std::virtual_unique_ptr<T>`. When it may be null (unannotated or
`absl_nullable`), Crubit wraps it in `cpp_std::Nullable<Ptr>`. Convert either to
`Option<Ptr>` using `cpp_std::OptionLike` (`into_option`, `as_option`,
`as_option_mut`), which is implemented on both `Nullable<Ptr>` and bare `Ptr` so
callers keep compiling when nullability annotations are added.

Both `unique_ptr<T>` and `virtual_unique_ptr<T>` implement `Deref<Target = T>`
and `DerefMut<Target = T>` (when `T: Unpin`). To heap-construct a
non-Rust-movable (`!Unpin`) `T` in place from a `ctor::Ctor`, use
`cpp_std::unique_ptr::emplace` (or `try_emplace`). While C++ `std::unique_ptr`
is only shallow-const (i.e. `const std::unique_ptr<T>&` allows mutating the
underlying `T`), it is conventionally treated as deep-const. In order for
`&unique_ptr<T>` to be usable at all from Rust, we treat it as deep-const. C++
code which mutates a value of type `T` while Rust has obtained a `&T` via
`&unique_ptr<T>` -\> `&T` deref will result in undefined behavior.

API: support/cc_std_impl/unique_ptr.rs,
support/cc_std_impl/nullable.rs

See also: crubit.rs/errors/delete

## `std::shared_ptr<T>` {#shared_ptr}

A non-null C++ `std::shared_ptr<T>` (`absl_nonnull` or `_Nonnull`) becomes a
Rust `cpp_std::shared_ptr<T>`, while an unannotated or `absl_nullable`
`std::shared_ptr<T>` becomes `cpp_std::Nullable<cpp_std::shared_ptr<T>>`
(convertible to `Option` via `cpp_std::OptionLike`). `cpp_std::shared_ptr<T>`
implements `Deref<Target = T>`, allowing `&shared_ptr<T>` to be dereferenced to
`&T`, and provides `shared_ptr::emplace` (and `try_emplace`) to construct
`!Unpin` pointees in place from a `ctor::Ctor`.

API: support/cc_std_impl/shared_ptr.rs

## `std::vector<T>` {#vector}

A C++ `std::vector<T>` becomes a Rust `cpp_std::vector<T>`.

API: support/cc_std_impl/vector.rs

## `std::optional<T>` {#optional}

`std::optional<T>` in C++ is bound as `cpp_std::optional<T>` or
`cpp_std::trivial_optional<T>` in Rust. `trivial_optional` is used when
`T: Copy`, which allows `optional` of trivially copyable types to also be
trivially copyable (see the `trivial_optional` docs in the API below). Convert
to and from a Rust `Option<T>` with `.into_option()` or `From`/`Into`, e.g.
`opt.into_option()`, `Option::from(opt)`, or `Some(x).into()`. When `T` is
`!Unpin`, construct `cpp_std::optional<T>` in place via
`ctor::CtorNew<Option<A>>` (`optional::<T>::ctor_new(opt)`) or
`optional::ctor_from_option`.

API: support/cc_std_impl/optional.rs

## Unsupported `std` templates

Concrete instantiations of unsupported `std` templates still receive opaque
bindings you can pass around (by `Ctor!` or `Pin`) but not inspect. For example,
a C++ function returning `std::map<int, Reading>` is not rejected; Crubit binds
the concrete instantiation as an opaque, non-Rust-movable type:

```rust
pub fn LatestReadings() -> ::ctor::Ctor![crate::__CcTemplateInstNSt3__u3mapIi…];
```

Member types like `value_compare` are reported as
`// error: … could not be bound`. Supported `std` types (`string_view`,
`unique_ptr`, `vector`, `optional`, etc.) get ergonomic `cpp_std` mappings. Use
`--config=crubit-genfiles` to see which you got.
