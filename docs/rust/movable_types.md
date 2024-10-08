# Movable types

WARNING: This page describes an unreleased feature of Crubit.

Crubit requires types to be "movable" to be passed by value: if a Rust type does
not logically support a C++ move operation, then it can receive bindings, but it
cannot be passed by value.

A Rust type can be made movable in C++ in one of three ways:

1.  [**Copyable**](#copyable): the Rust type implements `Clone`.
2.  [**Trivially move-constructible and destructible**](#trivial): the Rust type
    does not have a
    [destructor](https://doc.rust-lang.org/reference/destructors.html). (It does
    not implement `Drop`, and nor do any of its fields.)
3.  [**Non-trivially move-constructible**](#nontrivial): the Rust type has a
    destructor, but implements `Default`.

The easiest way to ensure your type is useful to end users, even if it is
changed in the future, is to implement `Clone` and `Default`. This makes the
type default-constructible and copyable[^semiregular], as well as efficiently
movable.

## Copyable {#copyable}

If the Rust type implements `Clone`, then the C++ type will be copyable:

*   Copy construction has the same behavior as `Clone::clone`.
*   Copy assignment has the same behavior as `Clone::clone_from`.

Because the type is copyable, it is also movable, at worst by a copy operation.

## Trivially move-constructible and destructible {#trivial}

If no logic occurs during destruction, because the type doesn't implement
`Drop`, and none of its fields do, then the C++ type will be trivially-movable
and trivially-destructible:

*   Move construction and assignment copy the bytes of the object, with the same
    behavior as a Rust move operation.

NOTE: All `Copy` types are guaranteed to be trivially move-constructible and
destructible.

If the Rust type is `Copy`, then the moved-from object is guaranteed to hold its
old value, and be valid for all operations.

Otherwise, the object is only valid for assignment and destruction, and the
behavior of performing any other operation is undefined.

## Non-trivially move-constructible {#nontrivial}

If the Rust type is not trivially movable and destructible, but implements
`Default`, then the resulting C++ type will be (non-trivially) move
constructible:

*   Move construction has the same behavior as
    [`std::mem::take()`](https://doc.rust-lang.org/std/mem/fn.take.html): it
    copies the bytes to the new object, as if by a Rust move, and replaces the
    moved-from object with `Default::default()`.
*   Move assignment copies the bytes to the new object, as if by a Rust move,
    and replaces the moved-from object with an unspecified but valid object.

## Why is this required? {#why}

In general, Crubit needs to be able to move objects as part of the
implementation of pass-by-value, even in C++17, due to platform ABI
restrictions. Even without this requirement, types are not very useful in C++ if
they are not movable.

Unlike Rust, C++ has no "destructive move". There is no way to change an
object's location in memory, only to create a new object with the same value,
and leave behind something in the old (still valid) object. Sometimes, what's
left behind is an identical copy of the object state: this is a copy operation,
implemented by the C++ copy constructor or copy assignment operator. But
sometimes, copying is expensive, and instead what we might leave behind is some
kind of junk value. It still must be a valid object (at least so that its
destructor and assignment operator can be invoked), but it might represent some
invalid or moved-from state.

For example, to "move" a `unique_ptr` (the C++ equivalent of `Box`) from one
variable to another, you copy the bytes, and then replace the old location with
a special null value representing an unoccupied / moved-from `unique_ptr`. This
is why `unique_ptr` **must** be nullable in the C++ type system: otherwise, it
could not be moved!

[^semiregular]: The combination of default-constructible and copyable is so
    important for making types useful in C++ that it even has a
    name:
    ["semiregular"](https://en.cppreference.com/w/cpp/concepts/semiregular)
