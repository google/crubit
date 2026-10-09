# Rust bindings for C++ references and lifetimes

Rust references have lifetimes, which let the Rust compiler check that a
reference doesn't outlive the value it refers to. C++ references don't have
lifetimes. To generate Rust bindings for C++ functions and methods that take or
return C++ references (`T&` or `const T&`), Crubit needs to know their
lifetimes. Crubit gets them from:

*   [Rust's lifetime elision rules](#elision), or
*   [Explicit lifetime annotations](#explicit) in the C++ code.

This page also explains what happens if lifetime elision
[doesn't apply](#elision_fails) or [gives wrong lifetimes](#elision_wrong), and
describes the [`CRef` and `CMut` types](#cref_and_cmut) that Crubit uses for
references returned from C++.

NOTE: C++ pointers don't have lifetimes in the generated bindings. They map to
Rust raw pointers. See [Pointer types](../types/pointer.md).

## Lifetime elision {#elision}

Crubit applies
[Rust's lifetime elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html)
to C++ references. For example, given the following C++ header:

```
{{ #include ../../examples/cpp/references_and_lifetimes/example.h }}
```
<!--  class:Counter -->


Crubit will generate the following bindings (notice the inferred `'__this`
lifetime):

```
{{ #include ../../examples/cpp/references_and_lifetimes/example_generated.rs }}
```
<!--  content:"(?m)impl Counter.*?(^$|^    \})" -->


Reference parameters become Rust references, and each of them gets its own
lifetime (e.g. see `'x` below):

```
{{ #include ../../examples/cpp/references_and_lifetimes/example_generated.rs }}
```
<!--  function:\bAddOne\b -->


## When lifetime elision doesn't apply {#elision_fails}

Lifetime elision doesn't apply to some functions. For example, in Rust, `fn
smaller(x: &i32, y: &i32) -> &i32` doesn't compile (error `E0106`), because the
result can refer to `x` or to `y`. See also
["Generic Lifetimes in Functions"](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#generic-lifetimes-in-functions)
in the Rust Book.

For such C++ functions, Crubit generates bindings that return a raw pointer
instead of a reference. For example, given the following C++ function:

```
{{ #include ../../examples/cpp/references_and_lifetimes/example.h }}
```
<!--  function:\bSmaller\b -->


Crubit will generate the following bindings:

```
{{ #include ../../examples/cpp/references_and_lifetimes/example_generated.rs }}
```
<!--  function:\bSmaller\b -->


Rust code needs `unsafe` to dereference the returned pointer. To bind the result
as a reference, add [explicit lifetime annotations](#explicit) to the C++
function.

## Explicit lifetime annotations {#explicit}

C++ code can use explicit lifetime annotations (e.g. `$a`) to specify lifetimes
that lifetime elision can't infer. For example, given the following C++
function:

```
{{ #include ../../examples/cpp/references_and_lifetimes/example.h }}
```
<!--  content:^inline\ const\ int&\ \$a\ SmallerWithLifetimes.*?^\} -->


Crubit will generate the following bindings:

```
{{ #include ../../examples/cpp/references_and_lifetimes/example_generated.rs }}
```
<!--  function:SmallerWithLifetimes -->


Notes:

*   The `$a`, `$b`, etc. macros come from
    `"support/lifetime_annotations.h"`.
*   `$a` annotates the lifetime of the reference that precedes it.
*   In a method, `$a` after the parameter list (and after `const`, if present)
    annotates the lifetime of `this`. See [the example below](#elision_wrong).
*   `$static` annotates a `'static` lifetime.

See [Lifetime Annotations for C++](../design/lifetime_annotations_cpp.md) for
more details and design discussions.

## When lifetime elision gives wrong lifetimes {#elision_wrong}

Lifetime elision is a good default for most C++ APIs, but neither the C++
compiler nor Crubit checks that the lifetimes match what the C++ code does. For
example, given the following C++ struct:

```
{{ #include ../../examples/cpp/references_and_lifetimes/example.h }}
```
<!--  class:Number -->


Crubit will generate the following bindings:

```
{{ #include ../../examples/cpp/references_and_lifetimes/example_generated.rs }}
```
<!--  content:"(?m)impl Number.*?(^$|^    \})" -->


Lifetime elision ties the result of `GetLarger` only to `self`. This is wrong,
because the result can refer to `other`. Rust code can keep the result after
`other` is dropped, and then the result is a dangling reference. This can cause
undefined behavior.

`GetLargerWithLifetimes` uses explicit lifetime annotations to tie the result to
both `self` and `other`. If lifetime elision gives wrong lifetimes, then please
add explicit lifetime annotations to the C++ API.

## `CRef` and `CMut` {#cref_and_cmut}

C++ references returned from C++ functions and methods become `CRef<'a, T>` (for
`const T&`) or `CMut<'a, T>` (for `T&`). `CRef` and `CMut` are defined in the
`cref` crate (`//support:cref`).

Crubit uses `CRef` and `CMut` (rather than Rust references) to accurately
represent that C++ references don't offer the same guarantees as Rust
references. For example:

*   A Rust shared reference (`&T`) promises that nothing mutates the value while
    the reference is in use. On the other hand, `const T&` in C++ allows
    mutating the referenced value (e.g. through a `mutable` field, or through a
    `const_cast` if the referenced object itself is not `const`).
*   A Rust mutable reference (`&mut T`) promises that the reference is the only
    way to access the value while it is in use. On the other hand, C++ allows
    multiple `T&` (and/or `const T&`) to co-exist.

See the documentation of the `cref` crate for more details.

### Methods {#cref_and_cmut_methods}

Today Crubit binds `this` as `&self` or `&mut self` (rather than as `self:
CRef<Self>` or `self: CMut<Self>`).

Future Rust language features may make `CRef` and `CMut` more ergonomic to use
as method receivers (see
[arbitrary self types](https://rust-lang.github.io/rfcs/3519-arbitrary-self-types-v2.html)
and [autoref](https://rust-lang.github.io/beyond-refs/autoref.html) in the
["Beyond Rust References"](https://rust-lang.github.io/beyond-refs/)
initiative). Once that happens, Crubit may start binding `this` as `CRef<Self>`
or `CMut<Self>` (at least for inherent methods - trait methods may still require
`&self` or `&mut self`).
