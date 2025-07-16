<!-- <internal link> -->

# Non-Rust-Movable Types

WARNING: This is early documentation for an unreleased feature, only currently
available in `:wrapper` mode. The documentation is a WIP.

Many, if not most, types in C++ are not Rust-movable. That is, you cannot
initialize them using `let x = y`, you cannot assign them using `x = y`, and you
cannot otherwise relocate the bits in memory, such as by `std::mem::swap`.

In general, it's better to make these types Rust-movable. See [Rust-movable
classes](classes_and_structs.md#rust_movable) and [Making types
Rust-movable](cookbook.md#rust_movable).

If the class cannot be made Rust-movable, then it will be given a less ergonomic
interface, using the `Ctor` trait.

## What is the `Ctor` trait?

NOTE: The API documentation and source code is at
support/ctor.rs

`Ctor<Output=T, Error=E>` is a trait for implementing **lazily evaluated
values**. These values are constructed in-place, in a C++-compatible way, and
pinned.

However, `Ctor` is not a lazily-initialized value itself. It is a value
initialization procedure, which returns `T` upon success and `E` upon failure.
So a `Ctor` creates a value upon request, which is why we describe it as lazy.

Since exceptions are disabled at Google, we currently only work with
`Error=Infallible`, and for exposition will omit the error type.

Functions accepting and returning a non-Rust movable value in C++ will accept
and return an `impl Ctor` in Rust, as so:

```rust
pub fn accepts_value(x: impl Ctor<Output=CppType, ...>) {...}
pub fn returns() -> impl Ctor<Output=CppType, ...> {...}
```

The easiest way to work with these types in Rust is to box them into a
`Pin<Box<T>>` using `Box::emplace()`. Then they can be passed by value using
`mov!(x)` (similar to `std::move` in C++), and by mutable reference using
`.as_mut()`. For example:

```rust
use ctor::{Emplace, mov};  // gives Box::emplace and mov!().
let mut x /* : Pin<Box<T>> */ = Box::emplace(returns());

takes_mutable_reference(x.as_mut());
takes_const_reference(&x);

accepts_value(mov!(x));
```

NOTE: The call to `returns()` does not construct a value of type `CppType`, it
merely returns a `Ctor<Output=CppType>`, which is a procedure to lazily create
such a value. A `CppType` is created only when the `Ctor` is executed as part of
the `Box::emplace(...)` expression in the caller.

If you happen to be *directly* passing a return value into a parameter, you can
avoid the intermediate boxed value:

```rust
accepts(returns());
```

If you want to avoid heap allocations in general, you will need to use the more
advanced features of `ctor.rs`.

<!-- TODO(b/432107690): Move advanced docs to a sibling file. -->
