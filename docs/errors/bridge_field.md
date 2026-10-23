# Bridge types as struct fields

## Overview

Crubit does not support exposing struct or union fields with bridge types.

Bridge types are types that are converted at runtime between C++ and Rust, but
have different underlying representations. Examples include `absl::AnyInvocable`
(mapped to `Box<dyn Fn...>`) and types annotated with `CRUBIT_BRIDGE`. (See
crubit.rs/types)

If a field is a bridge type, this conversion is impossible: field accesses do
not run any conversion logic, and the field is required to have identical
representation across both languages. As a result, the field will not have the
bridge type conversion applied, but instead will be replaced by an opaque blob
of bytes.

## Example

The examples below use `either::Either<L, R>`, a C++ struct that is bridged to a
Rust `enum Either<L, R>`. It is defined in
[examples/types/bridging](https://github.com/google/crubit/tree/main/examples/types/bridging/).

Consider the following C++ struct:

```c++
#include "examples/types/bridging/either.h"

struct Config {
  either::Either<int, bool> value;
};
```

Crubit will treat the `value` field of `Config` as an opaque blob of bytes that
cannot be accessed directly from Rust.

## Workaround: Add getter and setter methods {#workaround}

To access or modify the field from Rust, you can add getter and setter methods.
Crubit *can* generate bindings for functions that accept or return bridge types
by value.

For example:

```c++
#include "examples/types/bridging/either.h"

struct Config {
  either::Either<int, bool> value;

  // Getter returning by value (mapped to `Either<i32, bool>` in Rust).
  either::Either<int, bool> get_value() const { return value; }

  // Setter accepting by value.
  void set_value(either::Either<int, bool> v) { value = v; }
};
```

Then in Rust, you can use these methods:

```rust
use either::either::Either;

let mut config = Config::default();
config.set_value(Either::Left(42));
assert_eq!(config.get_value(), Either::Left(42));
```

See crubit.rs/cpp/best_practices#bridging for more information.
