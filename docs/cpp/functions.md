# Rust bindings for C++ functions

<internal link>/cpp/functions

Rust code can call `extern "C"` functions defined in C++, provided that the
parameter and return types are supported by Crubit.

## Example

Given the following C++ header:

```live-snippet
cs/file:examples/cpp/function/example.h content:^([^/#\n])[^\n]*
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/cpp/function/example_generated.rs content:^([^/\n])([^!\n]|$)[^\n]*
```

## `unsafe` functions

Functions accepting simple types, like integers, are (nominally) safe. However,
functions which accept a raw pointer are automatically marked **`unsafe`**. To
ensure that the behavior is defined, one must:

1.  Ensure that the pointer being passed to C++ is a valid C++ pointer. In
    particular, it must not be dangling (e.g. `Nonnull::dangling()`).

2.  Ensure that the safety conditions documented in C++ are upheld. For example,
    if the C++ function accepts a reference or non-null pointer, then do not
    pass in `0 as *const _`.

However, even "safe" C++ functions are still dangerous, and can still have
undefined behavior when called from C++. You must still adhere to all documented
function preconditions.

## Function Attributes

Function attributes are **not currently supported**. Functions marked
`[[noreturn]]`, `[[nodiscard]]`, etc. will not receive bindings.
