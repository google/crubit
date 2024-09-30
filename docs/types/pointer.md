# Pointer types

C++ defines two categories of pointer types, while Rust adds a third. They are:

*   Pointers to some (non-function) object, without lifetime information. C++
    calls these
    [object pointers](https://en.cppreference.com/w/cpp/language/pointer#Pointers_to_objects),
    while Rust calls them
    [raw pointers](https://doc.rust-lang.org/reference/types/pointer.html#raw-pointers-const-and-mut).
*   Function pointers
    ([C++](https://en.cppreference.com/w/cpp/language/pointer#Pointers_to_functions),
    [Rust](https://doc.rust-lang.org/reference/types/function-pointer.html)).
*   Finally, Rust references: non-aliasing pointers with lifetime information.

With the exception of [Rust references](#rust_references), which are only
permitted in limited circumstances, pointer types are fully supported as long as
the type they point to is supported. For example, `const int32_t*` maps
bidirectionally to `*const i32`, and `void (*)(int32_t)` maps bidirectionally to
`fn(i32)`.

## Object pointers {#object}

An "object pointer" is the C++ terminology for any pointer that is not a
function pointer. Rust would call these "raw pointers". These are mapped to each
other bidirectionally:

C++        | Rust
---------- | ----------
`const T*` | `*const T`
`T*`       | `*mut T`

### C++ pointers with lifetime {#object_lifetime}

C++ allows attaching lifetime annotations to arbitrary types, including
pointers. There are two competing annotations for this, neither of which are
supported in Rust bindings yet:

*   `[[clang::lifetimebound]]`
*   [Lifetime attributes](https://discourse.llvm.org/t/rfc-lifetime-annotations-for-c/61377)

## Function pointers {#function}

C++ function pointers map to Rust `extern "C" fn(...) -> ...` function pointers,
and vice versa:

C++                                   | Rust
------------------------------------- | -------------------------------
`void(&)(int32_t)>`                   | `extern "C" fn(i32)`
`void(*)(int32_t)`                    | `Option<extern "C" fn(i32)>`
`std::type_identity_t<void(int32_t)>` | Not supported [^function_types]

If the corresponding C++ function definition would be `unsafe` in Rust (per the
rules for [C++ function declarations](../cpp/functions#unsafe)), then so is the
function pointer – for example, a C++ reference to `void(void*)` becomes a Rust
`unsafe extern "C" fn(_: *mut c_void)`.

Not all function pointers receive bindings. If the function cannot be called
directly, due to a known or potential ABI mismatch between Rust and C++, then
the function pointer receives no bindings. In particular, function pointers
cannot take layout-compatible types by value. You can work around this by taking
or returning such problematic types by pointer instead of by value.

### Lifetime {#function_lifetime}

All function pointers are `'static`.

There is no way to specify the lifetime of a function pointer in Rust, nor in
C++: both assume a `'static` lifetime. In scenarios where the lifetime may be
shorter than `'static` (e.g., JIT compilation, or dynamic loading and unloading
of shared libraries at runtime), the developer is responsible for managing the
lifetime of the function pointer.

[^function_types]: C++ has plain
    [function types](https://en.cppreference.com/w/cpp/types/is_function):
    the type pointed to by function pointers. There is no Rust
    equivalent. However, since C++ functions implicitly coerce to
    function pointers, this only comes up in template classes
    like
    [`std::function`](https://en.cppreference.com/w/cpp/utility/functional/function)
    or
    [`absl::AnyInvocable`](https://github.com/abseil/abseil-cpp/blob/master/absl/functional/any_invocable.h).
    Or, in this case, `type_identity_t`.

## Rust references {#rust_references}

TODO(jeanpierreda): Move this to Rust function documentation, and link from here.

In general, Rust references are not exposed to C++. However, some Rust functions
which accept reference parameters do get mapped to C++ functions accepting a C++
reference:

*   All references must have an unbound parameter lifetime -- not `'static`, for
    example.
*   Only the parameter itself can be a reference type.
*   If there is a `mut` reference parameter, it is the **only** reference
    parameter.

This set of rules is intended to describe a safe subset of Rust functions, which
do not introduce substantial aliasing risk to mixed C++/Rust codebases.

```rust {.good}
fn foo(&self) {}
fn foo(_: &i32) {}
fn foo(_: &i32, _: &i32) {}
```

```rust {.bad}
fn foo(_: &'static i32) {}  // 'static lifetime is bound
fn foo(_: &&i32) {}  // Reference in non-parameter type
fn foo(_: &mut i32, _: &i32) {}  // More than one reference, one of which is mut
fn foo(_: &'a i32) {}  // 'a is bound to something else, not a parameter
```
