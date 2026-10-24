# Box types

Rust's owning box types map to C++ class templates in `rs`:

Rust          | C++               | Header
------------- | ----------------- | -------------------------------
`Box<T>`      | `rs::Box<T>`      | `crubit/support/rs/box.h`
`Pin<Box<T>>` | `rs::PinBox<T>`   | `crubit/support/rs/pin_box.h`
`Box<[T]>`    | `rs::SliceBox<T>` | `crubit/support/rs/slice_box.h`
`Box<str>`    | `rs::StrBox`      | `crubit/support/rs/str_box.h`

These are ordinary C++ templates: `T` can be any complete C++ or Rust type.
`Box<T>` and `PinBox<T>` have the layout of a single non-null `T*`;
`SliceBox<T>` and `StrBox` have the layout of a Rust fat pointer (data pointer
and length), the same as [`rs::SliceRef<T>`](../rust/functions.md) and
`rs::StrRef`.

The mapping is bidirectional: a Rust function taking or returning `Box<T>` gets
C++ bindings using `rs::Box<T>`, and a C++ function using `rs::Box<T>` gets Rust
bindings using `Box<T>`. The types can be used in fields, as parameters and
return values, and behind pointers and references.

## Ownership and allocation

A box always owns memory allocated by the **Rust global allocator**, regardless
of which language created it. Use `rs::make_box` (or `rs::make_pin_box`) to
allocate and construct the pointee in place, analogous to `std::make_unique`:

```c++
#include "crubit/support/rs/box.h"

rs::Box<MyStruct> b = rs::make_box<MyStruct>(/*args...*/);
b->field = 1;
MyStruct& inner = *b;
// `~Box()` destroys `*b` and returns the memory to the Rust allocator.
```

`rs::make_box` and `rs::make_pin_box` also accept an `rs::Movable<T>` directly
(deducing `T`), so a non-C++-movable Rust value returned by value can be boxed
without spelling `<T>`:

```c++
auto b = rs::make_box(return_non_cpp_movable_rust_type());
b->method();
```

`rs::Movable<rs::Box<T>>` and `rs::Movable<rs::PinBox<T>>` are pointer-sized
partial specializations that forward `operator->`, `operator*`, and `get()`
directly to the pointee `T` (like `std::unique_ptr<T>`), so
`auto b = rs::make_box<MyStruct>(...)` and `rs::make_box<MyStruct>(...)->field`
do not require double-dereferencing. When an lvalue reference to the
`rs::Box<T>` or `rs::PinBox<T>` itself is needed (for example to call a Rust
`fn(&Box<T>)`), use `b.as_box()` or `b.as_pin_box()`.

Convenience constructors `Box(T&&)`, `Box(const T&)`, `Box(rs::Movable<T>)`, and
`Box(std::in_place, /*args...*/)` are also available. `SliceBox<T>` can be built
from a `std::span<const T>`, an initializer list, or a count and fill value.
`StrBox` follows the same UTF-8 conversion rules as `rs::StrRef`: its
constructor accepts `rs::StrRef` (including `consteval`-checked string literals
and `constexpr` `std::string_view`s), while runtime strings use the explicit
fallible `rs::StrBox::FromUtf8(sv)` (returning
`std::optional<rs::Movable<rs::StrBox>>`) or
`rs::StrBox::FromUtf8Unchecked(sv)`.

`unsafe_from_raw` adopts a pointer that was allocated with `crubit_alloc` with
the size and alignment of `T`; it is primarily for code that already interacts
with the allocator shims directly.

## Not C++-movable {#not_movable}

Rust boxes have no null or moved-from state, so the C++ types delete their copy
and move constructors and assignment operators. They can still be:

*   **returned by value**, using C++17 guaranteed copy elision:

    ```c++
    rs::Box<int> make() { return rs::make_box<int>(42); }
    ```

*   **destructively relocated** via the `crubit::UnsafeRelocateTag` constructor,
    which is what `rs::Movable<T>` and Crubit's generated thunks use.

*   **passed by value to Rust** through
    [`rs::Movable<rs::Box<T>>`](../rust/movable_types.md#movable_wrapper), which
    is what `rs::make_box` and `rs::make_pin_box` return and how
    `cpp_api_from_rust` spells by-value `Box<T>` parameters:

    ```c++
    // Rust: pub fn consume(b: Box<i32>)
    consume(rs::make_box<int32_t>(42));
    ```

See [Movable types](../rust/movable_types.md) for the general rules.

## Inside `Option`

An `Option<Box<T>>` (as well as options of pinned or slice boxes) is represented
as `rs::Option<rs::Box<T>>`. Because these boxes use a non-null pointer
representation, the `Option` takes advantage of Rust's niche layout
optimization: the `rs::Option` is pointer-sized and layout-compatible with
`Option<Box<T>>`, with the `None` state represented as a null pointer. The same
applies for other non-movable payloads that fall back to `rs::Option` instead of
`std::optional`.

## `Box<T>` requires `T` to be Rust-movable {#pinbox}

In Rust, the contents of a `Box<T>` can be moved out by value (for example with
`*b`), which relocates the `T` by copying its bytes. That is only sound if `T`
is Rust-movable, i.e. `Unpin`. A C++ type whose move constructor is non-trivial
or deleted is [not Rust-movable](non_rust_movable/intro_short.md): Rust bindings
for it are `!Unpin`, and relocating it by `memcpy` would be undefined behavior.

For that reason, `rust_api_from_cpp` **refuses to generate bindings** for a C++
API that uses `rs::Box<T>` or `rs::SliceBox<T>` with a `T` that is not
Rust-movable. Use `rs::PinBox<T>` instead: it maps to `Pin<Box<T>>`, which
guarantees that Rust never moves the pointee. In C++, `PinBox<T>` offers the
same `get()`, `operator*`, and `operator->` accessors as `Box<T>`; pinning only
restricts what the Rust side may do.

```c++
// `NonTrivial` has a user-defined move constructor, so it is `!Unpin` in Rust.
rs::Box<NonTrivial> Rejected();     // error: use rs::PinBox<NonTrivial>
rs::PinBox<NonTrivial> Accepted();  // Rust: fn Accepted() -> Pin<Box<NonTrivial>>
```

`cpp_api_from_rust` does not need this restriction: a Rust `Box<T>` of a
`!Unpin` Rust type is still a valid `Box<T>`, and the C++ bindings for `T` are
not C++-movable, so C++ cannot move the pointee either.

For the background on how Crubit decides whether a type is Rust-movable, see
[Unpin](../design/unpin.md) and
[trivially relocatable types](../cpp/classes_and_structs.md#trivially_relocatable).
