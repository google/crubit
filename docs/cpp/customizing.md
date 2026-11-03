# Customizing bindings using annotations

The Rust bindings for a C++ declaration can be customized using an attribute
macro from `"support/annotations.h"`.

For instance:

*   A function can be marked `unsafe` in Rust, even if Crubit would otherwise
    assume it was safe, using `CRUBIT_UNSAFE`.
*   Missing bindings for an item can be treated as an error, instead of ignored,
    using `CRUBIT_MUST_BIND`.
*   An item can be given a different name in Rust using
    `CRUBIT_RUST_NAME("rust_name_here")`.
*   A function or method can be given `pub(crate)` visibility in Rust, limiting
    its visibility to the generated crate (including custom `srcs` on
    `rust_api_from_cpp`), using `CRUBIT_PUB_CRATE`.
*   A C++ type can be mapped directly to a layout-equivalent existing Rust type
    (suppressing automated Rust bindings for the C++ declaration) using
    `CRUBIT_RUST_LAYOUT_EQUIVALENT("crate::MyRustType")` (and, when needed for
    external crates in separate Bazel targets,
    `CRUBIT_RUST_LAYOUT_EQUIVALENT_LABEL_HINT("//pkg:target")`).

<!-- TODO(jeanpierreda): should we fully enumerate everything, on this page? -->

More information:

*   **Dependency**: `//support:annotations`
*   **Include**: `#include "support/annotations.h"`
*   Full API documentation: support/annotations.h

## Example

Given the following C++ header:

```live-snippet
cs/file:examples/cpp/unsafe_attributes/example.h content:SafeSignatureButAnnotatedUnsafe
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/cpp/unsafe_attributes/example_generated.rs symbol:SafeSignatureButAnnotatedUnsafe
```
