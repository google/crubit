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
rules for [C++ function declarations](../cpp/functions.md#unsafe)), then so is
the function pointer – for example, a C++ reference to `void(void*)` becomes a
Rust `unsafe extern "C" fn(_: *mut c_void)`.

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

Rust references, unlike C++ references, cannot mutably alias. This introduces a
new form of Undefined Behavior (UB) that many C++ programmers may not be
accustomed to. For now, C++ pointers and references do **not** map to Rust
references. Instead, they map to Rust raw pointers. Vice versa, Rust references
are an unsupported type which do not map to any C++ type at all.

The one exception to this rule are function parameters. In some limited
circumstances, Rust functions may accept references, and the corresponding C++
interface will accept C++ references. This is documented in
<internal link>/rust/functions.
