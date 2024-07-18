# C++ bindings for `use` statement

Crubit supports `use` statement for functions and user-defined types. For the
following Rust code:

```rust
pub use pub_module::pub_function;
pub use pub_module::pub_struct;
pub use pub_module::pub_struct2 as pub_struct3;
```

Crubit will generate the following bindings:

```cpp
using ::pub_module::pub_function;
using ::pub_module::pub_struct;
using pub_struct3 = ::pub_module::pub_struct2;
```

Currently, Crubit doesn't support the following cases:

### `use` with multiple items or glob

```rust
pub use some_module::{a, b}; // Not supported yet.
pub use some_module::*; // Not supported yet.`
```

### `use` a module

```rust
pub use some_module; // Not supported yet.
```

### `use` a non-directly public item

```rust
mod private_mod {
  pub fn func() {}
}

pub use private_mod::func; // Not supported yet. `func` is not directly public
                           // because `private_mod` is private.
```
