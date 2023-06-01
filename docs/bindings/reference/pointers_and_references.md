# Bindings for pointer and reference types

Here we describe how Crubit maps pointer and reference types.

## Rust bindings for C++ APIs

Rust bindings for lifetime-annotated C++ pointers and references look as
follows:

<!-- The examples in the table below are based on
`FunctionTakingPointersAndReferences` from
`rs_bindings_from_cc/test/golden/types_rs_api.rs`.  Note that
`FieldTypeTestStruct` can't be used because its fields are not
lifetime-annotated (lifetime elision doesn't work with structs). -->

C++ API       | Rust bindings
------------- | -------------------
`const T& $a` | `&'a T`
`T& $a`       | `&'a mut T`
`const T* $a` | `Option<&'a T>`
`T* $a`       | `Option<&'a mut T>`

TODO: Document how explicit lifetime annotations work. (A prerequisite might be
defining `$a` macros via a new header under `crubit/support`.)

TODO: Document how `#pragma clang lifetime_elision` works.

C++ pointers and references that are *not* annotated with lifetimes look as
follows:

C++ API    | Rust bindings
---------- | -------------
`const T&` | `*const T`
`T&`       | `*mut T`
`const T*` | `*const T`
`T*`       | `*mut T`

TODO: Document what happens for `void*`.

## C++ bindings for Rust APIs

Rust bindings for lifetime-annotated C++ pointers and references look as
follows:

Rust API   | C++ bindings
---------- | -------------------------------------
`*const T` | `const T*`
`*mut T`   | `T*`
`&T`       | TODO(b/258235219): Not supported yet.
`&mut T`   | TODO(b/258235219): Not supported yet.
`&str`     | TODO(b/262580415): Not supported yet.
`&mut str` | TODO(b/262580415): Not supported yet.
`&[T]`     | TODO(b/271016831): Not supported yet.
`&mut[T]`  | TODO(b/271016831): Not supported yet.

## Safety notes

Rust reference
[documents Undefined Behavior (UB)](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
and says that "it is the programmer's responsibility when writing unsafe code to
ensure that any safe code interacting with the unsafe code cannot trigger these
[undefined] behaviors". A programmer using Crubit bindings has the same
responsibility: Crubit is implicitly "unsafe", and incorrect usage can cause UB.
The sections below document requirements for safely using Crubit when working
with Rust and C++ references and pointers. The requirements are the same as the
ones
[documented in the Rust reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html),
but are rephrased below from Crubit perspective.

The safety requirements below focus on avoiding UB related to Rust references
and therefore matter in scenarios where Crubit-generated code may create Rust
references out of C++ references or C++ pointers:

-   Today: Using Crubit-generated Rust bindings for C++ APIs if the bindings
    accept, store, or return Rust references. TODO: In the future Crubit should
    use `CppRef<T>` in the generated bindings (and this should mitigate some of
    the memory safety concerns).
-   In the future: Using Crubit-generated C++ bindings for Rust APIs if the
    bindings wrap Rust APIs that accept, store, or return Rust references.
    TODO(b/258235219): Add support for C++ bindings for Rust APIs using `&T`.

### Incorrect C++ lifetime annotations

Incorrect lifetime annotations may lead to UB. Rust's borrow checker prevents
incorrect lifetime annotations, but lifetime annotations of C++ APIs are not
verified by the C++ compiler and Crubit's optional lifetime analysis can't
detect all incorrect annotations. Note that Crubit assumes that lifetime
annotations are correct both for explicit annotations (e.g. `int& $a f2(int&
$a);`) as well as for annotations provided by `#pragma clang lifetime_elision`.

### C++ mutating values referenced by Rust

Mutating a value in C++ may lead to UB if the mutation happens while Rust holds
a references to that value. This applies to Rust shared references (e.g. `&T`)
and to exclusive references (e.g. `&mut T`).

Examples of C++ features that may mutate a value that Rust holds a reference to:

*   Using copy assignment operator of C++ value that Rust has a
    reference to.
*   Mutating public fields of a C++ struct that Rust has a reference to.

TODO: Try to succintly mention the idea that short-lived / non-retained
references are safe from the mutation risk.

### Dangling or null references

All references and
[`NonNull` pointers](https://doc.rust-lang.org/std/ptr/struct.NonNull.html) must
not be null, and if they point to a nonzero span of memory, must not be
dangling. (The behavior of a program which violates these rules is undefined.)

C++ doesn't share these rules, and care must be taken when converting Rust
references to and from C++ pointers. For example, spans/slices are particularly
error-prone: a Rust empty slice uses a dangling pointer (which is UB in C++),
and a C++ empty span (often) uses nullptr (which is UB in Rust). To effectively
use spans in FFI, one must either use non-native types, or perform a conversion
operation which rewrites the pointer values. For that, we recommend using the
conversion routines provided by Crubit support library (e.g. `impl
From<string_view> for &[u8]`).

TODO(b/271016831, b/262580415): Cover `rs_std::Slice<T>` and/or `rs_std::str`
above once these types are provided by Crubit.

### Breaking Rust aliasing rules

Constructing a Rust references that aliases the same address as an already
existing exclusive reference `&mut T` may lead to UB.

TODO: Provide FFI-related examples.

TODO: Document what runtime checks are provided by Crubit (and link to a
separate md document that explains why general checks are infeasible).
