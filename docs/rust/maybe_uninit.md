# Rust's MaybeUninit type

Rust provides a type called
[`MaybeUninit<T>`](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html),
which represents a `T` which may be incompletely initialized, or even
totally uninitialized.  While a variable of type `T`, `&T` or `&mut T` must refer to a
fully initialized `T`, a variable of type `MaybeUninit<T>`, `&MaybeUninit<T>`, or
`&mut MaybeUninit<T>` may refer to an incompletely initialized and invalid `T`.
This allows working with uninitialized memory, even
though Rust otherwise requires initialization.

C++ is different: many types `T` can be uninitialized, and every
pointer or reference (`const T&`, `T&`, `const T*`, or `T*`) can point to
uninitialized memory.

Correspondingly, Rust references or pointers to `MaybeUninit<T>` are treated the
same as Rust references or pointer to `T`. In all other contexts, including when
passing or returning a `MaybeUninit<T>` by value, `MaybeUninit<T>` does not map to
any C++ type[^cpp_maybeuninit].

Rust                    | C++
----------------------- | ----------
`&MaybeUninit<T>`       | `const T&`
`&mut MaybeUninit<T>`   | `T&`
`*const MaybeUninit<T>` | `const T*`
`*mut MaybeUninit<T>`   | `T*`
`MaybeUninit<T>`        | no bindings

## Why only by pointer/reference?

Given C++'s take on uninitialized memory, one could draw the conclusion that it
would be fine to always represent a `MaybeUninit<T>` as a `T` on the C++ side
(as is done by
[cbindgen](https://github.com/mozilla/cbindgen/blob/822bde0/docs.md#std-types)).

However, this can quickly create UB. For instance, take the following Rust function:

```rust
fn foo() -> MaybeUninit<String> {
  MaybeUninit::uninit()
}
```

If we make this callable via something like `String foo()` in C++,
the behavior is undefined. For example, when the returned `String` is destroyed,
it would access uninitialized bytes as part of its destructor.

[^cpp_maybeuninit]: We investigated creating a C++ type that shadows behaviour
of `MaybeUninit` in C++, but concluded that this would only be properly feasible
with C++ 23 and only for a subset of types. (See b/362475441#comment3.)
