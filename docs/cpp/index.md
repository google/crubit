# Rust bindings for C++ libraries

When a C++ library enables Crubit, that library can be used directly from Rust.
This page documents roughly what that entails, and additional subpages
(available in the left-hand navigation) document specific aspects of the
generated bindings.

Tip: The code examples below are pulled straight from
<https://github.com/google/crubit/tree/main/examples/cpp/function/>. The other examples
in <https://github.com/google/crubit/tree/main/examples/cpp/> are also useful. If you
prefer just copy-pasting something, start there.

## How to use Crubit {#introduction}

Crubit allows you to call some C++ interfaces from Rust. It supports
[functions](functions.md), [classes and structs](classes_and_structs.md), and
[enums](enums.md). Crubit does **not** support advanced features like templates
or virtual inheritance.

The rest of this document goes over how to create a C++ library that can be
called from Rust, and how to actually call it from Rust. The quick summary is:

1.  Define a `rust_api_from_cpp` target in the same `BUILD` file as your
    `cc_library`.

2.  Add the generated `.hint` target (e.g. `:<name_of_rust_target>.hint`) to the
    `aspect_hints` of your `cc_library`.

3.  Depend on the `rust_api_from_cpp` target in the `deps` of your Rust target.
    In Rust, the crate is named after the `cc_library`, not the
    `rust_api_from_cpp` target. See [Crate name](#crate_name).

The bindings can be previewed using the following command:

```sh
$ bazel build --config=crubit-genfiles //path/to:target
```

### Write a `cc_library` target {#cc_library}

The first part of creating a library that can be used by Crubit is to write a
`cc_library` target. For example:

```
{{ #include ../../examples/cpp/function/example.h }}
```
<!--  -->


If you write a BUILD target as normal, it will not actually get Crubit bindings,
but we'll start from there:

```
{{ #include ../../examples/cpp/function/BUILD }}
```
<!--  symbol:example_lib_broken -->


### Enable Crubit on a target {#enable}

To enable Crubit on a C++ target, you must define a `rust_api_from_cpp` target
associated with it, and link them using `aspect_hints`.

Behind the scenes, Rust APIs are generated via Bazel aspects which run on the
`cc_library` target. When examining a `cc_library`, Rust API generation looks
for the `aspect_hints` so that it can find the corresponding `rust_api_from_cpp`
target.

Define a `rust_api_from_cpp` target in the same `BUILD` file as your
`cc_library`, and add its `.hint` target to the `aspect_hints` of the
`cc_library`:

```
{{ #include ../../examples/cpp/function/BUILD }}
```
<!--  symbol:\bexample_lib\b -->


```
{{ #include ../../examples/cpp/function/BUILD }}
```
<!--  symbol:\bexample_lib_rust\b -->


The `.hint` target is automatically created by the `rust_api_from_cpp` macro
(named `<name>.hint`) and is used to avoid circular dependencies between the C++
library and the generated Rust API.

Note that having Rust callers does constrain library evolution. Certain changes
cannot be made in C++ without breaking Rust callers, unless care is taken.
crubit.rs/cpp/cookbook#compatibility

### Look at the generated bindings {#examine}

To examine the generated C++ bindings for the target, you can run the following
command:

```sh
$ bazel build --config=crubit-genfiles //examples/cpp/function:example_lib
```

This is the best way to preview the generated bindings for a given C++ target
right now. You might end up using this a lot, so keep it in your shell history.

If you run the above command, you should see some output like the following:

```
Aspect //rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl%rust_bindings_from_cc_aspect of //examples/cpp/function:example_lib up-to-date:
  bazel-bin/examples/cpp/function/example_lib_rust_api_impl.cc
  bazel-bin/examples/cpp/function/example_lib_rust_api.rs
  bazel-bin/examples/cpp/function/example_lib_namespaces.json
```

These files **are** the generated bindings which are used under the hood when
depending on a C++ target from Rust. They consist of:

1.  The supporting C++ code to glue Rust and C++ together. (The `.cc` file.)
2.  The public Rust interface. (The `.rs` file.)
3.  Supporting internal implementation details. (The `.json` file.)

You don't need to check them in, as they are regenerated automatically whenever
you build a Rust build target which depends on C++.

The `.rs` file is the interesting one for end users. It should contain an
actually useful API for the target:

```
{{ #include ../../examples/cpp/function/example_generated.rs }}
```
<!--  -->


#### When something is missing {#missing}

If Crubit can't generate bindings for a function or type, the build doesn't fail
by default. Crubit leaves the item out of the generated `.rs` file and writes a
comment in its place explaining why:

```rust
// error: function `UseNotCrubitExposed` could not be bound
//   Unsupported parameter type `NotCrubitExposed not_crubit_exposed`:
//     Crubit is not enabled on defining target:
//       //path/to:not_crubit_exposed
```

The first error you see is usually in the Rust code that calls the missing item,
which doesn't say why it's missing. Search the generated `.rs` file for the
item's name to find the reason. The reason often names a type rather than the
function itself; fix or wrap that type, and the function gets bindings too.
crubit.rs/errors explains the most common reasons.

To make a missing item fail the build instead, apply the `CRUBIT_MUST_BIND`
attribute to it. See [Customizing the generated bindings](customizing.md).

### Use a C++ library from Rust {#use}

To depend on a C++ library from Rust, add the corresponding `rust_api_from_cpp`
target to your Rust target's `deps`:

```
{{ #include ../../examples/cpp/function/BUILD }}
```
<!--  symbol:main -->


At that point, the bindings are directly usable from Rust. The interface is
identical to the `.rs` file previewed earlier, but can be used directly:

```
{{ #include ../../examples/cpp/function/main.rs }}
```
<!--  -->


#### Crate name {#crate_name}

The Rust crate is named after the `cc_library`, not the `rust_api_from_cpp`
target. In the example above, the crate is `example_lib`, not
`example_lib_rust`. C++ namespaces become Rust modules inside it, so
`gshoe::add_two_integers` in C++ is `example_lib::gshoe::add_two_integers` in
Rust.

The crate name is the `cc_library`'s target name with every character that isn't
a letter or digit replaced by `_`. The package path isn't part of it. Two
exceptions:

*   If the name starts with a digit, it gets an `n` prefix: `3d_math` becomes
    `n3d_math`.
*   A target named `core` becomes `core_` plus the last directory of its
    package, so that it doesn't shadow Rust's `core` crate: `//foo/bar:core`
    becomes `core_bar`.

## Common Errors {#errors}

See crubit.rs/errors

### Unsupported features

Some features are either unsupported, or else only supported with experimental
feature flags
. In
order to get bindings for a C++ interface, that interface must only use the
subset of features currently supported.

The way to work around this kind of problem, in all cases, is to wrap or hide
the problematic interface behind an interface Crubit can handle:

*   Hide unsupported types behind a wrapper. For example, a `std::set<T>` is not
    supported, but a struct which wraps a `std::set<T>` is.
    crubit.rs/errors/unsupported_type describes the process in more detail.
*   Wrap unsupported functions, in general, behind wrappers.
*   **Embed C++ directly in Rust**: Use [**`inline_cpp!`**](inline_cpp.md) to
    allow embedding arbitrary C++ code within a Rust target using the
    `rust_library_with_embedded_cpp` Bazel rule.

[^aspects]: Crubit is an aspect: an automatically generated entity that exists
    on every build target. It is disabled by default, so that Rust callers don't
    accidentally impose on C++ libraries that weren't expecting them.

    Aspects allow Crubit to fully understand the dependency graph: the bindings
    for X are in the Crubit aspect of X. This allows Crubit to generate bindings
    which themselves rely on bindings: if a function in target `A` returns a
    struct from target `B`, we know that the bindings for `A` will depend on the
    bindings for `B`. Because Crubit is an aspect, it already knows the name of
    the bindings for `B`: it's simply the Crubit aspect on `B`!

    Without aspects, or something like aspects, you would need to write down,
    for every library, the location of its Rust bindings. There is no need for
    that kind of boilerplate when aspects are involved, and that is why most
    things shaped like Crubit use aspects. For example, protocol buffers use
    aspects for their generated implementations in multiple languages. (They
    *also* use named rules, but the rules simply re-export the aspect, and the
    underlying aspect is what is used within the rule for referring to
    transitive dependencies.) Thanks to aspects, the `proto_library` doesn't
    need to re-specify "ah, and the Go proto is named `'x'`".

    Be not afraid! Aspects are what make transitive dependencies work
    seamlessly, without boilerplate. So when you see aspect this, or aspect
    that, remember: this is a Good Thing.
