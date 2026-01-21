# Non-Rust-Movable Types

This is an in-depth exploration of Crubit's support for non-Rust movable types.
For a quick introduction, see crubit.rs/types/non_rust_movable/intro_short

## Overview {#overview}

`ctor` is an attempt at a satisfactory solution to representing
**non-Rust-movable C++ objects in Rust**. We cover how to store and manipulate
non-Rust-movable C++ objects as **local variables** (as well as temporaries), as
**heap-allocated values**, and as **by-value struct data members**.

### Objective {#objective}

If our C++ codebase were to be ported to Rust, it would be ideal if any C++
function or type had an equivalent Rust function or type which does the same
thing: takes the same parameters, stores the same values, with the same
performance characteristics and safety tradeoffs.

However, C++ has types which at first seem impossible to represent in Rust:
types which have nontrivial move/copy constructors, and can't be relocated via
memcpy. For example, if a struct has a self-referential pointer, this is fine in
C++: the move constructor will re-point the pointer to its new target location,
and it will never be invalid. But in Rust, the built-in "move" operation (and
swap operation, and so on) does not run a user-provided move constructor, but
instead simply runs `memcpy`, performing a relocation. Since a self-referential
type is **not Rust-movable**, Rust code cannot be allowed to access it by value
like this.

The traditional approach to solving this is to box a non-Rust-movable value in a
heap-allocated pointer: the pointer is relocatable, even if its target is not.
If we forbid dereferencing the pointer in safe code (using `Pin`, described
[below](#introduction-pinned-pointers)), then we're good, and we're back to
having basically normal Rust values.

`ctor` extends the approach to storing not just in a heap-allocated pointer, but
by value in all the usual places. This allows *any* C++ function or type to be
rewritten in equivalent Rust code, even if it deals with by-value
non-Rust-movable types.

## Introduction: pinned pointers {#introduction-pinned-pointers}

In safe Rust code, we can expect to encounter a non-Rust-movable C++ type `T`
through the following three families of Rust types:

*   **`Pin<Box<T>>`**: equivalent to a non-null C++ `unique_ptr`.
    *   As well as `Pin<Arc<T>>` (`shared_ptr`), `Pin<Rc<T>>` (non-atomic
        `shared_ptr`), etc.
*   **`Pin<&mut T>`**: equivalent to a C++ non-const reference.
*   **`&T`**: equivalent to a C++ const reference.

Safe Rust code cannot ever directly access a C++ value which is not
Rust-movable, because then it would be able to move it, causing UB! Instead, it
is safe only to deal indirectly with such values behind a reference. If the
reference is mutable, it must be wrapped behind a `Pin`, which makes it
impossible to access the underlying storage in safe code.

Unsafe code *is* allowed to access the underlying storage, and as part of its
safety guarantee, it promises to do so without invalidating the object.

### Accessing Pinned pointers {#accessing-pinned-pointers}

#### Unpin {#unpin}

If `T` is relocatable, `Pin<&mut T>` etc. is pointless: it isn't protecting
against anything. Therefore, for such a type, Rust allows you to
[undo the Pin](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.into_inner)
in safe code.

#### Immutable access {#immutable-access}

`Pin<P>` implements `Deref<Target = P::Target>`. In other words, for
non-mutating methods, a `foo` of type `Pin<P>` can be used just as if it were of
type `P`: you can just call `foo.bar()` either way. And to obtain a `&T` from a
`Pin<&mut T>` or `Pin<Box<T>>` and so on, you can use `&*foo` as normal.

#### Mutable access {#mutable-access}

Mutating methods are callable from a `Pin<&mut T>` if they have that type as
their receiver. For example, if `std::vector` were a non-Rust-movable type, its
`clear` method as seen from Rust would have the following method signature:

```rust
fn clear(self: Pin<&mut vector>) {
  /* body omitted -- it uses the unsafe interface of Pin to access the underlying
     storage. */
}
```

This is callable via `my_ptr.as_mut().clear()`.

Why not `my_ptr.clear()` ? That works if `my_ptr` is of type `Pin<&mut T>`, but
since it takes `self` by value, `my_ptr` would no longer be usable after the
call. And that also doesn't work for `my_ptr : Pin<Box<T>>`!

Most mutable accesses to pinned pointers like `my_ptr` are done via
`my_ptr.as_mut()` to **reborrow** the pointer. With `&mut`, the reborrowing is
implicit and part of the language, but `Pin` is a library type, and requires
doing it by hand. Callers will use `.as_mut()` frequently, as it's required
whenever mutating and then continuing to use a pinned pointer.

--------------------------------------------------------------------------------

## Storage of pinned values {#storage-of-pinned-values}

### The Ctor trait {#the-ctor-trait}

So as to make initialization composable, let's start with a trait for
construction: `Ctor`. `Ctor` takes an uninitialized `*mut T` and initializes it,
promising as a postcondition that the memory has been fully initialized if no
error (such as a C++ exception) occurred.

```rust
pub trait Ctor {
    type Output;
    type Error;
    unsafe fn ctor(self, dest: *mut Self::Output) -> Result<(), Self::Error>;
}
```

`Ctor` is a lazy value, akin to a prvalue in C++. Its main purpose is as the
building block for initialization so that we can write generic code, defer to
subroutines, etc. However, since it is lazy, it also allows for guaranteed copy
elision in much the same way as C++: as long as you keep forwarding a `Ctor`
around without calling `ctor()`, you are not yet performing any copies or moves.

#### Universal initialization {#universal-initialization}

If we take the point of view that `Unpin` types are all safe to move by value,
and `!Unpin` types are all dangerous and must use `Ctor` to initialize something
in place, the following blanket impl makes `Ctor` an extension of safe
initialization, by implicitly defining it already for all `Unpin` types:

```rust
impl<T: Unpin> Ctor for T {
    type Output = T;
    type Error = Infallible;
    unsafe fn ctor(self, mut dest: *mut T) {
        dest.write(self);
    }
}
```

This ends up being very convenient when initialization of relocatable and
non-Rust-movable types is mixed together, such as with `ctor!` below.
Conceptually, we should imagine ourselves as having extended Rust initialization
to also cover types which can't be relocated, and pretend that all `!Unpin`
types are non-Rust-movable.

However, this means that a custom `Ctor` type must artificially declare itself
to be `!Unpin`, only so that it is omitted from the blanket impl and can be a
`Ctor` for something other than itself.

<!-- TODO(b/477144850): rewrite above when it's no longer true. -->

### Heap allocation {#heap-allocation}

The easiest way to store a pinned value is to invoke a static method on `Box`
with the following signature:

```rust
fn emplace<C: Ctor<Error=Infallible>>(ctor: C) -> Pin<Box<C::Output>>;
```

The resulting pinned box is a completely ordinary Rust value, safe to relocate
etc.

Just to give the reader a picture about how all of these are implemented, here
is what the source code for `emplace` would look like:

```rust
fn emplace<C: Ctor<Error=Infallible>>(ctor: C) -> Pin<Box<C::Output>> {
  let mut uninit = Box::new(MaybeUninit::<T>::uninit());
  unsafe {
    ctor.ctor(uninit.as_mut_ptr()).unwrap();
    Pin::new_unchecked(uninit.assume_init())
  }
}
```

The unsafe code here and inside of the `Ctor::ctor` definition is allowed to
unwrap the `Pin`, arbitrarily destructively mutate it, etc., but by the end of
it all, this eventually returns a pinned owned pointer to a fully initialized
value.

(This emplace function can be thought of as the `Ctor`-based dual of
[`Box::pin`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.pin).)

### Local variables {#local-variables}

The second easiest way to store a pinned value is to store it as a local. A
common misconception is that this is impossible, but it's actually nearly
identical to the `Box` case. Instead of creating a `Box<T>` and then wrapping
the `Box` in a `Pin`, we create a plain `T` and hide it, only exposing it via a
variable with type `Pin<&mut T>` that points back to the original value. This
makes that local variable impossible to move to a new location using a Rust
move, but of course, that was the whole point. If you want to move it to a new
location, you will need to do it some other way, such as via a move constructor,
which will be covered later.

This entire mechanism is hidden behind a macro (defined
[here](http://support/ctor.rs?q=macro_rules!%5C%20emplace)),
which can be used as so:

```rust
// Example use:
let x = emplace!(some_ctor);
// x is now a fully-initialized variable of type Pin<&mut T>:
let y: Pin<&mut T> = x;
```

(This `emplace` macro can be thought of as the `Ctor`-based dual of
[`std::pin`](https://doc.rust-lang.org/std/pin/macro.pin.html).)

### Struct fields {#struct-fields}

If you store the `T` as a `Pin<Box<T>>`, then that's safe to store in **any Rust
data structure**, and the problem is solved. If, however, you want to store it
by value as a `T`, then this forces the parent struct to itself also be
non-Rust-movable. This propagates the problem to the parent struct, which itself
can be stored as a local, on the heap, or, recursively, in another struct.

Let's start with the struct:

```rust
struct MyStruct {
  normal_value: u32,
  non_movable: T,
}
```

We would like to initialize `non_movable` in-place, and `normal_value` -- well,
`normal_value` can be initialized however we like. This is just a handful of
steps:

1.  As a parameter, receive a pointer to an uninitialized `MyStruct` -- this
    allows us to initialize the struct anywhere we want to place it, whether in
    a `Box`, a local, or another struct. Indeed, this means we will be
    implementing `Ctor`!
2.  Obtain the pointers for `normal_value` and `non_movable` so that we can
    initialize them in place. This can be done with `addr_of_mut!`.
3.  Initialize in place.

This is all wrapped up in a macro. The `ctor!` macro returns an `impl Ctor`
(step #1), the implementation of which looks up the different fields (step #2),
and initializes them using user-provided `Ctor`s (#3). Since relocatable types
like `u32` are their own `Ctor`, this looks much like normal initialization,
generalized to work in-place with custom `Ctor` implementations for `!Unpin`
types:

```rust
let sub_ctor = …; // a Ctor<Output=T>
let my_struct = emplace!(ctor!(MyStruct {
    normal_value: 0,
    non_movable: sub_ctor,
}));
```

Due to the complexity of struct syntax, this macro is a bit involved to define,
but its definition is
[here](http://support/ctor.rs?q=macro_rules!%5C%20ctor).

### Return values and parameters {#return-values-and-parameters}

TL;DR: where in C++ it would be a prvalue, in Rust use a `Ctor`.

Functions often return by value, which in C++ will construct the object in a
caller-determined location. Since non-Rust-movable values cannot be moved, and
Rust returns are always moves, this means that non-Rust-movable values **cannot
be returned by value as normal**. The tricks used for locals and temporaries
won't work either. Something else must be done.

To solve this, we use **lazy function evaluation**: every function which in C++
would accept or return a non-Rust-movable type by value, must in Rust, return an
`impl Ctor<Output=T, Error=Infallible>`. In effect, since return values and
arguments don't have a known location yet, we defer initialization until the
location *is* known. The expectation is that the `Ctor` is **immediately
invoked**, to best match normal initialization behavior.

`impl Ctor` is so common that it's given a convenience macro: `Ctor![X]` is
`impl Ctor<Output=X, Error=Infallible>`, and will be used for all examples.

So where in C++ we might write, for a non-Rust-movable type `MyStruct`, the
function `MyStruct ReturnsByValue(…) { return MyStruct {normal_value: 0, … };
}`, in Rust we would write:

```rust
fn returns_by_value(...) -> Ctor![MyStruct] {
  ctor!(MyStruct { normal_value: 0, … })
}
```

And if wrapping such a function that was written in C++, we would return a
`Ctor` which does not invoke the C++ function until the `Ctor` itself is
invoked.

Similarly, for parameters:

```rust
fn accepts_by_ctor(x: Ctor![MyStruct]) { … }
// example use:
accepts_by_ctor(MyStruct::ctor_new(...));
```

The common thread here is that `Ctor` really looks like, and is used like, a C++
prvalue.

#### Object-compatibility {#object-safety}

The cost of using `Ctor![]` for a *parameter* value is that a function doing so
can no longer be a method on a trait object. Since we do not use trait objects
anywhere in Crubit, that isn't immediately a concern. In the case that user code
needs to pass non-Rust-movable objects into a dynamically dispatched trait
method, it can do so by reference, materializing the `Ctor` in the caller,
rather than passing by value.

### Unions

Unsafe unions of nontrivial types were added in C++11, but there are currently
few 1000 uses of them. All other unions are of
trivial types, which would get ported to Rust unions without using the `Ctor`
functionality described in this document.

### Conclusion

We can initialize pinned, non-Rust-movable values in-place approximately
anywhere that we would expect to for Rust types, provided the storage can be
made non-Rust-movable-aware. By doing so, we propagate the constraints to where
we store it: if it is in a `Box`, we prevent direct access to the `Box`
contents. if it is in a local, we prevent that local from being assigned
elsewhere. And if it is in a struct, then that struct itself becomes
non-Rust-movable. If we are forced to store a value in something which requires
relocatability, we must heap-allocate the storage.

--------------------------------------------------------------------------------

## Moving and mutating {#moving-and-mutating}

Given that we can safely initialize non-Rust-movable values in a pinned
location, but cannot relocate them, it next falls on us to ask how to mutate
them and move them around.

C++ invented all the solutions for this. We want to reimplement copy and move
constructors, and copy and move assignment. While we're at it, we'll do
overloaded constructors and `operator=` in general.

Though, before we can build up abstractions for mutating move, we first need to
actually build support for mutation to begin with.

### Mutating pinned values {#mutating-pinned-values}

The previous section constructed a `T`, and hid it behind a pinned pointer (e.g.
`Pin<&mut T>`). This prevents direct access to the underlying `&mut T` in safe
code. So how on earth does one mutate it, without access to a mutable reference?

The first answer is "unsafe code". Indeed, if this is a C++ class, then the
generated method implementations will look something like this:

```rust
impl MyType {
  fn Mutate(self: Pin<&mut MyType>) {
    unsafe {
      // Pin::into_inner_unchecked() allows obtaining a real mutable reference in
      // unsafe code. This could be mutated directly, in pure Rust, or passed to an
      // FFI function like this, if wrapping C++.
      _ZN6MyType6MutateEv(Pin::into_inner_unchecked(self))
    }
  }
}
```

When writing our own struct, we can use libraries that help safely mutate a
pinned struct. In particular, we introduce a macro which allows for
pin-projection, analogous to the
[pin-project](https://crates.io/crates/pin-project). For example, if we were to
wrap `MyType` into another struct by value, and wished to mutate the wrapped
type:

```rust
#[recursively_pinned]
struct MyWrapperStruct {
  field_1: u32,
  field_2: MyType,
}

impl MyWrapperStruct {
  pub fn Mutate(self: Pin<&mut MyWrapperStruct>) {
    let mut s = self.project_pin();
    *s.field_1 += 1;  // u32 is Unpin, can directly mutate it.
    s.field_2.as_mut().Mutate();  // call into the Mutate function defined above.
  }
}
```

Here, `#[recursively_pinned]` is allowing us to define a struct which can be
mutated while pinned. (We use our own macro, instead of pin-project, because
`ctor!()` requires all fields to be pinned. This macro guarantees that (it never
creates non-pinned fields, unlike pin-project) and implements the unsafe marker
trait `RecursivelyPinned` to advertise this fact to `ctor!()`. It also supports
negative impls of `!Unpin`, which pin-project does not currently support.)

So in summary: wrapped C++ types can have generated `unsafe` code which obtains
a raw pointer and passes it to C++ for mutation. Manually written
non-Rust-movable types can use a macro to access pinned fields without writing
unsafe code.

### Rvalue References in Rust {#rvalue-references-in-rust}

The next thing we need is to represent a reference that is "intended to be moved
from", as opposed to ordinary mutable and immutable references.

```rust
// T&& equivalents:
pub struct RvalueReference<'a, T>(Pin<&'a mut T>);  // has as_mut() -> Pin<&mut T>, get_ref() -> &T
// std::move()
macro_rules! mov {
    ($p:expr) => {
        $crate::RvalueReference(Pin::as_mut(&mut { $p }))
    };
}

// const T&& equivalents
pub struct ConstRvalueReference<'a, T>(pub &'a T);  // has get_ref() -> &T
macro_rules! const_mov {
    ($p:expr) => {
        $crate::ConstRvalueReference(Pin::as_mut(&mut { $p }))
    };
}
```

(`const T&&` is not very important, as it is not really used except for overload
resolution tricks. However, since we would like to be able to represent *any*
C++ function in Rust, and some take `const T&&`, it is simpler not to omit it
entirely. If we do omit it, we need a different story for how to translate every
piece of existing code which uses the type.)

**When moving a value behind a pinned pointer**, we use the `mov!` macro: it
takes any pinned pointer type, e.g. `Pin<Box<P>>` or `Pin<&mut T>`, consumes it,
and evaluates to an `RvalueReference` with temporary lifetime which can be used
to trigger move construction and assignment.

By default, we should prefer to use pointers by value: `mov!(x)` makes `x`
inaccessible to Rust code thereafter, as it has been Rust-moved into a
temporary. Only if we specifically want to use a pointer after it has been moved
do we use `mov!(x.as_mut())`.

The `const T&&` APIs parallel the non-const APIs. They are rarely useful, but
are included for parity with C++.

### Constructors and assignment operators {#constructors-and-assignment-operators}

With these in place, we now have the tools in place to define the construction
and assignment traits:

```rust
pub trait CtorNew<ConstructorArgs> {
    type CtorType: Ctor<Output = Self, Error=Self::Error>;
    type Error;
    fn ctor_new(args: ConstructorArgs) -> Self::CtorType;
}

pub trait Assign<From> {
    fn assign(self: Pin<&mut Self>, src: From);
}
```

#### Implicit conversion, moves, and copies {#implicit-conversion-moves-and-copies}

In general, this scheme does not support implicit conversion. All constructor
calls must be explicit.

However, in the narrow case where a type `U` only exists in order to implicitly
convert to `T`, we can make that type itself implement `Ctor<Output=T>`., and
implement `Assign<U>` for `T`.

And so we have a rough solution for the two most common implicit conversions in
C++, to try to make them a little more ergonomic:

**moving**:

```rust
let mut x = emplace!(mov!(y));
x.assign(mov!(z));
```

`RvalueReference<T>` (returned by the `mov!` macro) implements `Ctor<Output=T>`,
for move-constructible `T` so that you can just `let x = emplace!(mov!(y));`
where in C++ you would `auto x = std::move(y)`. Conversely, move-assignable `T`
implements `Assign<RvalueReference<T>>`, so that you can `x.assign(mov!(y))`
where in C++ you would `x = std::move(y);`.

**copying**:

```rust
let mut x = emplace!(copy(y));
x.assign(copy(z));  // or assign(&*z)
```

`Copy<P>` (returned by the `copy` function) implements `Ctor<Output=P::Target>`,
for copy-constructible `P::Target` so that you can just `let x =
emplace!(copy(y));` where in C++ you would `auto x = y`. Conversely,
move-assignable `P:` implements `Assign<Copy<P>>` for all `P :
Deref&lt;Target=T>`, so that you can `x.assign(copy(y))` where in C++ you would
`x = y;` – although `&*y` works as well and is preferable.

(For copying, there are good reasons not to have `let x = c;` work. Also,
references are `Unpin` object types, and so they already implement `Ctor` for
themselves -- remember, this is an *extension* of regular Rust initialization!)

Note that for both copying and moving, nothing happens until the actual call to
`ctor` or `assign`.

#### Example use {#example-use}

```rust
let original_value = emplace!(...);
let copied = emplace!(copy(&*original_value));  // or: MyType::ctor_new(&*original_value)
let moved = emplace!(mov!(original_value.as_mut()));  // or: MyType::ctor_new(mov!(original_value.as_mut()))
// original_value is still available here, but moved-from
let moved_2 = emplace!(mov!(original_value));
// original_value is no longer available, since we used the form of mov that
// prohibits use-after-move.
// ERROR: let ... = something(&*original_value)

moved_2.assign(mov!(moved.as_mut()));
// as before, moved is still around, but moved-from
moved.assign(mov!(copied));
// this, otoh, used the preferred form of mov which prohibits use-after-move,
// and is no longer available.
// ERROR: something(&*copied)
```

#### Example definition of a copyable, movable type {#example-definition-of-a-copyable-movable-type}

The following definitions would be automatically generated for a C++ type, and
user-defined types should automatically derive them instead. Here is what the
automatic derivations might expand to, using `ctor!` and
[`#![feature(min_type_alias_impl_trait)]`](https://github.com/rust-lang/rust/issues/63063)

<section class="multicol">

<section class="table-header">**C++**</section>

<section class="table-header">**Rust**</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
MyType(const MyType& other) :
  field_(other.field_) {}
```

</section>

<section>

```rust {.no-copy}
impl CtorNew<&MyType> for MyType {
  type CtorType = Ctor![MyType];
  type Error = Infallible;
  fn ctor_new(other: &Self) -> Ctor![MyType] {
    ctor!(MyType {
      field: FieldType::ctor_new(&other.field),
    })
  }
}
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
MyType(const MyType&& other) :
  field_(std::move(other.field_)) {}
```

</section>

<section>

```rust {.no-copy}
impl CtorNew<RvalueReference<'_, Self>> for MyType {
  type CtorType = Ctor![Self];
  type Error = Infallible;
  fn ctor_new(other: &Self) -> Ctor![Self] {
    ctor!(MyType {
      field: FieldType::ctor_new(mov!(other.as_mut().project().field),
    })
  }
}
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
MyType& operator=(const MyType& other) {
  self.field_ = other.field_;
}
```

</section>

<section>

```rust {.no-copy}
impl Assign<&MyType> for MyType {
  fn assign(self: Pin<&mut Self>, other: &Self) -> Self::CtorType {
    let this = self.project();

    this.field.assign(&other.field);
  }
}
```

</section>

</section>

<section class="multicol">

<section>

```c++ {.no-copy}
MyType& operator=(MyType&& other) {
  self.field_ = std::move(other.field_);
}
```

</section>

<section>

```rust {.no-copy}
impl Assign<RvalueReference<'_, Self>> for MyType {
  fn assign(self: Pin<&mut Self>,
            other: RvalueReference<'_, Self>) -> Self::CtorType {
    let this = self.project();
    let other = other.as_mut().project();

    this.field.assign(mov!(other.field);
  }
}
```

</section>

</section>

### Reconstruction instead of operator= {#reconstruction-instead-of-operator=}

Where in C++, temporary materialization is implicit, in Rust it is explicit. We
can see every move, and wish it were elided. Well, why not elide it?

Instead of using `operator=`, we could use a `reconstruct` function, which
destroys the existing object, and constructs a new one in its place. This is
implemented
[here](support/ctor.rs?q=%22pub%20unsafe%20fn%20reconstruct%22)..

```rust
let mut x = emplace!(MyType::ctor_new(0));

// constructs as a temporary, and then move-assigns:
x.assign(mov!(emplace!(MyType::ctor_new(42))));

// destroys, and then constructs in place:
unsafe {
  reconstruct(x.as_mut(), MyType::ctor_new(100));
}
```

`reconstruct` is, in effect, a `Ctor`-based dual of
[`Pin::set`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.set).

However, this unsafe, because the behavior is undefined for base class
subobjects and `[[no_unique_address]]` fields. This is not suitable for general
use, and should only be used when performance considerations demand eliding as
many moves as possible.

### Conclusion

We've now explained and even implemented every operation in the table at the
start of this doc.
