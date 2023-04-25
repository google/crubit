# [Pre-RFC] Allow stride != size

## Summary

Rust should allow for values to be placed at the next aligned position after the
previous value, ignoring the tail padding of that previous field. This requires
changing the meaning of "size", so that a value's size in memory (for the
purpose of reference semantics and layout) is not definitionally the same as the
distance between consecutive values of that type (its "stride").

## Motivation

Some other languages (C++ and Swift, in particular) can lay out values more
compactly than Rust conventionally can, leading to better performance at greater
convenience, and less than ideal Rust interoperability.

### Optimization opportunity

Consider the difference between `(u16, u8, u8)` and `((u16, u8), u8)`. The first
can fit in 4 bytes, while the second requires 6. A `(u16, u8)` is a 4 byte value
with 1 byte of tail padding. And a `(T, u8)` can't just stuff the `u8` inside
the tail padding for `T`! If, instead, we declared that `(u16, u8)` were a **3**
byte value with alignment 2, then `((u16, u8), u8)` could be 4 bytes instead of
6. This is not possible today.

(For backwards compatibility reasons described later, we can't literally do this
for tuples, but only for user-defined types. But this gives the gist of the
optimization opportunity this proposal supports.)

By inventing the concept of a "data size", which doesn't need to be a multiple
of the alignment, we can allow fields in specially-designed types to be packed
closer together than they would be today, saving space. This is similar to the
performance benefits of `#[repr(packed)]`, but safer: all values would still be
correctly aligned, just placed more closely together.

This optimization has already been implemented in other programming languages.
Swift applies this to every type and every field: a type's size excludes tail
padding, and a neighboring value can be laid out immediately next to it when
stored in the same type, with no padding between the two. In C++, the
optimization automatically applies to base classes
(["EBO"](https://en.cppreference.com/w/cpp/language/ebo), the Empty Base
Optimization), and is opt-in on fields via the
[`[[no_unique_address]]`](https://en.cppreference.com/w/cpp/language/attributes/no_unique_address)
attribute.

For example, here's an example in [Swift](https://godbolt.org/z/G74ejjsvc) and
in [C++](https://godbolt.org/z/4esbYrv39). These types are compact! Rust does
not work like this today.

### Interoperability with C++ and Swift

(Note that the author works on C++ interop, Swift is mentioned for
completeness.)

In fact, exactly because this optimization is already implemented in other
languages, those languages are theoretically not as compatible with Rust as they
are with each other. In C++ and Swift, writing to a pointer or reference does
not write to neighboring fields. But if that pointer or reference were passed to
Rust, and you used any Rust facility to write to it -- whether it were vanilla
assignment or `ptr::write` -- Rust could overwrite that neighboring field.
Because the use of this optimization is pervasive in both Swift and C++,
interoperating with these languages is difficult to do safely.

Concretely, consider the following C++ struct:

```c++
struct MyStruct {
    [[no_unique_address]] T1 x;
    [[no_unique_address]] T2 y;
    ...
};
```

Which is equivalent to this Swift struct:

```swift
struct MyStruct {
    let x: T1
    let y: T2
    ...
}
```

If you are working with cross-language interop, and obtain in Rust a `&mut T1`
which refers to `x`, and a `&mut T2` which refers to `y`, it may be immediately
UB, because these references can overlap in Rust: `y` may be located inside what
Rust would consider the tail padding of the `T1` reference.

For the same reason, even if you avoid aliasing, if you obtain a `&mut T1` for
`x`, and then write to it, it may partially overwrite `y` with garbage data,
causing unexpected or undefined behavior down the line.

This also cannot be avoided by forbidding the use of `MyStruct`: even if you do
not directly use it from Rust, from the point of view of Swift and C++, it is
just a normal struct, and Swift and C++ codebases can freely pass around
references and pointers to its interior. Someone passing a reference to a `T1`
may have no idea whether it came from `MyStruct` (unsafe to pass to Rust) or an
array (safe). You would need to ban (or correctly handle) any C++ and Swift type
which can have tail padding, in case that padding contains another object.

(To add insult to injury, the struct `MyStruct` itself -- not just references to
fields inside it -- cannot be represented directly as so in Rust, either.)

And anyway, such structs are unavoidable. In Swift, this is the default
behavior, and pervasive. In C++, `[[no_unique_address]]` is permitted to be used
pervasively in the standard library, and it is impractical to only interoperate
with C++ codebases that avoid the standard library.

In order for C++ and Swift pointers/references to be safely representable in
Rust as mut references, a `&mut T1` would need to exclude the tail padding,
which means that Rust would need to separate out the concept of a type's
interior size from its array stride. And in order to represent `MyStruct` in
Rust, we would need a way to use the same layout rules that are available in
these other languages.

## Explanation

(I haven't separated this out to guide-level vs reference-level -- this is a
pre-RFC! Also, all names TBD.)

As a quick summary, the proposal is to introduce the following new traits,
functions, and attributes, and behaviors:

*   `std::mem::data_size_of<T>()`, returning the size but not necessarily
    rounded to alignment / not necessarily the same as stride.
*   In the memory model, pointers and references only refer to
    `data_size_of::<T>()` bytes.
*   `AlignSized`, a trait for types where the data size and stride are the same.
*   `#[repr(compact)]`, to mark a type as not implementing `AlignSized`, and
    thus having a potentially smaller data size.
*   `#[compact]`, to mark a field as laid out using the data size instead of the
    stride.

## Data size vs stride

Semantically, Rust types would gain a new kind of size: "data size". This is the
size of the type, minus the tail padding. In fact, it's in some sense the "true"
size of the type: array stride is the data size rounded up to alignment.

Data size would be exposed via a new function `std::mem::data_size_of::<T>()`;
array stride continues to be returned by `std::mem::size_of::<T>()`.

The semantics of a write (e.g. via `ptr::write`, `mem::swap`, or assignment) are
to only write "data size" number of bytes, and a `&T` or `&mut T` would only
refer to "data size" number of bytes for the purpose of provenance and aliasing
semantics. (`&[T; 1]`, in contrast, continues to refer to `size_of::<T>()`
bytes.)

## The `AlignSized` trait and `std::array::from_ref`

It is fundamentally a backwards-incompatible change to make stride and size not
the same thing, because of functions like
[`std::array::from_ref`](https://doc.rust-lang.org/stable/std/array/fn.from_ref.html)
and
[`std::slice::from_ref`](https://doc.rust-lang.org/stable/std/slice/fn.from_ref.html).
The existence of these functions means that Rust guarantees that for an
arbitrary generic type today, that type has identical size and stride.

This means that if we want to allow for data size and stride to be different,
they must not be different for any generic type as written today. Existing code
without trait bounds can call `from_ref`! So we must add an implicit trait bound
on `AlignSized : Sized`, which, like `Sized`, guarantees that the data size and
the stride are the same. This trait would be automatically implemented for all
pre-existing types, which retain their current layout rules.

In other words, the following two generics are equivalent:

```rs
fn foo<T>() {}
fn foo<T: Sized + AlignSized>() {}
```

... and to opt out of requiring `AlignSized`, one must explicitly remove a trait
bound:

```rs
fn foo2<T: ?AlignSized>() {}
// AlignSized requires Sized, and so this will also do it:
fn foo3<T: ?Sized>() {}
```

To opt out of implementing this trait, and to opt in to being placed closer to
neighboring types inside a compound data structure, types can mark themselves as
`#[repr(compact)]`. This causes the data size not to be rounded up to alignment:

```rs
#[repr(C, compact)]
struct MyCompactType(u16, u8);
// data_size_of::<MyCompactType>() == 3
// size_of::<MyCompactType>() == 4
```

## Taking advantage of non-`AlignSized` types with `#[compact]` storage

If a field is marked `#[compact]`, then the next field is placed after the data
size of that field, not after the stride. (These can only differ for a
non-`AlignSized` type.) This provides easy control, and provides compatibility
with C++, where this behavior can be configured per-field.

It is an error to apply this attribute on non-`#[repr(C)]` types.

```rs
#[repr(C, compact)]
struct MyCompactType(u16, u8);

#[repr(C)]
struct S {
    #[compact]
    a: MyCompactType,  // occupies the first 3 bytes
    b: u8,             // occupies the 4th byte
}
// data_size_of::<S>() == size_of::<S>() == 4
```

## Example

Putting everything together:

```rs
#[repr(C, compact)]
struct MyCompactType(u16, u8);
// data_size_of::<MyCompactType>() == 3
// size_of::<MyCompactType>() == 4

#[repr(C)]
struct S {
    #[compact]
    a: MyCompactType,  // occupies the first 3 bytes
    b: u8,             // occupies the 4th byte
}

// data_size_of::<S>() == size_of::<S>() == 4
```

We can take `mut` references to both fields `a` and `b`, and writes to those
references will not overlap:

```rs
let mut x : S = ...;
let S {a, b} = &mut x;
*a = MyCompactType(4, 2);  // writes 3 bytes
*b = 0;  // writes 1 byte
```

If we had not applied the `repr(compact)` attribute, **or** had not applied the
`#[compact]` attribute, then `data_size_of<S>()` would have been 6, and so would
`size_of<S>()`. The assignment `*a = ...` would have (potentially) written 4
bytes.

## Drawbacks

### Backwards compatibility and the `AlignSized` trait

In order to be backwards compatible, this change requires a new implicit trait
bound, applied everywhere. However, that makes this change substantially less
useful. If that became the way things worked forever, then `#[repr(compact)]`
types would be very difficult to use, as almost no generic functions would
accept them. Very few functions *actually* need `AlignSized`, but every generic
function would get it implicitly.

We could change this at an edition boundary: a later edition could drop the
implicit `AlignSized` bound on all generics, and automated migration tooling
could remove the implicit bound from any generic function which doesn't use the
bound, and add an explicit bound for everything that does. After enough
iterations, the only code with a bound on `AlignSized` would be code which
transmutes between `T` and `[T]`/`[T; 1]`. Though this would be a disruptive and
long migration.

Alternatively, we could simply live with `repr(compact)` types being difficult
and usually not usable in generic code. They would still be useful in
non-generic code, and in cross-language interop.

### `alloc::Layout`

`std::alloc::Layout` might not work as is. Consider the following function:

```rs
fn make_c_struct() -> Layout {
    Layout::from_size_align(0, 1)?
        .extend(Layout::new::<T1>())?.0
        .extend(Layout::new::<T2>())?.0
        .pad_to_align()
}
```

This function was intended to return a `Layout` that is interchangeable with
this Rust struct:

```rs
#[repr(C)]
struct S {
  x: T1,
  y: T2,
}
```

In order for this to continue returning the same `Layout`, it must work the same
even if `T1` is changed to be `repr(compact)`. In other words, if `Layout::new`
is to accept `?AlignSized` types, it must use the stride as the size. The same
applies to `for_value*`.

(Alternatively, it may be okay to reject non-`AlignSized` types.)

One assumes, then, that we need `*_compact` versions of all the layout
functions, which use data size instead of stride. And then:

```rs
fn make_c_struct() -> Layout {
    Layout::from_size_align(0, 1)?
        .extend(Layout::new_compact::<T1>())?.0
        .extend(Layout::new::<T2>())?.0
        .pad_to_align()
}
```

Would generate the same `Layout` as for the following struct:

```rs
#[repr(C)]
struct S {
  #[compact] x: T1,
  y: T2,
}
```

Alternatively, perhaps we could introduce separated `data_size` and `stride`
fields into the `Layout`, and have `extend` and `extend_compact`, supplementing
`from_size_align(stride, align)` with `from_data_size_stride_align(data_size,
stride, align)`.

... but this author is very interested to hear opinions about how this should
all work out.

### It's yet another (implicit) size/alignment trait

There is also some desire for
[an `Aligned` trait](https://internals.rust-lang.org/t/aligned-trait/17443) or
[a `DynSized` trait](https://github.com/rust-lang/rust/issues/43467#issuecomment-317733674).
This would be yet another one, which may require changes throughout the Rust
standard library and ecosystem to support everywhere one would ideally hope.

## Rationale and alternatives

### Alternative: manual layout

One could in theory do it all by hand.

#### User-defined padding-less references

Instead of references, one could use `Pin`-like smart pointer types which
forbids direct writes and reads. To avoid aliasing UB, this cannot actually be
`Pin<&mut T>` etc. -- it must be a (wrapper around a) raw pointer, as one must
never actually hold a `&mut T` or even a `&T`. This must be done for *all* Swift
or C++ types which contain (what Rust would consider) tail padding, unless it is
specifically known that they are held in an array, where it's safe to use Rust
references.

Something like this:

```rs
struct PadlessRefMut<'a, T>(*mut T, PhantomData<&'a mut T>);
```

Unfortunately, today, a generic type like `PadlessRefMut` is difficult to use:
you cannot use it as a `self` type for methods, for instance, though
[there are workarounds](https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Extending.20.60arbitrary_self_types.60.20with.20.60UnsafeDeref.60).

Even there, various bits of the Rust ecosystem expect references: for instance,
you can't return a `PadlessRef` or `PadlessRefMut` from an `Index` or `IndexMut`
implementation. This, too, could be fixed by replacing the indexing traits (and
everything else with similar APIs) with a more general trait that uses GATs...
but we can see already that, at least right now, this type would be quite
unpleasant.

#### Layout

For emulating the layout rules of Swift and C++, you could manually lay out
structs (e.g. via a proc macro) and use the same `Pin`-like pointer type:

```rs
// instead of C++:
//     `struct Foo {[[no_unique_address]] T1 x; [[no_unique_address]] T2 y; }`
##[repr(C, align( /* max(align_of<T1>(), align_of<T2>()) */ ... ))]
struct Foo {
    // These arrays are not of size size_of<T1>() etc., but rather the same as the proposed data_size_of<T1>().
    x: [u8; SIZE_OF_T1_DATA],
    y: [u8; SIZE_OF_T2_DATA],
}

impl Foo {
    fn x_mut(&mut self) -> PadlessRefMut<'_, T1> {
        PadlessRefMut::new((&mut self.x).as_mut_ptr() as *mut _)
    }
    // etc.
}
```

This is especially easy to do when writing a bindings generator, since you can
automatically query the other language's to find the struct layout, and
automatically generate the corresponding Rust.) But otherwise, it's quite a
pain -- one would hope, perhaps, for a proc macro to automate this, similar to
how Rust automatically infers layout for paddingful structs and types.

#### Conclusion: manual layout is unpleasant

Almost nothing is impossible in Rust, including this. But it does mean virtually
abandoning Rust in a practical sense: Rust's references cannot exclude tail
padding, so we use raw pointers instead. Rust's layout rules cannot omit
padding, and so we replace the layout algorithm with a pile of manually placed
`u8` arrays and manually specified alignment. And the result integrates poorly
with the rest of the Rust ecosystem, where most things expect conventional
references, and things that don't or can't use references are difficult to work
with.

### Alternative: `repr(packed)`, but with aligned fields

We could replicate the layout of C++ and Swift structs, but make them very
unsafe to use, similar to `repr(packed)`. One would still, like `repr(packed)`,
avoid taking or using references to fields inside such structs, and these are
still going to be difficult to work with as a result.

## Prior art

### Languages with this feature

**Swift:** Swift implicitly employs this layout strategy for all types and all
fields. A type has three size-related properties: its "size", meaning the
literal size taken up by its field, not including padding; its "stride", meaning
the difference between addresses of consecutive elements in an array; and its
alignment.

**C++:** Unlike Swift, C++ does not separate out size and stride into separate
concepts. Instead, it claims that array stride and size are the same thing, as
they are in Rust and C, but that objects can live inside the tail padding of
other objects and that you are simply mutably aliasing into the tail padding in
a way which the language defines the behavior for. C++ nominally allows this for
the tail padding of all types, but only when they are stored in certain places:
objects may be placed inside the tail padding of the previous object when that
previous object is a subobject in the same struct (not, for instance, a separate
local variable), and it is either a base class subobject (so-called "EBO"), or a
`[[no_unique_address]]` data member ("field"). In practice, however, the
compiler is free to not reuse the tail padding for some types. In the
[Itanium ABI](https://itanium-cxx-abi.github.io/cxx-abi/abi.html), C-like
structs ("POD" types, with
[an Itanium-ABI-specific definition of "POD"](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#POD))
do not allow their tail padding to be reused.

### Papers and blog posts

*   I worked around this in Crubit, a C++/Rust bindings generator. The design is
    here: https://github.com/google/crubit/blob/main/docs/unpin.md . tl;dr: if
    we assume that the only source of this layout phenomenon is base classes,
    then only non-`final` classes needed to get the uncomfortable `Pin`-like
    API. Unfortunately, this does not work if `[[no_unique_address]]` becomes
    pervasive.

## Unresolved questions

-   What do we do about `std::alloc::Layout`?
-   What's the long term future of the `AlignSized` bound?
-   Clearly, for compatibility reasons if nothing else, Rust types must not have
    reusable tail padding unless specially marked. But what about fields: should
    it be opt-in per field (like C++), or automatic (like Swift)? In this doc,
    it's assumed to be opt-in per field for `repr(C)` (for C++-compatibility),
    and automatic for `repr(Rust)`.
-   How free should Rust be to represent fields compactly in `repr(Rust)` types?
-   Is `repr(C)` allowed to use this new layout strategy with specially marked
    fields using a new attribute, or do we need a new `repr`? The documentation
    is
    [very prescriptive](https://doc.rust-lang.org/std/mem/fn.size_of.html#size-of-reprc-items).
-   This is part of a family of issues with interop, where Rust reference
    semantics do not match other languages' reference semantics. (The other
    prominent member of the family is "aliasing".) Part of the reason for
    wanting to use Rust references is simply the raw ergonomics: generic APIs
    take and return `&T`, self types requires `Deref` (which requires
    reference-compatible semantics), etc. It is worth asking: rather than
    modifying references, does this cross the line to where we should instead
    make it more pleasant to use pointers that cannot safely deref?
-   "Language lawyering": how does this interact with existing features? For
    example, is a `repr(transparent)` type also `repr(compact)`? (I *believe*
    the answer should be yes.)
-   TODO:  `repr(compact -`, "data size"
    and `data_size_of`. `AlignSized` especially.
-   How much of the standard library should be updated to `?AlignSized`?
