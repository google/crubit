# C++ bindings for Rust libraries

Rust libraries can be used directly from C++. This page documents roughly what
that entails, and additional subpages (available in the left-hand navigation)
document specific aspects of the generated bindings.

Tip: The code examples below are pulled straight from
examples/rust/function/. The other examples in
examples/rust/ are also useful. If you prefer just
copy-pasting something, start there.

## How to use Crubit {#introduction}

Crubit allows you to call some Rust interfaces from C++. It supports
[functions](functions) (including methods), [structs](structs), and even
[enums](enums) as "opaque" objects. Crubit does **not** support advanced
features like generics or dynamic dispatch with `dyn`.

The rest of this document goes over how to create a Rust library that can be
called from C++, and how to actually use it from C++. The quick summary is:

*   All `rust_library` targets can receive C++ bindings.
*   To use the bindings for a target `//path/to:example_crate`, you must create
    a C++ rule exporting the bindings, using
    `cc_bindings_from_rust(name="any_name_here", crate=":example_crate")`.
*   The header name is the Rust target's label with a `.h` appended: to include
    the header for the Rust library `//path/to:example_crate`, you use `#include
    "path/to/example_crate.h"`.
*   The namespace name is the Rust target name, e.g. `example_crate`. To change
    the namespace, use `cc_bindings_from_rust_library_config`, described below.
*   To see the generated C++ API, right click the `"path/to/example_crate.h"`
    include in Cider, and select "Go to Definition".

    NOTE: In some cases the generated file in Cider may be out of date. If it
    isn't refreshing, you can manually inspect the bindings using the workaround
    command in b/391395849.

### Write a `rust_library` target {#rust_library}

The first part of creating a library that can be used by Crubit is to write a
`rust_library` target. For example:

```live-snippet
cs/file:examples/rust/function/example.rs content:^[^/].*
```

In the BUILD file, in addition to defining the `rust_library`, you should also
define the `cc_bindings_from_rust` target to make it easier to use from C++:

```live-snippet
cs/file:examples/rust/function/BUILD symbol:example_crate|example_crate_cc_api
```

Example: If your Rust library is named `//path/to:example_crate`, then the C++
header file is `"path/to/example_crate.h"`, and the C++ namespace is
`example_crate` by default.

### Use a Rust library from C++ {#use}

C++ build rules do not have a `rust_deps` parameter, so to depend on the C++
bindings for a target, they must depend on the `cc_bindings_from_rust` rule.

For example:

```live-snippet
cs/file:examples/rust/function/BUILD symbol:main
```

```live-snippet
cs/file:examples/rust/function/main.cc content:^[^/\n].*
```

NOTE: Other than for declaring the dependency, all other information about the
generated bindings comes from the actual `rust_library` rule. For example, the
`#include` for the above is `#include
"examples/rust/function/example_crate.h"`, **not**
`example_crate_cc_api.h`.

### (Optional) Customize the generated C++ API {#cc_bindings_from_rust_library_config}

#### Give it a better namespace {#namespace}

The crate name might make a poor namespace. In addition, typically, multiple C++
headers and build targets share the same namespace. To customize the namespace
name, use `cc_bindings_from_rust_library_config`:

```live-snippet
cs/file:examples/rust/library_config/BUILD symbol:custom_namespace|example_crate
```

Now, instead of the crate name, the generated bindings will use the namespace
name you provided:

```live-snippet
cs/file:examples/rust/library_config/main.cc content:^[^/\n].*
```

### Look at the generated bindings {#examine}

There are two ways to look at the generated header file:

*   Click through the `#include` in Cider. Given the following C++ code:

    ```c++
    #include "path/to/example_crate.h"
    ```

    If you right click the file path, and select "Go to Definition", you will be
    taken to a file starting with `// Automatically @generated C++ bindings`.

*   Run `bazel build //path/to:example_crate --config=crubit-genfiles`, and open
    `bazel-bin/path/to/example_crate.h` in your text editor of choice.

## Common Errors {#errors}

### Unsupported features

Some features are either unsupported, or else only supported with experimental
feature flags (<internal link>). In order to get bindings for a Rust
interface, that interface must only use the subset of features currently
supported.

For a particularly notable example, references are only supported as function
parameters, and only in a subset of cases that we can prove does not add
aliasing UB to C++ callers.

The way to work around this kind of problem, in all cases, is to wrap or hide
the problematic interface behind an interface Crubit can handle:

*   Use raw pointers instead of references, if this use of references falls into
    a case Crubit does not support.
*   Hide unsupported types behind a wrapper type. For example, a `Vec<T>` is not
    supported by Crubit, but `pub struct MyStruct(Vec<i32>);` is.
