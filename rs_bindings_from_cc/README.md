# rs_bindings_from_cc

`:rs_bindings_from_cc` parses C++ headers and generates:

*   a Rust source file with bindings for the C++ API
*   a C++ source file with the implementation of the bindings

For convenience, `:test_wrapper` is a shell script that passes all Clang command
line flags from the current Blaze C++ toolchain:

```
bazel run //rs_bindings_from_cc:test_wrapper -- hello_world.h
```

or:

```
bazel build //rs_bindings_from_cc:test_wrapper
bazel-bin/rs_bindings_from_cc/test_wrapper hello_world.h
```

## Testing

Write unit tests in the language of the code they cover, and put them next to
(in the same package as) the code they cover.

Put integration tests into `test` package as follows:

*   Write tests for the command line interface of interop tools as `sh_test`s.
*   Write tests verifying that interop tools and Blaze rules generate outputs
    that can be built and executed by C++ or Rust rules as small projects with a
    `rust_test` or `cc_test` on top in subpackages of `test`.
*   Write tests verifying that intermediate outputs created by building outputs
    of interop tools (for example checking that an object file produced by
    compiling `rs_api_impl.cc` file has a specific symbol defined) as small
    projects with a `sh_test` on top.
