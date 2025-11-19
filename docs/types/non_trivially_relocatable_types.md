# Non-Trivially-Relocatable C++ Objects in Rust {.solace-rhs-ignore}

<internal link>

**Summary:** Bridging the C++ and Rust object models so that we can access and
store non trivially relocatable C++ objects in Rust code.

[TOC]

## Overview {#overview}

This is an attempt at a satisfactory solution to representing
**non-trivially-relocatable C++ objects in Rust**. (That's a mouthful, so we'll
call them simply "non-relocatable" from now on.) We cover how to store and
manipulate non-relocatable C++ objects as **local variables** (as well as
temporaries), as **heap-allocated values**, and as **by-value struct data
members**. The extension of this to container types is explained but not
implemented, as are extensions of the approach to also include sum types, which
exist in Rust but not C++.

In parallel with this document, the approach is largely implemented in prototype
form,
[here](support/ctor.rs).

### Objective {#objective}

If our C++ codebase were to be ported to Rust, it would be ideal if any C++
function or type had an equivalent Rust function or type which does the same
thing: takes the same parameters, stores the same values, with the same
performance characteristics and safety tradeoffs. (Note:
[this will be complex](#common-complaint-this-is-really-complicated).)

However, C++ has types which at first seem impossible to represent in Rust:
types which are not [(trivially) relocatable](https://wg21.link/p1144). For
example, if a struct has a self-referential pointer, this is fine in C++: the
move constructor will re-point the pointer to its new target location, and it
will never be invalid. But in Rust, the built-in "move" operation (and swap
operation, and so on) does not run a user-provided move constructor, but instead
simply runs `memcpy`, performing a relocation. Since a self-referential type is
**not relocatable**, Rust code cannot be allowed to access it by value like
this.

The traditional approach to solving this is to box a non-relocatable value in a
heap-allocated pointer: the pointer is relocatable, even if its target is not.
If we forbid dereferencing the pointer in safe code (using `Pin`, described
[below](#introduction-pinned-pointers)), then we're good, and we're back to
having basically normal Rust values.

This document extends the approach to storing not just in a heap-allocated
pointer, but by value in all the usual places. This allows *any* C++ function or
type to be rewritten in equivalent Rust code, even if it deals with by-value
non-relocatable types.

<!-- Using a google include directive because this will definitely, definitely
     not work in github at all. Also, it's really, REALLY big in terms of LOC,
     making it hard to edit this file.
-->
<!--#include file="non_trivially_relocatable_types_comparison_table.md"-->

## Introduction: pinned pointers {#introduction-pinned-pointers}

In safe Rust code, we can expect to encounter a non-relocatable C++ type `T`
through the following three families of Rust types:

*   **`Pin<Box<T>>`**: equivalent to a non-null C++ `unique_ptr`.
    *   Possibly also `Pin<Arc<T>>` (`shared_ptr`), `Pin<Rc<T>>` (non-atomic
        `shared_ptr`), etc. (These are marginally less useful in Rust, due to
        restrictions on mutable aliasing.)
*   **`Pin<&mut T>`**: equivalent to a C++ non-const reference.
*   **`&T`**: equivalent to a C++ const reference.

Safe Rust code cannot ever directly access a C++ value which is not relocatable,
because then it would be able to relocate it, causing UB! Instead, it is safe
only to deal indirectly with such values behind a reference. If the reference is
mutable, it must be wrapped behind a `Pin`, which makes it impossible to access
the underlying storage in safe code.

Unsafe code *is* allowed to access the underlying storage, and as part of its
safety guarantee, it promises to do so without invalidating the object.

### Accessing Pinned pointers {#accessing-pinned-pointers}

#### Unpin {#unpin}

If `T` is relocatable, `Pin<&mut T>` etc. is pointless: it isn't protecting
against anything. Therefore, for such a type, Rust allows you to
[unwrap the Pin](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.into_inner)
in safe code.

#### Immutable access {#immutable-access}

`Pin<P>` implements `Deref<Target = P::Target>`. In other words, for
non-mutating methods, a `foo` of type `Pin<P>` can be used just as if it were of
type `P`: you can just call `foo.bar()` either way. And to obtain a `&T` from a
`Pin<&mut T>` or `Pin<Box<T>>` and so on, you can use `&*foo` as normal. (You
can also use `get_ref()`, with a
[subtle lifetime difference](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.get_ref),
but this is going to be rare, as it is surprisingly annoying to use and usually
not important).

#### Mutable access {#mutable-access}

Mutating methods are callable from a `Pin<&mut T>` if they have that type as
their receiver. For example, if `std::vector` were a non-relocatable type, its
`clear` method as seen from Rust would have the following method signature:

```rs
fn clear(self: Pin<&mut CxxVector>) {
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
construction: `Ctor`. `Ctor` takes a `&mut MaybeUninit<T>` and initializes it,
promising as a postcondition that the memory has been fully initialized.

```rs
pub trait Ctor {
    type Output;
    unsafe fn ctor(self, dest: Pin<&mut MaybeUninit<Self::Output>>);
}
```

`Ctor` is a lazy value, akin to a prvalue in C++. Its main purpose is as the
building block for initialization so that we can write generic code, defer to
subroutines, etc. However, since it is lazy, it also allows for guaranteed copy
elision in much the same way as C++: as long as you keep forwarding a `Ctor`
around without calling `ctor()`, you are not yet performing any copies or moves.

(The choice of `Output` being an associated type vs type parameter is a bit
arbitrary, but see [Ctor&lt;T> vs Ctor&lt;Output=T>](#ctor_t-vs-ctor_output_t)
in the alternatives considered.)

#### Universal initialization {#universal-initialization}

If we take the point of view that `Unpin` types are all safe to move by value,
and `!Unpin` types are all dangerous and must use `Ctor` to initialize something
in place, the following blanket impl makes `Ctor` an extension of safe
initialization, by implicitly defining it already for all `Unpin` types:

```rs
impl<T: Unpin> Ctor for T {
    type Output = T;
    unsafe fn ctor(self, mut dest: Pin<&mut MaybeUninit<Self>>) {
        dest.as_mut_ptr().write(self);
    }
}
```

This ends up being very convenient when initialization of relocatable and
non-relocatable types is mixed together, such as with `ctor!` below.
Conceptually, we should imagine ourselves as having extended Rust initialization
to also cover types which can't be relocated, and pretend that all `!Unpin`
types are non-relocatable.

However, this means that a custom `Ctor` type must artificially declare itself
to be `!Unpin`, only so that it is omitted from the blanket impl and can be a
`Ctor` for something other than itself.

#### Post-construction mutation {#post-construction-mutation}

Every `Ctor` is also given a `ctor_then` for safe initialization steps. For
example, to overwrite values that were initialized with different defaults, or
call some methods after initialization.

```rs
fn ctor_then<F: FnOnce(Pin<&mut Self::Output>)>(self, f: F) -> CtorThen<Self, F>
  where Self : Sized { … }
```

(This is much like how a C++ constructor has a separate member initialization
list vs constructor body.) The main use case for this is to allow returning an
`impl Ctor` that does more work than just initialize a value – where in C++ you
might create a variable, do a bunch of work, and then return it, in Rust the
return value location is not yet known, so all of that must happen inside the
`Ctor`, where it *is* known. The easiest way is to do the work inside the
`ctor_then` callback:

```rs
// Equivalent to C++ "MyType foo() { Mytype x = bar(); x.baz(); return x; }" and NRVO
fn foo() -> impl Ctor<Output=MyType> {
  bar().ctor_then(|mut x| {
    x.baz();
  })
}
```

This is implemented and documented
[here](support/ctor.rs?q=ctor_then).

### Heap allocation {#heap-allocation}

The easiest way to store a pinned value is to invoke a static method on Box with
the following signature:

```rs
fn emplace<C: Ctor>(ctor: C) -> Pin<Box<C::Output>>;
```

The resulting pinned box is a completely ordinary Rust value, safe to relocate
etc.

Just to give the reader a picture about how all of these are implemented, here
is the actual source code for `emplace`:

```rs
fn emplace<C: Ctor>(ctor: C) -> Pin<Box<C::Output>> {
  let mut uninit = Box::new(MaybeUninit::<T>::uninit());
  unsafe {
    let pinned = Pin::new_unchecked(&mut *uninit);
    ctor.ctor(pinned);
    Pin::new_unchecked(Box::from_raw(Box::into_raw(uninit).cast::<T>()))
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

```rs
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
non-relocatable. This propagates the problem to the parent struct, which itself
can be stored as a local, on the heap, or, recursively, in another struct.

Let's start with the struct:

```rs
struct MyStruct {
  normal_value: u32,
  non_relocatable: T,
}
```

We would like to initialize `non_relocatable` in-place, and `normal_value` --
well, `normal_value` can be initialized however we like. This is just a handful
of steps:

1.  As a parameter, receive a pointer to an uninitialized `MyStruct` -- this
    allows us to initialize the struct anywhere we want to place it, whether in
    a `Box`, a local, or another struct. Indeed, this means we will be
    implementing `Ctor`!
2.  Obtain the pointers for `normal_value` and `non_relocatable` so that we can
    initialize them in place. This can be done with `addr_of_mut!`.
3.  Initialize in place.

This is all wrapped up in a macro. The `ctor!` macro returns an `impl Ctor`
(step #1), the implementation of which looks up the different fields (step #2),
and initializes them using user-provided `Ctor`s (#3). Since relocatable types
like `u32` are their own `Ctor`, this looks much like normal initialization,
generalized to work in-place with custom `Ctor` implementations for `!Unpin`
types:

```rs
let sub_ctor = …; // a Ctor<Output=T>
let my_struct = emplace!(ctor!(MyStruct {
    normal_value: 0,
    non_relocatable: sub_ctor,
}));
```

Due to the complexity of struct syntax, this macro is a bit involved to define,
but its definition is
[here](http://support/ctor.rs?q=macro_rules!%5C%20ctor).

### Return values and parameters {#return-values-and-parameters}

TL;DR: where in C++ it would be a prvalue, in Rust use a `Ctor`.

Functions often return by value, which in C++ will construct the object in a
caller-determined location. Since non-relocatable values cannot be moved, and
Rust returns are always moves, this means that non-relocatable values **cannot
be returned by value as normal**. The tricks used for locals and temporaries
won't work either. Something else must be done.

We could require boxing across function boundaries, but this restricts us to
only transmitting non-relocatable types between Rust and C++ when a heap
allocation is OK.

We could use an output parameter, but this would be clunky and not as easily
composable, as well as losing C++'s copy/move elision. See
[Return values using output parameters](#return-values-using-output-parameters)
in the alternatives section.

Instead, as shown above, we shall use **lazy function evaluation**: every
function which in C++ would return a non-relocatable type by value, must in
Rust, return a `Ctor`. In effect, since return values don't have a known
location, we defer initialization until the location *is* known. The expectation
is that the returned `Ctor` is **immediately invoked**, to best match normal
initialization behavior. (Deferring initialization is also OK, and memory safety
of this is checked by the type system and lifetime annotations.)

So where in C++ we might write, for a non-relocatable type `MyStruct`, the
function `MyStruct ReturnsByValue(…) { return MyStruct {normal_value: 0, … };
}`, in Rust we would write:

```rs
fn returns_by_value(...) -> impl Ctor<Output=MyStruct> {
  ctor!(MyStruct { normal_value: 0, … })
}
```

And if wrapping such a function that was written in C++, we would return a
`Ctor` which does not invoke the C++ function until the `Ctor` itself is
invoked.

This can be extended to parameters: unlike return values, it would not be
necessary to heap allocate to transfer ownership, but it would still be an extra
local construction, just to pass it to the function which then needs to
move-construct a new copy. (A facility we have not yet introduced.) If by-value
parameters are instead `impl Ctor`, then we get copy/move elision.

```rs
fn accepts_by_ctor(x: impl Ctor<Output=MyStruct>) { … }
// example use:
accepts_by_ctor(MyStruct::ctor_new(...));
```

The common thread here is that `Ctor` really looks like, and is used like, a C++
prvalue.

#### Object-safety {#object-safety}

The cost of using `impl Ctor` for a *parameter* value is that a function doing
so can no longer be a method on a trait object. Since we do not use trait
objects anywhere in the interop system, that isn't immediately a concern. In the
case that user code needs to pass non-relocatable objects into a dynamically
dispatched trait method, it can do so by reference, materializing the `Ctor` in
the caller, rather than passing by value.

### Unions {#unions}

Unsafe unions of nontrivial types were added in C++11, but there are currently
few 1000 uses of them. All other unions are of
trivial types, which would get ported to Rust unions without using the `Ctor`
functionality described in this document.

Support for unions of nontrivial types is mechanically annoying, as it involves
unsafely initializing and dropping union members, but not otherwise very
difficult. Omitting the details here, but see e.g. cl/383974242 &lt;TODO: link
to test once that's submitted>.

### Not implemented {#not-implemented}

#### Vec, HashMap, etc. {#vec-hashmap-etc}

These are TODO. Or rather, equivalent types are TODO.

We've seen now that we can store non-relocatable values in a struct, but that
struct itself becomes also non-relocatable. This means that **you cannot store
these values inside of a `Vec`, `HashMap`, or any other struct that doesn't
explicitly support these pinned values.**

Fortunately, given that the project is to interact with C++, we have a neat
option available: use C++ `std::vector`, `absl::flat_hash_map`, etc. instead of
regular Rust collections to store non-relocatable values. Alternatively, wrap
such values in a `Pin<Box>` and then use regular Rust collections.

#### Enums (sum types) {#enums-sum-types}

These are TODO.

Rust
[does not have](https://github.com/rust-lang/unsafe-code-guidelines/issues/298)
built-in facilities to initialize an enum in place, the way it does for a
struct. However, a `repr(C)` enum has the same representation as the same enum
with all fields replaced with `MaybeUninit<TypeOfField>`, so we could write a
macro which creates the associated `MaybeUninit` enum for purposes of in-place
construction, before reinterpreting it in-place as fully initialized. We could
also, equivalently, decompose into a struct of union of structs.

This is left sketched out, rather than completely implemented, because C++ does
not have sum types, and so this is not a direct barrier to porting code to Rust.
It could come up during the evolution of the codebase of an existing **Rust**
project, if it wanted to add a C++ type to an enum it already had. We can cross
that bridge when we come to it.

#### Tuples {#tuples}

Rust tuples will never be supported, as it is unsound.

We can't unilaterally decide that tuples are structurally pinned, because
another crate can, with equal validity, decide that they *aren't*, and it would
be UB to mix the two. One decision must be made consistently per type, but no
crate owns tuple types and so no crate can make the decision on their behalf.

Safer, instead, would be to store in a C++ `std::tuple` or a Rust tuple-struct.

### Conclusion

We can initialize pinned, non-relocatable values in-place approximately anywhere
that we would expect to for Rust types, provided the storage can be made
non-relocatable-aware. By doing so, we propagate the constraints to where we
store it: if it is in a `Box`, we prevent direct access to the `Box` contents.
if it is in a local, we prevent that local from being assigned elsewhere. And if
it is in a struct, then that struct itself becomes non-relocatable. If we are
forced to store a value in something which requires relocatability, we must
heap-allocate the storage.

--------------------------------------------------------------------------------

## Moving and mutating {#moving-and-mutating}

Given that we can safely initialize non-relocatable values in a pinned location,
but cannot relocate them, it next falls on us to ask how to mutate them and move
them around.

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

```rs
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

```rs
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
non-relocatable types can use a macro to access pinned fields without writing
unsafe code.

### Rvalue References in Rust {#rvalue-references-in-rust}

The next thing we need is to represent a reference that is "intended to be moved
from", as opposed to ordinary mutable and immutable references.

```rs
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

```rs
pub trait CtorNew<ConstructorArgs> {
    type CtorType: Ctor<Output = Self>;
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

```rs
let mut x = emplace!(mov!(y));
x.assign(mov!(z));
```

`RvalueReference<T>` (returned by the `mov!` macro) implements `Ctor<Output=T>`,
for move-constructible `T` so that you can just `let x = emplace!(mov!(y));`
where in C++ you would `auto x = std::move(y)`. Conversely, move-assignable `T`
implements `Assign<RvalueReference<T>>`, so that you can `x.assign(mov!(y))`
where in C++ you would `x = std::move(y);`.

**copying**:

```rs
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

```rs
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

```rs {.no-copy}
impl CtorNew<&MyType> for MyType {
  type CtorType = impl Ctor<Output = Self>;
  fn ctor_new(other: &Self) -> Self::CtorType {
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

```rs {.no-copy}
impl CtorNew<RvalueReference<'_, Self>> for MyType {
  type CtorType = impl Ctor<Output = Self>;
  fn ctor_new(other: &Self) -> Self::CtorType {
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

```rs {.no-copy}
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

```rs {.no-copy}
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

This is quite ugly, and yet will be common, so we should add some equivalent of
"= default" -- a derive, arguments to `#[recursively_pinned]`, or something to
that effect. For example, perhaps:

```rs
#[derive(CtorNew_Default, CtorNew_Copy, CtorNew_Move, Assign_Copy, Assign_New)]
```

This is still a work in progress, but not expected to be challenging.

### Reconstruction instead of operator= {#reconstruction-instead-of-operator=}

Where in C++, temporary materialization is implicit, in Rust it is explicit. We
can see every move, and wish it were elided. Well, why not elide it?

Instead of using `operator=`, we could define a `reconstruct` function, which
destroys the existing object, and constructs a new one in its place. This is
implemented
[here](support/ctor.rs?q=%22pub%20unsafe%20trait%20Reconstruct%22)..

```rs
let mut x = emplace!(MyType::ctor_new(0));

// constructs as a temporary, and then move-assigns:
x.assign(mov!(emplace!(MyType::ctor_new(42))));

// destroys, and then constructs in place:
x.reconstruct(MyType::ctor_new(100));
```

`reconstruct` is, in effect, a `Ctor`-based dual of
[`Pin::set`](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.set).

However, this is only safe for `final` types, and probably not suitable for
general use, only when performance considerations demand eliding as many moves
as possible. Feedback would be appreciated. (Future work:
[In-place reconstruction instead of move?](#in-place-reconstruction-instead-of-move))

### Conclusion

<!-- disableFinding(LINK_ID) -->

<!-- (Disabled because it's defined in an include.) -->

We've now explained and even implemented every operation in the table at the
start of this doc. Now would be a good time to [go back](#the-cheat-sheet) to
that table and make sure you follow.

<!-- enableFinding(LINK_ID) -->

--------------------------------------------------------------------------------

## Future Work {#future-work}

### How common are non-relocatable types? {#how-common-are-non-relocatable-types}

The number of types that *need* this kind of facility will determine how much
more effort should be put into making it ergonomic.

For example: `std::string` and `std::vector` are (perhaps surprisingly)
relocatable in libc++, but `std::list` and `std::map` are not, because the
container embeds a root node within itself that its elements point to
([example](https://github.com/llvm/llvm-project/blob/9b7fd0099e79b0f5b824027cbae8a25356486ac9/libcxx/include/__tree#L876)).
[This blog post](https://quuxplusone.github.io/blog/2019/02/20/p1144-what-types-are-relocatable/)
from the author of P1144 outlines which types in the standard library are
trivially relocatable in practice. What about user-defined types? We would need
to run a ClangMR. :)

### Non-relocatable-compatible containers {#non-relocatable-compatible-containers}

We either need to be able to use the C++ containers -- `std::vector`,
`absl::flat_hash_map`, and even `std::tuple`, and so on -- or else we need new
containers. `Vec`, `HashMap`, etc. cannot store non-relocatable types by value.

### In-place reconstruction instead of move? {#in-place-reconstruction-instead-of-move}

<!-- disableFinding("first-class") -->

<!-- There's no synonym / analogous word for "first class functions",
which this is referencing. -->

With C++17 copy elision rules even in C++, and with the first-class `Ctor`
values in Rust, it becomes very tempting to use primarily in-place
reconstruction like `reconstruct`, not `assign`. For example, perhaps
`reconstruct` should be the default for new code, but `assign` for code that was
ported from existing C++, or that specifically has to use `operator=` to
interface with existing C++. Is this reasonable?

<!-- enableFinding("first-class") -->

Note that, unlike assignment, `reconstruct` can have undefined behavior:

*   Reconstructing a base class subobject is UB, and so this cannot be safe for
    any pinned pointer, only for those which are `final`.
*   Reconstructing a `[[no_unique_address]]` member is also UB, but we can
    forgive this, as those will be unsafe for use in Rust anyway.
*   Reconstructing a top-level `const` object is also UB, but we restrict to
    `mut` references.

While most of the concerns are forgivable, the first is not: this would not be
callable on base classes without unsafe code, and it would be UB if they were a
base class subobject!

TODO: Are we happy with this conclusion? If so, move this out of future-work.

### Overloaded functions {#overloaded-functions}

Discussion of overloaded functions in general is going to be the subject of a
future design doc. The intent is that `ctor_new()` will follow the same
convention as everything else follows. If overloaded functions exposed from C++
usually can be called without the extra parens, then `ctor_new()` will be
callable without the extra parens. If overloaded functions generally have a
hardcoded new name, then this trait will go away and instead we will give each
constructor signature its own name.

Assignment, however, will stay as is: it is a natural fit for the trait system.

### Aliases {#aliases}

C++ and Rust have incompatible aliasing models: C++ assumes mutable aliases are
permitted, but aliases can't cross type boundaries (with some exceptions), while
Rust forbids mutable aliasing but allows non-mutable aliasing across type
boundaries for types with compatible layouts. While this doc covers how mutation
and creation of objects work, it does not include a full accounting of the
interop story, including aliasing. This should be addressed in a later document.

### Actually binding C++/Rust {#actually-binding-c-rust}

This document covers (most of) the object model, but not the actual translation
of C++ interfaces to equivalent Rust interfaces. This is a precondition for that
work.

### TODO {#todo}

**TODO: double check I can just define a constructor which takes things by
value. (It should *just work*.)**

**TODO: implements copy, deletes move**

--------------------------------------------------------------------------------

## Alternatives considered {#alternatives-considered}

### moveit {#moveit}

[The moveit crate](https://docs.rs/moveit/0.2.0/moveit/) from is an
implementation of C++-style construction semantics in Rust. It is the direct
inspiration of the approach described here. In fact, the `Ctor` trait here is
identical to the `Ctor` trait from moveit.

Why didn't we just use moveit? moveit is not a perfect fit for interop, because
there exist some functions in C++ which can't be called from Rust, and some C++
patterns of object use that can't be replicated in Rust with the moveit crate.
In particular, moveit makes a deliberate choice to disallow use-after-move and
to focus on the case where Rust moves *are* C++ moves.

With the approach described here, if you have a `Pin<Box<T>>` and move it, that
is also a rust move, and you can't use it after move. That's the same as in
moveit, which allows the same thing (because `Box` is `DerefMove`). However, in
the approach described here, you can also move from a `Pin<&mut T>`, which you
can't do in moveit. (moveit does have a custom reference type, which takes the
place of `&mut T` for local variables -- however, since this reference type
can't be obtained from a `Pin<P>`, only from local emplacement, it still
prohibits use-after-move in general.)

As an example,. take `swap(x, y)`. We don't care about swapping in particular,
but it makes a good demonstration of what code looks like that needs to
use-after-move. With the current approach:

```rs
fn swap(mut x: Pin<&mut T>, mut y: Pin<&mut T>) {
  let mut tmp = emplace!(mov!(x.as_mut()));
  x.assign(mov!(y.as_mut()));
  y.assign(mov!(tmp));
}
```

In moveit, we would need to transmute ("`std::bitcast`") the `&mut T` into a
`MoveRef`, save the memory locations, destructively move `x` into a temporary
and `y` into the now uninitialized memory of `x`, and then destructively move
the temporary into the now uninitialized memory for `y`.

(We also differ in other, more superficial ways. The approach here is "curried"
and treats `Ctor` as the fundamental unit of abstraction, whereas `moveit` has
separate traits for e.g. move and copy construction, which must actually invoke
a `Ctor` themselves rather than merely returning one. This curried approach
could one day find its way into a future version of moveit, since it composes
very well and works nicely with e.g. the `ctor!()` macro. On that note, moveit
doesn't yet have `ctor!()` and `emplace!(expr)`, but those are very likely to be
added.)

### Unified Initialization {#unified-initialization}

#### Specialized universal initialization {#specialized-universal-initialization}

Rather than only implementing `Ctor<Output=Self>` for `Unpin` types to give
universal initialization, we could default **all** types to be their own `Ctor`
by value, and only specialize some types to be the `Ctor` for a different value.

This approach is conceptually purer (if you do have a `!Unpin` value, you ought
to be able to rust-move it safely just fine), but requires full
specialization -- not even `min_specialization` is enough, as we need to
specialize the associated type. Unfortunately, full specialization is incomplete
and unsound, and unlikely to be stabilized any time soon.

#### Non-unified initialization {#non-unified-initialization}

We could take the position that invoked lazy initialization always looks
different from regular initialization, using an introducer keyword such as `new`
(the syntax
[used by moveit](https://github.com/google/moveit/blob/master/src/move_ref.rs#L335)).
This has the advantage that we no longer depend on negative impls (which is the
only reason moveit uses a special syntax), and arguably it is clearer to make
`Ctor` invocations explicit. This would look something like:

```rs
let x = emplace!(
  new ctor!(Foo { direct_field: 7, indirect_field: new mov!(...) })
);
```

Alternatively, we could require non-lazy initialization of regular Rust values
to be wrapped in a `Ctor`, like this:

```rs
let x = emplace!(
  ctor!(Foo { direct_field: RustCtor(7), indirect_field: mov!(...) })
);
```

In either case, we get different spellings for trivially-relocatable values that
don't support `Ctor`, vs values built via `Ctor`.

However, we expect every type to eventually become relocatable. At first, the
differing spellings will carry weight: it distinguishes between a regular value,
and a `Ctor` which is now being emplaced. But any type that becomes relocatable
will switch from one spelling to the other, which is backwards incompatible. So
instead, at that point, it must continue using `Ctor` for compatibility in the
short term, and migrate callers to use the value syntax.

If we standardize that `Unpin` types are their own `Ctor`, and emplacement is a
generalization of initialization, then we simplify the syntax and can
standardize the migration process. Rather than requiring people to move off of
the `Ctor` interface, we allow it to continue to work as-is, and have fixers in
critique/etc. that can automatically replace unnecessary uses of `Ctor` where
plain Rust semantics are equivalent.

If we are okay with relying on negative impls (or specialization) to unify the
syntax, it seems preferable.

##### Avoiding negative impls? {#avoiding-negative-impls}

\
Negative impls are probably going to be stabilized eventually, but not until
after [Chalk](https://github.com/rust-lang/chalk) (Rust's next-generation trait
engine).

We do have alternatives in the form of specialization (currently unsafe), or
switching to an alternate syntax, so it is not a deal-breaker if we must move
off of negative impls as a solution.

Since it will probably be stabilized, and we can in the worst case migrate to
something else, it seems relatively harmless.

<!-- disableFinding(HEADING_FUNCTION) -->

### Standard Ctor(args) type {#standard-ctor-args-type}

<!-- enableFinding(HEADING_FUNCTION) -->

A common pattern with defining an overloaded constructor is to create a custom
type which stores the args and implements Ctor, and then return that from
`ctor_new`. We could simplify this by only requiring the first step, so that a
new user-defined constructor looks like this:

```rs
impl Ctor for OverloadedCtor<MyType, (Arg, Types)> {
  fn ctor(...) { … }
}
```

However, thanks to
[`#![feature(min_type_alias_impl_trait)]`](https://github.com/rust-lang/rust/issues/63063),
user-defined code never has to explicitly define a new type or implement or call
`Ctor`. Instead, user code can use the `ctor!()` macro and infer an `impl Ctor`.
This is a small amount of boilerplate, easily reduced with a macro or ignored by
a human. This alternative approach would require users to directly engage with
`Ctor`, which they wouldn't have to do otherwise, and is actually more code
overall, and invokes a larger number of abstractions. In other words, it fails
to deliver.

Current approach:

```rs
impl CtorNew<(Arg, Types)> for MyType {
  type CtorType = impl Ctor;
  fn ctor_new(args: (Arg, Types)) -> Self::CtorType {
    ctor!(MyType {...})
  }
}
```

`OverloadedCtor` approach:

```rs
impl Ctor for OverloadedCtor<MyType, (Arg, Types)> {
  fn ctor(self, dest: Pin<&mut MaybeUninit<Self::Output>>) {
    ctor!(MyType {...}).ctor(dest)
  }
}
```

The remaining sources of verbosity (e.g. specifying argument types twice) can be
solved with aliases or macros or the like.

We can also use a macro in the (unlikely) case that `min_type_alias_impl_trait`
is not stabilized, either to generate the constructor type (which `ctor!()`
assumes to exist), or to replicate something like the current syntax (see also:
[nightly-crimes](https://crates.io/crates/nightly-crimes) ;]).

### Return values using output parameters {#return-values-using-output-parameters}

Rather than returning `impl Ctor<Output=T>`, one alternative is to use output
parameters. This disables copy elision, and requires explicitly passing in the
storage that will be used for the return value.

In particular, we can expose a slot type, which acts like an `Option` that is
structurally pinned, and has the following methods:

```rs
impl<T> Slot<T> {
    pub fn uninit() -> impl Ctor<Output = Self> { ... }
    pub fn replace(mut self: Pin<&mut Self>, value: impl Ctor<Output = T>) -> Pin<&mut T> {...}
    pub fn as_opt_mut(self: Pin<&mut Self>) -> Option<Pin<&mut T>> {...}
}
```

(We already need something like this in order to implement `emplace!()` to begin
with. The source code for `Slot` is
[here](support/ctor.rs?q=%22pub%20struct%20Slot%22).
This also means that the code examples here are actually storing a slot within
another slot…!)

Once we have this `Slot` type, returning an object can be done one of two ways:
by writing to the `Slot`, and letting the caller pull it out via `as_opt_mut()`,
or by writing to the slot and then returning the resulting pinned pointer
directly:

```rs
fn slotted_rv(slot: Pin<&mut Slot<u32>>, x: impl Ctor<Output=u32>) -> Pin<&mut u32> {
    slot.replace(x)
}
// Example use:
let slot = emplace!(Slot::uninit());
let rv = slotted_rv(slot, 42);
assert_eq!(*rv, 42);
```

```rs
fn slotted_out_param(slot: Pin<&mut Slot<u32>>, x: impl Ctor<Output=u32>) {
    slot.replace(x);
}
// example use:
let mut slot = emplace!(Slot::uninit());
slotted_out_param(slot.as_mut(), 42);
assert_eq!(*slot.as_opt_mut().unwrap(), 42);
```

Unfortunately, this specifically inhibits copy elision where it would take place
in C++. Also, if some things use `Ctor` and some use slotted outputs of some
kind, mixing the two is awkward: once a `Ctor` has been materialized, the only
way to convert it back to a `Ctor` is the move constructor, which requires the
value live around. Using `Ctor` exclusively (or near-exclusively) reduces the
number of abstractions and move operations.

### `Ctor<T>` vs `Ctor<Output=T>` {#ctor_t-vs-ctor_output_t}

The current approach has constructors implement `Ctor<Output=T>`, with blanket
impls for relocatable types, which implement `Unpin`. A constructor must
implement `!Unpin` to disable the blanket impl, so that there is exactly one
implementation of `Ctor` for each type.

What if, instead of an associated type, the output were a type parameter? This
allows a type to implement `Ctor` for multiple different outputs, in theory
allowing us to remove all of the special-casing around `Unpin`, and to avoid
defining one type per `Ctor` output.

However, I think it doesn't work out so cleanly.

With the current approach, you get a compilation error if you forget to declare
a new Ctor type as `!Unpin`, because of overlapping impls. But with the
`Ctor<T>` approach, this is fine -- the error only occurs when you try to *use*
the Ctor. But the error still occurs:

```rs
let x = emplace!(MyCtor);
```

If `MyCtor` implements `Ctor<T>` and `Ctor<U>`, Rust may complain that it does
not know which type parameter to use. In particular, if `MyCtor` is `Unpin`,
then it will implement construction both for the intended output type, and for
itself.

If everything at all times only ever talks about `impl Ctor<SomeSpecificType>`
this is fine, but that is currently not true. For example, `mov!` evaluates to a
concrete type `RvalueReference<'a, T'>`, not just an `impl Ctor`, so that it can
be used by both `Ctor` and `Assign`. Any time we talk about concrete types,
we'll want it to only implement `Ctor` for one output type, so that we can't get
inference errors. (It would be very bad if `let x = emplace!(mov!(y));` didn't
work!)

The end result, in my estimation, would be roughly the same: every public type
will want to implement `Ctor<T>` for exactly one `T`, either because it is `impl
Ctor<T>`, or because the type specifically erases any blanket impls to avoid
ambiguity. For the cases where it erases blanket impls, that's identical to what
it would do with associated types. For the case where it avoids them due to
specifying in `impl Ctor<T>`, then the most that `Ctor<T>` buys us is that we
can avoid creating a new wrapper type that satisfies the impl, but the public
interface is also still the same.

### Naming {#naming}

The names in this doc are not final.

Some constraints:

1.  Generally speaking, single-method traits should have the same name as their
    method. E.g. `Ctor`/`ctor`, `CtorNew`/`ctor_new`.
    1.  This makes a name like `LazyValue` or `PrValue` less tempting as an
        alternative to `Ctor`.
2.  `impl Ctor` is going to be common on interface boundaries, `ctor_new` will
    be common for creating new objects. But the functions `ctor` and the trait
    name `CtorNew` will be infrequently mentioned.
3.  Ideally, we want this to look similar to the normal Rust or C++ patterns.
    Since `Ctor` is a generalization of initialization, we want it to look just
    like initialization.
    1.  It's therefore tempting to call it `new` or `ctor_new` or similar, to
        evoke Rust `new` (as well as C++ placement `new`).
4.  We do not want to conflict with common inherent methods, because then if you
    try to call the inherent method when the trait is in scope… (so don't call
    `ctor_new` `new`.)

The current names are `Ctor`, `CtorNew`, and `Assign`, with identical
snake\_cased methods. This identifies `Ctor` ("Constructor") as the primary
abstraction (often used with `impl Ctor`), and `CtorNew` is "new, via Ctor". (As
opposed to `NewCtor`!)

Questions:

*   How important is it really that the trait name and method name are the same?
*   Should we drop the idea that this is a generalization of initialization, and
    make it look strictly different? That opens up the name space considerably.

#### Alternative names {#alternative-names}

*   **`Ctor`:** `PrValue`, `LazyValue`, `Lazy`, `Constructible`, `Constructor`,
    `Materialize`, `Materializable`, `Emplace`, `Emplaceable`
*   **`Ctor::ctor()`:** `construct()`, `value()`, `materialize()`,
    `lazy_value()`, `lazy()`, `emplace()`
*   **`CtorNew`:** `New`, `CNew` ("ctor" new, or "c++" new!), `CreateLazy`,
    `CreatePrValue`, `CtorFrom`, `CFrom`, `ConstructibleFrom`, `Ctor`
*   **`CtorNew::ctor_new()`:** `new()`, `cnew()`, `create_lazy()`,
    `create_prvalue()`, `create()`, `make()`, `ctor_from()`, `cfrom()`, `ctor()`

#### TODO: have one big bikeshed session where we decide on final names. {#todo-have-one-big-bikeshed-session-where-we-decide-on-final-names}

--------------------------------------------------------------------------------

## Appendix {#appendix}

### Features I wish Rust had {#features-i-wish-rust-had}

The following features would make the implementation simpler and easier :(

*   Macros don't have capture types for several common AST subtrees:
    *   The `$foo` in the struct literals `$foo {}` and `$foo ()`
        *   `path` isn't quite right: you can't parse the second one if `$foo`
            is a `path`, because parentheses are valid in path elements.
    *   Field names: the thing after the dot in `foo.bar` and `foo.0`.
        *   `ident` isn't quite right: integers aren't identifiers, but are
            field names, and so are valid in struct literals and field accesses.
    *   The (possibly empty) `mut` specifier -- would be nice to match `let
        $mut:mut $var:ident = $e:expr;`, much like we can do for `pub`.
        *   Note: we may be able to *match* the mut, via `mut?`, but since it
            wouldn't be bound to a variable, we can't re-inject it into the
            token stream. i.e. there's no *easy* way right now to write a macro
            which goes, for example, `($mut:mut $x:ident) => { let $mut $x = 3;
            };`, so that you could pass in either `var` or `mut var`.
*   Negative trait impls -- the approach described here relies on `impl !Unpin`,
    because blanket impls on `T : Unpin` are treated as possibly-applicable if
    you use `PhantomPinned`, but not `impl !Unpin`.
    *   Alternatively, we could use
        [full specialization](#specialized-universal-initialization), if that
        were made less dangerous.
*   An equivalent to `$crate` for proc macros: if we, for example, wrap
    pin-project with a macro, the user of our wrapper must directly depend on
    pin-project, because `#[pin_project]` uses `::pin_project`, which only
    exists if you directly depend on it.
*   Alternatively: the ability to write a derive or attribute macro in macro by
    example.
*   The ability to get the address of an enum variant's field, rather than
    manually decomposing it into an equivalent representation that we can
    initialize in place. Our approach can only work with `repr(C)` enums.

### Common complaint: this is really complicated! {#common-complaint-this-is-really-complicated}

Since the goal is fine-grained high-fidelity interop with C++, with the ability
to substitute in Rust anywhere you'd use C++, this requires adding approximately
100% of C++'s complexity into Rust. We save a little complexity here and there
because e.g. the style guide forbids things, or they are absurdly uncommon and
can be dealt with by hand, but for the most part, this complexity is the cost of
doing fine-grained interop.

This can be avoided by doing non-fine-grained interop, at module boundaries. In
that case, view this (and other proposals) as an insurance policy for the
absolute worst case, where we are forced to deal with C++ in a granular,
high-fidelity way. At the very least, we want that to remain *possible*.

The overall approach is described more in
[High-level Design of C++/Rust Interop](https://docs.google.com/document/u/0/d/1FDBHv0qQQpvV8URRW1crgKWlVm3UEh87pJrmFAxuFrI/edit).
