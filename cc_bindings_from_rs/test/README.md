# End-to-end tests for cc_bindings_from_rs

Each test is located in its own directory, e.g., `test/functions`.

## Adding a new test

*   Create a new directory under `test/`, e.g., `test/foo`.
*   Create a `foo.rs` file with the Rust code you want to test.
*   Create empty files named `foo_cc_api.h` and `foo_cc_api_impl.rs` in that
    directory.
*   Create a `BUILD` file similar to other tests (e.g., `test/functions/BUILD`),
    defining `rust_library`, `cc_bindings_from_rust`, `golden_test`, and
    `crubit_cc_test` targets.
*   Create a C++ test file `foo_test.cc`.
*   Run `common/golden_update.sh` to populate the golden
    files.

## Style Guidelines

*   **C++ Test Namespace**: Do not place the C++ tests inside a `crubit`
    namespace. Leave them in a top-level anonymous namespace.

    ```cpp
    // Correct:
    namespace {
    TEST(FooTest, Bar) { ... }
    }  // namespace
    ```
