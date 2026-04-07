# Problem Description: forward-declared types

C++ allows for the same type to be defined in multiple places, where each of
these declarations refers to *the same type*. Types in C++ are not "owned" by
any particular target, and anyone can redefine it as a forward declaration /
opaque enum, so long as their definition is compatible with the complete
definition.

```cpp
// h1.h
class Foo;  // a forward declaration
enum Bar: int;  // an opaque enum declaration
```

```cpp
// h2.h
class Foo;  // the same type as Foo above
enum Bar: int;  // the same type as Bar above
```

```cpp
// h3.h
class Foo {};  // the same type, but if this is included, it now has known size/methods
enum Bar: int {};  // the same type, but if this is included, enumerators are known
```

The same is also true, implicitly, of template specializations: the same class
template can be instantiated from multiple places, and each instantiation refers
to the *same type*, even if they are totally independent.

This is basically incompatible with Rust, where types have one canonical owner:
two different targets *cannot* define the same type, and have it be the same in
type, except for the special cases of tuple types (which are structurally
typed), generic instantiations, and `dyn Trait`. Thankfully, this does mean we
have tools at our disposal to emulate what C++ can do.

## Design axioms for a solution {#axioms}

Example headers:

```cpp
// incomplete1.h
class Foo;
Foo* CreateIncomplete1();
void ConsumeIncomplete1(Foo*);
```

```cpp
// incomplete2.h
class Foo;
Foo* CreateIncomplete2();
void ConsumeIncomplete2(Foo*);
```

```cpp
// complete.h
class Foo {};
Foo* CreateComplete();
void ConsumeComplete(Foo*);
```

### 1. Dependency Graph {#dependency-graph}

The libraries must be usable standalone. For each of these headers, it must be
possible to use the header from Rust without modifying any of them to become
aware of the others. In C++, it would be valid for `complete` to depend on
`incomplete1`, and for `incomplete2` to depend on `complete`, so we can't
require modifications of the dependency graph without grappling with dependency
cycles and a global analysis of the codebase.

### 2. Fidelity {#fidelity}

The functions must be usable. Users must be able to call every combination of
functions from these three libraries, though perhaps not with the same syntax as
C++. For example:

<!-- mdformat off(multiline tables) -->

|     | Operation                              | Example code                              |
| --- | -------------------------------------- | ----------------------------------------- |
| A   | Incomplete → Incomplete (same library) | `ConsumeIncomplete1(CreateIncomplete1())` |
| B   | Incomplete → Incomplete                | `ConsumeIncomplete1(CreateIncomplete2())` |
| C   | Incomplete → Complete                  | `ConsumeComplete(CreateIncomplete1())`    |
| D   | Complete → Incomplete                  | `ConsumeIncomplete1(CreateComplete())`    |
| E   | Complete → Complete                    | `ConsumeComplete(CreateComplete())`       |

<!-- mdformat on -->

### 3. Code Maintenance {#code-maintenance}

To the greatest extent possible, backwards compatible changes in C++ should also
be backwards-compatible in Rust. This means that changing from a forward
declaration to a complete type (or dependency on the complete type) should not
be a breaking change to Rust callers. It is fine for the reverse to break
callers: it is expected that switching to forward declarations might break
callers.

In particular, that means that the exact same source code should continue to
compile and behave the same way if we perform any of the following changes:

*   A→E (Incomplete→Incomplete, but change it to Complete)
*   B→C (Incomplete→Incomplete, change recipient to Complete)
*   B→D (Incomplete→Incomplete, change source to Complete)
*   B→E (Incomplete→Incomplete, change source and destination to Complete)
*   C→E (Incomplete→Complete, change source to Complete)
*   D→E (Complete→Incomplete, change destination to Complete)

### 4. Type Aliases {#type-aliases}

This is a bit of an aside, but it is tricky to get right. In general, when
analyzing the source code for a C++ header, you cannot actually know if a type
was complete or incomplete from the perspective of a foreign library. In
particular, given the following header:

```cpp
// alias.h
#include "..."
using FooAlias = Foo;

// complete.h
#include "alias.h"
struct Foo {};
void Foo(FooAlias);
```

If you are analyzing `alias.h` from a transitive include graph that includes a
complete definition for `Foo`, it is *difficult* to work out whether `FooAlias`
will always be complete in any include graph or not. Is `FooAlias` complete
because `alias.h` transitively included `complete.h`, or is `Foo` complete
because something else happened to both include `alias.h` and also completed the
type?

The naive approach, analyzing `alias.h` purely on the basis of what we know in
the caller, can produce a compilation error: from the POV of `complete.h`,
looking at the C++ AST, `FooAlias` is the same type as `Foo`, and so reusing the
type alias in the generated Rust function looks fine: `pub fn Foo(_:
alias::FooAlias)`. Unfortunately, if `alias.h` does not have a complete type
definition when viewed alone, this results in compilation errors in the
generated code.

This matters for type aliases -- and only type aliases -- because types are the
only thing that can affect public APIs in Crubit (if we "mispredict" a type, we
get compilation errors), and because type aliases are the only place where this
confusion surfaces as a top-level type. Whether a type is forward-declared or
not never changes the type properties of a struct, because if it could be
either, then it will only ever be held behind e.g. a pointer or similar compound
data type which erases any difference from the top level POV. A struct will be
trivial or not independent of whether the `T` in its `T*` field is complete or
incomplete. We can still be confused and "mispredict" the types of function
parameters or fields in other libraries, but because Crubit does not generate
calls or field accesses to entities defined in foreign libraries, this is never
an issue.

Any mapping of forward declared types (and other redeclarable types) to Rust
should evade this problem somehow! Some options:

1.  Track the include chain so that there is absolute certainty about whether a
    type is complete because it includes the completed definition, instead of
    simply asking Clang whether it's complete or not.
2.  (What we do today) **do not use type aliases in other libraries**, at least
    if the type alias refers to a complete type that could hypothetically be
    incomplete.
3.  (Impossible?) make it not a compilation error to guess wrong, somehow.
4.  Don't use the C++ AST to determine if aliases in a dependency are to a
    complete type.
    1.  Use an IDL approach instead of Crubit's "automatic" approach.
    2.  Store the answer to the question and make it available to people that
        depend on you. (Linearize the build.)

## Non-solution: extern types

Rust has an (unstable) "extern types" feature, usable as so:

```rust
unsafe extern "C" {
  type Foo;
}
```

This defines a `PointeeSized` type `Foo` -- analogous to C++'s "incomplete type",
a `PointeeSized` type has no known size/alignment and cannot be held by value,
only behind a pointer.

This does not, by itself, present a solution to the design problem above. Two
different crates which define an extern type `Foo` are not, from Rust's point of
view, defining the *same* type `Foo`, and there is no way to define a non-extern
type `Foo` which is the same ("completed") type. This means that, absent
additional work, design axiom 2 (and/or 1) are not satisfied. However, we can
build on it to define an auxiliary wrapper type or set of conversions.

(Extern types also do not help with opaque enums, or the related problem of
"redeclared" template instantiations.)

## Existing Solution: `forward_declare.rs` {#existing-solution}

Very briefly, `forward_declare` currently only supports forward-declared struct
types (not opaque enum definitions), which is present as a simple wrapper around
an extern type with conversions and reasonably safe type properties (e.g.
`!Send`, `!Sync`, `!Unpin`).

The core idea is to define equivalence classes of "types which are different in
Rust, but the same type in C++", via the `CppCast` trait. If two types are the
same type in C++, but forced to be different types in Rust, then you should be
able to call `.cpp_cast()` to perform the conversion. For example, the
equivalent of C++ `ConsumeIncomplete1(CreateComplete())` is
`ConsumeIncomplete1(CreateComplete().cpp_cast())`.

For the sake of code maintenance concerns, above, always use a unique per-crate
forward declared type. If you hold an `x: &Foo` (incomplete), and you call into
a library which accepts a `&Foo` (incomplete), then even though these are "the
same" -- they're both forward declarations to the same class -- we force the use
of `.cpp_cast()`. This ensures that if either you or the library you are calling
ever switch to including the complete type definition, code will continue to
compile unchanged. In other words, we force a call to `.cpp_cast()` just in case
it ever becomes necessary later.

This means that the table of calls above, in [2. Fidelity](#fidelity), is as
follows:

<!-- mdformat off(multiline tables) -->

|     | Operation                              | Example code                                         |
| --- | -------------------------------------- | ---------------------------------------------------- |
| A   | Incomplete → Incomplete (same library) | `ConsumeIncomplete1(CreateIncomplete1())`            |
| B   | Incomplete → Incomplete                | `ConsumeIncomplete1(CreateIncomplete2().cpp_cast())` |
| C   | Incomplete → Complete                  | `ConsumeComplete(CreateIncomplete1().cpp_cast())`    |
| D   | Complete → Incomplete                  | `ConsumeIncomplete1(CreateComplete().cpp_cast())`    |
| E   | Complete → Complete                    | `ConsumeComplete(CreateComplete())`                  |

<!-- mdformat on -->

In theory we could have dropped the requirement for `.cpp_cast()` from case B,
but this would make it a backwards-**incompatible** change to do any of B→C or
B→D, which was called out as a requirement above in
[3. Code Maintenance](#code-maintenance)

### Mutable Details

The following details to how `forward_declare` works are not core to how it
satisfies the design axioms.

#### How are types determined to be the same?

At its core, `forward_declare` has a blanket impl that goes something like this:

```rust
impl<T, U> CppCast<U> for T where same_type!(T, U) {...}
```

How do we create this "same type" relationship when neither T nor U can directly
mention each other? The answer is to lean on something I mentioned *way* at the
start: Rust *does* have a couple of ways of defining the same type from multiple
targets.

In particular, today, `forward_declare` does the following:

```rust
/// # Safety
/// if `X : CppType<Name=Y>`, then `X` has the same layout and set of valid
/// representations as everything with name `Y`
pub unsafe trait CppType {
  type Name;
}

impl<T, U> CppCast<U> for T where T: CppCast, U: CppCast<Name=T::Name> {...}
```

With this, we can lean on any way of declaring two types to be the same. In
stable Rust, you can do it like so:

```rust
pub struct C<C: const char>;
unsafe impl CppType for Foo {
  type Name = (C<'F'>, C<'o'>, C<'o'>);
}
```

But this produces *truly unreadable* error messages, and we can do the same
trick a bit better in unstable Rust:

```rust
#![feature(adt_const_params, unsized_const_params)]
pub struct Symbol<S: const &'static str>;
unsafe impl CppType for Foo {
  type Name = Symbol<"Foo">;
}
```

(I mentioned above that traits also are structural -- but please do not do `trait
C<I: usize, C: const char> {} type Name = dyn C<0, 'F'> + C<1, 'o'> + C<2,
'o'>;`)

##### Alternative: constant unification

Instead of using *type* unification, and relying on unique constants to
distinguish types, unstable Rust does allow actually directly using constant
unification in trait bounds:

```rust
#![feature(generic_const_exprs, min_generic_const_args, unsized_const_params, adt_const_params)]
pub trait CppType {
    type const Name: &'static str;
}
impl<T, U> CppCast<U> for T where T: CppCast, U: CppCast<Name={T::Name}> {...}
```

The main downside to this is that it does not have a clean "fallback" if the
unstable features are revoked or changed substantially. With the type based
approach, users can directly write something `type Name = symbol!("abc")` and we
can hide whether `symbol!()` expands to a tuple or a string container type. The
direct use of constants has the risk of leaking into more places and being
harder to unwind.

#### How are forward declarations made to be unique?

This is the part of `forward_declare` I think is the most likely to be outright
wrong!

Currently, forward declare defines a type as follows:

```rust
pub struct Incomplete<Name, Crate>(...);

unsafe impl<Name, Crate> CppType for Incomplete<Name, Crate> {
  type Name = Name;
}
```

Users that wish to use `Incomplete` create a local alias, which defines a
crate-local `Crate` type. The equivalent of C++ `struct Foo;` is Rust
`forward_declare!(pub Foo = symbol!("Foo"));`, which expands to:

```rust
pub struct _forward_declare_Foo;
pub type Foo = Incomplete<symbol!("Foo"), _forward_declare_Foo>;
```

This means that every user of forward declarations is using the same
`Incomplete` type, but with different type parameters to make them distinct
types. This does have the following advantages compared to the obvious
alternative of explicitly defining a new type:

1.  Users do not need to unsafely implement a trait to consume incomplete types
    in pure Rust code. They can simply use `Incomplete<...>`, even via a helper
    macro to declare it, and there is no unsafe code anywhere here. (The
    `CppType` trait itself must necessarily be unsafe.)
2.  Theoretically, only requires defining one extra type per crate, instead of N
    (where N is the number of forward declarations).
3.  Extends easily to "comprehensive fallbacks": if a function is private, it
    can use `Incomplete<Name, ()>`. For understandable architectural reasons, it
    is simpler to *use* a type from deep within our bindings generation logic,
    than to compel Crubit to define a new type, and so "comprehensive fallbacks"
    -- where we substitute in a forward declaration any time we see an
    unsupported type -- is much easier to support if we can reuse an existing
    type.

As well as the following downsides:

1.  Users can be generic over `Incomplete<Name, _>` when they should instead be
    generic over `T: CppType<Name=Name>` (which also includes the complete
    type).
2.  Because it's a foreign type, forward declarations can't add extra trait
    impls or inherent impls.
3.  It is kind of strange. If you want a new type, shouldn't you just… define
    that type?

##### Alternative: explicit definition

Instead, forward declarations could expand to:

```rust
pub struct Foo;
unsafe impl CppType for Foo {type Name = symbol!("Foo");}
```

### Missing Features

#### Enums

Unlike class types, opaque "forward-declared" enums have a known size. If
`forward_declare` kept its current direction, one would likely imagine that they
are implemented as something like:

```rust
struct OpaqueEnum<Name, Underlying>(pub Underlying);
// Add Underlying to the name so that these are actually safe to use --
// if we just used `Name`, then totally safe code would get UB when it did
// something stupid like mix OpaqueEnum<symbol!("Bar", i32) with
// OpaqueEnum<symbol!("Bar"), i8>
unsafe impl CppType<Name, Underlying> for Foo {type Name = (Name, Underlying);}
```

And the equivalent of `enum Bar: int32_t;` would be:

```rust
pub type Bar = forward_declare::OpaqueEnum<symbol!("Bar"), i32>;
```

#### Conversions

There are a *lot* of conversions that need to exist for the `CppType`
equivalence class setup, and only a small fraction are implemented. For example,
we currently implement conversions for `Vec`, but not `std::vector`.

#### Compatibility for `Pin`

The compatibility story above is imperfect when the complete type is non-Rust
movable, and the type is behind a mutable reference.

When the complete type is Rust-movable, then moving referents from incomplete to
complete means moving from (something like) `Pin<&mut T>` to `&mut T`. This is
not in general backwards-compatible.

In particular, this breaks calls of the form `<...>.as_mut().cpp_cast()`: if
`<...>` was incomplete (and therefore pinned), but becomes complete (and
therefore Rust-movable and behind only a `&mut T`), then the calls stop working,
as `&mut T` doesn't have a `.as_mut()` method.

Changing the *destination* to be complete can be made to work: if `.cpp_cast()`
also can wrap in a `Pin`, then `cpp_cast`-conversion of `&mut T` to `Pin<&mut T>`
is the same syntax as `cpp_cast`-conversion of `&mut T` to itself.

Full matrix, in the presence of a reborrow:

    | Operation                              | Example code
--- | -------------------------------------- | ------------
A   | Incomplete → Incomplete (same library) | `ConsumeIncomplete1(CreateIncomplete1().as_mut())`
B   | Incomplete → Incomplete                | `ConsumeIncomplete1(CreateIncomplete2().as_mut().cpp_cast())`
C   | Incomplete → Complete                  | `ConsumeComplete(CreateIncomplete1().as_mut().cpp_cast())`
D   | Complete → Incomplete                  | `ConsumeIncomplete1(CreateComplete().cpp_cast())`
E   | Complete → Complete                    | `ConsumeComplete(CreateComplete())`

Moving from A/B/C to D/E is compatibility breaking if a call to `.as_mut()` was
present.

##### Solutions

1.  Pin ergonomics might make reborrowing a pinned reference implicit. We will
    still suffer from all the other reasons that `Pin<&mut T>` is not 1:1
    compatible with `&mut T`, but these can't be made perfectly compatible.

2.  The actual cause is not forward declarations per se, but the fact that
    Crubit conditionally uses `Pin`, which is a compatibility hazard. If we
    always used `Pin<&mut T>`, or always used a type like `CMut<'_, T>`, which
    implicitly pins when possible, then the compatibility concerns for forward
    declarations vanish.
