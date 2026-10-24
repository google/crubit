# Compound types containing bridge types

## Overview

Crubit does not support compound data types containing bridge types, unless the
compound data type itself can be bridged.

Bridge types are types that are converted at runtime between C++ and Rust, but
have different underlying representations. Examples include `absl::AnyInvocable`
(mapped to `Box<dyn Fn...>`) and types annotated with `CRUBIT_BRIDGE`. (See
crubit.rs/types.)

This conversion is only possible when the bridge type is used by value. It is
not possible in fields (crubit.rs/errors/bridge_field), inside of a pointer, or
more generally inside of a compound data type that doesn't support recursively
bridging its interior (such as `std::vector`).

## Example

The examples below use `either::Either<L, R>`, a C++ struct that is bridged to a
Rust `enum Either<L, R>`. It is defined in
[examples/types/bridging](https://github.com/google/crubit/tree/main/examples/types/bridging/).

Consider the following C++ function:

```c++
#include "examples/types/bridging/either.h"

const either::Either<int, bool>* Foo();
```

While Crubit can bridge an `either::Either<int, bool>` return value, it cannot
bridge it when it is behind a pointer. It cannot reuse the existing storage for
bridging, as it doesn't know if the storage is valid, its size, or anything
else. And since Crubit does not have bindings for `either::Either` except via
bridge types and runtime conversion, this means that `Foo()` does not receive
bindings.

## Workaround: Wrap the function {#workaround}

Whenever a type does not receive bindings, you can wrap the function with
something that does. For example:

```c++
#include "examples/types/bridging/either.h"

const either::Either<int, bool>* Foo();

// A version of Foo() that returns a copy of the value instead of a pointer.
// Bridge types can be returned by value. This assumes Foo() never returns
// null.
inline either::Either<int, bool> FooForRust() { return *Foo(); }
```

For more workarounds for unsupported types, see
crubit.rs/errors/unsupported_type.

For more information on bridging, see crubit.rs/cpp/best_practices#bridging.
