# rs_bindings_from_cc

Disclaimer: This project is experimental, under heavy development, and should
be used yet.

`:rs_bindings_from_cc` parses C++ headers and generates:

*   a Rust source file with bindings for the C++ API
*   a C++ source file with the implementation of the bindings

For convenience, `:test_wrapper` is a shell script that passes all Clang command
line flags from the current Blaze C++ toolchain:

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
    [`src_code_gen`](/rs_bindings_from_cc/src_code_gen.rs)
    should be:
    *   written in Rust
    *   have snippets of C++ as input
    *   use
        [`assert_cc_matches!/assert_rs_matches!/assert_cc_not_matches!/assert_rs_not_matches!`](/rs_bindings_from_cc/token_stream_matchers.rs)
        macros
*   Unit tests for the
    [`ast_visitor`](/rs_bindings_from_cc/ast_visitor.h)
    should be:
    *   written in Rust
        ([`ir_from_cc_test.rs`](/rs_bindings_from_cc/ir_from_cc_test.rs))
        so they cover both AST logic and IR serialization/deserialization, but
        C++ tests (thanks to its nice matchers) are also OK at the moment
        ([`ast_visitor_test.cc`](/rs_bindings_from_cc/ast_visitor_test.cc))
    *   have snippets of C++ as input
    *   make assertions on the content of the IR
*   Write tests for the command line interface of interop tools in
    [`rs_bindings_from_cc_test.sh`](/rs_bindings_from_cc/test/rs_bindings_from_cc_test.sh).
*   Write golden file tests (comparing both the C++ and Rust generated source
    code against the checked-in files) in
    [`test/golden`](/rs_bindings_from_cc/test/golden/).
    Run
    [`rs_bindings_from_cc/test/golden/update.sh`](/rs_bindings_from_cc/test/golden/update.sh)
    to regenerate checked-in files.
*   Write full executable end to end tests (verifying that interop tools and
    Blaze rules generate outputs that can be built and executed) as small
    projects with a `rust_test` or `cc_test` on top in subpackages of `test`.

## Contributing

Chat room: https://chat.google.com/room/AAAAImO--WA

20% starter projects list: b/hotlists/3645339
