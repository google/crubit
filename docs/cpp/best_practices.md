# Best Practices Writing Rust Bindings for Existing C++ Libraries

## Introduction

This document is an attempt at guidance for how Rust changes can be made to
existing C++ libraries, including core foundational libraries.

For an introduction, see [Rust Bindings for C++ Libraries](index.md).

## Code Organization {#organization}

For technical reasons, it is generally necessary for the C++ library and its
Rust bindings to be the same Bazel target. It is not possible to define the Rust
bindings for a target as a completely separate and independent target. The
automatically generated bindings, and their configuration, must be on and in the
C++ target itself.

<section class="zippy" markdown="1">

The reasons why are fairly technical, and you can stop reading here if you're OK
with this.

### Technical Justification

Crubit generates bindings using
[Bazel **aspects**](https://bazel.build/extending/aspects): given an arbitrary
C++ Bazel target, Crubit generates, in an aspect, the Rust library which wraps
it. To users it appears as if the Bazel target was both a C++ and a Rust
library.

This is necessary for the same reason that it's necessary for protocol buffers.
And, just like protocol buffers, this means that we don't have a `rust_library`
target where we could customize its behavior using Bazel attributes.

Specifically, we cannot use a regular Bazel rule for bindings generation because
the rule cannot generate bindings for transitive dependencies: if A depends on
B, then bindings(A) depends on bindings(B), so that bindings(A) can wrap
functions in A that return types from B, and so on. (See
[FAQ: Why can't we use separate rules?](#faq_separate_rules))

Because bindings are generated in an aspect, and not a rule, there are only two
places to configure the bindings of a target A:

*   In the **source code** of the target receiving Rust support, using
    configuration pragmas or attributes. (This is similar to protocol buffers.)
*   In the **BUILD file**, on the target receiving Rust support, via
    `aspect_hints`. Aspect hints are a storage location for configuration data,
    readable by the aspect, placed directly on the target that the aspect runs
    on.

Generally speaking, it's better to modify the source code than to configure
externally via aspect hints. However, some source code annotations are
nonstandard and can have performance implications (see b/321933939). In addition
to this, source code is not readable from the build system itself, and so where
configuring a target requires customizing the build graph, these must go in
aspect hints.

For these reasons, currently most publicly available methods of customizing
bindings occur in aspect hints.

In any case, any configuration or support for Rust is done *directly* to the
target.

### Example

To enable Crubit on a C++ target, one actually modifies the target itself,
adding `aspect_hints = ["//features:supported"]`. This must
be an aspect hint, not a source code annotation, for all of the above reasons:

1.  It makes the build faster and more resilient: when Crubit is disabled on a
    target, Bazel needs to know so it can completely avoid running Crubit on it.
2.  There is no stable, reliable, and style-approved header-wide pragma we can
    use for enabling/disabling Crubit, but `aspect_hints` does work.

### FAQ: Why can't we use separate rules? {#faq_separate_rules}

A library `A`, and its bindings `bindings(A)`, must be linked together in the
build graph: if `B` uses a type from `A`, then `bindings(B)` uses a type from
`bindings(A)`.

Crucially, this also goes in reverse: if a *Rust library* `C` uses a type from
`bindings(A)`, then `reverse_bindings(C)` uses a type from `A`. This forms a
natural dependency cycle: the build graph must understand both the link from `A`
to `bindings(A)`, and the link from `bindings(A)` to `A`.

Crubit resolves this by making `A` and `bindings(A)` the same target in the
build graph: bindings for a target are obtained by reading an aspect on the
target.

It is not possible to make `A` one build target, and `bindings(A)` a separate
build target, call it `X`:

1.  We cannot literally configure on `A` that its bindings are in a different
    target `X`, because this ends up producing a real dependency cycle, as
    mentioned above: if `bindings(A)` = `X`, then `reverse_bindings(X)` = `A`.
2.  We cannot avoid the cycle by creating the dependency "lazily", or
    "dynamically" based on e.g. a naming scheme during Bazel analysis. Bazel
    dependencies cannot be discovered dynamically; once Bazel reaches this point
    of evaluation, dependencies need to be fully resolved: labels in `deps` are
    no longer strings in this stage, they are edges in a dependency graph. That
    graph must not have cycles.
3.  In some limited cases, we *can* hardcode the relationship within Crubit:
    Crubit is actually two aspects, each of which handles a single direction of
    interop. So Crubit can hardcode inside of itself that `bindings(A)` = `X`,
    and in the other half, that `reverse_bindings(X)` = `A`. This requires that
    Crubit itself depends on `A` and `X`. Therefore, to avoid another dependency
    cycle, neither `A` nor `X` can depend/use Crubit in their transitive
    dependencies. This is not feasible except in very isolated cases. Currently,
    we only do this for the Rust and C++ standard libraries.

To compare with another similar technology, PyCLIF avoids this problem because
it only supports "one-directional" interop, and so it doesn't need to avoid
dependency cycles. Crubit is bidirectional, and this comes with some technical
restrictions.

### FAQ: Why are there extra dependencies in `deps(target)`? {#faq_dependency_edges}

Because the Rust bindings are created using an **aspect** on the C++ target,
everything that the Rust bindings need to depend on will appear in a Bazel query
/ depserver query for `deps(target)`.

For example, if you wanted to add some extra source file to the Rust bindings,
you might specify them in `aspect_hints`. This file will show up in
`deps(target)`.

These Rust-only deps are not used at all in pure-C++ builds (the Bazel actions
registered by them won't be executed), but they will show up in the dependency
graph anyway, due to how Bazel query and depserver track dependencies.

NOTE: In particular, if your project has tests that count/limit the transitive
dependencies of a C++ binary, they will overcount the dependencies, and the
overcounting will get worse as Rust support is rolled out through the C++ build
graph.

</section>

## Wrapping and type bridging vs direct use of types {#bridging}

Crubit automatically generates layout-compatible Rust equivalents of C++ types.
When the C++ type is [Rust-movable](classes_and_structs.md#rust_movable), the
Crubit-generated Rust type is Rust-movable, these can be used by value, by
pointer, in struct fields, arrays, and any other compound data type. A C++
pointer `const T*` can become a Rust `*const T`, and a C++ `T` field can become
a Rust `T` field, and so on, with few restrictions.

For example, the following C++ type:

```c++
struct Vec2d {
    float x;
    float y;
};
```

Becomes (roughly) the following Rust type:

```rust
#[repr(C)]
struct Vec2d {
    pub x: f32,
    pub y: f32,
}
```

These have an identical layout, and so a C++ pointer or field containing a C++
`Vec2d` is exactly equivalent to a Rust pointer or field containing a Rust
`Vec2d`.

(See [Types](../types/) for more information about layout-compatibility.)

Because of this, it is often not required to manually write any new types. The
bindings generated by Crubit will produce a working type automatically.

### When to wrap a type

There are, still, a handful of reasons to manually write "wrapper" types which
encapsulate or replace the original C++ type (or its Crubit-generated Rust
type).

*   If the type is **not** naturally Rust-movable, but it's important for the
    Rust type to be Rust-movable. It may be possible to make changes to the C++
    code to make the type Rust-movable using some of the strategies described in
    [the cookbook](cookbook.md#rust_movable). This allows the greatest
    flexibility, as the type becomes usable in almost every context. But if that
    is not possible, writing a new "wrapper" type can keep Rust programmers
    productive.
*   Some Rust types have very special semantics, which are impossible to
    implement in the bindings for a C++ type. For example, Rust has special
    support for `Result` and `Option` in error handling via the `?` operator,
    which cannot yet be implemented by `Status` or `std::optional` using stable
    Rust features. These privileged Rust types can be used instead of the
    equivalent C++ types, as a wrapper type.

*   The type is simply not supported/supportable in Rust, and needs a wrapper as
    a workaround. (See also: crubit.rs/errors/unsupported_type.)

In these cases, Crubit may bridge to a wrapper type as a workaround, while we
hopefully fix the underlying issues that mean we cannot directly use the
underlying type. This offers us a subset of the API we want, and allows
continued progress.

### Why not to wrap a type

Wrapper types work best when passed by value: if you return a `T` in C++, the
corresponding Rust function can automatically convert it to and return a
`WrappedT`.

However, no conversion is possible for references or fields, which really are
the original type, with its size and alignment and address in memory - to make
this work transparently requires an ever-expanding network of wrapper types, one
for every compound data type that might contain `T`:

*   `T` must become `WrappedT`
*   `const T&`, if it is supported at all, must become something like
    `TRef<'a>`, or a dynamically sized `&TView`.
*   `std::vector<T>`, if it is supported at all, must become something like
    `TVector`.
*   `struct MyStruct {T x;}` must become a wrapped `WrappedMyStruct`.
*   ...

The problems introduced by wrapper types can easily outweigh the benefits that
they bring. Crubit aims to reduce their necessity to zero over time.

#### *Bad reasons to wrap a type*

In most other circumstances where one might *want* to reach for wrapper types,
alternatives exist:

*   If we want to use a wrapper type in order to give the type a nicer Rust API,
    then, as an alternative, one can customize the Rust API of the wrapped type
    using an aspect hint. You can define new methods and trait implementations
    to the side, without altering any C++ code.

*   If we want to use a wrapper type in order to change the type invariants – to
    make them stricter or looser – this is fine, as long as it doesn't *replace*
    the not-as-nice type. For example, if a C++ API returns `std::string`
    (bytes, "probably" UTF-8), the Rust equivalent should not return a Rust
    `String` (Unicode, definitely UTF-8). Changing type invariants in-place
    causes some APIs to become impossible to call, and causes the Rust and C++
    ecosystems to diverge and become incompatible. The bindings should be high
    fidelity. Wrapper types of this form should be optional, and available
    equally to both C++ and Rust to avoid fragmenting the ecosystem.

</section>

## Fidelity {#fidelity}

Anything possible in C++ should be possible in Rust. See
[<internal link>](http://goto.google.com/no-hitchhikers).

The Rust API for a given C++ API should not try to make the interface "better"
at more than a superficial level, because it can compromise the ability of other
teams to write new Rust code, or port existing C++ code to Rust.

**Good changes:**

*   Changing method names, especially to names that Rust callers might expect.
    For example, changing `Status::ok()` (C++) to `Status::is_ok()` (Rust) –
    Rust callers expect many of these boolean functions to be prefixed with
    `is_`.
*   Adding new APIs that Rust users expect. For example, trait implementations
    that allow the type to better interoperate with the Rust ecosystem, or
    functions which accept a `Path` or `&str` in *addition* to a raw C++
    `string_view`.
*   Reifying C++ comments around lifetime or safety as actual lifetime
    annotations or `unsafe` declarations.

If the Rust type is outright unnatural to use, people won't use it, and it's
worse for the ecosystem to have two APIs than one API.

**Bad changes:**

*   Removing deprecated APIs which still have C++ callers.
*   Placing new requirements on Rust callers that were not placed on C++
    callers, such as requiring UTF-8 when C++ does not.
