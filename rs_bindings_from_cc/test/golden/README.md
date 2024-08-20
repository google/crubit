# Golden file tests

## Instructions

*   Add a new test by adding a `foo.h` file, create two empty files named
    `foo_rs_api.rs` and `foo_rs_api_impl.cc`, and then executing
    `cc_bindings_from_rs/test/golden/update.sh`. This will
    generate the corresponding bindings files.
*   If a test in this directory fails, look at the output. It should contain a
    diff of the failure.
*   If you get spurious failures in this directory: Run
    `cc_bindings_from_rs/test/golden/update.sh`.
