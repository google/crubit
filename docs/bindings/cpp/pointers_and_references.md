# Rust bindings for C++ pointer and reference types

NOTE: This document describes unreleased / experimental features of Crubit
(lifetime annotations and nullability annotations).

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
