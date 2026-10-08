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

An existing `std::string_view` (or `absl::string_view`) can be converted to a
Rust `String` using `rs_std::StrRef::FromUtf8`:

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

An instance of `String` can be converted back to a `std::string` by calling
`.as_str()` and using the resulting `rs_str::StrRef` to construct a
`std::string`:

```c++
std::string s2(s.as_str());
EXPECT_EQUAL(s2, "hello, world");
```

`rs_std::StrRef` supports implicit conversion to `std::string_view` to make this
work.

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

## `Vec`

The Rust `Vec<T>` generic receives C++ bindings as `rs_std::Vec<T>`, so long as
`T` is a type supported by Crubit. Each `rs_std::Vec<T>` is a generated
specialization that inherits its public API from the common `VecBase<T>` base
class in support/rs_std/vec.h. The API is modeled on
[`std::vector`](https://en.cppreference.com/cpp/container/vector), with a few
differences noted below.

`rs_std::Vec<T>` is a real Rust `Vec` under the hood: it has the same layout,
and its buffer is allocated and freed through Rust's global allocator. A `Vec`
created in C++ can therefore be handed to Rust (and vice versa) without any
copying or conversion.

### Element access and iteration

`rs_std::Vec<T>` provides the usual contiguous-container accessors:

*   `size()`, `capacity()`, `empty()`
*   `data()` (returns a pointer to the contiguous element buffer)
*   `operator[]`, `front()`, `back()` -- these are **bounds-checked** and abort
    the process on misuse (e.g. an out-of-range index or `front()` on an empty
    vector), mirroring the panicking behavior of Rust's `Vec`.
*   `begin()` / `end()`, `cbegin()` / `cend()`, `rbegin()` / `rend()`,
    `crbegin()` / `crend()`

Iterators are raw pointers, so `rs_std::Vec<T>` works with range-based `for`
loops and standard algorithms, and satisfies `std::ranges::contiguous_range`.
This also makes it implicitly convertible to `std::span<const T>` /
`std::span<T>` and, by extension, to `rs_std::SliceRef<const T>` /
`rs_std::SliceRef<T>`:

```c++
rs_std::Vec<int32_t> v = my_crate::return_vec();
for (int32_t& x : v) {
  x += 1;
}
int32_t sum = std::accumulate(v.begin(), v.end(), 0);

std::span<const int32_t> span = v;
rs_std::SliceRef<const int32_t> slice = v;
```

### Modifying a `Vec` from C++

Elements can be added, removed, and constructed in place from C++:

```c++
rs_std::Vec<int32_t> v;
v.reserve(3);         // Total capacity, like std::vector::reserve.
v.push_back(1);
v.push_back(2);
v.emplace_back(3);    // Constructs in place; returns a T&.
v.insert(0, 0);       // Inserts at *index* 0, shifting the rest right.
v.pop_back();         // Removes and destroys the last element.
v.clear();            // Destroys all elements but keeps the buffer.
```

Differences from `std::vector` worth knowing about:

*   `insert(index, value)` takes an index rather than an iterator (as in Rust's
    `Vec::insert`).
*   `reserve(new_cap)` has `std::vector::reserve` semantics: it ensures the
    *total* capacity is at least `new_cap`. To request room for `n` *additional*
    elements (Rust's `Vec::reserve(additional)` semantics), use
    `reserve_additional_capacity(n)` instead.
*   Only the operations listed above are currently supported. In particular,
    there is no `erase`, `resize`, or range `insert`. For other operations,
    write a small Rust helper function and call it through its Crubit bindings.

## Availability of generic `rs_std` specializations

Generic standard library types such as `rs_std::Option<T>`,
`rs_std::Result<T, E>`, `rs_std::Tuple<Ts...>`, and `rs_std::Vec<T>` are only
defined for type arguments that are actually used in a Crubit-bound Rust API.
The support headers (`option.h`, `result.h`, `tuple.h`, `vec.h`) provide only
the generic base implementations (`OptionBase`, `ResultBase`, `VecBase`, etc.)
and leave the primary template uninstantiable; the concrete specialization for a
given set of type arguments is emitted into the generated header of a crate
whose public API mentions that instantiation. Attempting to use one of these
templates with type arguments that do not appear in any of the Rust APIs you
depend on is a compile-time error.
