# Overview of Crubit's C++/Rust bindings

## Motivation for automating generation of FFI bindings

To call a C++ function from Rust, one can manually declare the foreign function
interface (FFI) as follows:

```rust
extern "C" {
    fn cpp_multiplication_function(x: i32, y: i32) -> i64;
}
```

This works, but has some disadvantages:

*   The function signature in Rust's `extern` declaration needs to match the
    signature of the actual C++ function (e.g. we can't say `x: i16` when the
    C++ parameter is `int32_t x` - doing this would lead to Undefined Behavior).
    Keeping those function signatures in sync can be error prone.
*   Rust does not support the C++ calling convention or any C++ features not
    present in C - this means that bindings for some functions may require an
    extra "thunk" (an extra function that exposes C ABI and forwards calls to
    the target function).
*   It is possible to manually replicate in C++ the memory layout and ABI of
    `#[repr(C)]` and `#[repr(transparent)]` structs, but not the
    implementation-defined layout of `#[repr(Rust)]` structs.

Similar disadvantages exist when manually setting up FFI bindings for calling a
Rust function from C++. Automating generation of FFI bindings with Crubit can
help alleviate the disadvantages above.

## C++ bindings for Rust APIs

Crubit can take a Rust crate and create a C++ library which exposes Rust
functions and types to C++ callers.

Let's walk through the
[`cc_bindings_from_rs_basics`](../../../examples/cc_bindings_from_rs_basics/README.md)
example. In the example, Bazel is instructed to provide C++ bindings for a Rust
library:

```
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)

rust_library(
    name = "example_crate",
    srcs = ["example.rs"],
)

# This declares an "example_crate_cc_api" target that provides Crubit-generated
# C++ bindings for the Rust crate behind the `":example_crate"` target.
cc_bindings_from_rust(
    name = "example_crate_cc_api",
    crate = ":example_crate",
)
```

With the above, C++ code (e.g. `cc_binary`, `cc_library`, `cc_test`, etc.) can
just list the bindings in its `deps`:

```
cc_binary(
    name = "main",
    srcs = ["main.cc"],

    # Declaring a dependency on C++ bindings for calling into the Rust
    # `example` library:
    deps = [ ":example_crate_cc_api" ],
)
```

The scaffolding above will let C++ call into Rust as follows:

```rust
// example.rs:
pub fn add_two_integers(x: i32, y: i32) -> i32 { x + y }
```

```c++
// main.cc:

// The generated bindings are in a header at the same path as the `rust_library`,
// and with the name that follows the `<crate name>_cc_api.h` pattern:
#include "examples/cc_bindings_from_rs_basics/example_crate.h"

int main(int argc, char* argv[]) {
  // The generated bindings are in a namespace with the same name as the
  // target crate:
  int32_t sum = example_crate::add_two_integers(2, 2);

  // ...
}
```

If needed, one can find and inspect the generated bindings in the "Generated
Files and Directories" section for that directory in Code Search or manually
building the bindings using:

```sh
$ bazel build --config=crubit-genfiles \
    //examples/rust/function:example_crate
$ cat "$(bazel info bazel-bin)/examples/rust/function/example_crate.h"
```

Inspecting the generated file may be useful to look at comments that Crubit
leaves behind when it is unable to generate bindings for a given Rust API -
e.g.:

```c++
// Error generating bindings for `reinterpret_cast` defined at path/lib.rs;l=123:
// Error formatting function name:
// `reinterpret_cast` is a C++ reserved keyword and can't be used as a C++ identifier.
```

TODO: Provide integration with Chromium/GN and provide GN-oriented examples.

## Highlights

For a comprehensive description of the generated FFI bindings, please see the
"FFI Bindings Reference". The list below highlights a few aspects of the FFI
bindings that Crubit generates:

*   Crubit can generate FFI bindings for C++ and Rust functions even if they
    don't follow the `extern "C"` calling convention.
*   Crubit can replicate the memory layout of most C++ and Rust structs. This
    includes non-`#[repr(C)]` Rust structs and inheritance hierarchy of C++
    classes.
*   Crubit can generate bindings for APIs that use types from other libraries.
    For example, if a function provided by `foo` library returns a struct
    defined by a separate `bar` library, then `foo` bindings will automatically
    reuse bindings generated for `bar`.
*   Crubit supports all the usual ways to pass function parameters or return
    values. In particular, structs can be passed by reference or by value.
    Values from either language can be moved across stack and heap memory on
    either side of the FFI boundary. This includes C++ structs with non-trivial,
    user-defined move constructors.
*   Crubit supports generating bindings for specific instantiations of arbitrary
    C++ class templates. In particular, bindings for `std::string_view` from C++
    standard library can be generated even though `rs_bindings_from_cc` has
    minimal knowledge about this specific type (except for instructing
    `rs_bindings_from_cc` to include `From` trait implementations from
    `crubit/support/cc_std/string_view.rs`).

--------------------------------------------------------------------------------

TODO: Help users make an informed decision when to use Crubit rather than other
FFI tools. (This probably should become a separate sub-chapter of the overview.
Some raw information can be currently found in the [design.md](../../design.md)
doc.)
