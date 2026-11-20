# Rust bindings for the C++ standard library

To use C++ standard library types from Rust, depend on the `cpp_std` support
library in your `deps`:

```
deps = ["//support:cpp_std"]
```

and refer to C++ `std` items directly via `cpp_std::{sometype}` (for example,
`cpp_std::unique_ptr<T>` or `cpp_std::raw_string_view`).

## Relationship between `cpp_std` and `cc_std`

-   `//support:cpp_std` is the **public, user-facing crate**
    that re-exports `std::*` bindings at crate root without the `cc_std::std::*`
    module stutter. All new and migrated Rust callers should depend on
    `//support:cpp_std`.
-   `//support/cc_std` is the current internal
    `bindings_for_toolchain_headers` crate that binds both libc++ headers
    (`namespace std`) and a small set of POSIX C headers (`LIBC_HEADERS` in the
    global namespace) alongside manually authored trait and type implementations
    in `//support/cc_std_impl` (such as
    `impl From<&'static str> for raw_string_view`).
