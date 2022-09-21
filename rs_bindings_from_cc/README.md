# rs_bindings_from_cc

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.

`:rs_bindings_from_cc` parses C++ headers and generates:

*   a Rust source file with bindings for the C++ API
*   a C++ source file with the implementation of the bindings

For convenience, `:test_wrapper` is a shell script that passes all Clang command
line flags from the current Bazel C++ toolchain:

```
bazel run //rs_bindings_from_cc:test_wrapper -- --public_headers=hello_world.h
```

or:

```
bazel build //rs_bindings_from_cc:test_wrapper
bazel-bin/rs_bindings_from_cc/test_wrapper --public_headers=hello_world.h
```

## Testing

If possible follow these recommendations:

*   Unit tests for
    [`src_code_gen`](rs_bindings_from_cc/src_code_gen.rs)
    should:
    *   be written in Rust
    *   have snippets of C++ as input
    *   use
        [`assert_cc_matches!/assert_rs_matches!/assert_cc_not_matches!/assert_rs_not_matches!`](rs_bindings_from_cc/token_stream_matchers.rs)
        macros
*   Unit tests for the
    [`importer`](rs_bindings_from_cc/importer.h)
    should:
    *   be written in Rust
        ([`ir_from_cc_test.rs`](rs_bindings_from_cc/ir_from_cc_test.rs))
        so they cover both AST logic and IR serialization/deserialization, but
        C++ tests (thanks to its nice matchers) are also OK at the moment
        ([`importer_test.cc`](rs_bindings_from_cc/importer_test.cc))
    *   have snippets of C++ as input
    *   make assertions on the content of the IR
*   Write tests for the command line interface of interop tools in
    [`rs_bindings_from_cc_test.sh`](rs_bindings_from_cc/test/rs_bindings_from_cc_test.sh).
*   Write golden file tests (comparing both the C++ and Rust generated source
    code against the checked-in files) in
    [`test/golden`](rs_bindings_from_cc/test/golden/).
    *   Run
        [`rs_bindings_from_cc/test/golden/update.sh`](rs_bindings_from_cc/test/golden/update.sh)
        to regenerate checked-in files.
*   Write full executable end-to-end tests (verifying that interop tools and
    Bazel rules generate outputs that can be built and executed) as small
    projects with a `rust_test` or `cc_test` on top in subpackages of `test`.

To run individual rust tests with `bazel test` (like `bazel test --test_filter=<test>` for gtest cases), give the test function name as `--test_arg=<test>`.

To get Rust backtraces for `rs_bindings_from_cc` when running end-to-end tests,
use `bazel test --action_env=RUST_BACKTRACE=1` to run the tests.

## Debugging

If you want to build the tool specially, for example using sanitizers, use the
script at `rs_bindings_from_cc/generate_bindings_for_target_with_tool_flags.sh`, for example:

```
rs_bindings_from_cc/generate_bindings_for_target_with_tool_flags.sh \
  //base \
  --config=asan
```

If you want to build the tool specially and use it for generating bindings from
a golden file, use the `<header basename>_rs_test` target. For `types.h` the
command would be:

```
rs_bindings_from_cc/generate_bindings_for_target_with_tool_flags.sh \
  //rs_bindings_from_cc/test/golden:types_rs_test \
  --config=asan
```

If you want to see the Clang AST dump of some file (generated files work too), run:

```
bazel build --per_file_copt=<PATH_TO_FILE>@-Xclang,-ast-dump,-fno-color-diagnostics <TARGET> > /tmp/output_file
```

## Contributing

Chat room (internal): https://chat.google.com/room/AAAAImO--WA

20% starter projects list (internal): b/hotlists/3645339
