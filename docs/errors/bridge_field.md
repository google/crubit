<!-- <internal link> -->

# Bridge types as struct fields

## Overview

Crubit does not support exposing struct or union fields with bridge types.

Bridge types are types that are converted at runtime between C++ and Rust
(`std::optional` mapping to `Option`), but have different underlying
representations. (See crubit.rs/types)

If a field is a bridge type, this conversion is impossible: field accesses do
not run any conversion logic, and the field is required to have identical
representation across both languages. As a result, the field will not have the
bridge type conversion applied, but instead will be replaced by an opaque blob
of bytes.

## Example

Consider the following C++ struct:

```c++
#include <optional>
#include <cstdint>

struct Config {
  std::optional<int> foo;
};
```

Crubit will treat the `foo` field of `Config` as an opaque blob of bytes that
cannot be accessed directly from Rust.

## Workaround: Add getter and setter methods {#workaround}

To access or modify the field from Rust, you can add getter and setter methods.
Crubit *can* generate bindings for functions that accept or return bridge types
by value.

For example:

```c++
#include <optional>

struct Config {
  std::optional<int> foo;

  // Getter returning by value (mapped to Option<i32> in Rust)
  std::optional<int> get_foo() const { return foo; }

  // Setter accepting by value
  void set_foo(std::optional<int> val) { foo = val; }
};
```

Then in Rust, you can use these methods:

```rust
let mut config = Config::default();
config.set_foo(Some(42));
assert_eq!(config.get_foo(), Some(42));
```

See crubit.rs/cpp/best_practices#bridging for more information.
