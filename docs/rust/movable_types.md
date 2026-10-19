# Movable types

Rust types do not need to be movable in C++ to receive bindings. If a Rust type
does not logically support a C++ move operation, it still receives bindings, but
its C++ move constructor and move assignment operator are deleted. Such types
can still be used by reference, returned by value, and passed by value when
wrapped in [`rs::Movable<T>`](#non_cpp_movable).

A Rust type is movable in C++ if any of the following hold, checked in order:

1.  [**Trivially move-constructible and destructible**](#trivial): the Rust type
    does not have a
    [destructor](https://doc.rust-lang.org/reference/destructors.html). (It does
    not implement `Drop`, and nor do any of its fields.)
2.  [**Non-trivially move-constructible**](#nontrivial): the Rust type has a
    destructor, but implements `Default` and is `Unpin`.
3.  [**Copyable**](#copyable): the Rust type implements `Clone` (or `Copy`).

The easiest way to ensure your type is useful to end users, even if it is
changed in the future, is to implement `Clone` and `Default`. This makes the
type default-constructible and copyable[^semiregular], as well as efficiently
movable.

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
`Default` and is `Unpin`, then the resulting C++ type will be (non-trivially)
move constructible:

*   Move construction has the same behavior as
    [`std::mem::take()`](https://doc.rust-lang.org/std/mem/fn.take.html): it
    copies the bytes to the new object, as if by a Rust move, and replaces the
    moved-from object with `Default::default()`.
*   Move assignment copies the bytes to the new object, as if by a Rust move,
    and replaces the moved-from object with an unspecified but valid object.

## Copyable {#copyable}

If the Rust type implements `Clone`, then the C++ type will be copyable:

*   Copy construction has the same behavior as `Clone::clone`.
*   Copy assignment has the same behavior as `Clone::clone_from`.

Because the type is copyable, it is also movable, at worst by a copy operation:
if the type has a destructor but does not implement `Default`, C++ move
construction and assignment fall back to the copy operations, and the moved-from
object is left unchanged.

## Non-C++-movable types and `rs::Movable<T>` {#non_cpp_movable}

If a Rust type meets none of the criteria above (for example, a type that
implements `Drop` but implements neither `Default` nor `Clone`), it still
receives C++ bindings, but its C++ move constructor and move assignment operator
are deleted:

```c++
// C++ move operations are unavailable for this type. See
// http://crubit.rs/rust/movable_types for an explanation of Rust types that
// are C++ movable.
MyType(MyType&&) = delete;
MyType& operator=(MyType&&) = delete;
```

Non-C++-movable types can still be used across the language boundary:

*   **By reference or pointer**: `const T&`, `T&`, and `T*` parameters and
    return types work as usual.
*   **Returned by value**: a Rust function returning `T` becomes a C++ function
    returning `T`. The value is constructed directly into the caller's storage,
    so no move is needed. (C++17 guaranteed copy elision lets the caller
    initialize a local from the return value.)
*   **Passed by value**: a Rust function taking `T` by value becomes a C++
    function taking `rs::Movable<T>` (from `crubit/support/movable.h`).
*   **Inside bridged types**: when `T` appears inside a bridged type such as a
    tuple, in either a parameter or a return position, it is wrapped as
    `rs::Movable<T>` (for example, `(T, i32)` becomes
    `std::tuple<rs::Movable<T>, std::int32_t>`).

`rs::Movable<T>` is a C++-movable wrapper around a `T`. It tracks whether its
value has been moved out with an explicit `valueless_after_move()` flag, so
moving it does not require `T` itself to have a moved-from state. It is **not**
layout-compatible with `T`. Its main operations are:

*   `rs::Movable<T>(std::in_place, args...)`: constructs the `T` in place using
    one of `T`'s own C++ constructors (for example, the constructor synthesized
    for a Rust tuple struct).
*   `rs::Movable<T>::TakeFromSlot(std::move(slot))`: takes ownership of a `T`
    that was constructed into a `crubit::Slot<T>` (from
    `crubit/support/internal/slot.h`). Use this to wrap a value returned from
    Rust, since a non-C++-movable `T` cannot be moved into the wrapper.
*   `*m` and `m->`: access the contained `T`.
*   `std::move(m)`: moves the value to the new wrapper and leaves `m` valueless.

For example, given the Rust API:

```rust
pub struct Point { pub x: i32, pub y: i32 }
impl Drop for Point { fn drop(&mut self) { /* ... */ } }

pub fn create(x: i32, y: i32) -> Point { Point { x, y } }
pub fn consume(p: Point) -> i32 { p.x + p.y }
```

Crubit generates `Point create(std::int32_t, std::int32_t)` and
`std::int32_t consume(rs::Movable<Point>)`, which C++ callers can use like so:

```c++
// Returned by value: no move required.
Point p = create(1, 2);

// Passed by value: construct the value into a slot, then wrap it.
crubit::Slot<Point> slot;
new (slot.Get()) Point(create(3, 4));
std::int32_t sum = consume(rs::Movable<Point>::TakeFromSlot(std::move(slot)));
```

If you control the Rust type, it is usually simpler to make it C++-movable by
implementing `Default` or `Clone`, so that callers can write
`consume(create(3, 4))` directly.

## Why can't every type be moved? {#why}

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

## The `rs::Movable` wrapper {#movable_wrapper}

If a Rust type is not C++ movable (has a destructor or drop glue, but does not
implement `Clone` or `Default`), Crubit still generates C++ bindings for it, but
its C++ move constructor and move assignment operator will be `= delete`d:

```c++
// C++ move operations are unavailable for this type.
MyType(MyType&&) = delete;
MyType& operator=(MyType&&) = delete;
```

Because C++ requires moved-from objects to remain valid and destructible, Crubit
cannot generate a native C++ move constructor for such types.

Instead, when a function accepts such a type by value as a parameter, Crubit
wraps the parameter in the
[`rs::Movable<T>`](https://github.com/google/crubit/blob/main/support/movable.h)
wrapper type.

`rs::Movable<T>` makes the type movable in C++ by tracking whether the
underlying value has been moved out, ensuring that the destructor is not run on
moved-from instances:

*   **Accessing members**: Like a smart pointer or `std::optional`, you can
    access the underlying `T`'s fields and methods using `operator*` and
    `operator->` (e.g. `val->method()`, `(*val).field`). Note that
    `rs::Movable<T>` does not provide implicit conversions to `T` or `T&`.
*   **Move semantics**: You can move `rs::Movable<T>` instances with `std::move`
    to pass them by value into functions. Moving relocates the underlying Rust
    object into the target without calling intermediate destructors.
*   **Destruction**: `~Movable()` invokes `T`'s destructor only if the value has
    not been moved out.

### Example

Given the following Rust code:

```rust
pub struct Unmovable(pub i32);
impl Drop for Unmovable {
    fn drop(&mut self) {}
}

pub fn make_unmovable(x: i32) -> Unmovable {
    Unmovable(x)
}

pub fn consume_unmovable(x: Unmovable) {
    // ...
}
```

Crubit generates C++ bindings where `make_unmovable` returns `Unmovable`
directly and `consume_unmovable` accepts `rs::Movable<Unmovable>`:

```c++
Unmovable make_unmovable(::std::int32_t x);
void consume_unmovable(::rs::Movable<Unmovable> x);
```

In C++, you call and manipulate them like this:

```c++
// Receiving an unmovable return value directly into a local variable
// (uses C++17 guaranteed copy elision):
Unmovable val = make_unmovable(42);
std::cout << val.__field0 << "\n";

// Passing to a function accepting rs::Movable<T> by constructing in-place:
consume_unmovable(rs::Movable<Unmovable>(std::in_place, 100));

// Passing an rs::Movable<T> received from another function or parameter:
void forward_unmovable(rs::Movable<Unmovable> movable) {
  // Accessing fields and methods through rs::Movable<T>:
  std::cout << movable->__field0 << "\n";    // operator->
  std::cout << (*movable).__field0 << "\n";  // operator*

  // Passing an rs::Movable<T> by value:
  consume_unmovable(std::move(movable));
}
```

[^semiregular]: The combination of default-constructible and copyable is so
    important for making types useful in C++ that it even has a name:
    ["semiregular"](https://en.cppreference.com/w/cpp/concepts/semiregular)
