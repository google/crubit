# Reference Safety

The Rust reference documents [Undefined Behavior
(UB)](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
and says that "it is the programmer's responsibility when writing unsafe code to
ensure that any safe code interacting with the unsafe code cannot trigger these
\[undefined\] behaviors". A programmer using Crubit bindings has the same
responsibility: Crubit is implicitly "unsafe", and incorrect usage can cause UB.
The sections below document requirements for safely using Crubit when working
with Rust and C++ references and pointers. The requirements are the same as the
ones [documented in the Rust
reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html),
but are rephrased below from Crubit perspective.

The safety requirements below focus on avoiding UB related to Rust references
and therefore matter in scenarios where Crubit-generated code may create Rust
references out of C++ references or C++ pointers:

-   Today: Using Crubit-generated Rust bindings for C++ APIs if the bindings
    accept, store, or return Rust references. TODO: In the future Crubit should
    use `CppRef<T>` in the generated bindings (and this should mitigate some of
    the memory safety concerns).
-   Using Crubit-generated C++ bindings for Rust APIs if the bindings wrap Rust
    APIs that accept, store, or return Rust references.

## Incorrect C++ lifetime annotations

Incorrect lifetime annotations may lead to UB. Rust's borrow checker prevents
incorrect lifetime annotations, but lifetime annotations of C++ APIs are not
verified by the C++ compiler and Crubit's optional lifetime analysis can't
detect all incorrect annotations. Note that Crubit assumes that lifetime
annotations are correct both for explicit annotations (e.g.
`int& $a f2(int& $a);`) as well as for annotations provided by
`#pragma clang lifetime_elision`.

## C++ mutating values referenced by Rust

Mutating a value in C++ is UB if the mutation happens while Rust holds a live
reference (`&T` or `&mut T`) to that value, unless the mutated bytes are inside
an `UnsafeCell` (such as `core::cell::Cell` or a `CRUBIT_THREAD_SAFE` type) and
any concurrent access is properly synchronized.

Examples of C++ features that may mutate a value that Rust holds a reference to:

*   Using copy or move assignment operator of C++ value that Rust has a
    reference to.
*   Mutating public fields of a C++ struct that Rust has a reference to.
*   Calling a C++ `const` method that mutates non-`mutable` fields (e.g., via
    `const_cast`) or mutating an unannotated value through an aliased C++
    pointer while Rust holds `&T`.

Short-lived, non-retained references are safe from concurrent mutation across
separate operations: Rust aliasing UB only applies while a `&T` or `&mut T` is
live. If Rust borrows `&T` briefly, drops the reference before external
synchronization allows C++ to mutate `T`, and never retains `&T` across the
mutation window, no aliasing violation occurs.

### How Crubit models C++ interior mutability (`UnsafeCell` and `Cell`) {#interior_mutability}

C++ has no direct equivalent to Rust's `UnsafeCell`: in C++, `const` expresses
logical non-mutability, whereas in Rust `&T` forbids any mutation of the
referenced bytes unless they are wrapped in `UnsafeCell`. To bridge the two
models without silently causing UB, Crubit models C++ interior mutability as
follows:

*   **`CRUBIT_THREAD_SAFE` types:** Crubit replaces the generated Rust struct
    body with an opaque `core::cell::UnsafeCell`, making all bytes of the type
    valid to mutate behind `&T` (provided the C++ object is never created as a
    top-level `const` without `mutable` fields).
*   **`mutable` fields and opaque state on non-`Copy` types:** Crubit wraps
    `mutable` public fields—and opaque byte blobs (private fields and padding)
    on non-`Copy` records—in `core::cell::Cell`. As a result, C++ `const`
    methods that mutate `mutable` fields (or opaque private fields on non-`Copy`
    types) do not violate Rust's `&T` rules.
*   **All other fields and `Copy` records:** These are *not* wrapped in
    `UnsafeCell` or `Cell`. Mutating them while Rust holds `&T` is UB.
*   **Pointer fields and smart pointers (`std::unique_ptr`,
    `std::shared_ptr`):** Mutating the *pointee* behind a pointer stored inside
    a struct does not mutate the struct's own bytes. However, dereferencing
    `&cpp_std::unique_ptr<T>` or `&cpp_std::shared_ptr<T>` produces a Rust `&T`
    (deep-const semantics); mutating `T` in C++ while that `&T` is live is UB
    unless `T` is annotated `CRUBIT_THREAD_SAFE`.

See [Behavior objects](../cpp/cookbook.md#behavior_objects) and
[Thread-safety](../cpp/cookbook.md#thread_safety) for practical guidance.

## Dangling or null references

All references and [`NonNull`
pointers](https://doc.rust-lang.org/std/ptr/struct.NonNull.html) must not be
null, and if they point to a nonzero span of memory, must not be dangling. (The
behavior of a program which violates these rules is undefined.)

C++ doesn't share these rules, and care must be taken when converting Rust
references to and from C++ pointers.

Spans/slices are particularly error-prone: a Rust empty slice (`&[]`) is
represented using a dangling pointer with length zero, while a C++ empty span
typically uses nullptr. Using a nullptr with a Rust empty slice would result in
UB, as would using a dangling pointer in C++. In order to address this, Crubit
provides non-native types with conversion operators:

*   Rust has `absl::{string_view, span<T>}` types which provide `From`-based
    conversions to `&str` and `&[T]`.
*   C++ has `rs::{StrRef, SliceRef<T>}` types which provide implicit conversions
    to and from `string_view` and `span<T>`.

## References to uninitialized memory or invalid values

Creating a Rust reference that points to uninitialized memory is UB.

Care should be taken when passing C++ references across the FFI boundary to
avoid creating Rust references that point to uninitialized memory. This is
especially important for references to types that don't enforce proper
initialization through their constructors or other factory APIs - primitive
types like integers are one example.

Note that Crubit-generated C++ bindings for Rust code won't create C++
references to uninitialized memory for:

*   Rust `struct`s, `enum`s, and `union`s - all the constructors generated by
    Crubit guarantee proper initialization
*   `rs::char_` when constructed in C++ via `char_::from_u32` or when passed
    from Rust. (`char_::from_u32_unchecked` is unsafe in Rust sense)

TODO: b/296287315: Support `MaybeUninit<T>`.

## Breaking Rust aliasing rules

Constructing a Rust reference that aliases the same memory as an already
existing exclusive reference `&mut T` (or mutating memory reachable through a
live `&T` outside of an `UnsafeCell`) is UB.

Common FFI pitfalls include:

*   **Aliasing `&mut T` from C++ pointers:** Turning two C++ pointers (`T*`)
    that point to the same object into `&mut T` simultaneously in `unsafe` Rust.
*   **Holding `&T` across C++ mutation of an unannotated type:** Dereferencing
    `&cpp_std::shared_ptr<T>` or `&cpp_std::unique_ptr<T>` yields a Rust `&T`.
    If C++ concurrently mutates `T` through another `std::shared_ptr<T>` or
    pointer while that `&T` is live, it is UB unless `T` is annotated
    `CRUBIT_THREAD_SAFE` (so that its representation lives in `UnsafeCell`).
*   **Mutating a `CRUBIT_THREAD_SAFE` object declared `const` in C++:** If a C++
    caller defines a `const MyThreadSafeType x;` without marking mutated fields
    `mutable` and passes it to Rust (or invokes a mutating method through a
    `CRUBIT_THREAD_SAFE` binding), mutating non-`mutable` fields of a `const`
    object is UB in C++.

Crubit enforces what it can statically through generated types—wrapping
`CRUBIT_THREAD_SAFE` structs in `UnsafeCell`, emitting non-`const` `T&`
parameters in `cc_bindings_from_rs` for `&T` on `CRUBIT_THREAD_SAFE` types,
wrapping `mutable` and non-`Copy` opaque fields in `Cell`, and only exposing
`Deref` (never `DerefMut`) on `cpp_std::shared_ptr<T>`. General runtime aliasing
checks across the C++/Rust boundary are infeasible because C++ does not track
borrow state or pointer provenance at runtime for arbitrary heap or stack
objects; avoiding these aliasing violations is therefore an API and `unsafe`
review responsibility (see [Memory Safety](./safety.md)).
