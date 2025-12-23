<!-- <internal link> -->

# Customizing bindings using annotations

The Rust bindings for a C++ declaration can be customized using an attribute
macro from `<crubit/support/annotations.h>`.

For instance:

  * A function can be marked `unsafe` in Rust, even if Crubit would otherwise
    assume it was safe, using `CRUBIT_UNSAFE`.
  * Missing bindings for an item can be treated as an error, instead of ignored,
    using `CRUBIT_MUST_BIND`.
  * An item can be given a different name in Rust using
    `CRUBIT_RUST_NAME("rust_name_here")`.

<!-- TODO(jeanpierreda): should we fully enumerate everything, on this page? -->

More information:

  * **Dependency**: `//support:annotations`
  * **Include**: `#include <crubit/support/annotations.h>`
  * Full API documentation:
    support/annotations.h

## Example

Given the following C++ header:

```
{{ #include ../../examples/cpp/unsafe_attributes/example.h }}
```
<!--  symbol:SafeSignatureButAnnotatedUnsafe -->


Crubit will generate the following bindings:

```
{{ #include ../../examples/cpp/unsafe_attributes/example_generated.rs }}
```
<!--  symbol:SafeSignatureButAnnotatedUnsafe -->

