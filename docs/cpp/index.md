# Rust bindings for C++ libraries

[TOC]

When a C++ library enables Crubit, that library can be used directly from Rust.
This page documents roughly what that entails, and additional subpages
(available in the left-hand navigation) document specific aspects of the
generated bindings.

Tip: The code examples below are pulled straight from
examples/cpp/function/. The other examples in
examples/cpp/ are also useful. If you prefer just
copy-pasting something, start there.

## How to use Crubit {#introduction}

Crubit allows you to call a C-like interface from Rust. That is, an interface
where all [functions are `extern "C"`](functions),
[classes and structs are rust-movable](classes_and_structs), and there are no
advanced features like templates or virtual inheritance.

The rest of this document goes over how to create a C++ library that can be
called from Rust, and how to actually call it from Rust. The quick summary is:

1.  A `cc_library` gets (nonempty) Rust bindings if it specifies
    `aspect_hints = ["//features:extern_c"]`.

2.  Any Rust build target can depend on the bindings for a `cc_library`, by
    specifying `cc_deps=["//path/to:target"]`.

3.  The bindings can be previewed using the following command:

    ```sh
    $ bazel build --aspects \
      //rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl%rust_bindings_from_cc_aspect \
      --output_groups=out \
      //path/to:target
    ```

### Write a `cc_library` target {#cc_library}

The first part of creating a library that can be used by Crubit is to write a
`cc_library` target. For example:

```live-snippet
cs/file:examples/cpp/function/example.h
```

If you write a BUILD target as normal, it will not actually get Crubit bindings,
but we'll start from there:

```live-snippet
cs/file:examples/cpp/function/BUILD symbol:example_lib_broken
```

### Look at the generated bindings {#examine}

Bindings can be generated for any C++ target, anywhere in the build graph.
(Crubit is an **aspect**[^aspects] on all C++ targets.) However, that is not to
say that the generated bindings will be useful: by default, Crubit doesn't
generate any bindings. Try it!

To examine the generated C++ bindings for the target, you can run the following
command:

```sh
$ bazel build --aspects \
  //rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl%rust_bindings_from_cc_aspect \
  --output_groups=out \
  //examples/cpp/function:example_lib_broken
```

This is the best way to preview the generated bindings for a given C++ target
right now. (b/319926369 will make this a bit easier in some circumstances.) You might
end up using this a lot, so keep it in your shell history.

If you run the above command, you should see some output like the following:

```
Aspect //rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl%rust_bindings_from_cc_aspect of //examples/cpp/function:example_lib_broken up-to-date:
  bazel-bin/examples/cpp/function/example_lib_broken_rust_api_impl.cc
  bazel-bin/examples/cpp/function/example_lib_broken_rust_api.rs
  bazel-bin/examples/cpp/function/example_lib_broken_namespaces.json
```

These files **are** the generated bindings which are used under the hood when
depending on a C++ target from Rust. They consist of:

1.  The supporting C++ code to glue Rust and C++ together. (The `.cc` file.)
2.  The public Rust interface. (The `.rs` file.)
3.  Supporting information that is used by bindings that *depend* on these
    bindings. (The `.json` file.)

You don't need to check them in, as they are regenerated automatically whenever
you build a Rust build target which depends on C++.

The `.rs` file is the interesting one for end users. For a library like `:example_lib_broken`, which does not enable Crubit, the
`.rs` file will be essentially empty, only consisting of comments describing the
bindings it did not generate:

```rust
// Generated from: examples/cpp/function/example.h;l=11
// Error while generating bindings for item 'crubit_add_two_integers':
// Can't generate bindings for crubit_add_two_integers, because of missing required features (<internal link>):
// //examples/cpp/function:example_lib_broken needs [//features:extern_c] for crubit_add_two_integers (return type)
// //examples/cpp/function:example_lib_broken needs [//features:extern_c] for crubit_add_two_integers (the type of x (parameter #0))
// //examples/cpp/function:example_lib_broken needs [//features:extern_c] for crubit_add_two_integers (the type of y (parameter #1))
// //examples/cpp/function:example_lib_broken needs [//features:extern_c] for crubit_add_two_integers (extern \"C\" function)
```

This error is saying something important. It was trying to generate bindings for
the function `crubit_add_two_integers`, but it couldn't, because four different
things about the function require the `extern_c` feature to be enabled on the
target. The parameter and return types require `extern_c`, as
does the function itself in the abstract (as it is an `extern "C"` function).

`extern_c` is the name of the Crubit features related to C-like interfaces.
Other functions and classes might require `experimental`, for experimental
features. For example, if we had defined an `operator+`, or if the function were
not `extern "C"`. For more on this, see <internal link>.

### Enable Crubit on a target {#enable}

To enable Crubit on a C++ target, one must pass an argument, via `aspect_hints`.
Specifically, as mentioned in the comments, the target must enable the
`extern_c` feature:

```live-snippet
cs/file:examples/cpp/function/BUILD symbol:\bexample_lib\b
```

This tells Crubit that it can generate bindings for this target, for any part of
the library that uses features from `extern_c`. Now, if we look at a preview of
the automatically generated bindings:

```sh
$ bazel build --aspects \
  //rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl%rust_bindings_from_cc_aspect \
  --output_groups=out \
  //examples/cpp/function:example_lib
```

We can see the fully-fledged bindings for the library:

```live-snippet
cs/file:examples/cpp/function/example_generated.rs
```

### Use a C++ library from Rust {#use}

To depend on a C++ library from Rust, add it to `cc_deps`:

```live-snippet
cs/file:examples/cpp/function/BUILD symbol:main
```

At that point, the bindings are directly usable from Rust. The interface is
identical to the `.rs` file previewed earlier, but can be used directly:

```live-snippet
cs/file:examples/cpp/function/main.rs
```

## Common Errors {#errors}

### Unsupported features

Some features are either unsupported, or else only supported with experimental
feature flags (<internal link>). In order to get bindings for a C++
interface, that interface must only use the subset of features currently
supported.

For a particularly notable example, a class cannot have a `std::string` field,
because `std::string` has properties around move semantics that Crubit does not
yet support. In turn, this means the class *containing* the `std::string` has
semantics that Crubit doesn't yet support.

The way to work around this kind of problem, in all cases, is to wrap or hide the problematic
interface behind an interface Crubit can handle:

*   Move nontrivial types behind a `unique_ptr<T>`. A `std::string` field is not
    rust-movable, but a `unique_ptr<std::string>` field is.
*   Hide unsupported types, in general, behind a wrapper. For example, a
    `std::vector<T>` is not supported, but a struct which wraps a
    `unique_ptr<std::vector<T>>` is.
*   Wrap unsupported functions behind `extern "C"` wrappers. For example,
    methods are not yet supported, but top-level functions are, and can invoke
    methods.

[^aspects]: Crubit is an aspect: an automatically generated entity that exists
    on every build target. It is disabled by default, so that Rust
    callers don't accidentally impose on C++ libraries that weren't
    expecting them.

    Aspects allow Crubit to fully understand the dependency graph: the
    bindings for X are in the Crubit aspect of X. This allows Crubit to
    generate bindings which themselves rely on bindings: if a function
    in target `A` returns a struct from target `B`, we know that the
    bindings for `A` will depend on the bindings for `B`. Because Crubit
    is an aspect, it already knows the name of the bindings for `B`:
    it's simply the Crubit aspect on `B`!

    Without aspects, or something like aspects, you would need to write
    down, for every library, the location of its Rust bindings. There is
    no need for that kind of boilerplate when aspects are involved, and
    that is why most things shaped like Crubit use aspects. For example,
    protocol buffers use aspects for their generated implementations in
    multiple languages. (They *also* use named rules, but the rules
    simply re-export the aspect, and the underlying aspect is what is
    used within the rule for referring to transitive dependencies.)
    Thanks to aspects, the `proto_library` doesn't need to re-specify
    "ah, and the Go proto is named `'x'`".

    Be not afraid! Aspects are what make transitive dependencies work
    seamlessly, without boilerplate. So when you see aspect this, or
    aspect that, remember: this is a Good Thing.
