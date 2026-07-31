<!-- <internal link> -->

# Bridge types as struct fields

## Overview

Crubit does not support compound data types containing bridge types, unless the
compound data type itself can be bridged.

Bridge types are types that are converted at runtime between C++ and Rust
(e.g. `std::optional` mapping to `Option`), but have different underlying
representations. (See crubit.rs/types.)

This conversion is only possible when the bridge type is used by value. It is
not possible in fields (crubit.rs/errors/bridge_field), inside of a pointer, or
more generally inside of a compound data type that doesn't support recursively
bridging its interior (such as `std::vector`).

## Example

Consider the following C++ function:

```c++
#include <optional>

const std::optional<int>* Foo();
```

While Crubit can bridge a `std::optional<int>` return value, it cannot bridge it
when it is behind a pointer. It cannot reuse the existing storage for bridging,
as it doesn't know if the storage is valid, its size, or anything else. And
since Crubit does not have bindings for `std::optional` except via bridge types
and runtime conversion, this means that `Foo()` does not receive bindings.

## Workaround: Wrap the function {#workaround}

Whenever a type does not receive bindings, you can wrap the function with
something that does. For example:

```c++
#include <optional>

const std::optional<int>* Foo();

// A version of Foo() that returns a supported type.
//
// In this case, we flatten the pointer to optional, but we could also
// return a wrapper type (to distinguish between null vs nullopt).
inline const int* FooForRust() {
  const std::optional<int>* p = Foo();
  if (p != nullptr && p->has_value()) {
    return &**p;
  }
  return nullptr;
}
```

For more workarounds for unsupported types, see
crubit.rs/errors/unsupported_type.

For more information on bridging, see crubit.rs/cpp/best_practices#bridging.
