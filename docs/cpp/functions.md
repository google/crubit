# Rust bindings for C++ functions

Rust code can call (non-member) functions defined in C++, provided that the
parameter and return types are supported by Crubit:

*   If a parameter or return type is a [primitive type](../types/primitive),
    then the bindings for the function use the corresponding Rust type.
*   Similarly, if a parameter or return type is a
    [pointer type](../types/pointer), then the bindings for the function use the
    corresponding Rust pointer type.
*   If the type is a user-defined type, such as a
    [class type](classes_and_structs) or [enum](enums), then the bindings for
    the function use the bindings for that type.

Additionally, code can call member functions defined in C++ if the parameter and
return types are supported by Crubit (see above). Currently, member functions
are translated as non-method associated functions.

## Examples

### Functions

Given the following C++ header:

```live-snippet
cs/file:examples/cpp/function/example.h function:add_two_integers
```

Crubit will generate the following bindings, with a safe public function
that calls into the corresponding FFI glue:

```live-snippet
cs/file:examples/cpp/function/example_generated.rs function:add_two_integers
```

### Methods

Given the following C++ header:

```live-snippet
cs/file:examples/cpp/method/example.h class:Bar
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/cpp/method/example_generated.rs class:Bar
```

```live-snippet
cs/file:examples/cpp/method/example_generated.rs snippet:0,6 "impl Bar"
```

### `unsafe` functions {#unsafe}

#### Which C++ functions are marked `unsafe` in Rust? {#unsafe-inference}

By default, the Rust binding to a C++ function is marked as safe or `unsafe`
based on the types of its parameters. If a C++ function accepts only simple
types like integers, the resulting Rust binding will be marked as safe.
Functions which accept a raw pointer are automatically marked as `unsafe`.

This behavior can be overridden using the `CRUBIT_UNSAFE`,
`CRUBIT_UNSAFE_MARK_SAFE` and `CRUBIT_OVERRIDE_UNSAFE(is_unsafe)` macros.

For example, given the following C++ header:

```live-snippet
cs/file:examples/cpp/unsafe_attributes/example.h content:^([^/#\n])[^\n]*
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/cpp/unsafe_attributes/example_generated.rs content:^([^/\n])([^!\n]|$)[^\n]*
```

#### Correct usage of `unsafe` {#using-unsafe}

Functions marked **`unsafe`** cannot be called outside of an `unsafe` block. In
order to avoid undefined behavior when using `unsafe`, callers must:

*   Ensure that the pointer being passed to C++ is a valid C++ pointer. In
    particular, it must not be dangling (e.g. `Nonnull::dangling()`).

*   Ensure that the safety conditions documented in C++ are upheld. For example,
    if the C++ function accepts a reference or non-null pointer, then do not
    pass in `0 as *const _`.

#### Soundness

Note that many "safe" C++ functions may still trigger undefined behavior if used
incorrectly. Regardless of whether a C++ function is marked as `unsafe`, calls
into C++ will only be memory-safe if the caller verifies that all function
preconditions are met.

## Function Attributes

Function attributes are **not currently supported**. Functions marked
`[[noreturn]]`, `[[nodiscard]]`, etc. do not have bindings.
