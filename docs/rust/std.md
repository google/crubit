# C++ bindings for Rust standard library (`std`) types

When generating C++ bindings for a Rust crate (via `cpp_api_from_rust`) bindings
to the Rust standard library types (`std`, `alloc`, and `core`) are
automatically generated for those types under the namespaces `rs::std`,
`rs::alloc`, and `rs::core`.

## Working with `String` in C++ {#string}

Rust's `String` becomes `rs::std::string::String` in C++ (also spelled
`rs::alloc::string::String`). It can be constructed from a C++ string literal:

```c++
rs::std::string::String s("hello, world!")
```

An existing `std::string_view` (or `absl::string_view`) can be converted to a Rust `String` using `rs_std::StrRef::FromUtf8`:

```c++
void AcceptsStringView(std::string_view view) {
  std::optional<rs_std::StrRef> str_ref = rs_std::StrRef::FromUtf8(view);
  if (!str_ref.has_value()) {
    return;
  }
  rs::std::string::String s(*str_ref);
  // Continue on with using your Rust String...
}
```

An instance of `String` can be converted back to a `std::string` by calling `.as_str()` and using the resulting `rs_str::StrRef` to construct a `std::string`:

```c++
std::string s2(s.as_str());
EXPECT_EQUAL(s2, "hello, world");
```

`rs_std::StrRef` supports implicit conversion to `std::string_view` to make this work.

A Rust method that takes an `&mut String` such as:

```rust
pub fn append_to_rust_string(val: &mut String, s: &str) {
    val.push_str(s);
}
```

becomes a C++ method taking a reference to a `rs::std::string::String`:

```c++
// You could just as easily call `push_str` directly. It receives Crubit
// bindings. We use a wrapper method here for expository purposes.
void append_to_rust_string(rs::std::string::String& val, rs_std::StrRef s);
```

Allowing for C++ to call it:

```c++
append_to_rust_string(s, " I'm a neat addition");
```

## `Result`

The Rust `Result<T, E>` generic receives Rust bindings as `rs_std::Result<T,
E>`, so long as both `T` and `E` are non-ZST types supported by Crubit.

`rs_std::Result<T, E>` has a similar API to
[`std::expected`](https://en.cppreference.com/cpp/utility/expected).
(Alternatively, it has a similar API to
[`std::optional`](https://en.cppreference.com/cpp/utility/optional), but with an
additional error payload.) If `has_value()` returns `false`, then `error()` will
return a reference to the error payload of type `E`.

```c++
if (myresult.has_value()) {
  Foo(*myresult);
} else {
  Foo(myresult.error());
}
```

A more complete description of the API is in the common `ResultBase` public base
class: support/rs_std/result.h

## `Option`

The Rust `Option<T>` generic receives Rust bindings as `rs_std::Option<T>`, so
long as `T` is a non-ZST type supported by Crubit. It has a similar API to
[`std::optional`](https://en.cppreference.com/cpp/utility/optional), and
implicitly converts to and from `std::optional`.

```c++
if (myoption.has_value()) {
  Foo(*myoption);
}
```

A more complete description of the API is in the common `OptionBase` public base
class: support/rs_std/option.h
