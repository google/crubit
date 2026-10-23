# Movable types

Crubit requires types to be "movable" to be passed directly by value: if a Rust
type does not logically support a C++ move operation, then it can receive
bindings, but it cannot be passed directly by value (it is passed via the
[`rs::Movable`](#movable_wrapper) wrapper type).

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
