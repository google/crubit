# Bindings for function pointers

Here we describe how Crubit maps function pointer types.

## Rust bindings for C++ function pointer types

C++ function pointer types map into the corresponding Rust types as follows:

C++                                     | Rust
--------------------------------------- | ----------------------------
`std::type_identity_t<void(int32_t)> &` | `extern "C" fn(i32)`
`std::type_identity_t<void(int32_t)> *` | `Option<extern "C" fn(i32)>`
`std::type_identity_t<void(int32_t)>`   | Not supported

The bindings support function pointers with non-`"C"` calling conventions that
are supported by both `clang` and Rust - e.g. `"fastcall"`, `"vectorcall`,
`"thiscall"`, or `"stdcall"`.

If the function definition would be `unsafe`, then so is the function pointer --
for example, a C++ reference to `void(void*)` becomes a Rust `unsafe extern "C"
fn(_: *mut c_void)`.

The bindings fail for bindings that might require thunks (e.g. when parameter
types or a return type require passing structs by value).

## C++ bindings for Rust function pointer types

When used as function parameter types or function return types, Rust function
pointer types map into the corresponding C++ types as follows:

Rust                         | C++
---------------------------- | --------------------------------------
`extern "C" fn(i32)`         | `std::type_identity_t<void(int32_t)>&`
`Option<extern "C" fn(i32)>` | Not supported

In other scenarios Rust function pointer types map into the corresponding C++
types as follows:

Rust                         | C++
---------------------------- | --------------------------------------
`extern "C" fn(i32)`         | `std::type_identity_t<void(int32_t)>*`
`Option<extern "C" fn(i32)>` | Not supported

TODO: Link to the not-yet-written `references.md` with a longer explanation why
Rust references are sometimes bound to C++ references and sometimes to C++
pointers (C++ references can only be bound once; C++ temporaries).

## Other notes

The examples above use
[`std::type_identity_t` (C++20)](https://en.cppreference.com/w/cpp/types/type_identity)
to provide a more convenient syntax. Crubit doesn't actually require using
`std::type_identity_t` in the C++ APIs that it generates bindings for.

Note that C++ function pointers are nullable, but Rust function pointers are
not. (C++ function references are a separate, non-nullable type.)

There is no way to specify the lifetime of a function pointer in Rust, nor using
Crubit's lifetime annotations - both assume a `'static` lifetime. In scenarios
where the lifetime may be shorter than `'static` (e.g. JIT compilation, or
dynamic loading of plugins at runtime) the developer is responsible for managing
the lifetime of the function pointer.

The default calling convention of a Rust functions and function pointers is not
compatible with `"C"` ABI - directly using function pointers in FFI may require
explicitly specifying the ABI as follows: `extern "C" fn(i32, i32) -> i32`.

C++ also has plain
[function types](https://en.cppreference.com/w/cpp/types/is_function) (that is,
the type pointed to by function pointers). There is no Rust equivalent. However,
since C++ functions implicitly coerce to function pointers, this only comes up
in template classes like
[`std::function`](https://en.cppreference.com/w/cpp/utility/functional/function)
or
[`absl::AnyInvocable`](https://github.com/abseil/abseil-cpp/blob/master/absl/functional/any_invocable.h).

TODO: To support bindings of types like `std::function`, `absl::AnyInvocable`,
etc., Crubit may eventually need to provide a way to represent function types in
Rust using a custom marker type provided via `crubit/support/cc_std`.
