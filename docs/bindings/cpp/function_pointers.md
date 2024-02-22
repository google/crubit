# Rust bindings for C++ function pointer types

C++ function pointer map to Rust `extern "C" fn(...) -> ...` types:

| C++                                   | Rust                            |
| ------------------------------------- | ------------------------------- |
| `std::type_identity_t<void(int32_t)>  | `extern "C" fn(i32)`            |
: &`[^type_identity]                    :                                 :
| `std::type_identity_t<void(int32_t)>  | `Option<extern "C" fn(i32)>`    |
: *`                                    :                                 :
| `std::type_identity_t<void(int32_t)>` | Not supported [^function_types] |

If the function definition would be `unsafe`, then so is the function pointer --
for example, a C++ reference to `void(void*)` becomes a Rust `unsafe extern "C"
fn(_: *mut c_void)`.

Not all function pointers receive bindings. If Rust cannot call the function
directly, due to known or potential ABI mismatch between Rust and C++, then the
function pointer receives no bindings.

In particular, function pointers currently cannot take structs by value.

This restriction will be relaxed over time, as more and more parts of the ABI
are successfully translated to Rust. It can be worked around by taking or
returning such problematic types by pointer instead of by value.

## Lifetime {#lifetime}

All function pointers are `'static`.

There is no way to specify the lifetime of a function pointer in Rust, nor in
C++: both assume a `'static` lifetime. In scenarios where the lifetime may be
shorter than `'static` (e.g. JIT compilation, or dynamic loading and unloading
of shared libraries at runtime) the developer is responsible for managing the
lifetime of the function pointer.

[^type_identity]: The examples use
    [`std::type_identity_t` (C++20)](https://en.cppreference.com/w/cpp/types/type_identity)
    to provide a more convenient syntax. Crubit doesn't actually
    require using `std::type_identity_t` in the C++ APIs that it
    generates bindings for.
[^function_types]: C++ has plain
    [function types](https://en.cppreference.com/w/cpp/types/is_function):
    the type pointed to by function pointers. There is no Rust
    equivalent. However, since C++ functions implicitly coerce to
    function pointers, this only comes up in template classes
    like
    [`std::function`](https://en.cppreference.com/w/cpp/utility/functional/function)
    or
    [`absl::AnyInvocable`](https://github.com/abseil/abseil-cpp/blob/master/absl/functional/any_invocable.h).
