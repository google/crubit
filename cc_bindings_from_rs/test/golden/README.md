# Golden file tests

## Instructions

*   Add a new test by adding a `foo.rs` file, create two empty files named
    `foo_cc_api.h` and `foo_cc_api_impl.rs`, and then executing
    `common/golden_update.sh`. This will generate the
    corresponding bindings files.
*   If a test in this directory fails, look at the output. It should contain a
    diff of the failure.
*   If you get spurious failures in this directory: Run
    `common/golden_update.sh`.
