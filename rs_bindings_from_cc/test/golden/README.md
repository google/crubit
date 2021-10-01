# Golden file tests

## Instructions

*   Add a new test by adding a `foo.h` file and executing `./update.sh`. This
    will generate the corresponding bindings files `foo.cc` and `foo.rs`.
*   If a test in this directory fails, look at the output. It should contain a
    diff of the failure.
*   If you get spurious failures in this directory: Run `./update.sh`.
