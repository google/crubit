# Rust bindings for C++ functions

Rust code can call (non-member) functions defined in C++, provided that the
parameter and return types are supported by Crubit:

*   If a parameter or return type is a [fundamental type](fundamental_types),
    then the bindings for the function use the corresponding Rust type.
*   Similarly, if a parameter or return type is a [pointer type](pointer_types),
    then the bindings for the function use the corresponding Rust pointer type.
*   If the type is a user-defined type, such as a
    [class type](classes_and_structs) or [enum](enums), then the bindings for
    the function use the bindings for that type.

## Example

Given the following C++ header:

```live-snippet
cs/file:examples/cpp/function/example.h content:^([^/#\n])[^\n]*
```

Crubit will generate the following bindings, containing a safe public function
and the corresponding FFI glue:

```live-snippet
cs/file:examples/cpp/function/example_generated.rs content:^([^/\n])([^!\n]|$)[^\n]*
```

## `unsafe` functions

Functions accepting or returning simple types, like integers, can be called from
safe code. However, functions which accept a raw pointer are automatically
marked **`unsafe`**, and cannot be called outside of an `unsafe` block. To
ensure that the behavior is defined, callers must:

*   Ensure that the pointer being passed to C++ is a valid C++ pointer. In
    particular, it must not be dangling (e.g. `Nonnull::dangling()`).

*   Ensure that the safety conditions documented in C++ are upheld. For example,
    if the C++ function accepts a reference or non-null pointer, then do not
    pass in `0 as *const _`.

However, even "safe" C++ functions are still potentially dangerous, and can
still have undefined behavior when called. Callers must still adhere to all
documented function preconditions.

## Function Attributes

Function attributes are **not currently supported**. Functions marked
`[[noreturn]]`, `[[nodiscard]]`, etc. do not have bindings.
